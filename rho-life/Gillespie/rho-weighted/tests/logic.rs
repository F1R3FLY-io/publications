//! The behavioural connective, and the discipline around it.
//!
//! `⟨K_j⟩` is indexed by rewrite rule *and choice of redex position*. The
//! index is not decoration: an action-labelled logic sees one label where this
//! one sees as many as there are enabled interactions. `successors` is the
//! redex enumerator, so a modality costs one enumeration per level — which is
//! why modal depth is budgeted and why `Checkable` refuses a formula it cannot
//! afford, naming the subformula rather than hanging.

use rho_weighted::examples::two_state;
use rho_weighted::logic::check::{sat, sat_struct, successors};
use rho_weighted::logic::formula::{Checkable, Cmp, NamePred, WhyNot};
use rho_weighted::logic::partition::complete_with_default_checked;
use rho_weighted::logic::{Budget, Formula};
use rho_weighted::matching::SimpleMatcher;
use rho_weighted::space::Space;
use rho_weighted::syntax::{chan, structured, Ground, Pattern, Term};

fn budget() -> Budget {
    Budget::default()
}

// ---------------------------------------------------------------------------
// The modality
// ---------------------------------------------------------------------------

#[test]
fn dia_top_holds_exactly_when_a_step_is_enabled() {
    let (_t, term) = two_state(1.0, 1.0);
    let live = Space::install(&term);
    assert!(!rho_weighted::enumerate(&live, &SimpleMatcher).is_empty());
    assert!(sat(&live, &Formula::dia(Formula::Top), &SimpleMatcher, &mut budget()).unwrap());

    // A send with no receiver: structurally non-trivial, behaviourally stuck.
    let stuck = Space::install(&Term::send(chan("a"), vec![Term::Zero]));
    assert!(rho_weighted::enumerate(&stuck, &SimpleMatcher).is_empty());
    assert!(!sat(&stuck, &Formula::dia(Formula::Top), &SimpleMatcher, &mut budget()).unwrap());
}

#[test]
fn box_is_dual_to_dia() {
    let (_t, term) = two_state(1.0, 1.0);
    let s = Space::install(&term);
    // [K]F is false when a step exists; <K>T is true.
    assert!(!sat(&s, &Formula::boxm(Formula::Bot), &SimpleMatcher, &mut budget()).unwrap());
    // Vacuously true at a stuck state.
    let stuck = Space::install(&Term::send(chan("a"), vec![Term::Zero]));
    assert!(sat(&stuck, &Formula::boxm(Formula::Bot), &SimpleMatcher, &mut budget()).unwrap());
}

#[test]
fn the_modality_is_context_labelled_not_action_labelled() {
    // Two pending messages on one channel give two *positions*, hence two
    // labelled transitions, even though there is only one "action".
    let term = Term::par(vec![
        Term::recv_persistent(chan("a"), vec![Pattern::Wildcard], Term::Zero),
        Term::send(chan("a"), vec![Term::Zero]),
        Term::send(chan("a"), vec![Term::Zero]),
    ]);
    let s = Space::install(&term);
    let succ = successors(&s, &SimpleMatcher);
    assert_eq!(succ.len(), 2, "two positions, two labelled transitions");
    let labels: Vec<&str> = succ.iter().map(|(l, _, _)| l.as_str()).collect();
    assert_ne!(labels[0], labels[1], "positions must be distinguishable");
}

#[test]
fn dia_at_a_position_selects_by_channel() {
    let term = Term::par(vec![
        Term::recv_persistent(chan("a"), vec![Pattern::Wildcard], Term::Zero),
        Term::recv_persistent(chan("b"), vec![Pattern::Wildcard], Term::Zero),
        Term::send(chan("a"), vec![Term::Zero]),
    ]);
    let s = Space::install(&term);
    let at_a = Formula::Dia {
        rule: rho_weighted::COMM,
        pos: rho_weighted::logic::PosQuant::At(chan("a").key()),
        body: Box::new(Formula::Top),
    };
    let at_b = Formula::Dia {
        rule: rho_weighted::COMM,
        pos: rho_weighted::logic::PosQuant::At(chan("b").key()),
        body: Box::new(Formula::Top),
    };
    assert!(sat(&s, &at_a, &SimpleMatcher, &mut budget()).unwrap());
    assert!(!sat(&s, &at_b, &SimpleMatcher, &mut budget()).unwrap());
}

