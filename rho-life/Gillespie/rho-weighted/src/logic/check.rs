//! Satisfaction.
//!
//! Two judgments, deliberately separated.
//!
//! * [`sat_struct`] handles everything below the modality, against a *term*.
//!   Refinement keys are evaluated here, on the instance `Lσ` of a rule's
//!   left-hand side.
//! * [`sat`] handles the whole logic, against a *space*, because the
//!   behavioural modality quantifies over transitions and a term has none.
//!
//! `successors` is the redex enumerator. A redex is a labelled transition, so
//! `⟨K_j⟩φ` at modal depth 1 costs one enumeration plus one structural check
//! per successor. This is the payoff of the simulator being a separate
//! execution path: producing *all* one-step successors without committing to
//! any is trivial against an immutable space and structurally impossible for
//! the interpreter, whose next state is whatever RSpace and tokio decided.

use std::collections::{BTreeMap, BTreeSet};

use crate::logic::formula::{Budget, CheckError, Cmp, Formula, NamePred, PosQuant};
use crate::matching::Matching;
use crate::redex::{enumerate, Redex};
use crate::space::Space;
use crate::syntax::{Name, Term};

/// Environment for fixed-point variables: variable to the set of marking keys
/// currently believed to satisfy it.
type FixEnv = BTreeMap<String, BTreeSet<String>>;

// ---------------------------------------------------------------------------
// Structural fragment
// ---------------------------------------------------------------------------

/// `t ⊨ φ` for the structural and propositional layers.
///
/// The separating conjunction is the interesting case: it splits the multiset
/// of parallel components every way and asks whether some split satisfies both
/// sides. That is exponential in the number of components, which is why keys
/// carry a depth bound and why the budget is threaded through.
pub fn sat_struct(t: &Term, f: &Formula, b: &mut Budget) -> Result<bool, CheckError> {
    b.spend(1)?;
    Ok(match f {
        Formula::Top => true,
        Formula::Bot => false,
        Formula::Zero => matches!(t.canonical(), Term::Zero),
        Formula::Eq(u) => t.canonical() == u.canonical(),
        Formula::ListAt { index, body } => match t.canonical() {
            Term::List(items) => match items.get(*index) {
                Some(item) => sat_struct(item, body, b)?,
                None => false,
            },
            _ => false,
        },
        Formula::And(x, y) => sat_struct(t, x, b)? && sat_struct(t, y, b)?,
        Formula::Or(x, y) => sat_struct(t, x, b)? || sat_struct(t, y, b)?,
        Formula::Not(x) => !sat_struct(t, x, b)?,

        Formula::Out { chan, body } => {
            let mut found = false;
            for c in t.components() {
                if let Term::Send {
                    chan: ch, data, ..
                } = &c
                {
                    if name_sat(ch, chan, b)? {
                        let payload = Term::Par(data.clone()).canonical();
                        if sat_struct(&payload, body, b)? {
                            found = true;
                            break;
                        }
                    }
                }
            }
            found
        }

        Formula::In { chan, body } => {
            let mut found = false;
            for c in t.components() {
                if let Term::Receive { binds, body: k, .. } = &c {
                    for bind in binds {
                        if name_sat(&bind.source, chan, b)? && sat_struct(k, body, b)? {
                            found = true;
                            break;
                        }
                    }
                }
                if found {
                    break;
                }
            }
            found
        }

        Formula::Count { chan, cmp, n } => {
            let mut count = 0usize;
            for c in t.components() {
                if let Term::Send { chan: ch, .. } = &c {
                    if name_sat(ch, chan, b)? {
                        count += 1;
                    }
                }
            }
            match cmp {
                Cmp::Ge => count >= *n,
                Cmp::Le => count <= *n,
                Cmp::Eq => count == *n,
            }
        }

        Formula::Sep(x, y) => {
            let comps = t.components();
            let n = comps.len();
            if n > 20 {
                // Beyond this the split enumeration is not a checkable
                // fragment; refuse rather than hang.
                return Err(CheckError::BudgetExhausted);
            }
            let mut ok = false;
            for mask in 0u32..(1u32 << n) {
                b.spend(1)?;
                let mut left = Vec::new();
                let mut right = Vec::new();
                for (i, c) in comps.iter().enumerate() {
                    if mask & (1 << i) != 0 {
                        left.push(c.clone());
                    } else {
                        right.push(c.clone());
                    }
                }
                let lt = Term::Par(left).canonical();
                let rt = Term::Par(right).canonical();
                if sat_struct(&lt, x, b)? && sat_struct(&rt, y, b)? {
                    ok = true;
                    break;
                }
            }
            ok
        }

        // Behavioural formers have no meaning on a bare term.
        Formula::Dia { .. } | Formula::Boxm { .. } | Formula::Nu { .. } | Formula::Var(_) => false,
    })
}

