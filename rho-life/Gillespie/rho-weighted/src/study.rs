//! Studies.
//!
//! The unit of work. A study is *(theory, initial term, parameter grid,
//! observables, budget)* and it produces graphs. This is the shape the whole
//! crate exists to serve, and it is why the simulator is a separate execution
//! path: the interpreter answers *what is the state*, and a study answers *what
//! would happen across this family of configurations*.
//!
//! Every result carries provenance (DR-W10). A graph is a scientific artifact;
//! one that cannot be traced to the theory, seed and budget that produced it —
//! or that was truncated without saying so — is worse than no graph.

use std::collections::BTreeMap;

use crate::explore::{exhaustive_graph, Ensemble, Exploration, Stop};
use crate::matching::Matching;
use crate::theory::{Configuration, RateValue, WeightMap, WeightedTheory};

pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone, Debug)]
pub struct Provenance {
    pub theory_fingerprint: String,
    pub initial_marking: String,
    pub seed: u64,
    pub max_steps: usize,
    pub max_states: usize,
    pub horizon: f64,
    pub crate_version: String,
    pub stop: Stop,
}

impl Provenance {
    pub fn budget_exhausted(&self) -> bool {
        self.stop.is_truncation()
    }

    pub fn to_json(&self) -> String {
        format!(
            "{{\"theory\":\"{}\",\"initial\":\"{}\",\"seed\":{},\"max_steps\":{},\
             \"max_states\":{},\"horizon\":{},\"version\":\"{}\",\"stop\":\"{:?}\",\
             \"truncated\":{}}}",
            esc(&self.theory_fingerprint),
            esc(&self.initial_marking),
            self.seed,
            self.max_steps,
            self.max_states,
            // JSON has no infinity; an unbounded horizon is `null`.
            if self.horizon.is_finite() {
                format!("{}", self.horizon)
            } else {
                "null".to_string()
            },
            self.crate_version,
            self.stop,
            self.budget_exhausted()
        )
    }
}

/// One point of a parameter grid: an override of some weight-map entries.
#[derive(Clone, Debug)]
pub struct GridPoint {
    pub label: String,
    pub overrides: Vec<((crate::redex::RuleId, usize), f64)>,
}

impl GridPoint {
    pub fn apply(&self, base: &WeightMap) -> WeightMap {
        let mut w = base.clone();
        for (k, v) in &self.overrides {
            w.set(*k, RateValue::Real(*v));
        }
        w
    }
}

/// A one-dimensional sweep of a single weight entry.
pub fn sweep_1d(
    key: (crate::redex::RuleId, usize),
    name: &str,
    values: &[f64],
) -> Vec<GridPoint> {
    values
        .iter()
        .map(|v| GridPoint {
            label: format!("{name}={v}"),
            overrides: vec![(key, *v)],
        })
        .collect()
}

#[derive(Clone, Debug)]
pub struct PointResult {
    pub label: String,
    pub provenance: Provenance,
    pub states: usize,
    pub edges: usize,
    /// Stationary distribution over markings, when the exhaustive path was used
    /// and the chain admits one.
    pub stationary: Option<BTreeMap<String, f64>>,
    /// Empirical occupancy from the sampled ensemble, when one was run.
    pub empirical: Option<BTreeMap<String, f64>>,
    pub dot: String,
}

pub struct StudyResult {
    pub points: Vec<PointResult>,
}