#[test]
fn nested_modalities_look_two_steps_ahead() {
    let (_t, term) = two_state(1.0, 1.0);
    let s = Space::install(&term);
    // The chain is a 2-cycle, so any depth is reachable.
    let f = Formula::dia(Formula::dia(Formula::dia(Formula::Top)));
    assert!(sat(&s, &f, &SimpleMatcher, &mut budget()).unwrap());

    // A one-shot: linear receive, so exactly one step is available.
    let once = Term::par(vec![
        Term::recv(chan("a"), vec![Pattern::Wildcard], Term::Zero),
        Term::send(chan("a"), vec![Term::Zero]),
    ]);
    let s1 = Space::install(&once);
    assert!(sat(&s1, &Formula::dia(Formula::Top), &SimpleMatcher, &mut budget()).unwrap());
    assert!(!sat(
        &s1,
        &Formula::dia(Formula::dia(Formula::Top)),
        &SimpleMatcher,
        &mut budget()
    )
    .unwrap());
}

// ---------------------------------------------------------------------------
// The greatest fixed point
// ---------------------------------------------------------------------------

#[test]
fn nu_decides_liveness() {
    // nu X . <K> X — "a step is always available".
    let live_forever = Formula::Nu {
        var: "X".into(),
        body: Box::new(Formula::dia(Formula::Var("X".into()))),
    };

    let (_t, term) = two_state(1.0, 1.0);
    let live = Space::install(&term);
    assert!(
        sat(&live, &live_forever, &SimpleMatcher, &mut budget()).unwrap(),
        "the two-state cycle is live"
    );

    let once = Term::par(vec![
        Term::recv(chan("a"), vec![Pattern::Wildcard], Term::Zero),
        Term::send(chan("a"), vec![Term::Zero]),
    ]);
    let dying = Space::install(&once);
    assert!(
        !sat(&dying, &live_forever, &SimpleMatcher, &mut budget()).unwrap(),
        "a one-shot reaches a stuck state, so liveness fails"
    );
}

// ---------------------------------------------------------------------------
// The ideal / checkable split, as a type
// ---------------------------------------------------------------------------

#[test]
fn an_over_deep_key_is_refused_at_construction_with_the_subformula_named() {
    let b = Budget {
        modal_depth: 2,
        ..Budget::default()
    };
    let f = Formula::dia(Formula::dia(Formula::dia(Formula::Top)));
    match Checkable::try_new(f, &b, true) {
        Err(WhyNot::TooDeep { got, budget, at }) => {
            assert_eq!(got, 3);
            assert_eq!(budget, 2);
            assert!(at.contains("<K>"), "diagnostic must name the subformula: {at}");
        }
        other => panic!("expected refusal, got {other:?}"),
    }
}

#[test]
fn a_fixed_point_needs_a_finite_state_space() {
    let f = Formula::Nu {
        var: "X".into(),
        body: Box::new(Formula::dia(Formula::Var("X".into()))),
    };
    assert!(matches!(
        Checkable::try_new(f.clone(), &Budget::default(), false),
        Err(WhyNot::UnboundedFixpoint { .. })
    ));
    assert!(Checkable::try_new(f, &Budget::default(), true).is_ok());
}

#[test]
fn a_negative_fixed_point_variable_is_refused() {
    let f = Formula::Nu {
        var: "X".into(),
        body: Box::new(Formula::not(Formula::Var("X".into()))),
    };
    assert!(matches!(
        Checkable::try_new(f, &Budget::default(), true),
        Err(WhyNot::NegativeFixpointVariable { .. })
    ));
}

#[test]
fn the_budget_stops_a_check_rather_than_hanging() {
    let mut tiny = Budget {
        inference_tokens: 3,
        ..Budget::default()
    };
    let (_t, term) = two_state(1.0, 1.0);
    let s = Space::install(&term);
    let deep = Formula::dia(Formula::dia(Formula::dia(Formula::dia(Formula::Top))));
    assert!(sat(&s, &deep, &SimpleMatcher, &mut tiny).is_err());
}

