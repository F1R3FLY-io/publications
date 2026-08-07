//! # Unified Simulator Interface
//!
//! Dispatches between the classical Gillespie simulator and the quantum
//! CTMC simulator based on whether annotation maps contain real or complex
//! rate values.

use crate::augmented_rule::{RewriteSystem, TermRef};
use crate::gillespie;
use crate::quantum;
use crate::rate_map::{MapMode, RateMap};
use std::fmt;

/// Simulation mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulatorMode {
    Classical,
    Quantum,
}

/// A step from either simulator.
#[derive(Debug, Clone)]
pub enum SimulationStep {
    Classical(gillespie::SimStep),
    Quantum(quantum::QuantumStep),
}

impl SimulationStep {
    pub fn time(&self) -> f64 {
        match self {
            SimulationStep::Classical(s) => s.time,
            SimulationStep::Quantum(s) => s.time,
        }
    }
}

impl fmt::Display for SimulationStep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SimulationStep::Classical(s) => {
                write!(f, "t={:.6} [{}] → {}", s.time, s.rule_name, s.result_term)
            }
            SimulationStep::Quantum(s) => {
                write!(
                    f,
                    "t={:.6} dim={} p_total={:.6}",
                    s.time,
                    s.state.dimension(),
                    s.state.total_probability()
                )
            }
        }
    }
}

/// A simulation trace.
#[derive(Debug, Clone)]
pub struct SimulationTrace {
    pub mode: SimulatorMode,
    pub steps: Vec<SimulationStep>,
    pub initial_term: TermRef,
}

impl SimulationTrace {
    pub fn total_time(&self) -> f64 {
        self.steps.last().map(|s| s.time()).unwrap_or(0.0)
    }

    pub fn num_steps(&self) -> usize {
        self.steps.len()
    }
}

/// Unified simulator.
pub enum Simulator {
    Classical(gillespie::Simulator),
    Quantum(quantum::QuantumSimulator),
}

impl Simulator {
    /// Auto-detect mode from the initial annotation map.
    pub fn new(
        initial_term: TermRef,
        initial_map: RateMap,
        system: RewriteSystem,
    ) -> Self {
        let mode = match initial_map.mode() {
            Some(MapMode::Quantum) => SimulatorMode::Quantum,
            _ => SimulatorMode::Classical,
        };
        Self::with_mode(initial_term, initial_map, system, mode)
    }

    /// Force a specific mode.
    pub fn with_mode(
        initial_term: TermRef,
        initial_map: RateMap,
        system: RewriteSystem,
        mode: SimulatorMode,
    ) -> Self {
        match mode {
            SimulatorMode::Classical => {
                Simulator::Classical(gillespie::Simulator::new(
                    initial_term,
                    initial_map,
                    system,
                ))
            }
            SimulatorMode::Quantum => {
                Simulator::Quantum(quantum::QuantumSimulator::new(
                    initial_term,
                    initial_map,
                    system,
                ))
            }
        }
    }

    pub fn mode(&self) -> SimulatorMode {
        match self {
            Simulator::Classical(_) => SimulatorMode::Classical,
            Simulator::Quantum(_) => SimulatorMode::Quantum,
        }
    }

    pub fn step(&mut self) -> Option<SimulationStep> {
        match self {
            Simulator::Classical(sim) => sim.step().map(SimulationStep::Classical),
            Simulator::Quantum(sim) => sim.step().map(SimulationStep::Quantum),
        }
    }

    pub fn run(&mut self, max_steps: usize) -> SimulationTrace {
        let initial = match self {
            Simulator::Classical(sim) => sim.current_term.clone(),
            Simulator::Quantum(sim) => sim
                .state
                .components
                .values()
                .next()
                .map(|(t, _, _)| t.clone())
                .unwrap_or_else(|| TermRef::new(0, "?", "?")),
        };

        let mut steps = Vec::new();
        for _ in 0..max_steps {
            match self.step() {
                Some(step) => steps.push(step),
                None => break,
            }
        }

        SimulationTrace {
            mode: self.mode(),
            steps,
            initial_term: initial,
        }
    }

    pub fn time(&self) -> f64 {
        match self {
            Simulator::Classical(sim) => sim.time,
            Simulator::Quantum(sim) => sim.time,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::augmented_rule::{BaseRule, RefinementEntry};
    use crate::rate_value::RateValue;
    use crate::spatial_behavior::SpatialBehavior;

    #[test]
    fn test_unified_classical() {
        let system = RewriteSystem::new().add_base_rule(
            BaseRule::new(
                "TEST",
                TermRef::new(1, "P", "A"),
                TermRef::new(2, "P", "B"),
                0.5,
            )
            .add_refinement(RefinementEntry::new(
                SpatialBehavior::local("x"),
                0.5,
                |map| map.clone(),
            )),
        );

        let mut sim = Simulator::new(
            TermRef::new(1, "P", "A"),
            RateMap::new(),
            system,
        );
        assert_eq!(sim.mode(), SimulatorMode::Classical);
        let trace = sim.run(5);
        assert!(!trace.steps.is_empty());
    }
}
