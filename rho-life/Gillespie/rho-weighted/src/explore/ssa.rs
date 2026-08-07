//! The stochastic simulation algorithm, over configurations.
//!
//! Gillespie's direct method verbatim, with one addition: after firing, the
//! selected refinement's update function transforms the weight map. Because the
//! map is part of the configuration, the process remains a time-homogeneous
//! CTMC — on `(space, map)` pairs, not on terms. Were the map mutable but
//! outside the state, the process on terms would be non-Markovian, there would
//! be no exact simulation algorithm, and no CTMC to model check.

use crate::explore::Stop;
use crate::logic::Budget;
use crate::matching::Matching;
use crate::propensity::{propensities, Propensities};
use crate::rng::Rng;
use crate::theory::{Configuration, UpdateCtx, WeightedTheory};

#[derive(Clone, Debug)]
pub struct SimStep {
    pub time: f64,
    pub tau: f64,
    pub rule: crate::redex::RuleId,
    pub class: usize,
    pub position: String,
    pub total_propensity: f64,
    /// The configuration *after* the step.
    pub marking_key: String,
    pub weights_fingerprint: String,
}

#[derive(Clone, Debug)]
pub struct Trace {
    pub steps: Vec<SimStep>,
    pub final_config_key: String,
    pub end_time: f64,
    pub stop: Stop,
}

impl Trace {
    pub fn len(&self) -> usize {
        self.steps.len()
    }
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }

    /// Time-weighted occupancy of each marking, i.e. the empirical measure the
    /// stationary distribution should be compared against.
    pub fn occupancy(&self) -> std::collections::BTreeMap<String, f64> {
        let mut out: std::collections::BTreeMap<String, f64> = Default::default();
        for w in self.steps.windows(2) {
            let dt = w[1].time - w[0].time;
            *out.entry(w[0].marking_key.clone()).or_insert(0.0) += dt;
        }
        out
    }

    /// The sampled waiting times, for the exponentiality check.
    pub fn taus(&self) -> Vec<f64> {
        self.steps.iter().map(|s| s.tau).collect()
    }
}

pub struct Sim<'a, M: Matching> {
    pub theory: &'a WeightedTheory,
    pub matcher: &'a M,
    pub config: Configuration,
    pub rng: Rng,
    pub time: f64,
    pub budget: Budget,
}

impl<'a, M: Matching> Sim<'a, M> {
    pub fn new(
        theory: &'a WeightedTheory,
        matcher: &'a M,
        config: Configuration,
        seed: u64,
    ) -> Sim<'a, M> {
        Sim {
            theory,
            matcher,
            config,
            rng: Rng::seeded(seed),
            time: 0.0,
            budget: Budget::default(),
        }
    }

    pub fn propensities(&mut self) -> Propensities {
        propensities(self.theory, &self.config, self.matcher, &mut self.budget)
    }

    /// One step of the direct method. `None` at an absorbing configuration.
    pub fn step(&mut self) -> Option<SimStep> {
        let props = self.propensities();
        if props.is_absorbing() {
            return None;
        }
        let a0 = props.total;

        // Waiting time: exponential with parameter a0.
        let u1 = self.rng.unit();
        let tau = (1.0 / a0) * (1.0 / u1).ln();

        // Select a redex with probability proportional to its own rate. Note
        // that selecting the redex directly, rather than selecting a class and
        // then a position within it, is equivalent by Prop. 20 and avoids a
        // second normalisation.
        let u2 = self.rng.unit() * a0;
        let mut acc = 0.0;
        let mut chosen = None;
        for w in &props.items {
            acc += w.rate;
            if acc >= u2 {
                chosen = Some(w.clone());
                break;
            }
        }
        let w = chosen.or_else(|| props.items.iter().rev().find(|w| w.rate > 0.0).cloned())?;

        // Fire.
        let next_space = self.config.space.fire(&w.redex);
        let key = (w.redex.rule, w.class);
        self.time += tau;

        // Update the weight map. Inference and learning are the same
        // transition relation, differing only in which component of the state
        // they touch (note, Theorem 47).
        let position = w.redex.position();
        let ctx = UpdateCtx {
            key,
            bindings: &w.redex.bindings,
            position: &position,
            time: self.time,
            trace: &self.config.trace,
        };
        let rule = self.theory.rule(w.redex.rule)?;
        let new_weights = (rule.entries[w.class].update)(&self.config.weights, &ctx);

        let mut trace = self.config.trace.clone();
        trace.last_fired.insert(key, self.time);
        *trace.eligibility.entry(key).or_insert(0.0) += 1.0;

        self.config = Configuration {
            space: next_space,
            weights: new_weights,
            trace,
        };

        Some(SimStep {
            time: self.time,
            tau,
            rule: w.redex.rule,
            class: w.class,
            position,
            total_propensity: a0,
            marking_key: self.config.space.marking().key(),
            weights_fingerprint: self.config.weights.fingerprint(),
        })
    }

    pub fn run(&mut self, max_steps: usize, horizon: f64) -> Trace {
        let mut steps = Vec::new();
        // Seed the trace with the initial configuration so that occupancy has a
        // first interval.
        steps.push(SimStep {
            time: 0.0,
            tau: 0.0,
            rule: crate::redex::COMM,
            class: usize::MAX,
            position: "init".into(),
            total_propensity: 0.0,
            marking_key: self.config.space.marking().key(),
            weights_fingerprint: self.config.weights.fingerprint(),
        });
        let stop = loop {
            if steps.len() > max_steps {
                break Stop::StepBudget;
            }
            if self.time >= horizon {
                break Stop::TimeHorizon;
            }
            match self.step() {
                Some(s) => steps.push(s),
                None => break Stop::Absorbing,
            }
        };
        Trace {
            final_config_key: self.config.key(),
            end_time: self.time,
            steps,
            stop,
        }
    }
}