// ---------------------------------------------------------------------------
// The structural layer
// ---------------------------------------------------------------------------

#[test]
fn separating_conjunction_splits_the_multiset() {
    let t = Term::par(vec![
        Term::send(chan("a"), vec![Term::Zero]),
        Term::send(chan("b"), vec![Term::Zero]),
    ]);
    let f = Formula::sep(Formula::out(chan("a")), Formula::out(chan("b")));
    assert!(sat_struct(&t, &f, &mut budget()).unwrap());

    // One message cannot satisfy both sides of a separating conjunction.
    let single = Term::send(chan("a"), vec![Term::Zero]);
    let both_a = Formula::sep(Formula::out(chan("a")), Formula::out(chan("a")));
    assert!(!sat_struct(&single, &both_a, &mut budget()).unwrap());
}

#[test]
fn count_is_sugar_for_iterated_separation() {
    let t = Term::par(vec![
        Term::send(chan("a"), vec![Term::Zero]),
        Term::send(chan("a"), vec![Term::Zero]),
        Term::send(chan("a"), vec![Term::Zero]),
    ]);
    let ge3 = Formula::Count {
        chan: NamePred::Exactly(chan("a")),
        cmp: Cmp::Ge,
        n: 3,
    };
    let ge4 = Formula::Count {
        chan: NamePred::Exactly(chan("a")),
        cmp: Cmp::Ge,
        n: 4,
    };
    assert!(sat_struct(&t, &ge3, &mut budget()).unwrap());
    assert!(!sat_struct(&t, &ge4, &mut budget()).unwrap());

    // Agreement with the nested form it abbreviates.
    let nested = Formula::sep(
        Formula::out(chan("a")),
        Formula::sep(Formula::out(chan("a")), Formula::out(chan("a"))),
    );
    assert!(sat_struct(&t, &nested, &mut budget()).unwrap());
}

#[test]
fn a_name_predicate_sees_into_the_structure_of_a_name() {
    // Namespace logic: one formula for a whole family of structured channels.
    // In an atomic-name calculus the indices would have to travel as payloads
    // and the logic could reach them only by observing traffic.
    let syn = structured("syn", &[0, 1]);
    let other = structured("axon", &[0, 1]);
    let tag = Term::Ground(Ground::Str("syn".into()));

    let t = Term::par(vec![
        Term::recv_persistent(syn.clone(), vec![Pattern::Wildcard], Term::Zero),
        Term::send(syn, vec![Term::Zero]),
    ]);
    let f = Formula::comm_in_namespace(tag.clone());
    assert!(
        sat_struct(&t, &f, &mut budget()).unwrap(),
        "the synapse namespace predicate must hold"
    );

    let u = Term::par(vec![
        Term::recv_persistent(other.clone(), vec![Pattern::Wildcard], Term::Zero),
        Term::send(other, vec![Term::Zero]),
    ]);
    assert!(
        !sat_struct(&u, &f, &mut budget()).unwrap(),
        "and must not hold of a different namespace"
    );
}

#[test]
fn one_namespace_key_covers_a_growing_family() {
    let tag = Term::Ground(Ground::Str("syn".into()));
    let f = Formula::comm_in_namespace(tag);
    // Same single formula, three different synapses — the uniformity that
    // makes a classifier a classifier rather than something regenerated per
    // input.
    for (j, i) in [(0, 1), (1, 0), (7, 42)] {
        let c = structured("syn", &[j, i]);
        let t = Term::par(vec![
            Term::recv_persistent(c.clone(), vec![Pattern::Wildcard], Term::Zero),
            Term::send(c, vec![Term::Zero]),
        ]);
        assert!(sat_struct(&t, &f, &mut budget()).unwrap(), "syn({j},{i})");
    }
}

// ---------------------------------------------------------------------------
// (R3), structurality: what may be a key, as against what may be a property
// ---------------------------------------------------------------------------

