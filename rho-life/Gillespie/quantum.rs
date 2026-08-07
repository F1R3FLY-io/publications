//! # Quantum Simulation via CTMC
//!
//! Extends the Gillespie simulator for complex-valued amplitude maps,
//! enabling quantum model checking over MeTTaIL rewrite systems.
//!
//! In quantum mode, the update functions in base rule refinements produce
//! annotation maps with complex amplitudes. The fold functions in context
//! rules combine these via operations that preserve quantum coherence
//! (e.g., tensor products rather than simple merges).
//!
//! The quantum state is a superposition of (term, annotation_map) pairs,
//! weighted by complex amplitudes. Measurement collapses to a classical
//! distribution via |z|².

use crate::augmented_rule::{BaseRule, RewriteSystem, Rule, TermRef};
use crate::rate_map::RateMap;
use num_complex::Complex64;
use rand::Rng;
use std::collections::HashMap;

/// A quantum state: superposition of (term, annotation_map) pairs.
#[derive(Debug, Clone)]
pub struct QuantumState {
    /// Components: term_id → (term, amplitude, annotation_map)
    pub components: HashMap<u64, (TermRef, Complex64, RateMap)>,
}

impl QuantumState {
    /// Create a pure state.
    pub fn pure(term: TermRef, map: RateMap) -> Self {
        let id = term.id;
        let mut components = HashMap::new();
        components.insert(id, (term, Complex64::new(1.0, 0.0), map));
        QuantumState { components }
    }

    /// Create a superposition (amplitudes for same term_id interfere).
    pub fn superposition(entries: Vec<(TermRef, Complex64, RateMap)>) -> Self {
        let mut components = HashMap::new();
        for (term, amp, map) in entries {
            let entry = components
                .entry(term.id)
                .or_insert_with(|| (term.clone(), Complex64::new(0.0, 0.0), map.clone()));
            entry.1 += amp;
        }
        QuantumState { components }
    }

    /// Measurement distribution: term_id → probability.
    pub fn distribution(&self) -> Vec<(u64, &TermRef, f64)> {
        self.components
            .iter()
            .map(|(id, (term, amp, _))| (*id, term, amp.norm_sqr()))
            .filter(|(_, _, p)| *p > 1e-15)
            .collect()
    }

    /// Perform a measurement (collapse).
    pub fn measure(&self, rng: &mut impl Rng) -> Option<(TermRef, RateMap)> {
        let dist = self.distribution();
        if dist.is_empty() {
            return None;
        }
        let total: f64 = dist.iter().map(|(_, _, p)| p).sum();
        let r: f64 = rng.gen::<f64>() * total;
        let mut cumulative = 0.0;
        for (id, _, prob) in &dist {
            cumulative += prob;
            if cumulative >= r {
                let (term, _, map) = &self.components[id];
                return Some((term.clone(), map.clone()));
            }
        }
        let last = dist.last().unwrap().0;
        let (term, _, map) = &self.components[&last];
        Some((term.clone(), map.clone()))
    }

    /// Total probability mass.
    pub fn total_probability(&self) -> f64 {
        self.components.values().map(|(_, amp, _)| amp.norm_sqr()).sum()
    }

    /// Normalize so total probability = 1.
    pub fn normalize(&mut self) {
        let total = self.total_probability();
        if total > 1e-15 {
            let factor = 1.0 / total.sqrt();
            for (_, amp, _) in self.components.values_mut() {
                *amp *= factor;
            }
        }
    }

    /// Number of components in the superposition.
    pub fn dimension(&self) -> usize {
        self.components.len()
    }
}

/// A step in the quantum simulation.
#[derive(Debug, Clone)]
pub struct QuantumStep {
    pub time: f64,
    pub tau: f64,
    pub state: QuantumState,
    pub measured: Option<(TermRef, RateMap)>,
}

/// Quantum Gillespie simulator.
///
/// Each step evolves the superposition by applying all matching base rules'
/// refinement update functions (weighted by their amplitudes), producing
/// new superposition components that may interfere.
pub struct QuantumSimulator {
    pub time: f64,
    pub state: QuantumState,
    pub system: RewriteSystem,
    pub context_maps: Vec<RateMap>,
    pub measure_each_step: bool,
    rng: rand::rngs::ThreadRng,
}

impl QuantumSimulator {
    pub fn new(
        initial_term: TermRef,
        initial_map: RateMap,
        system: RewriteSystem,
    ) -> Self {
        QuantumSimulator {
            time: 0.0,
            state: QuantumState::pure(initial_term, initial_map),
            system,
            context_maps: Vec::new(),
            measure_each_step: false,
            rng: rand::thread_rng(),
        }
    }

    pub fn with_measurement(mut self, measure: bool) -> Self {
        self.measure_each_step = measure;
        self
    }

    pub fn with_context_maps(mut self, maps: Vec<RateMap>) -> Self {
        self.context_maps = maps;
        self
    }

