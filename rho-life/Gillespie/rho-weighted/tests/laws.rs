//! Conformance laws.
//!
//! Following the idiom of the branch's `cost_accounting::oslf` module: laws are
//! reusable functions that can be run against *any* implementation of a port,
//! so the same suite exercises the hand-written `RhoLogic` today and an
//! OSLF-generated logic when one lands.

use rho_weighted::examples::{birth_death, channel_keyed, comm_lhs, two_state};
use rho_weighted::logic::check::{sat_struct, successors};
use rho_weighted::logic::partition::{check_partition, PartitionError};
use rho_weighted::logic::{complete_with_default, Budget, Checkable, Formula};
use rho_weighted::matching::SimpleMatcher;
use rho_weighted::propensity::propensities;
use rho_weighted::space::Space;
use rho_weighted::syntax::{chan, Pattern, Term};
use rho_weighted::theory::{classify, open_gate, unit_geometry, Configuration};

fn b() -> Budget {
    Budget::default()
}

// ---------------------------------------------------------------------------
// Logic
// ---------------------------------------------------------------------------

/// Satisfaction respects structural congruence. If it did not, a configuration
/// would not be a state.
#[test]
fn law_congruence() {
    let f = Formula::sep(Formula::out(chan("a")), Formula::out(chan("b")));
    let t1 = Term::Par(vec![
        Term::send(chan("a"), vec![Term::Zero]),
        Term::send(chan("b"), vec![Term::Zero]),
        Term::Zero,
    ]);
    let t2 = Term::Par(vec![
        Term::Zero,
        Term::send(chan("b"), vec![Term::Zero]),
        Term::Par(vec![Term::send(chan("a"), vec![Term::Zero])]),
    ]);
    assert_eq!(t1.canonical(), t2.canonical());
    assert_eq!(
        sat_struct(&t1, &f, &mut b()).unwrap(),
        sat_struct(&t2, &f, &mut b()).unwrap()
    );
}

// ---------------------------------------------------------------------------
// Partition
// ---------------------------------------------------------------------------

/// Every redex classifies to exactly one key — total and single-valued. This
/// is what lets propensity be accumulated by redex rather than by key.
#[test]
fn law_partition_total() {
    let (theory, term) = birth_death(2.0, 1.0, 5);
    let space = Space::install(&term);
    for r in rho_weighted::enumerate(&space, &SimpleMatcher) {
        let c = classify(&theory, &space, &r, &mut b());
        assert!(c.is_some(), "unclassified redex {}", r.position());
    }
}

/// Adding `default` completes exhaustiveness. Exclusivity is the real
/// constraint; exhaustiveness is a completion.
#[test]
fn law_default_completes() {
    let keys = vec![Checkable::trusted(Formula::comm_on(chan("a")))];
    let p = complete_with_default(keys, &comm_lhs());
    assert_eq!(p.default_index, Some(1));

    // A redex on a channel with no key of its own lands in `default`.
    let t = Term::par(vec![
        Term::recv_persistent(chan("z"), vec![Pattern::Wildcard], Term::Zero),
        Term::send(chan("z"), vec![Term::Zero]),
    ]);
    assert_eq!(p.classify(&t, &mut b()), Some(1));

    // And one with a key lands in that key.
    let t2 = Term::par(vec![
        Term::recv_persistent(chan("a"), vec![Pattern::Wildcard], Term::Zero),
        Term::send(chan("a"), vec![Term::Zero]),
    ]);
    assert_eq!(p.classify(&t2, &mut b()), Some(0));
}

/// A non-exclusive key set is an error, and the diagnostic names the overlap.
/// It is never a silent aggregation convention, because each convention yields
/// a *different simulator from the same specification*.
#[test]
fn law_overlap_is_an_error_with_a_named_witness() {
    let overlapping = vec![
        Checkable::trusted(Formula::comm_on(chan("a"))),
        // Entailed by the first: every redex on `a` also satisfies this.
        Checkable::trusted(comm_lhs()),
    ];
    let p = complete_with_default(overlapping, &comm_lhs());
    let witness = Term::par(vec![
        Term::recv_persistent(chan("a"), vec![Pattern::Wildcard], Term::Zero),
        Term::send(chan("a"), vec![Term::Zero]),
    ]);
    match check_partition(&p, &comm_lhs(), &[witness], &mut b()) {
        Err(PartitionError::Overlap {
            i, j, witness: w, ..
        }) => {
            assert_eq!((i, j), (0, 1));
            assert!(!w.is_empty(), "the diagnostic must name the witness");
        }
        other => panic!("expected an overlap error, got {other:?}"),
    }
}

#[test]
fn law_exclusive_keys_pass_the_check() {
    let theory = channel_keyed(
        &[(chan("a"), 1.0), (chan("b"), 1.0)],
        unit_geometry(),
        open_gate(),
    );
    let witnesses: Vec<Term> = ["a", "b", "z"]
        .iter()
        .map(|c| {
            Term::par(vec![
                Term::recv_persistent(chan(c), vec![Pattern::Wildcard], Term::Zero),
                Term::send(chan(c), vec![Term::Zero]),
            ])
        })
        .collect();
    assert!(
        check_partition(
            &theory.rules[0].partition,
            &comm_lhs(),
            &witnesses,
            &mut b()
        )
        .is_ok(),
        "distinct channels are exclusive by construction"
    );
}

