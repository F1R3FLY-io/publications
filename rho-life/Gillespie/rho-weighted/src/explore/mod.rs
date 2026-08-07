//! Exploration strategies.
//!
//! Two, and they cross-check each other. [`ssa`] samples trajectories through
//! the configuration space; [`exhaustive`] builds the whole reachable graph and
//! its generator. On models small enough for both, the sampled occupancy must
//! converge to the exhaustive stationary distribution — which is the cheapest
//! available way to catch a systematic bias in either.
//!
//! Exhaustive construction is affordable *because the simulator is offline*.
//! The interpreter could never do this; it has one history and a phlogiston
//! budget. That difference is also what makes the greatest fixed point, model
//! checking and the quantum path viable here.

pub mod ensemble;
pub mod exhaustive;
pub mod ssa;

pub use ensemble::{Ensemble, EnsembleSummary};
pub use exhaustive::{exhaustive_graph, Exploration};
pub use ssa::{Sim, SimStep, Trace};

/// Why a run or an exploration stopped. A truncated result that does not
/// announce itself is worse than no result (DR-W10).
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Stop {
    /// `a₀ = 0`: every enabled redex is unfunded or zero-rated. An absorbing
    /// state, not a deadlock; the waiting time to the next event is infinite
    /// rather than undefined.
    Absorbing,
    StepBudget,
    TimeHorizon,
    StateBudget,
    CheckError(String),
}

impl Stop {
    /// Whether the result is complete or was cut short.
    pub fn is_truncation(&self) -> bool {
        !matches!(self, Stop::Absorbing | Stop::TimeHorizon)
    }
}
