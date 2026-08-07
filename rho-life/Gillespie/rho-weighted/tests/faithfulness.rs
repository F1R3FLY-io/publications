//! Faithfulness: the simulator and the interpreter must agree on a single step.
//!
//! In the standalone build there is no interpreter, so the differential runs
//! against an independently written reference matcher. That is weaker than the
//! workspace comparison DR-W9 asks for, and the gap is named rather than
//! papered over: see `the_real_obligation_is_still_open`.

use rho_weighted::faithfulness::{differential, expected_occupancy, ReferenceMatcher};
use rho_weighted::matching::{Matching, SimpleMatcher};
use rho_weighted::space::Space;
use rho_weighted::syntax::{chan, structured, Bind, Ground, Name, Pattern, Term};

/// A corpus that exercises the cases where the two real bugs in this crate's
/// history lived: structured names, ordered versus unordered formers, repeated
/// pattern variables, and arity.
fn corpus() -> Vec<(Bind, Vec<Term>)> {
    let mut out = Vec::new();
    let sources: Vec<Name> = vec![
        chan("a"),
        structured("syn", &[0, 1]),
        structured("syn", &[1, 0]),
        Name::quote(Term::Drop(Box::new(chan("a")))),
    ];
    let payloads: Vec<Vec<Term>> = vec![
        vec![],
        vec![Term::Zero],
        vec![Term::Ground(Ground::Int(1))],
        vec![Term::Ground(Ground::Int(1)), Term::Ground(Ground::Int(1))],
        vec![Term::Ground(Ground::Int(1)), Term::Ground(Ground::Int(2))],
        vec![Term::List(vec![
            Term::Ground(Ground::Int(0)),
            Term::Ground(Ground::Int(1)),
        ])],
        vec![Term::List(vec![
            Term::Ground(Ground::Int(1)),
            Term::Ground(Ground::Int(0)),
        ])],
        vec![Term::Par(vec![
            Term::send(chan("a"), vec![Term::Zero]),
            Term::send(chan("b"), vec![Term::Zero]),
        ])],
        vec![Term::Par(vec![
            Term::send(chan("b"), vec![Term::Zero]),
            Term::send(chan("a"), vec![Term::Zero]),
        ])],
    ];
    let patterns: Vec<Vec<Pattern>> = vec![
        vec![],
        vec![Pattern::Wildcard],
        vec![Pattern::Var("x".into())],
        vec![Pattern::Var("x".into()), Pattern::Var("x".into())],
        vec![Pattern::Var("x".into()), Pattern::Var("y".into())],
        vec![Pattern::Exact(Term::Zero)],
        vec![Pattern::Exact(Term::Ground(Ground::Int(1)))],
        vec![Pattern::Exact(Term::List(vec![
            Term::Ground(Ground::Int(0)),
            Term::Ground(Ground::Int(1)),
        ]))],
        vec![Pattern::Exact(Term::Par(vec![
            Term::send(chan("a"), vec![Term::Zero]),
            Term::send(chan("b"), vec![Term::Zero]),
        ]))],
        vec![Pattern::Wildcard, Pattern::Exact(Term::Ground(Ground::Int(2)))],
    ];
    for s in &sources {
        for p in &patterns {
            for d in &payloads {
                out.push((
                    Bind {
                        patterns: p.clone(),
                        source: s.clone(),
                    },
                    d.clone(),
                ));
            }
        }
    }
    out
}

#[test]
fn the_matcher_agrees_with_an_independent_implementation() {
    let c = corpus();
    assert!(c.len() > 300, "corpus too small to be evidence: {}", c.len());
    if let Err(d) = differential(&SimpleMatcher, &ReferenceMatcher, &c) {
        panic!("{d}");
    }
}

#[test]
fn the_corpus_can_actually_detect_a_disagreement() {
    // A deliberately wrong matcher, to show the harness is not vacuous.
    struct Broken;
    impl Matching for Broken {
        fn match_bind(
            &self,
            bind: &Bind,
            data: &[Term],
        ) -> Option<rho_weighted::matching::Bindings> {
            // Ignores repeated-variable consistency — the classic bug.
            if bind.patterns.len() != data.len() {
                return None;
            }
            let mut out = rho_weighted::matching::Bindings::new();
            for (p, d) in bind.patterns.iter().zip(data.iter()) {
                if let Pattern::Var(v) = p {
                    out.insert(v.clone(), d.canonical());
                } else if let Pattern::Exact(t) = p {
                    if t.canonical() != d.canonical() {
                        return None;
                    }
                }
            }
            Some(out)
        }
    }
    assert!(
        differential(&SimpleMatcher, &Broken, &corpus()).is_err(),
        "the corpus must be able to catch a broken matcher"
    );
}

