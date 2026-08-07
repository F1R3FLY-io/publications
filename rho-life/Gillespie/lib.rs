//! # MeTTaIL Gillespie Extensions
//!
//! This crate extends MeTTaIL's rewrite rule system with **weighted rules**,
//! **refinement update functions**, and **context fold functions** that produce
//! Gillespie-style simulators.
//!
//! ## Rule Structure
//!
//! ```text
//! rules {
//!     <base-rule> . <lhs> ~> <rhs> [<weight>] {
//!         <spatial-behavior>_i => (<weight>_i, <update-function>_i)
//!     }
//!     <ctxt-rule> . if <cond> then <lhs> ~> <rhs> [<weight>] {
//!         <fold-function>
//!     }
//! }
//! ```
//!
//! - **Base rules**: Each refinement of the LHS type carries a weight and an
//!   update function that transforms the entire annotation map.
//! - **Context rules**: A fold function combines context maps with the
//!   updated map from the applied rule.
//!
//! When rate values are **real**, the simulator is a stochastic π-machine.
//! When rate values are **complex** (|z|² ∈ [0,1]), the simulator constructs
//! continuous-time Markov chains for quantum model checking.
//!
//! ## Fuzzer
//!
//! The crate also provides a **weighted term fuzzer** for generating random
//! terms from a language spec, with configurable production weights.

pub mod rate_value;
pub mod spatial_behavior;
pub mod rate_map;
pub mod augmented_rule;
pub mod gillespie;
pub mod quantum;
pub mod simulator;
pub mod language_ext;
pub mod fuzzer;

// Re-exports
pub use rate_value::RateValue;
pub use spatial_behavior::SpatialBehavior;
pub use rate_map::RateMap;
pub use augmented_rule::{
    BaseRule, ContextRule, RefinementEntry, RewriteSystem, Rule, TermRef,
    UpdateFn, FoldFn,
};
pub use simulator::{Simulator, SimulatorMode, SimulationStep, SimulationTrace};