impl StudyResult {
    pub fn to_json(&self) -> String {
        let mut s = String::from("{\"points\":[");
        for (i, p) in self.points.iter().enumerate() {
            if i > 0 {
                s.push(',');
            }
            s.push_str(&format!(
                "{{\"label\":\"{}\",\"states\":{},\"edges\":{},\"provenance\":{}",
                esc(&p.label),
                p.states,
                p.edges,
                p.provenance.to_json()
            ));
            if let Some(st) = &p.stationary {
                s.push_str(",\"stationary\":{");
                for (j, (k, v)) in st.iter().enumerate() {
                    if j > 0 {
                        s.push(',');
                    }
                    s.push_str(&format!("\"{}\":{:.10}", esc(k), v));
                }
                s.push('}');
            }
            if let Some(em) = &p.empirical {
                s.push_str(",\"empirical\":{");
                for (j, (k, v)) in em.iter().enumerate() {
                    if j > 0 {
                        s.push(',');
                    }
                    s.push_str(&format!("\"{}\":{:.10}", esc(k), v));
                }
                s.push('}');
            }
            s.push('}');
        }
        s.push_str("]}");
        s
    }

    /// Any truncated point makes the whole study provisional.
    pub fn any_truncated(&self) -> bool {
        self.points.iter().any(|p| p.provenance.budget_exhausted())
    }
}

pub struct Study<'a, M: Matching> {
    pub theory: &'a WeightedTheory,
    pub matcher: &'a M,
    pub initial: Configuration,
    pub grid: Vec<GridPoint>,
    pub seed: u64,
    pub max_steps: usize,
    pub max_states: usize,
    pub horizon: f64,
    pub ensemble_size: usize,
}

impl<'a, M: Matching> Study<'a, M> {
    pub fn new(
        theory: &'a WeightedTheory,
        matcher: &'a M,
        initial: Configuration,
    ) -> Study<'a, M> {
        Study {
            theory,
            matcher,
            initial,
            grid: vec![GridPoint {
                label: "base".into(),
                overrides: vec![],
            }],
            seed: 0xC0FFEE,
            max_steps: 10_000,
            max_states: 5_000,
            horizon: f64::INFINITY,
            ensemble_size: 0,
        }
    }

    pub fn with_grid(mut self, grid: Vec<GridPoint>) -> Self {
        self.grid = grid;
        self
    }
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }
    pub fn with_ensemble(mut self, n: usize, horizon: f64) -> Self {
        self.ensemble_size = n;
        self.horizon = horizon;
        self
    }
    pub fn with_state_budget(mut self, n: usize) -> Self {
        self.max_states = n;
        self
    }

    pub fn run(&self) -> StudyResult {
        let base = self.initial.weights.clone();
        let mut points = Vec::new();
        for gp in &self.grid {
            let cfg = Configuration {
                space: self.initial.space.clone(),
                weights: gp.apply(&base),
                trace: self.initial.trace.clone(),
            };

            let ex: Exploration =
                exhaustive_graph(self.theory, &cfg, self.matcher, self.max_states);

            let stationary = ex.generator.stationary(20_000, 1e-12).map(|pi| {
                let mut out: BTreeMap<String, f64> = BTreeMap::new();
                for (i, p) in pi.iter().enumerate() {
                    *out.entry(ex.graph.node_labels[i].clone()).or_insert(0.0) += p;
                }
                out
            });

            let empirical = if self.ensemble_size > 0 {
                let e = Ensemble::run(
                    self.theory,
                    self.matcher,
                    &cfg,
                    self.seed,
                    self.ensemble_size,
                    self.max_steps,
                    self.horizon,
                );
                Some(e.mean_occupancy())
            } else {
                None
            };

            points.push(PointResult {
                label: gp.label.clone(),
                provenance: Provenance {
                    theory_fingerprint: self.theory.fingerprint(),
                    initial_marking: cfg.space.marking().key(),
                    seed: self.seed,
                    max_steps: self.max_steps,
                    max_states: self.max_states,
                    horizon: self.horizon,
                    crate_version: CRATE_VERSION.to_string(),
                    stop: ex.stop.clone(),
                },
                states: ex.graph.node_count(),
                edges: ex.graph.edge_count(),
                stationary,
                empirical,
                dot: ex.graph.to_dot(&gp.label),
            });
        }
        StudyResult { points }
    }
}

fn esc(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}