// ---------------------------------------------------------------------------
// Space
// ---------------------------------------------------------------------------

/// Firing is non-destructive. This is what makes branching cheap and
/// exhaustive graph construction possible at all.
#[test]
fn law_fire_is_pure() {
    let (_t, term) = two_state(1.0, 1.0);
    let s = Space::install(&term);
    let before = s.marking().key();
    let redexes = rho_weighted::enumerate(&s, &SimpleMatcher);
    for r in &redexes {
        let _ = s.fire(r);
    }
    assert_eq!(s.marking().key(), before, "fire must not mutate its source");
    assert_eq!(
        rho_weighted::enumerate(&s, &SimpleMatcher).len(),
        redexes.len(),
        "enumeration must be repeatable"
    );
}

#[test]
fn law_fork_is_observationally_the_identity() {
    let (_t, term) = birth_death(1.0, 1.0, 4);
    let s = Space::install(&term);
    let f = s.fork();
    assert_eq!(s.marking().key(), f.marking().key());
    // Mutating the fork leaves the original alone.
    let r = rho_weighted::enumerate(&f, &SimpleMatcher)[0].clone();
    let _ = f.fire(&r);
    assert_eq!(s.marking().key(), f.marking().key());
}

#[test]
fn law_enumeration_is_deterministic() {
    let (_t, term) = birth_death(1.0, 1.0, 4);
    let s = Space::install(&term);
    let a: Vec<String> = rho_weighted::enumerate(&s, &SimpleMatcher)
        .iter()
        .map(|r| r.position())
        .collect();
    let c: Vec<String> = rho_weighted::enumerate(&s, &SimpleMatcher)
        .iter()
        .map(|r| r.position())
        .collect();
    assert_eq!(a, c, "studies must be reproducible from (theory, seed)");
}

// ---------------------------------------------------------------------------
// Multiplicity
// ---------------------------------------------------------------------------

/// A binary interaction draws one reactant from each side, so the count is
/// `In · Out`. A persistent continuation is not consumed and therefore pairs
/// with *every* matching datum.
#[test]
fn law_multiplicity_counts_pairs() {
    for (ins, outs) in [(1usize, 1usize), (1, 5), (3, 1), (2, 4)] {
        let mut parts = Vec::new();
        for _ in 0..ins {
            parts.push(Term::recv(
                chan("a"),
                vec![Pattern::Wildcard],
                Term::Zero,
            ));
        }
        for _ in 0..outs {
            parts.push(Term::send(chan("a"), vec![Term::Zero]));
        }
        let s = Space::install(&Term::par(parts));
        assert_eq!(
            rho_weighted::enumerate(&s, &SimpleMatcher).len(),
            ins * outs,
            "In={ins}, Out={outs}"
        );
    }
}

/// A join needs a matching datum on every channel of its set, so the count is
/// the product over the set. This is why a threshold neuron is a join.
#[test]
fn law_join_multiplicity_is_the_product() {
    use rho_weighted::syntax::Bind;
    let binds = vec![
        Bind {
            patterns: vec![Pattern::Wildcard],
            source: chan("a"),
        },
        Bind {
            patterns: vec![Pattern::Wildcard],
            source: chan("b"),
        },
    ];
    let mut parts = vec![Term::join(binds, Term::Zero, true)];
    for _ in 0..3 {
        parts.push(Term::send(chan("a"), vec![Term::Zero]));
    }
    for _ in 0..2 {
        parts.push(Term::send(chan("b"), vec![Term::Zero]));
    }
    let s = Space::install(&Term::par(parts));
    assert_eq!(rho_weighted::enumerate(&s, &SimpleMatcher).len(), 3 * 2);
}

/// A join with an unpopulated channel offers nothing — the threshold is not met.
#[test]
fn law_join_needs_every_channel() {
    use rho_weighted::syntax::Bind;
    let binds = vec![
        Bind {
            patterns: vec![Pattern::Wildcard],
            source: chan("a"),
        },
        Bind {
            patterns: vec![Pattern::Wildcard],
            source: chan("b"),
        },
    ];
    let s = Space::install(&Term::par(vec![
        Term::join(binds, Term::Zero, true),
        Term::send(chan("a"), vec![Term::Zero]),
    ]));
    assert!(rho_weighted::enumerate(&s, &SimpleMatcher).is_empty());
}

// ---------------------------------------------------------------------------
// Propensity
// ---------------------------------------------------------------------------