/// The structural fragment is where the locality lemma is true.
///
/// `⟨K_j⟩φ` inspects the *successors* of a term, and `ν` may be refuted only by
/// unbounded inspection, so neither is confined by a depth bound. Admitting
/// either as a key silently costs a global reclassification on every step —
/// which is what this crate used to do, while the note claimed locality for
/// keys in general. (R3) is the requirement that closes the gap, and it is a
/// restriction on keys alone: both remain available as properties.
#[test]
fn the_structural_fragment_is_exactly_the_local_one() {
    let structural = [
        Formula::Top,
        Formula::comm_on(chan("a")),
        Formula::not(Formula::comm_on(chan("a"))),
        Formula::and(Formula::comm_on(chan("a")), Formula::Top),
        Formula::comm_in_namespace(Term::Ground(Ground::Str("syn".into()))),
    ];
    for f in structural {
        assert!(f.is_structural(), "must be admissible as a key: {}", f.render());
    }

    let nonstructural = [
        Formula::dia(Formula::Top),
        Formula::Nu {
            var: "X".into(),
            body: Box::new(Formula::dia(Formula::Var("X".into()))),
        },
        // Buried, so the walk has to descend rather than pattern-match the root.
        Formula::and(Formula::comm_on(chan("a")), Formula::dia(Formula::Top)),
        Formula::not(Formula::Sep(
            Box::new(Formula::Top),
            Box::new(Formula::dia(Formula::Top)),
        )),
    ];
    for f in nonstructural {
        assert!(!f.is_structural(), "must be refused as a key: {}", f.render());
        assert!(f.nonstructural_at().is_some());
    }
}

/// A modal formula is refused as a key and accepted as a property. Both halves
/// matter: refusing it everywhere would cost the logic its modalities, which is
/// not what the requirement says.
#[test]
fn a_modal_formula_is_a_property_but_not_a_key() {
    let f = Formula::dia(Formula::comm_on(chan("a")));
    let b = Budget::default();

    assert!(
        Checkable::try_new(f.clone(), &b, true).is_ok(),
        "still a perfectly good property"
    );
    match Checkable::try_key(f, &b, true) {
        Err(WhyNot::NotStructural { at }) => {
            assert!(at.contains("<K>"), "diagnostic must name the subformula: {at}");
        }
        other => panic!("expected an (R3) refusal, got {other:?}"),
    }
}

/// (R3) is enforced where partitions are assembled, which is what makes it an
/// *elaboration-time* requirement rather than an aspiration. Every partition in
/// the crate is built through this funnel, so a modal key cannot reach a theory
/// by any route.
#[test]
fn a_modal_key_cannot_reach_a_theory() {
    let lhs = Formula::comm_on(chan("a"));
    let good = vec![Checkable::trusted(Formula::comm_on(chan("a")))];
    assert!(complete_with_default_checked(good, &lhs).is_ok());

    // `trusted` bypasses the fragment check, so this is the adversarial case:
    // a key that lied about itself on the way in.
    let bad = vec![Checkable::trusted(Formula::dia(Formula::Top))];
    match complete_with_default_checked(bad, &lhs) {
        Err(WhyNot::NotStructural { at }) => assert!(at.contains("<K>"), "{at}"),
        other => panic!("expected an (R3) refusal at partition construction, got {other:?}"),
    }
}

/// The synthesised `default` key is structural whenever the supplied keys and
/// the left-hand side are, so completion never smuggles in a violation.
#[test]
fn completion_preserves_structurality() {
    let lhs = Formula::comm_on(chan("a"));
    let p = complete_with_default_checked(
        vec![Checkable::trusted(Formula::comm_on(chan("a")))],
        &lhs,
    )
    .unwrap();
    assert!(p.check_key_fragment().is_ok());
    assert!(p.keys()[p.default_index().unwrap()].formula().is_structural());
}

/// The shipped example theories satisfy (R3). A regression that introduced a
/// modal key into `examples.rs` would otherwise only show up as a slow
/// simulator.
#[test]
fn the_shipped_examples_satisfy_r3() {
    let (theory, _term) = two_state(2.0, 1.0);
    for r in &theory.rules {
        assert!(
            r.partition.check_key_fragment().is_ok(),
            "rule {} has a non-structural key",
            r.name
        );
    }
}
