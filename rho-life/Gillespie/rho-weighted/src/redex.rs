//! Redex enumeration.
//!
//! A redex is a *selection* of reactants from the multiset, not a term
//! identified up to structural congruence. Two indistinguishable messages on a
//! channel offer two ways for a communication to occur, not one; the note's
//! Definition 3 and Remark 18 make this explicit, and getting it wrong makes
//! the simulator run at `1/m` the correct rate for multiplicity `m`.
//!
//! This is also the enumerator that the behavioural modality uses: a redex *is*
//! a labelled transition, its label the position, so `successors` and
//! `enumerate` are the same function ([`crate::logic::check`]).
//!
//! The interpreter cannot offer this. Its next state is whatever RSpace and
//! tokio decided; it never sees the alternatives it did not take.

use crate::matching::{Bindings, Matching};
use crate::space::{ChanKey, ContId, DatumId, Space};

/// Which base rule fired. The rho calculus presented here has one interaction
/// rule; the type is an index so that richer theories slot in unchanged.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct RuleId(pub usize);

pub const COMM: RuleId = RuleId(0);

/// A selection of reactants: one continuation, one datum per bind.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Redex {
    pub rule: RuleId,
    pub cont: ContId,
    /// `(channel, datum id, datum was persistent)`, one per bind, in bind order.
    pub consumed: Vec<(ChanKey, DatumId, bool)>,
    pub bindings: Bindings,
}

impl Redex {
    /// The position at which the step fires — the label `k` of the modality
    /// `⟨K_j⟩`. In a tuple space a location is the channel set together with
    /// the identity of the matched reactants, which is the flattened form of
    /// the one-hole context in a term.
    pub fn position(&self) -> String {
        let cells: Vec<String> = self
            .consumed
            .iter()
            .map(|(c, i, _)| format!("{c}#{i}"))
            .collect();
        format!("k({};{})", self.cont, cells.join("&"))
    }

    /// The channels this redex draws from.
    pub fn channels(&self) -> Vec<ChanKey> {
        self.consumed.iter().map(|(c, _, _)| c.clone()).collect()
    }
}

/// All enabled redexes of a space.
///
/// For a single-bind continuation this yields one redex per matching datum, so
/// the count over a channel is `In_n · Out_n` — SPiM's activity. For a join it
/// yields the cartesian product across the binds, which is the product rule and
/// is exactly why a threshold neuron is a join (note, Prop. 41).
///
/// A persistent continuation is not consumed, so it pairs with every matching
/// datum rather than one; a persistent datum likewise. `peek` is enumerated but
/// does not change the marking.
pub fn enumerate<M: Matching>(space: &Space, m: &M) -> Vec<Redex> {
    let mut out = Vec::new();
    for cont in space.all_conts() {
        // Candidate datums per bind.
        let mut per_bind: Vec<Vec<(ChanKey, DatumId, bool, Bindings)>> = Vec::new();
        let mut viable = true;
        for b in &cont.binds {
            let key = b.source.key();
            let mut cands = Vec::new();
            for d in space.data_on(&key) {
                if let Some(bind) = m.match_bind(b, &d.data) {
                    cands.push((key.clone(), d.id, d.persistent, bind));
                }
            }
            if cands.is_empty() {
                viable = false;
                break;
            }
            per_bind.push(cands);
        }
        if !viable {
            continue;
        }

        // Cartesian product across binds, merging bindings and rejecting a
        // selection that would consume the same datum twice.
        let mut partial: Vec<(Vec<(ChanKey, DatumId, bool)>, Bindings)> =
            vec![(Vec::new(), Bindings::new())];
        for cands in &per_bind {
            let mut next = Vec::new();
            for (chosen, binds) in &partial {
                for (ck, id, pers, cb) in cands {
                    if chosen.iter().any(|(c, i, _)| c == ck && i == id) {
                        continue;
                    }
                    if let Some(merged) = m.merge(binds, cb) {
                        let mut ch = chosen.clone();
                        ch.push((ck.clone(), *id, *pers));
                        next.push((ch, merged));
                    }
                }
            }
            partial = next;
            if partial.is_empty() {
                break;
            }
        }

        for (consumed, bindings) in partial {
            out.push(Redex {
                rule: COMM,
                cont: cont.id,
                consumed,
                bindings,
            });
        }
    }
    // Deterministic order: studies must be reproducible from (theory, seed).
    out.sort_by_key(|r| r.position());
    out
}

/// The local term of a redex — the instance `Lσ` of the rule's left-hand side.
/// Refinement keys are evaluated against this (note, Definition 5).
pub fn local_term(space: &Space, r: &Redex) -> crate::syntax::Term {
    use crate::syntax::Term;
    let mut parts = Vec::new();
    if let Some(c) = space.all_conts().find(|c| c.id == r.cont) {
        parts.push(Term::Receive {
            binds: c.binds.clone(),
            body: Box::new(c.body.clone()),
            persistent: c.persistent,
            peek: c.peek,
        });
    }
    for (ck, id, pers) in &r.consumed {
        if let Some(d) = space.data_on(ck).iter().find(|d| d.id == *id) {
            parts.push(Term::Send {
                chan: d.name.clone(),
                data: d.data.clone(),
                persistent: *pers,
            });
        }
    }
    Term::Par(parts).canonical()
}