    /// Evolve the quantum state by one step.
    pub fn step(&mut self) -> Option<QuantumStep> {
        if self.state.components.is_empty() {
            return None;
        }

        // Compute total transition rate
        let total_rate: f64 = self
            .state
            .components
            .values()
            .map(|(term, amp, _)| {
                let matching: f64 = self
                    .system
                    .rules
                    .iter()
                    .filter_map(|r| match r {
                        Rule::Base(br) if br.lhs.sort == term.sort => {
                            Some(br.propensity())
                        }
                        _ => None,
                    })
                    .sum();
                amp.norm_sqr() * matching
            })
            .sum();

        if total_rate < 1e-15 {
            return None;
        }

        // Sample waiting time
        let r1: f64 = self.rng.gen();
        let tau = (1.0 / total_rate) * (1.0 / r1).ln();

        // Evolve each component through all matching base rules
        let mut new_components: Vec<(TermRef, Complex64, RateMap)> = Vec::new();

        for (_, (term, amp, current_map)) in &self.state.components {
            let matching_bases: Vec<&BaseRule> = self
                .system
                .rules
                .iter()
                .filter_map(|r| match r {
                    Rule::Base(br) if br.lhs.sort == term.sort => Some(br),
                    _ => None,
                })
                .collect();

            if matching_bases.is_empty() {
                // No rules → component persists
                new_components.push((term.clone(), *amp, current_map.clone()));
                continue;
            }

            // For each matching base rule and each refinement,
            // produce a transition component
            for br in &matching_bases {
                for refinement in &br.refinements {
                    let transition_amp = Complex64::new(
                        (refinement.weight * br.weight).sqrt(),
                        0.0,
                    );
                    let new_amp = amp * transition_amp;
                    if new_amp.norm_sqr() > 1e-15 {
                        // Apply the update function
                        let updated_map = refinement.apply_update(current_map);

                        // If context rules exist, apply fold
                        let final_map = if !self.context_maps.is_empty() {
                            let ctx_refs: Vec<&RateMap> =
                                self.context_maps.iter().collect();
                            // Use first matching context rule's fold
                            self.system
                                .context_rules_for_sort(&term.sort)
                                .first()
                                .map(|cr| cr.apply_fold(&ctx_refs, &updated_map))
                                .unwrap_or(updated_map)
                        } else {
                            updated_map
                        };

                        new_components.push((
                            br.rhs.clone(),
                            new_amp,
                            final_map,
                        ));
                    }
                }
            }

            // Remaining amplitude (no transition)
            let transition_prob: f64 = matching_bases
                .iter()
                .map(|br| br.propensity())
                .sum::<f64>()
                .min(1.0);
            let remain_amp =
                amp * Complex64::new((1.0 - transition_prob).max(0.0).sqrt(), 0.0);
            if remain_amp.norm_sqr() > 1e-15 {
                new_components.push((term.clone(), remain_amp, current_map.clone()));
            }
        }

        let mut new_state = QuantumState::superposition(new_components);
        new_state.normalize();

        self.time += tau;

        let measured = if self.measure_each_step {
            new_state.measure(&mut self.rng)
        } else {
            None
        };

        if let Some((ref mterm, ref mmap)) = measured {
            new_state = QuantumState::pure(mterm.clone(), mmap.clone());
        }

        let step = QuantumStep {
            time: self.time,
            tau,
            state: new_state.clone(),
            measured,
        };

        self.state = new_state;
        Some(step)
    }

    /// Run for up to `max_steps`.
    pub fn run(&mut self, max_steps: usize) -> Vec<QuantumStep> {
        let mut trace = Vec::new();
        for _ in 0..max_steps {
            match self.step() {
                Some(step) => trace.push(step),
                None => break,
            }
        }
        trace
    }

    /// Sample the current state by measurement.
    pub fn sample(&mut self) -> Option<(TermRef, RateMap)> {
        self.state.measure(&mut self.rng)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::augmented_rule::RefinementEntry;
    use crate::rate_value::RateValue;
    use crate::spatial_behavior::SpatialBehavior;

    #[test]
    fn test_quantum_state_pure() {
        let state = QuantumState::pure(
            TermRef::new(1, "Proc", "P"),
            RateMap::new(),
        );
        assert!((state.total_probability() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_quantum_interference() {
        let term = TermRef::new(1, "Proc", "P");
        let s = 1.0 / 2.0_f64.sqrt();
        let state = QuantumState::superposition(vec![
            (term.clone(), Complex64::new(s, 0.0), RateMap::new()),
            (term.clone(), Complex64::new(s, 0.0), RateMap::new()),
        ]);
        // Constructive: (1/√2 + 1/√2)² = 2
        let amp = state.components[&1].1;
        assert!((amp.re - 2.0_f64.sqrt()).abs() < 1e-10);
    }

    #[test]
    fn test_quantum_destructive_interference() {
        let term = TermRef::new(1, "Proc", "P");
        let s = 1.0 / 2.0_f64.sqrt();
        let state = QuantumState::superposition(vec![
            (term.clone(), Complex64::new(s, 0.0), RateMap::new()),
            (term.clone(), Complex64::new(-s, 0.0), RateMap::new()),
        ]);
        let amp = state.components[&1].1;
        assert!(amp.norm_sqr() < 1e-10);
    }

    #[test]
    fn test_quantum_simulator_step() {
        let system = RewriteSystem::new().add_base_rule(
            BaseRule::new(
                "Q_COMM",
                TermRef::new(1, "Proc", "x?(y).P | x!(Q)"),
                TermRef::new(2, "Proc", "P{Q/y}"),
                0.5,
            )
            .add_refinement(RefinementEntry::new(
                SpatialBehavior::interaction("x", "x"),
                0.5,
                |map| map.clone(),
            )),
        );

        let mut sim = QuantumSimulator::new(
            TermRef::new(1, "Proc", "x?(y).P | x!(Q)"),
            RateMap::new(),
            system,
        );

        let step = sim.step();
        assert!(step.is_some());
    }
}