/// `n ⊨ ν` for name predicates. `NamePred::Quote` is namespace logic: a name
/// predicate sees into the structure of the name, because a name is a quoted
/// process.
fn name_sat(n: &Name, p: &NamePred, b: &mut Budget) -> Result<bool, CheckError> {
    b.spend(1)?;
    Ok(match p {
        NamePred::Any => true,
        NamePred::Exactly(m) => n.key() == m.key(),
        NamePred::Quote(f) => match n {
            Name::Quote(inner) => sat_struct(inner, f, b)?,
            Name::Var(_) => false,
        },
    })
}

// ---------------------------------------------------------------------------
// Full logic, over spaces
// ---------------------------------------------------------------------------

/// All one-step successors, with their labels. This *is* `enumerate`.
pub fn successors<M: Matching>(space: &Space, m: &M) -> Vec<(String, Redex, Space)> {
    enumerate(space, m)
        .into_iter()
        .map(|r| {
            let label = r.position();
            let next = space.fire(&r);
            (label, r, next)
        })
        .collect()
}

/// `s ⊨ φ`.
pub fn sat<M: Matching>(
    space: &Space,
    f: &Formula,
    m: &M,
    b: &mut Budget,
) -> Result<bool, CheckError> {
    let env = FixEnv::new();
    sat_env(space, f, m, b, &env)
}

fn sat_env<M: Matching>(
    space: &Space,
    f: &Formula,
    m: &M,
    b: &mut Budget,
    env: &FixEnv,
) -> Result<bool, CheckError> {
    b.spend(1)?;
    match f {
        Formula::Top => Ok(true),
        Formula::Bot => Ok(false),
        Formula::And(x, y) => Ok(sat_env(space, x, m, b, env)? && sat_env(space, y, m, b, env)?),
        Formula::Or(x, y) => Ok(sat_env(space, x, m, b, env)? || sat_env(space, y, m, b, env)?),
        Formula::Not(x) => Ok(!sat_env(space, x, m, b, env)?),

        Formula::Var(v) => match env.get(v) {
            Some(set) => Ok(set.contains(&space.marking().key())),
            None => Err(CheckError::UnboundVariable(v.clone())),
        },

        Formula::Dia { pos, body, .. } => {
            for (_label, r, next) in successors(space, m) {
                if !pos_matches(pos, &r) {
                    continue;
                }
                if sat_env(&next, body, m, b, env)? {
                    return Ok(true);
                }
            }
            Ok(false)
        }

        Formula::Boxm { pos, body, .. } => {
            for (_label, r, next) in successors(space, m) {
                if !pos_matches(pos, &r) {
                    continue;
                }
                if !sat_env(&next, body, m, b, env)? {
                    return Ok(false);
                }
            }
            Ok(true)
        }

        Formula::Nu { var, body } => {
            // Knaster--Tarski by iteration from the top: start with the whole
            // reachable set and shrink to the greatest post-fixed point. This
            // is the first casualty of a terminating fragment, which is why
            // `Checkable::try_new` refuses it without a finite state space.
            let reachable = reachable_set(space, m, b)?;
            let mut current: BTreeSet<String> = reachable.keys().cloned().collect();
            loop {
                b.spend(current.len() as u64 + 1)?;
                let mut next = BTreeSet::new();
                let mut env2 = env.clone();
                env2.insert(var.clone(), current.clone());
                for key in &current {
                    let s = &reachable[key];
                    if sat_env(s, body, m, b, &env2)? {
                        next.insert(key.clone());
                    }
                }
                if next == current {
                    break;
                }
                current = next;
            }
            Ok(current.contains(&space.marking().key()))
        }

        // Structural formers descend to the term reading of the space.
        _ => sat_struct(&space.to_term(), f, b),
    }
}

fn pos_matches(p: &PosQuant, r: &Redex) -> bool {
    match p {
        PosQuant::Exists | PosQuant::Forall => true,
        PosQuant::At(c) => r.channels().iter().any(|k| k == c),
    }
}

/// The reachable configurations, keyed by marking. Bounded by the budget; an
/// exploration that hits the bound reports it rather than truncating silently.
pub fn reachable_set<M: Matching>(
    space: &Space,
    m: &M,
    b: &mut Budget,
) -> Result<BTreeMap<String, Space>, CheckError> {
    let mut seen: BTreeMap<String, Space> = BTreeMap::new();
    let mut frontier = vec![space.clone()];
    seen.insert(space.marking().key(), space.clone());
    while let Some(s) = frontier.pop() {
        if seen.len() as u32 > b.states {
            return Err(CheckError::StateSpaceExceeded);
        }
        for (_l, _r, next) in successors(&s, m) {
            let k = next.marking().key();
            if !seen.contains_key(&k) {
                seen.insert(k, next.clone());
                frontier.push(next);
            }
        }
    }
    Ok(seen)
}
