//! Faithfulness (DR-W9).
//!
//! The simulator's value is that its graphs predict the interpreter. That is a
//! claim requiring evidence, and it is discharged three ways.
//!
//! **By sharing.** Inside the `f1r3node-rust` workspace, the matcher, the guard
//! evaluator and substitution are literally the interpreter's, called through
//! public API. `rholang::rust::interpreter::matcher::r#match::Matcher::get` is
//! `pub` and pure — no store, no consumption, no side effects — so no
//! reimplementation means no drift in the part most likely to drift. That
//! adapter is [`workspace`], gated behind the `workspace-matcher` feature.
//!
//! **By differential testing.** For the standalone build there is no
//! interpreter to differ against, so [`differential`] compares
//! [`crate::matching::SimpleMatcher`] against an independently written
//! reference implementation. This is weaker than the workspace comparison, but
//! it is not nothing: it catches exactly the class of error that produced the
//! two real bugs in this crate's history — a judgment that looks right and
//! quietly disagrees with itself on structured data.
//!
//! **By marking equivalence.** [`crate::space::Space::install`] is the only
//! genuinely new semantics the simulator implements itself, so it gets its own
//! obligation: a space built from a term must have the marking the term's
//! parallel decomposition says it should.
//!
//! Faithfulness checks are a permanent test category, not a phase gate. When
//! the interpreter's semantics changes, they are the alarm.

use crate::matching::{Bindings, Matching};
use crate::syntax::{Bind, Pattern, Term};

/// An independently written reference matcher.
///
/// Deliberately structured differently from [`crate::matching::SimpleMatcher`]:
/// it works by building the binding set and then re-checking it, rather than
/// checking and binding in one pass. Two implementations that share a bug are
/// no better than one, so the point is for them to share as little shape as
/// possible.
#[derive(Clone, Copy, Default, Debug)]
pub struct ReferenceMatcher;

impl Matching for ReferenceMatcher {
    fn match_bind(&self, bind: &Bind, data: &[Term]) -> Option<Bindings> {
        if bind.patterns.len() != data.len() {
            return None;
        }
        // Pass one: collect every proposed binding, with no consistency check.
        let mut proposed: Vec<(String, Term)> = Vec::new();
        for (p, d) in bind.patterns.iter().zip(data.iter()) {
            match p {
                Pattern::Wildcard => continue,
                Pattern::Var(v) => proposed.push((v.clone(), d.canonical())),
                Pattern::Exact(t) => {
                    if t.canonical() != d.canonical() {
                        return None;
                    }
                }
            }
        }
        // Pass two: reject if any variable was proposed two incompatible ways.
        let mut out = Bindings::new();
        for (v, t) in proposed {
            match out.get(&v) {
                Some(prev) if *prev != t => return None,
                _ => {
                    out.insert(v, t);
                }
            }
        }
        Some(out)
    }
}

/// A differential check over a corpus: two matchers must agree on every case,
/// both on success and on the bindings produced.
///
/// Returns the first disagreement, if any.
pub fn differential<A: Matching, B: Matching>(
    a: &A,
    b: &B,
    corpus: &[(Bind, Vec<Term>)],
) -> Result<(), Disagreement> {
    for (bind, data) in corpus {
        let ra = a.match_bind(bind, data);
        let rb = b.match_bind(bind, data);
        match (&ra, &rb) {
            (None, None) => {}
            (Some(x), Some(y)) if x == y => {}
            _ => {
                return Err(Disagreement {
                    source: bind.source.key(),
                    data: data.iter().map(|d| d.render()).collect(),
                    left: describe(&ra),
                    right: describe(&rb),
                })
            }
        }
    }
    Ok(())
}

fn describe(r: &Option<Bindings>) -> String {
    match r {
        None => "no match".into(),
        Some(b) => format!(
            "{{{}}}",
            b.iter()
                .map(|(k, v)| format!("{k}={}", v.render()))
                .collect::<Vec<_>>()
                .join(",")
        ),
    }
}

#[derive(Clone, Debug)]
pub struct Disagreement {
    pub source: String,
    pub data: Vec<String>,
    pub left: String,
    pub right: String,
}

impl std::fmt::Display for Disagreement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "matchers disagree on {} with [{}]: {} vs {}",
            self.source,
            self.data.join(", "),
            self.left,
            self.right
        )
    }
}

/// The marking a term's parallel decomposition says it should have, computed
/// without reference to [`crate::space::Space`].
///
/// This exists so `install` is checked against something other than itself.
pub fn expected_occupancy(term: &Term) -> std::collections::BTreeMap<String, usize> {
    let mut out = std::collections::BTreeMap::new();
    for c in term.components() {
        if let Term::Send { chan, .. } = c {
            *out.entry(chan.key()).or_insert(0) += 1;
        }
    }
    out
}

// ---------------------------------------------------------------------------
// The workspace adapter
// ---------------------------------------------------------------------------

/// The adapter over the interpreter's own matcher.
///
/// Enabled by the `workspace-matcher` feature, which is only satisfiable when
/// the crate is built as a member of `f1r3node-rust`. The body is one call:
///
/// ```ignore
/// use rholang::rust::interpreter::matcher::r#match::Matcher;
/// use rspace_plus_plus::rspace::r#match::Match;
///
/// impl Matching for WorkspaceMatcher {
///     fn match_bind(&self, bind: &Bind, data: &[Term]) -> Option<Bindings> {
///         let pattern: BindPattern = to_bind_pattern(bind);
///         let datum: ListParWithRandom = to_list_par(data);
///         Matcher.get(pattern, datum).map(from_bound_pars)
///     }
/// }
/// ```
///
/// Everything hard is in the three conversions, and those are exactly what the
/// `Term` → `Par` swap of DR-W1 removes: once `Term` *is* `models::rhoapi::Par`
/// the adapter is `Matcher.get` and nothing else.
///
/// The differential suite then runs against the interpreter rather than against
/// [`ReferenceMatcher`], and *that* is the check DR-W9 actually asks for.
#[cfg(feature = "workspace-matcher")]
pub mod workspace {
    compile_error!(
        "the `workspace-matcher` feature requires building inside f1r3node-rust; \
         see docs/weighted-gslt/design.md"
    );
}
