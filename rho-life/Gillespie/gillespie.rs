//! # Gillespie Simulation Engine
//!
//! Implements the Stochastic Simulation Algorithm (SSA) over the augmented
//! rewrite system with base rules (weight + refinement update functions)
//! and context rules (weight + fold functions).
//!
//! ## Algorithm
//!
//! 1. **Compute propensities**: For each base rule `r_i`, the propensity is
//!    `a_i = rule_weight_i × Σ_j refinement_weight_{ij}`.
//!    For context rules, propensity = rule weight (context is applied after).
//!
//! 2. **Total propensity**: `a_0 = Σ_i a_i`
//!
//! 3. **Sample waiting time**: `τ = (1/a_0) × ln(1/r_1)`, `r_1 ~ U(0,1)`
//!
//! 4. **Select rule**: Weighted by propensities.
//!
//! 5. **If base rule**: Select refinement by its weight, then apply the
//!    refinement's **update function** to the current annotation map.
//!
//! 6. **If context rule**: Apply the context rule's **fold function** to
//!    combine context annotation maps with the updated map from the
//!    inner derivation.
//!
//! 7. **Advance time**: `t ← t + τ`

use crate::augmented_rule::{BaseRule, ContextRule, RewriteSystem, Rule, TermRef};
use crate::rate_map::RateMap;
use rand::Rng;

/// The result of firing a base rule: which refinement was chosen,
/// and the updated annotation map produced by the update function.
#[derive(Debug, Clone)]
pub struct BaseRuleResult {
    /// Index of the selected refinement within the base rule.
    pub refinement_index: usize,
    /// The annotation map after applying the update function.
    pub updated_map: RateMap,
}

/// The result of applying a context rule: the folded annotation map.
#[derive(Debug, Clone)]
pub struct ContextRuleResult {
    /// The annotation map after applying the fold function.
    pub folded_map: RateMap,
}

/// A single step in the simulation trace.
#[derive(Debug, Clone)]
pub struct SimStep {
    /// The simulation time at this step.
    pub time: f64,
    /// The waiting time before this step.
    pub tau: f64,
    /// Index of the rule that fired (in the rewrite system).
    pub rule_index: usize,
    /// Name of the rule that fired.
    pub rule_name: String,
    /// The resulting term.
    pub result_term: TermRef,
    /// The annotation map after this step.
    pub result_map: RateMap,
    /// Details of how the map was produced.
    pub detail: StepDetail,
}

/// How the annotation map was produced at a given step.
#[derive(Debug, Clone)]
pub enum StepDetail {
    /// A base rule fired: a refinement was selected and its update applied.
    BaseRuleFired {
        refinement_index: usize,
        refinement_weight: f64,
    },
    /// A context rule fired: context maps were folded with the inner map.
    ContextRuleFired {
        num_context_maps: usize,
    },
}

/// The Gillespie simulator.
pub struct Simulator {
    /// Current simulation time.
    pub time: f64,
    /// Current term.
    pub current_term: TermRef,
    /// Current annotation map.
    pub current_map: RateMap,
    /// The rewrite system.
    pub system: RewriteSystem,
    /// Context annotation maps (from surrounding sub-terms).
    /// These are consumed by context rules' fold functions.
    pub context_maps: Vec<RateMap>,
    /// RNG.
    rng: rand::rngs::ThreadRng,
}

impl Simulator {
    /// Create a new simulator.
    pub fn new(
        initial_term: TermRef,
        initial_map: RateMap,
        system: RewriteSystem,
    ) -> Self {
        Simulator {
            time: 0.0,
            current_term: initial_term,
            current_map: initial_map,
            system,
            context_maps: Vec::new(),
            rng: rand::thread_rng(),
        }
    }

    /// Set context maps (annotation maps from surrounding sub-terms).
    pub fn with_context_maps(mut self, maps: Vec<RateMap>) -> Self {
        self.context_maps = maps;
        self
    }

