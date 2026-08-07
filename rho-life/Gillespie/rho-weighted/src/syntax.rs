//! The term language.
//!
//! This mirrors the *shape* of `models::rhoapi::Par` for the fragment the
//! weighted-GSLT construction needs: nil, parallel composition, send, receive
//! (with persistence and peek), name restriction, drop, and ground values.
//! Names are quoted processes, as the rho calculus requires — a name predicate
//! can therefore see into the structure of a name (namespace logic).
//!
//! # Seam
//!
//! When this crate is built inside the `f1r3node-rust` workspace, `Term` is
//! replaced by `models::rhoapi::Par` and this module becomes a conversion
//! adapter. Nothing outside [`crate::matching`] and [`crate::space`] pattern
//! matches on `Term` directly, so the swap is local. See DR-W1/DR-W3.

use std::collections::BTreeMap;
use std::fmt;

/// A name is a quoted process. `NVar` exists only during elaboration; a name
/// in an installed space is always a `Quote`.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Name {
    Quote(Box<Term>),
    Var(String),
}

impl Name {
    pub fn quote(t: Term) -> Name {
        // The QuoteDrop equation: @(*n) = n.
        match t {
            Term::Drop(n) => *n,
            other => Name::Quote(Box::new(other)),
        }
    }

    /// A stable, canonical key for indexing a channel.
    pub fn key(&self) -> String {
        match self {
            Name::Quote(t) => format!("@{}", t.canonical().render()),
            Name::Var(v) => format!("?{v}"),
        }
    }

