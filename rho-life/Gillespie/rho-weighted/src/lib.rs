//! # rho-weighted
//!
//! Weighted graph-structured lambda theories: a **simulator** for stochastic
//! (and, behind a feature flag, quantum) execution of rho theories.
//!
//! ## The simulator is not the interpreter
//!
//! The interpreter maintains the actual state of a running shard: one history,
//! authoritative, consensus-bound. This crate answers *what would happen if* —
//! it explores hypothetical configurations and produces **graphs**: reduction
//! graphs, generator matrices, trajectory ensembles, parameter sweeps.
//!
//! | | Interpreter | Simulator |
//! |---|---|---|
//! | Question | what *is* the state | what *would* happen |
//! | History | one, authoritative | many, hypothetical, branching |
//! | Nondeterminism | resolved by the tuple space, unobserved | enumerated, weighted, sampled *or* exhausted |
//! | Budget | phlogiston | steps, states, inference tokens |
//! | Output | new state | graphs |
//!
//! They share exactly one thing, and it is the important one: **the semantics
//! of a single step**. See [`matching`] for that seam, and `tests/faithfulness`
//! for the obligation it creates.
//!
//! This crate must never become a dependency of `node` or `casper`.
//!
//! ## The construction
//!
//! A rewrite rule's left-hand side is a type. Refine it with formulae of the
//! spatial--behavioural logic ([`logic`]), attach a weight to each refinement
//! ([`theory`]), and the enabled redexes of a configuration become a
//! distribution ([`propensity`]).
//!
//! Two things distinguish this from the stochastic pi-calculus literature.
//! Keys are *formulae*, not channel names — channel identity is one refinement
//! among many. And weight maps are *state*, not declaration, so a rewrite may
//! update them; rates change during execution, which is what makes synaptic
//! plasticity expressible as an ordinary transition ([`examples::rnn`]).
//!
//! ## Quick start
//!
//! ```
//! use rho_weighted::examples::two_state;
//! use rho_weighted::matching::SimpleMatcher;
//! use rho_weighted::space::Space;
//! use rho_weighted::theory::Configuration;
//! use rho_weighted::study::Study;
//!
//! let (theory, term) = two_state(2.0, 1.0);
//! let cfg = Configuration::new(Space::install(&term), theory.initial_weights());
//! let result = Study::new(&theory, &SimpleMatcher, cfg).run();
//! assert!(!result.any_truncated());
//! ```

pub mod examples;
pub mod faithfulness;
pub mod explore;
pub mod graph;
pub mod logic;
pub mod matching;
pub mod propensity;
pub mod redex;
pub mod rng;
pub mod space;
pub mod study;
pub mod syntax;
pub mod theory;

#[cfg(feature = "quantum")]
pub mod quantum;

pub use logic::{Budget, Checkable, Formula, RhoLogic, SpatialBehavioralLogic};
pub use matching::{Matching, SimpleMatcher};
pub use propensity::{propensities, Propensities};
pub use redex::{enumerate, Redex, RuleId, COMM};
pub use space::{Marking, Space};
pub use study::{Study, StudyResult};
pub use theory::{Configuration, RateValue, WeightMap, WeightedTheory};