    /// Compute propensities for all applicable rules.
    fn compute_propensities(&self) -> Vec<(usize, f64)> {
        self.system
            .rules
            .iter()
            .enumerate()
            .filter(|(_, rule)| rule.lhs_sort() == self.current_term.sort)
            .map(|(i, rule)| {
                let prop = match rule {
                    Rule::Base(br) => br.propensity(),
                    Rule::Context(cr) => cr.weight,
                };
                (i, prop)
            })
            .filter(|(_, p)| *p > 1e-15)
            .collect()
    }

    /// Execute one step of the Gillespie SSA.
    pub fn step(&mut self) -> Option<SimStep> {
        let propensities = self.compute_propensities();
        if propensities.is_empty() {
            return None;
        }

        // Total propensity
        let a0: f64 = propensities.iter().map(|(_, p)| p).sum();
        if a0 < 1e-15 {
            return None;
        }

        // Sample waiting time
        let r1: f64 = self.rng.gen();
        let tau = (1.0 / a0) * (1.0 / r1).ln();

        // Select which rule fires
        let r2: f64 = self.rng.gen();
        let threshold = r2 * a0;
        let mut cumulative = 0.0;
        let mut selected_index = propensities[0].0;
        for (idx, prop) in &propensities {
            cumulative += prop;
            if cumulative >= threshold {
                selected_index = *idx;
                break;
            }
        }

        // Extract what we need from the rule before mutable borrow
        let rule_name = self.system.rules[selected_index].name().to_string();
        let rule_clone = self.system.rules[selected_index].clone();
        let (result_term, result_map, detail) = match &rule_clone {
            Rule::Base(br) => self.fire_base_rule(br),
            Rule::Context(cr) => self.fire_context_rule(cr),
        };

        self.time += tau;

        let step = SimStep {
            time: self.time,
            tau,
            rule_index: selected_index,
            rule_name,
            result_term: result_term.clone(),
            result_map: result_map.clone(),
            detail,
        };

        // Update state
        self.current_term = result_term;
        self.current_map = result_map;

        Some(step)
    }

    /// Fire a base rule: select a refinement and apply its update function.
    fn fire_base_rule(&mut self, rule: &BaseRule) -> (TermRef, RateMap, StepDetail) {
        let r: f64 = self.rng.gen();
        let refinement_idx = rule.select_refinement(r).unwrap_or(0);
        let refinement = &rule.refinements[refinement_idx];

        // Apply the update function to the current annotation map
        let updated_map = refinement.apply_update(&self.current_map);

        let detail = StepDetail::BaseRuleFired {
            refinement_index: refinement_idx,
            refinement_weight: refinement.weight,
        };

        (rule.rhs.clone(), updated_map, detail)
    }

    /// Fire a context rule: apply its fold function to combine context maps
    /// with the current (updated) annotation map.
    fn fire_context_rule(
        &mut self,
        rule: &ContextRule,
    ) -> (TermRef, RateMap, StepDetail) {
        let ctx_refs: Vec<&RateMap> = self.context_maps.iter().collect();
        let folded_map = rule.apply_fold(&ctx_refs, &self.current_map);

        let detail = StepDetail::ContextRuleFired {
            num_context_maps: self.context_maps.len(),
        };

        (rule.rhs.clone(), folded_map, detail)
    }

    /// Run for up to `max_steps` steps.
    pub fn run(&mut self, max_steps: usize) -> Vec<SimStep> {
        let mut trace = Vec::new();
        for _ in 0..max_steps {
            match self.step() {
                Some(step) => trace.push(step),
                None => break,
            }
        }
        trace
    }

    /// Run until a time limit.
    pub fn run_until(&mut self, max_time: f64, max_steps: usize) -> Vec<SimStep> {
        let mut trace = Vec::new();
        for _ in 0..max_steps {
            match self.step() {
                Some(step) => {
                    if step.time > max_time {
                        self.time -= step.tau; // undo overshoot
                        break;
                    }
                    trace.push(step);
                }
                None => break,
            }
        }
        trace
    }
}

