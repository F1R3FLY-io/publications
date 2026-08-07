//! The matching port.
//!
//! This is the **shared-semantics seam** of DR-W9. The simulator and the
//! interpreter must agree on what matches what, or the simulator's graphs are
//! fiction. Inside the `f1r3node-rust` workspace the intended implementation is
//! an adapter over
//! `rholang::rust::interpreter::matcher::r#match::Matcher::get`, which is
//! already `pub` and pure — no store, no consumption, no side effects — so the
//! two execution paths share the semantics without sharing machinery.
//!
//! [`SimpleMatcher`] is the standalone stand-in used when the crate is built
//! outside the workspace. It implements the same judgment for the fragment of
//! [`crate::syntax`] this crate uses. `tests/faithfulness.rs` is where the two
//! are held to each other.

use std::collections::BTreeMap;

use crate::syntax::{Bind, Pattern, Term};

/// Variables bound by a successful match.
pub type Bindings = BTreeMap<String, Term>;

/// The one judgment the simulator borrows from the interpreter.
pub trait Matching {
    /// Match one bind's patterns against one datum's payload.
    ///
    /// Returns the bindings on success. Purity is required: implementations
    /// must not consume, mutate, or otherwise observe the caller's state.
    fn match_bind(&self, bind: &Bind, data: &[Term]) -> Option<Bindings>;

    /// Merge bindings from several binds of a join, failing on conflict.
    fn merge(&self, acc: &Bindings, next: &Bindings) -> Option<Bindings> {
        let mut out = acc.clone();
        for (k, v) in next {
            match out.get(k) {
                Some(existing) if existing.canonical() != v.canonical() => return None,
                _ => {
                    out.insert(k.clone(), v.clone());
                }
            }
        }
        Some(out)
    }
}

/// The standalone implementation. Positional, arity-sensitive, and exact up to
/// structural congruence — deliberately the simplest thing that is correct for
/// the fragment in use.
#[derive(Clone, Copy, Default, Debug)]
pub struct SimpleMatcher;

impl Matching for SimpleMatcher {
    fn match_bind(&self, bind: &Bind, data: &[Term]) -> Option<Bindings> {
        if bind.patterns.len() != data.len() {
            return None;
        }
        let mut out = Bindings::new();
        for (p, d) in bind.patterns.iter().zip(data.iter()) {
            match p {
                Pattern::Wildcard => {}
                Pattern::Var(v) => {
                    let dc = d.canonical();
                    if let Some(prev) = out.get(v) {
                        if prev.canonical() != dc {
                            return None;
                        }
                    }
                    out.insert(v.clone(), dc);
                }
                Pattern::Exact(t) => {
                    if t.canonical() != d.canonical() {
                        return None;
                    }
                }
            }
        }
        Some(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::{chan, Ground, Name};

    fn bind(pats: Vec<Pattern>) -> Bind {
        Bind {
            patterns: pats,
            source: chan("a"),
        }
    }

    #[test]
    fn arity_is_enforced() {
        let m = SimpleMatcher;
        assert!(m
            .match_bind(&bind(vec![Pattern::Wildcard]), &[Term::Zero, Term::Zero])
            .is_none());
    }

    #[test]
    fn a_variable_binds_and_a_repeat_must_agree() {
        let m = SimpleMatcher;
        let b = bind(vec![Pattern::Var("x".into()), Pattern::Var("x".into())]);
        assert!(m.match_bind(&b, &[Term::Zero, Term::Zero]).is_some());
        assert!(m
            .match_bind(&b, &[Term::Zero, Term::Ground(Ground::Int(1))])
            .is_none());
    }

    #[test]
    fn matching_respects_structural_congruence() {
        let m = SimpleMatcher;
        let lhs = Term::Par(vec![
            Term::send(chan("a"), vec![Term::Zero]),
            Term::send(chan("b"), vec![Term::Zero]),
        ]);
        let rhs = Term::Par(vec![
            Term::send(chan("b"), vec![Term::Zero]),
            Term::send(chan("a"), vec![Term::Zero]),
        ]);
        let b = bind(vec![Pattern::Exact(lhs)]);
        assert!(m.match_bind(&b, &[rhs]).is_some());
    }

    #[test]
    fn merge_rejects_conflicting_bindings() {
        let m = SimpleMatcher;
        let mut a = Bindings::new();
        a.insert("x".into(), Term::Zero);
        let mut b = Bindings::new();
        b.insert("x".into(), Term::Ground(Ground::Int(1)));
        assert!(m.merge(&a, &b).is_none());
    }

    #[test]
    fn quote_drop_equation_holds_on_keys() {
        let n = Name::quote(Term::Drop(Box::new(chan("a"))));
        assert_eq!(n.key(), chan("a").key());
    }
}