/// `install` is the only genuinely new semantics the simulator implements, so
/// it is checked against a marking computed without it.
#[test]
fn install_produces_the_marking_the_term_says_it_should() {
    let terms = vec![
        Term::par(vec![
            Term::send(chan("a"), vec![Term::Zero]),
            Term::send(chan("a"), vec![Term::Zero]),
            Term::send(chan("b"), vec![Term::Zero]),
        ]),
        Term::par(vec![
            Term::send(structured("syn", &[0, 1]), vec![Term::Zero]),
            Term::send(structured("syn", &[1, 0]), vec![Term::Zero]),
        ]),
        Term::par(vec![
            Term::recv_persistent(chan("a"), vec![Pattern::Wildcard], Term::Zero),
            Term::send(chan("a"), vec![Term::Zero]),
            Term::Zero,
        ]),
    ];
    for t in terms {
        let expected = expected_occupancy(&t);
        let s = Space::install(&t);
        let got = s.marking().occupancy;
        assert_eq!(
            got, expected,
            "install disagrees with the term's decomposition for `{}`",
            t.render()
        );
    }
}

/// Distinct structured names must stay distinct through installation. This is
/// the regression test for the ordered-versus-unordered bug: encoding indices
/// as a parallel composition made synapses (0,1) and (1,0) the same channel,
/// and the symptom was a silently doubled propensity.
#[test]
fn ordered_structured_names_stay_distinct() {
    let a = structured("syn", &[0, 1]);
    let b = structured("syn", &[1, 0]);
    assert_ne!(a.key(), b.key(), "an ordered tuple needs an ordered former");

    let t = Term::par(vec![
        Term::send(a.clone(), vec![Term::Zero]),
        Term::send(b.clone(), vec![Term::Zero]),
    ]);
    let s = Space::install(&t);
    assert_eq!(s.occupancy(&a.key()), 1);
    assert_eq!(s.occupancy(&b.key()), 1);
    assert_eq!(s.channels().len(), 2);
}

/// A name predicate must be able to see into a name's structure after
/// installation. This is the regression test for the other real bug: rebuilding
/// a channel's name from its key string lost the structure, and every key
/// silently fell through to `default`.
#[test]
fn installation_preserves_name_structure() {
    use rho_weighted::logic::check::sat_struct;
    use rho_weighted::logic::{Budget, Formula};

    let c = structured("syn", &[3, 4]);
    let t = Term::par(vec![
        Term::recv_persistent(c.clone(), vec![Pattern::Wildcard], Term::Zero),
        Term::send(c, vec![Term::Zero]),
    ]);
    let s = Space::install(&t);
    let readback = s.to_term();
    let f = Formula::comm_in_namespace(Term::Ground(Ground::Str("syn".into())));
    assert!(
        sat_struct(&readback, &f, &mut Budget::default()).unwrap(),
        "a name predicate must survive the round trip through the space"
    );
    // And the endpoint is recoverable, which is what makes one key cover a family.
    let onto_4 = Formula::comm_in_namespace_at(2, Formula::Eq(Term::Ground(Ground::Int(4))));
    let onto_9 = Formula::comm_in_namespace_at(2, Formula::Eq(Term::Ground(Ground::Int(9))));
    assert!(sat_struct(&readback, &onto_4, &mut Budget::default()).unwrap());
    assert!(!sat_struct(&readback, &onto_9, &mut Budget::default()).unwrap());
}

/// Names the gap rather than papering over it.
#[test]
fn the_real_obligation_is_still_open() {
    // DR-W9 asks for a differential against the *interpreter*, not against a
    // second implementation of ours. That requires building inside
    // f1r3node-rust, which requires the pinned nightly toolchain, protoc, and
    // the rholang-rs worktree the workspace `[patch]` points at.
    //
    // The adapter is written and gated (`faithfulness::workspace`); this test
    // exists so the gap is visible in the suite output rather than only in a
    // document.
    assert!(
        cfg!(not(feature = "workspace-matcher")),
        "with the workspace matcher enabled, the differential above should run \
         against `Matcher::get` instead of `ReferenceMatcher`"
    );
}