/// Display a simulation trace.
pub fn print_trace(trace: &[SimStep]) {
    for step in trace {
        let detail_str = match &step.detail {
            StepDetail::BaseRuleFired {
                refinement_index,
                refinement_weight,
            } => format!(
                "refinement[{}] w={:.4}",
                refinement_index, refinement_weight
            ),
            StepDetail::ContextRuleFired { num_context_maps } => {
                format!("fold over {} context map(s)", num_context_maps)
            }
        };
        println!(
            "  t={:.6} τ={:.6} [{}] {} → {} | map: {}",
            step.time, step.tau, step.rule_name, detail_str, step.result_term, step.result_map
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::augmented_rule::RefinementEntry;
    use crate::rate_value::RateValue;
    use crate::spatial_behavior::SpatialBehavior;

    fn make_test_system() -> (TermRef, RateMap, RewriteSystem) {
        let initial_term = TermRef::new(1, "Proc", "x?(y).P | x!(Q)");

        let mut initial_map = RateMap::new();
        initial_map.insert(
            SpatialBehavior::interaction("x", "x"),
            RateValue::real(0.8).unwrap(),
        );

        let system = RewriteSystem::new().add_base_rule(
            BaseRule::new(
                "COMM",
                TermRef::new(1, "Proc", "x?(y).P | x!(Q)"),
                TermRef::new(2, "Proc", "P{Q/y}"),
                0.7,
            )
            .add_refinement(RefinementEntry::new(
                SpatialBehavior::interaction("x", "x"),
                0.5,
                |map| {
                    // Scale all rates by 0.9 after communication
                    let mut new_map = RateMap::new();
                    for (sb, rv) in map.entries() {
                        let scaled = match rv {
                            RateValue::Real(r) => RateValue::Real(r * 0.9),
                            other => *other,
                        };
                        new_map.insert(sb.clone(), scaled);
                    }
                    new_map
                },
            )),
        );

        (initial_term, initial_map, system)
    }

    #[test]
    fn test_simulator_step() {
        let (term, map, system) = make_test_system();
        let mut sim = Simulator::new(term, map, system);
        let step = sim.step();
        assert!(step.is_some());
        let step = step.unwrap();
        assert!(step.time > 0.0);
        assert_eq!(step.rule_name, "COMM");
    }

    #[test]
    fn test_update_function_applied() {
        let (term, map, system) = make_test_system();
        let mut sim = Simulator::new(term, map, system);
        let step = sim.step().unwrap();

        // The update function scales by 0.9, so the interaction rate
        // should go from 0.8 to 0.72
        if let Some(rv) = step
            .result_map
            .get(&SpatialBehavior::interaction("x", "x"))
        {
            assert!((rv.probability() - 0.72).abs() < 1e-10);
        }
    }

    #[test]
    fn test_context_rule_fold() {
        let initial_term = TermRef::new(1, "Proc", "{S | rest}");
        let mut initial_map = RateMap::new();
        initial_map.insert(
            SpatialBehavior::local("x"),
            RateValue::real(0.3).unwrap(),
        );

        let mut ctx_map = RateMap::new();
        ctx_map.insert(
            SpatialBehavior::local("y"),
            RateValue::real(0.5).unwrap(),
        );

        let system = RewriteSystem::new().add_context_rule(
            crate::augmented_rule::ContextRule::new(
                "PAR_CTXT",
                TermRef::new(1, "Proc", "{S | rest}"),
                TermRef::new(2, "Proc", "{T | rest}"),
                1.0,
                "if S ~> T",
                |context_maps, rule_map| {
                    let mut result = rule_map.clone();
                    for ctx in context_maps {
                        result = result.merge(ctx);
                    }
                    result
                },
            ),
        );

        let mut sim = Simulator::new(initial_term, initial_map, system)
            .with_context_maps(vec![ctx_map]);

        let step = sim.step().unwrap();
        // Folded map should contain both x and y
        assert!(step.result_map.get(&SpatialBehavior::local("x")).is_some());
        assert!(step.result_map.get(&SpatialBehavior::local("y")).is_some());
    }

    #[test]
    fn test_run() {
        let (term, map, system) = make_test_system();
        let mut sim = Simulator::new(term, map, system);
        let trace = sim.run(10);
        assert!(!trace.is_empty());
    }
}