/// Summing by key and summing by redex agree. Without exclusivity the by-redex
/// walk over-counts; without exhaustiveness it under-counts. Neither error is
/// visible in a trace.
#[test]
fn law_propensity_by_redex() {
    let (theory, term) = birth_death(2.0, 3.0, 6);
    let cfg = Configuration::new(Space::install(&term), theory.initial_weights());
    let p = propensities(&theory, &cfg, &SimpleMatcher, &mut b());

    let by_redex: f64 = p.items.iter().map(|w| w.rate).sum();
    let by_class: f64 = p
        .by_class(rho_weighted::COMM)
        .into_iter()
        .map(|(_, r)| r)
        .sum();
    assert!((by_redex - by_class).abs() < 1e-12);
    assert!((p.total - by_redex).abs() < 1e-12);
}

/// A zero-weight class contributes nothing — which is how the synthesised
/// `default` makes unlisted redexes inert without being absent.
#[test]
fn law_default_class_is_inert() {
    let theory = channel_keyed(&[(chan("a"), 1.0)], unit_geometry(), open_gate());
    let term = Term::par(vec![
        Term::recv_persistent(chan("z"), vec![Pattern::Wildcard], Term::Zero),
        Term::send(chan("z"), vec![Term::Zero]),
    ]);
    let cfg = Configuration::new(Space::install(&term), theory.initial_weights());
    let p = propensities(&theory, &cfg, &SimpleMatcher, &mut b());
    assert_eq!(p.items.len(), 1, "the redex is enumerated");
    assert!(p.is_absorbing(), "but contributes zero propensity");
}

/// An unfunded redex is inert. `a0 = 0` is an absorbing state, not a deadlock:
/// the waiting time to the next event is infinite rather than undefined.
#[test]
fn law_unfunded_is_inert() {
    use std::sync::Arc;
    let closed: rho_weighted::theory::Gate = Arc::new(|_, _| false);
    let theory = channel_keyed(&[(chan("a"), 5.0)], unit_geometry(), closed);
    let term = Term::par(vec![
        Term::recv_persistent(chan("a"), vec![Pattern::Wildcard], Term::Zero),
        Term::send(chan("a"), vec![Term::Zero]),
    ]);
    let cfg = Configuration::new(Space::install(&term), theory.initial_weights());
    let p = propensities(&theory, &cfg, &SimpleMatcher, &mut b());
    assert!(p.is_absorbing());
    assert_eq!(p.items.len(), 1, "the redex exists; it is only unfunded");
}

// ---------------------------------------------------------------------------
// Rates
// ---------------------------------------------------------------------------

/// Rates are in R>=0, not the unit interval. A waiting time carries units of
/// time; there is nothing to normalise a rate against.
#[test]
fn law_rates_are_nonnegative_reals_not_probabilities() {
    use rho_weighted::theory::{RateError, RateValue};
    assert!(RateValue::real(17.5).is_ok(), "rates above 1 are ordinary");
    assert_eq!(RateValue::real(-0.1), Err(RateError::Negative(-0.1)));
    assert!(matches!(
        RateValue::real(f64::INFINITY),
        Err(RateError::NotFinite(_))
    ));
    // Amplitudes, by contrast, are bounded.
    assert!(RateValue::complex(0.5, 0.5).is_ok());
    assert!(RateValue::complex(1.0, 1.0).is_err());
}

// ---------------------------------------------------------------------------
// The map is in the state
// ---------------------------------------------------------------------------

/// Two runs reaching the same term with different maps have different futures.
/// No rate function of the term alone reproduces both, which is why the weight
/// map is part of the configuration and the chain is over configurations.
#[test]
fn law_map_in_state() {
    use rho_weighted::explore::ssa::Sim;
    let (mut theory, term) = two_state(1.0, 1.0);
    // Make the `a` class potentiate on firing.
    theory.rules[0].entries[0].update = rho_weighted::theory::saturating_add(1.0, 100.0);

    let cfg = Configuration::new(Space::install(&term), theory.initial_weights());
    let mut sim = Sim::new(&theory, &SimpleMatcher, cfg, 7);

    let s1 = sim.step().expect("first step");
    let _ = sim.step().expect("second step");
    let s3 = sim.step().expect("third step");

    assert_eq!(
        s1.marking_key, s3.marking_key,
        "the run returns to the same term"
    );
    assert_ne!(
        s1.weights_fingerprint, s3.weights_fingerprint,
        "but not to the same configuration: the map moved"
    );
    assert!(
        s3.total_propensity > s1.total_propensity,
        "and the future differs: {} then {}",
        s1.total_propensity,
        s3.total_propensity
    );
}

// ---------------------------------------------------------------------------
// Successors and the modality agree
// ---------------------------------------------------------------------------

/// `successors` *is* the enumerator. A redex is a labelled transition.
#[test]
fn law_successors_are_the_redexes() {
    let (_t, term) = birth_death(1.0, 1.0, 4);
    let s = Space::install(&term);
    let e = rho_weighted::enumerate(&s, &SimpleMatcher);
    let succ = successors(&s, &SimpleMatcher);
    assert_eq!(e.len(), succ.len());
    for (i, (label, r, _)) in succ.iter().enumerate() {
        assert_eq!(*label, r.position());
        assert_eq!(r.position(), e[i].position());
    }
}