    pub fn is_ground(&self) -> bool {
        matches!(self, Name::Quote(_))
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Ground {
    Int(i64),
    Bool(bool),
    Str(String),
}

/// A receive-bind: patterns drawn from one source channel.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Bind {
    pub patterns: Vec<Pattern>,
    pub source: Name,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Pattern {
    /// `_`
    Wildcard,
    /// A free variable, bound by a successful match.
    Var(String),
    /// A closed term that must be matched exactly (up to structural congruence).
    Exact(Term),
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Term {
    Zero,
    /// Parallel composition. Associative--commutative; see [`Term::canonical`].
    Par(Vec<Term>),
    Send {
        chan: Name,
        data: Vec<Term>,
        persistent: bool,
    },
    Receive {
        binds: Vec<Bind>,
        body: Box<Term>,
        persistent: bool,
        peek: bool,
    },
    New {
        names: Vec<String>,
        body: Box<Term>,
    },
    /// `*n`
    Drop(Box<Name>),
    Ground(Ground),
    /// An **ordered** list. Distinct from `Par`, and the distinction matters:
    /// parallel composition is a multiset, so it cannot encode a tuple —
    /// `@[syn, 0, 1]` and `@[syn, 1, 0]` would be the same channel. A
    /// reflective encoding of structured names needs an ordered former.
    List(Vec<Term>),
    /// A process variable, replaced by substitution.
    Var(String),
}

impl Term {
    pub fn par(terms: Vec<Term>) -> Term {
        Term::Par(terms).canonical()
    }

    pub fn send(chan: Name, data: Vec<Term>) -> Term {
        Term::Send {
            chan,
            data,
            persistent: false,
        }
    }

    /// `for(patterns <- source){ body }` — a single linear bind.
    pub fn recv(source: Name, patterns: Vec<Pattern>, body: Term) -> Term {
        Term::Receive {
            binds: vec![Bind { patterns, source }],
            body: Box::new(body),
            persistent: false,
            peek: false,
        }
    }

    /// `for(patterns <= source){ body }` — a single persistent bind.
    pub fn recv_persistent(source: Name, patterns: Vec<Pattern>, body: Term) -> Term {
        Term::Receive {
            binds: vec![Bind { patterns, source }],
            body: Box::new(body),
            persistent: true,
            peek: false,
        }
    }

    /// A join: all binds must be satisfied simultaneously (`&` in the surface).
    pub fn join(binds: Vec<Bind>, body: Term, persistent: bool) -> Term {
        Term::Receive {
            binds,
            body: Box::new(body),
            persistent,
            peek: false,
        }
    }

    /// Flatten parallel composition, drop nils, sort. This is the structural
    /// congruence made into a normal form, which is what lets configurations be
    /// compared and hashed as *states*.
    pub fn canonical(&self) -> Term {
        match self {
            Term::Par(ts) => {
                let mut flat: Vec<Term> = Vec::new();
                for t in ts {
                    match t.canonical() {
                        Term::Zero => {}
                        Term::Par(inner) => flat.extend(inner),
                        other => flat.push(other),
                    }
                }
                flat.sort();
                match flat.len() {
                    0 => Term::Zero,
                    1 => flat.pop().unwrap(),
                    _ => Term::Par(flat),
                }
            }
            Term::Send {
                chan,
                data,
                persistent,
            } => Term::Send {
                chan: chan.clone(),
                data: data.iter().map(|d| d.canonical()).collect(),
                persistent: *persistent,
            },
            Term::Receive {
                binds,
                body,
                persistent,
                peek,
            } => Term::Receive {
                binds: binds.clone(),
                body: Box::new(body.canonical()),
                persistent: *persistent,
                peek: *peek,
            },
            Term::New { names, body } => Term::New {
                names: names.clone(),
                body: Box::new(body.canonical()),
            },
            Term::List(ts) => Term::List(ts.iter().map(|t| t.canonical()).collect()),
            other => other.clone(),
        }
    }

    /// The top-level parallel components, after canonicalisation.
    pub fn components(&self) -> Vec<Term> {
        match self.canonical() {
            Term::Zero => vec![],
            Term::Par(ts) => ts,
            other => vec![other],
        }
    }

    /// Substitute process variables. Used to instantiate a continuation body
    /// with the bindings a match produced.
    pub fn substitute(&self, env: &BTreeMap<String, Term>) -> Term {
        match self {
            Term::Var(v) => env.get(v).cloned().unwrap_or_else(|| Term::Var(v.clone())),
            Term::Zero | Term::Ground(_) => self.clone(),
            Term::Par(ts) => Term::Par(ts.iter().map(|t| t.substitute(env)).collect()).canonical(),
            Term::Send {
                chan,
                data,
                persistent,
            } => Term::Send {
                chan: subst_name(chan, env),
                data: data.iter().map(|d| d.substitute(env)).collect(),
                persistent: *persistent,
            },
            Term::Receive {
                binds,
                body,
                persistent,
                peek,
            } => Term::Receive {
                binds: binds
                    .iter()
                    .map(|b| Bind {
                        patterns: b.patterns.clone(),
                        source: subst_name(&b.source, env),
                    })
                    .collect(),
                // Bound pattern variables shadow; we rely on distinct names,
                // which the builders in `crate::examples` guarantee.
                body: Box::new(body.substitute(env)),
                persistent: *persistent,
                peek: *peek,
            },
            Term::New { names, body } => Term::New {
                names: names.clone(),
                body: Box::new(body.substitute(env)),
            },
            Term::List(ts) => Term::List(ts.iter().map(|t| t.substitute(env)).collect()),
            Term::Drop(n) => match subst_name(n, env) {
                Name::Quote(inner) => *inner,
                other => Term::Drop(Box::new(other)),
            },
        }
    }

    /// A short rendering, used for canonical keys and for graph labels.
    pub fn render(&self) -> String {
        match self {
            Term::Zero => "0".into(),
            Term::Var(v) => v.clone(),
            Term::Ground(Ground::Int(i)) => i.to_string(),
            Term::Ground(Ground::Bool(b)) => b.to_string(),
            Term::Ground(Ground::Str(s)) => format!("\"{s}\""),
            Term::Par(ts) => ts
                .iter()
                .map(|t| t.render())
                .collect::<Vec<_>>()
                .join(" | "),
            Term::Send {
                chan,
                data,
                persistent,
            } => format!(
                "{}!{}({})",
                chan.key(),
                if *persistent { "!" } else { "" },
                data.iter().map(|d| d.render()).collect::<Vec<_>>().join(",")
            ),
            Term::Receive {
                binds,
                body,
                persistent,
                ..
            } => format!(
                "for({}){{{}}}",
                binds
                    .iter()
                    .map(|b| format!(
                        "{} {} {}",
                        b.patterns
                            .iter()
                            .map(render_pattern)
                            .collect::<Vec<_>>()
                            .join(","),
                        if *persistent { "<=" } else { "<-" },
                        b.source.key()
                    ))
                    .collect::<Vec<_>>()
                    .join(" & "),
                body.render()
            ),
            Term::New { names, body } => format!("new {} in {{{}}}", names.join(","), body.render()),
            Term::Drop(n) => format!("*{}", n.key()),
            Term::List(ts) => format!(
                "[{}]",
                ts.iter().map(|t| t.render()).collect::<Vec<_>>().join(",")
            ),
        }
    }
}

fn subst_name(n: &Name, env: &BTreeMap<String, Term>) -> Name {
    match n {
        Name::Var(v) => match env.get(v) {
            Some(Term::Drop(inner)) => (**inner).clone(),
            Some(t) => Name::quote(t.clone()),
            None => Name::Var(v.clone()),
        },
        Name::Quote(t) => Name::quote(t.substitute(env)),
    }
}

fn render_pattern(p: &Pattern) -> String {
    match p {
        Pattern::Wildcard => "_".into(),
        Pattern::Var(v) => v.clone(),
        Pattern::Exact(t) => t.render(),
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

/// Convenience: a channel named by a quoted string.
pub fn chan(s: &str) -> Name {
    Name::Quote(Box::new(Term::Ground(Ground::Str(s.to_string()))))
}

/// Convenience: a structured channel `@[tag, i, j]`, the reflective encoding
/// that lets a name predicate recover its indices (namespace logic).
pub fn structured(tag: &str, idx: &[i64]) -> Name {
    let mut parts = vec![Term::Ground(Ground::Str(tag.to_string()))];
    parts.extend(idx.iter().map(|i| Term::Ground(Ground::Int(*i))));
    Name::Quote(Box::new(Term::List(parts)))
}
