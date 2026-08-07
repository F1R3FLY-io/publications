//! The spatial--behavioural logic: formulae, satisfaction, and the partition
//! discipline.
//!
//! The `SpatialBehavioralLogic` port (DR-W4) is the seam at which a
//! hand-written instance is replaced by an OSLF-*generated* one, and at which
//! MeTTaIL becomes a second instance. Nothing downstream of this module knows
//! which it is talking to.

pub mod check;
pub mod formula;
pub mod partition;

pub use formula::{Budget, Checkable, Cmp, Formula, NamePred, PosQuant, WhyNot};
pub use partition::{
    check_partition, complete_with_default, complete_with_default_checked, Partition,
    PartitionError,
};

use crate::matching::Matching;
use crate::space::Space;
use crate::syntax::Term;

/// The port. One instance per GSLT presentation.
pub trait SpatialBehavioralLogic {
    /// The structural formula characterising a rule's left-hand side —
    /// `L^♯` in the note. Refinement keys must entail it.
    fn lhs_shape(&self, rule: crate::redex::RuleId) -> Formula;

    /// `t ⊨ φ` for the structural and propositional layers.
    fn satisfies_structural(
        &self,
        t: &Term,
        f: &Formula,
        b: &mut Budget,
    ) -> Result<bool, formula::CheckError>;

    /// All one-step successors. A redex is a labelled transition; this is the
    /// enumerator.
    fn successors<M: Matching>(&self, s: &Space, m: &M) -> Vec<(String, crate::redex::Redex, Space)>
    where
        Self: Sized;
}

/// The instance for the rho calculus as presented in [`crate::syntax`].
#[derive(Clone, Copy, Default, Debug)]
pub struct RhoLogic;

impl SpatialBehavioralLogic for RhoLogic {
    fn lhs_shape(&self, _rule: crate::redex::RuleId) -> Formula {
        // The communication rule: a receipt beside a send, on some channel.
        Formula::sep(
            Formula::In {
                chan: NamePred::Any,
                body: Box::new(Formula::Top),
            },
            Formula::Out {
                chan: NamePred::Any,
                body: Box::new(Formula::Top),
            },
        )
    }

    fn satisfies_structural(
        &self,
        t: &Term,
        f: &Formula,
        b: &mut Budget,
    ) -> Result<bool, formula::CheckError> {
        check::sat_struct(t, f, b)
    }

    fn successors<M: Matching>(&self, s: &Space, m: &M) -> Vec<(String, crate::redex::Redex, Space)> {
        check::successors(s, m)
    }
}
