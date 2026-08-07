//! Worked theories.
//!
//! Three families, each earning its place in the test suite:
//!
//! * [`two_state`] and [`birth_death`] — chains with closed-form stationary
//!   distributions, so the generator and the sampler can both be checked
//!   against arithmetic rather than against each other.
//! * [`channel_keyed`] — the SPiM restriction of the construction: channel
//!   identity as the refinement, identity updates, unit geometry, open gate.
//! * [`rnn`] — the note's §8 worked example.

use crate::logic::formula::{Checkable, NamePred};
use crate::logic::{complete_with_default, Formula, Partition};
use crate::redex::{RuleId, COMM};
use crate::syntax::{chan, structured, Bind, Ground, Name, Pattern, Term};
use crate::theory::{
    id_update, open_gate, saturating_add, unit_geometry, BaseRule, GeometricFactor, Gate,
    RateValue, RefinementEntry, WeightedTheory,
};

/// The left-hand side shape of the communication rule: a receipt beside a send.
pub fn comm_lhs() -> Formula {
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

/// Build a theory whose refinement keys are channel identities — the shape
/// Theorem 22 needs, and the shape for which exclusivity holds by construction
/// (distinct channels give disjoint sets of communication redexes).
pub fn channel_keyed(
    channels: &[(Name, f64)],
    geometric: GeometricFactor,
    gate: Gate,
) -> WeightedTheory {
    let keys: Vec<Checkable> = channels
        .iter()
        .map(|(c, _)| Checkable::trusted(Formula::comm_on(c.clone())))
        .collect();
    let partition: Partition = complete_with_default(keys, &comm_lhs());

    let mut entries: Vec<RefinementEntry> = channels
        .iter()
        .map(|(c, r)| RefinementEntry {
            weight: RateValue::real(*r).expect("rate must be finite and non-negative"),
            update: id_update(),
            label: format!("comm_on({})", c.key()),
        })
        .collect();
    // The synthesised `default` class: present so the partition is exhaustive,
    // weighted zero so unlisted redexes are inert.
    entries.push(RefinementEntry {
        weight: RateValue::Real(0.0),
        update: id_update(),
        label: "default".into(),
    });

    WeightedTheory {
        rules: vec![BaseRule {
            id: COMM,
            name: "COMM".into(),
            lhs_shape: comm_lhs(),
            partition,
            entries,
        }],
        geometric,
        gate,
    }
}

/// A two-state chain: `a ⇄ b`, each side a persistent receipt that re-emits on
/// the other channel. Stationary distribution `(r_ba, r_ab)/(r_ab + r_ba)`.
pub fn two_state(r_ab: f64, r_ba: f64) -> (WeightedTheory, Term) {
    let a = chan("a");
    let b = chan("b");
    let term = Term::par(vec![
        Term::recv_persistent(
            a.clone(),
            vec![Pattern::Wildcard],
            Term::send(b.clone(), vec![Term::Zero]),
        ),
        Term::recv_persistent(
            b.clone(),
            vec![Pattern::Wildcard],
            Term::send(a.clone(), vec![Term::Zero]),
        ),
        Term::send(a.clone(), vec![Term::Zero]),
    ]);
    let theory = channel_keyed(&[(a, r_ab), (b, r_ba)], unit_geometry(), open_gate());
    (theory, term)
}

/// A birth--death chain on one channel with capacity `cap`: a persistent
/// producer emits, a persistent consumer absorbs. With birth rate `lambda` and
/// death rate `mu` the occupancy is a truncated geometric with ratio
/// `lambda/mu` — and crucially the death propensity is *proportional to
/// occupancy*, which is exactly the multiplicity factor. A simulator that omits
/// multiplicity gets a flat distribution instead and fails the test.
pub fn birth_death(lambda: f64, mu: f64, cap: usize) -> (WeightedTheory, Term) {
    let tick = chan("tick");
    let pool = chan("pool");
    let term = Term::par(vec![
        // birth: consume a tick, emit into the pool and a fresh tick
        Term::recv_persistent(
            tick.clone(),
            vec![Pattern::Wildcard],
            Term::par(vec![
                Term::send(pool.clone(), vec![Term::Zero]),
                Term::send(tick.clone(), vec![Term::Zero]),
            ]),
        ),
        // death: consume from the pool
        Term::recv_persistent(pool.clone(), vec![Pattern::Wildcard], Term::Zero),
        Term::send(tick.clone(), vec![Term::Zero]),
    ]);
    let theory = channel_keyed(
        &[(tick, lambda), (pool.clone(), mu)],
        crate::theory::capacity(cap, vec![pool.key()]),
        open_gate(),
    );
    (theory, term)
}

// ---------------------------------------------------------------------------
// The recurrent neural network (note, §8)
// ---------------------------------------------------------------------------

/// A synapse channel `@[syn, j, i]` — a *structured* name, so a name predicate
/// can recover its endpoints and a single formula can describe the whole
/// namespace of synapses (note, §8.8). In an atomic-name calculus the indices
/// would have to travel as payloads and the logic could reach them only by
/// observing traffic.
pub fn synapse(j: usize, i: usize) -> Name {
    structured("syn", &[j as i64, i as i64])
}

pub struct NetSpec {
    /// `post[i]` = the neurons neuron `i` projects to.
    pub post: Vec<Vec<usize>>,
    /// Threshold per neuron.
    pub theta: Vec<usize>,
    /// Initial pending spikes per synapse.
    pub initial: Vec<((usize, usize), usize)>,
}

/// A neuron is a **persistent** for-comprehension whose join structure is its
/// threshold (note, Prop. 41 — McCulloch--Pitts).
///
/// Persistence is what makes the object a standing structure rather than a
/// trace through one: a linear receipt evaporates after a single firing, so a
/// network built from linear receipts models one sweep, not a network.
pub fn neuron(spec: &NetSpec, i: usize, presyn: &[usize]) -> Term {
    let body = Term::par(
        spec.post[i]
            .iter()
            .map(|&k| Term::send(synapse(i, k), vec![Term::Ground(Ground::Str("spk".into()))]))
            .collect(),
    );
    let theta = spec.theta[i].max(1).min(presyn.len().max(1));

    // One join group per theta-subset of the dendrites.
    let groups = subsets(presyn, theta);
    if groups.len() == 1 {
        let binds: Vec<Bind> = groups[0]
            .iter()
            .map(|&j| Bind {
                patterns: vec![Pattern::Wildcard],
                source: synapse(j, i),
            })
            .collect();
        return Term::join(binds, body, true);
    }
    // Alternatives (`;`) are separate persistent receipts over the same body.
    Term::par(
        groups
            .into_iter()
            .map(|g| {
                let binds: Vec<Bind> = g
                    .iter()
                    .map(|&j| Bind {
                        patterns: vec![Pattern::Wildcard],
                        source: synapse(j, i),
                    })
                    .collect();
                Term::join(binds, body.clone(), true)
            })
            .collect(),
    )
}

fn subsets(xs: &[usize], k: usize) -> Vec<Vec<usize>> {
    if k == 0 {
        return vec![vec![]];
    }
    if xs.len() < k {
        return vec![];
    }
    let mut out = Vec::new();
    let (head, tail) = xs.split_at(1);
    for mut s in subsets(tail, k - 1) {
        s.insert(0, head[0]);
        out.push(s);
    }
    out.extend(subsets(tail, k));
    out
}

/// Assemble the network term and a weighted theory whose refinement keys are
/// the synapses. Synaptic efficacies live in the **weight map**, not in any
/// payload: firing is a stochastic event rather than an arithmetic one, the
/// response nonlinearity is a property of the chain rather than a function
/// written down anywhere, and plasticity is the update functions.
pub fn rnn(
    spec: &NetSpec,
    efficacy: impl Fn(usize, usize) -> f64,
    plastic: Option<(f64, f64)>,
    cap: usize,
) -> (WeightedTheory, Term, Vec<(usize, usize)>) {
    let n = spec.post.len();
    // Presynaptic sets, inverted from `post`.
    let mut pre: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (j, targets) in spec.post.iter().enumerate() {
        for &i in targets {
            pre[i].push(j);
        }
    }

    let mut parts: Vec<Term> = Vec::new();
    for i in 0..n {
        if pre[i].is_empty() {
            continue;
        }
        parts.push(neuron(spec, i, &pre[i]));
    }
    for ((j, i), count) in &spec.initial {
        for _ in 0..*count {
            parts.push(Term::send(
                synapse(*j, *i),
                vec![Term::Ground(Ground::Str("spk".into()))],
            ));
        }
    }
    let term = Term::par(parts);

    // One key per synapse. Exclusive by construction: distinct synapses are
    // distinct channels.
    let mut synapses: Vec<(usize, usize)> = Vec::new();
    for (j, targets) in spec.post.iter().enumerate() {
        for &i in targets {
            synapses.push((j, i));
        }
    }
    synapses.sort();

    let keys: Vec<Checkable> = synapses
        .iter()
        .map(|(j, i)| Checkable::trusted(Formula::comm_on(synapse(*j, *i))))
        .collect();
    let partition = complete_with_default(keys, &comm_lhs());

    let mut entries: Vec<RefinementEntry> = synapses
        .iter()
        .map(|(j, i)| RefinementEntry {
            weight: RateValue::real(efficacy(*j, *i)).expect("efficacy must be a valid rate"),
            update: match plastic {
                Some((eta, w_max)) => saturating_add(eta, w_max),
                None => id_update(),
            },
            label: format!("syn({j},{i})"),
        })
        .collect();
    entries.push(RefinementEntry {
        weight: RateValue::Real(0.0),
        update: id_update(),
        label: "default".into(),
    });

    // Capacity is a *geometric* factor, not a term-level guard: it depends on
    // the surrounding configuration rather than on the redex's own shape, which
    // is precisely what `g(k)` is for. Zero factor on a saturated target keeps
    // the reachable state space finite, which is what exhaustive exploration
    // and the quantum construction need.
    let all: Vec<String> = synapses.iter().map(|(j, i)| synapse(*j, *i).key()).collect();
    let geometric: GeometricFactor = crate::theory::capacity(cap, all);

    let theory = WeightedTheory {
        rules: vec![BaseRule {
            id: COMM,
            name: "COMM".into(),
            lhs_shape: comm_lhs(),
            partition,
            entries,
        }],
        geometric,
        gate: open_gate(),
    };
    (theory, term, synapses)
}

/// The index of a synapse's refinement class, for reading its weight out of the
/// map.
pub fn synapse_class(synapses: &[(usize, usize)], j: usize, i: usize) -> Option<(RuleId, usize)> {
    synapses.iter().position(|s| *s == (j, i)).map(|k| (COMM, k))
}
