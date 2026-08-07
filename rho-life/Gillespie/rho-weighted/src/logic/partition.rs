//! The partition discipline (note, Definition 12).
//!
//! A key set must be pairwise exclusive (P1) and jointly exhaustive (P2) over
//! the rule's left-hand side. This is not a technicality. Without exclusivity
//! the classification is multi-valued and one must invent an aggregation
//! convention; without exhaustiveness it is partial and one must invent a
//! default. Each convention is defensible and each yields a *different
//! simulator from the same specification*, which is precisely the situation a
//! specification exists to prevent.
//!
//! A failed check is an error, never a silent fallback (DR-W5).
//!
//! Two consequences worth knowing. Stating (P1) and (P2) needs `¬`, `∧` and
//! `∨`, so a weighted theory cannot be specified over a target logic carrying
//! only the finite-limits fragment — the same connectives adequacy wants,
//! arrived at independently. And under the complex reading the classes become a
//! resolution of the identity, hence a projective measurement.

use crate::logic::check::sat_struct;
use crate::logic::formula::{Budget, Checkable, Formula};
use crate::syntax::Term;

#[derive(Clone, Debug)]
pub struct Partition {
    pub keys: Vec<Checkable>,
    /// Index of the synthesised `default` key, if one was added.
    pub default_index: Option<usize>,
}

#[derive(Clone, Debug)]
pub enum PartitionError {
    /// Two keys are simultaneously satisfied by a witness.
    Overlap {
        i: usize,
        j: usize,
        left: String,
        right: String,
        witness: String,
    },
    /// A witness of the left-hand side satisfies no key.
    NotExhaustive { witness: String },
    Budget,
}

impl std::fmt::Display for PartitionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PartitionError::Overlap {
                i,
                j,
                left,
                right,
                witness,
            } => write!(
                f,
                "keys {i} `{left}` and {j} `{right}` are not exclusive: both hold of `{witness}`"
            ),
            PartitionError::NotExhaustive { witness } => write!(
                f,
                "key set is not exhaustive: `{witness}` satisfies the left-hand side but no key. \
                 Add `default` to complete the partition."
            ),
            PartitionError::Budget => write!(f, "partition check exhausted its budget"),
        }
    }
}

impl Partition {
    pub fn len(&self) -> usize {
        self.keys.len()
    }
    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    /// Classify a witness: the unique index whose key it satisfies.
    ///
    /// Total and single-valued *because* the set is a partition (note,
    /// Prop. 14). This is what lets propensity be accumulated by redex rather
    /// than by key, with no risk of a redex counted twice or missed.
    pub fn classify(&self, t: &Term, b: &mut Budget) -> Option<usize> {
        for (i, k) in self.keys.iter().enumerate() {
            if matches!(sat_struct(t, k.formula(), b), Ok(true)) {
                return Some(i);
            }
        }
        None
    }
}

/// Synthesise `default` as `lhs ∧ ¬⋁ φᵢ` and append it, so a modeller writes
/// exclusive keys and gets a partition (note, Remark 8). Exclusivity is the
/// real constraint; exhaustiveness is a completion.
pub fn complete_with_default(keys: Vec<Checkable>, lhs: &Formula) -> Partition {
    let disj = keys
        .iter()
        .map(|k| k.formula().clone())
        .reduce(Formula::or)
        .unwrap_or(Formula::Bot);
    let default = Formula::and(lhs.clone(), Formula::not(disj));
    let idx = keys.len();
    let mut keys = keys;
    keys.push(Checkable::trusted(default));
    Partition {
        keys,
        default_index: Some(idx),
    }
}

/// Check (P1) and (P2) against a supplied set of witnesses.
///
/// Deciding these in general is undecidable, so we decide them over the
/// witnesses that matter — the reachable left-hand-side instances of the model
/// under study, which a study can supply from its own reachable set. For the
/// shapes that occur in practice (disjoint channels, disjoint namespaces)
/// exclusivity holds by construction and this is a cheap confirmation.
pub fn check_partition(
    p: &Partition,
    lhs: &Formula,
    witnesses: &[Term],
    b: &mut Budget,
) -> Result<(), PartitionError> {
    for w in witnesses {
        let mut hits: Vec<usize> = Vec::new();
        for (i, k) in p.keys.iter().enumerate() {
            match sat_struct(w, k.formula(), b) {
                Ok(true) => hits.push(i),
                Ok(false) => {}
                Err(_) => return Err(PartitionError::Budget),
            }
        }
        if hits.len() > 1 {
            return Err(PartitionError::Overlap {
                i: hits[0],
                j: hits[1],
                left: p.keys[hits[0]].render(),
                right: p.keys[hits[1]].render(),
                witness: w.render(),
            });
        }
        if hits.is_empty() {
            let on_lhs = matches!(sat_struct(w, lhs, b), Ok(true));
            if on_lhs {
                return Err(PartitionError::NotExhaustive {
                    witness: w.render(),
                });
            }
        }
    }
    Ok(())
}
