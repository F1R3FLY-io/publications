//! The recurrent neural network (note, §8).
//!
//! The point of the example is where things *are not*. The synaptic efficacies
//! are not in any payload — they are values of the weight map. The squashing
//! function is not written anywhere — it is the exponential waiting-time law of
//! the chain. The learning rule is not an outer loop — it is the refinement
//! entry's update function, fired by the same rewrite that performs inference.
//!
//! Each of those is asserted below rather than asserted in prose.

use std::collections::BTreeMap;

use rho_weighted::examples::{neuron, rnn, synapse, synapse_class, NetSpec};
use rho_weighted::explore::ssa::Sim;
use rho_weighted::explore::{exhaustive_graph, Ensemble};
use rho_weighted::logic::Budget;
use rho_weighted::matching::SimpleMatcher;
use rho_weighted::propensity::propensities;
use rho_weighted::space::Space;
use rho_weighted::syntax::{Ground, Term};
use rho_weighted::theory::{Configuration, RateValue};

fn b() -> Budget {
    Budget::default()
}

/// A recurrent pair: 0 → 1 → 0. Fan-out equals threshold, so the token count is
/// conserved and the network is a live, bounded, conservative marked graph.
fn pair_spec(spikes: usize) -> NetSpec {
    NetSpec {
        post: vec![vec![1], vec![0]],
        theta: vec![1, 1],
        initial: vec![((0, 1), spikes)],
    }
}

// ---------------------------------------------------------------------------
// Structure
// ---------------------------------------------------------------------------

#[test]
fn a_neuron_is_a_persistent_for_comprehension() {
    let spec = pair_spec(1);
    let n = neuron(&spec, 1, &[0]);
    match n {
        Term::Receive {
            persistent, binds, ..
        } => {
            assert!(
                persistent,
                "a linear receipt models one sweep, not a network"
            );
            assert_eq!(binds.len(), 1);
        }
        other => panic!("expected a receive, got {}", other.render()),
    }
}

/// McCulloch--Pitts (note, Prop. 41): a threshold unit *is* a for-comprehension,
/// with the threshold appearing as the arity of the joins and the alternatives
/// as the group separator. The count of groups is the count of theta-subsets.
#[test]
fn threshold_is_join_structure() {
    for (k, theta, groups, arity) in [(3usize, 1usize, 3usize, 1usize), (3, 2, 3, 2), (3, 3, 1, 3)] {
        let spec = NetSpec {
            post: vec![vec![3]; 4],
            theta: vec![theta; 4],
            initial: vec![],
        };
        let pre: Vec<usize> = (0..k).collect();
        let n = neuron(&spec, 3, &pre);
        let comps = n.components();
        assert_eq!(
            comps.len(),
            groups,
            "k={k}, theta={theta}: expected C(k,theta)={groups} join groups"
        );
        for c in comps {
            if let Term::Receive { binds, .. } = c {
                assert_eq!(binds.len(), arity, "each group joins theta binds");
            }
        }
    }
}

#[test]
fn recurrence_is_just_a_cycle_in_the_channel_graph() {
    let spec = pair_spec(1);
    let (_theory, term, syns) = rnn(&spec, |_, _| 1.0, None, 8);
    assert_eq!(syns, vec![(0, 1), (1, 0)]);
    let s = Space::install(&term);
    // Both synapse channels carry a persistent receipt: the loop is closed.
    assert_eq!(s.conts_on(&synapse(0, 1).key()).len(), 1);
    assert_eq!(s.conts_on(&synapse(1, 0).key()).len(), 1);
}

// ---------------------------------------------------------------------------
// The weighted sum, and the missing squashing function
// ---------------------------------------------------------------------------

/// The textbook weighted sum `Σ_j w_ji · m_ji` is not written into any term.
/// It is the propensity the simulator computes: the weight comes from the map,
/// the presynaptic activity `m_ji` from the marking.
#[test]
fn propensity_is_the_weighted_sum_of_presynaptic_activity() {
    for m in 1..=6usize {
        let spec = pair_spec(m);
        let w = 0.7;
        let (theory, term, _syns) = rnn(&spec, |_, _| w, None, 32);
        let cfg = Configuration::new(Space::install(&term), theory.initial_weights());
        let p = propensities(&theory, &cfg, &SimpleMatcher, &mut b());
        let expected = w * m as f64;
        assert!(
            (p.total - expected).abs() < 1e-12,
            "m={m}: propensity {} vs w*m {}",
            p.total,
            expected
        );
    }
}

#[test]
fn distinct_synapses_carry_distinct_weights() {
    let spec = pair_spec(3);
    let (theory, term, syns) = rnn(&spec, |j, i| if (j, i) == (0, 1) { 2.0 } else { 0.5 }, None, 32);
    let cfg = Configuration::new(Space::install(&term), theory.initial_weights());
    let k01 = synapse_class(&syns, 0, 1).unwrap();
    let k10 = synapse_class(&syns, 1, 0).unwrap();
    assert!((cfg.weights.rate(k01) - 2.0).abs() < 1e-12);
    assert!((cfg.weights.rate(k10) - 0.5).abs() < 1e-12);
    // Only the populated synapse contributes.
    let p = propensities(&theory, &cfg, &SimpleMatcher, &mut b());
    assert!((p.total - 2.0 * 3.0).abs() < 1e-12);
}

/// The logistic response of the artificial neuron is a compressed description
/// of a Poisson process. Here it *is* the Poisson process: no squashing
/// function was supplied, and the saturation is the exponential waiting-time
/// law — a neuron with enormous drive still cannot fire more than once per
/// event.
#[test]
fn the_response_is_logistic_and_nobody_wrote_it() {
    let delta = 0.25;
    for m in [1usize, 2, 4, 8] {
        let w = 0.9;
        let spec = pair_spec(m);
        let (theory, term, _s) = rnn(&spec, |_, _| w, None, 64);
        let cfg = Configuration::new(Space::install(&term), theory.initial_weights());

        let drive = w * m as f64;
        let predicted = 1.0 - (-delta * drive).exp();

        // Empirically: the fraction of runs in which a first firing occurs
        // before `delta`.
        let n = 4000;
        let mut fired = 0;
        for i in 0..n {
            let mut sim = Sim::new(&theory, &SimpleMatcher, cfg.clone(), 0xBEEF + i as u64);
            if let Some(step) = sim.step() {
                if step.time <= delta {
                    fired += 1;
                }
            }
        }
        let empirical = fired as f64 / n as f64;
        assert!(
            (empirical - predicted).abs() < 0.02,
            "drive={drive}: empirical {empirical:.4} vs 1-exp(-d*drive) {predicted:.4}"
        );
        // Saturation: monotone and bounded by 1.
        assert!(predicted < 1.0);
    }
}

#[test]
fn no_squashing_function_appears_in_the_sources() {
    // The claim in the note is that the nonlinearity is emergent, so the
    // network's own definition must not contain one. This is a source-level
    // assertion because it is a claim about *where* things are.
    let src = include_str!("../src/examples.rs");
    for banned in ["sigmoid", "tanh", "logistic(", "softmax", "relu"] {
        assert!(
            !src.to_lowercase().contains(banned),
            "the RNN builder must not define an activation function: found `{banned}`"
        );
    }
}

// ---------------------------------------------------------------------------
// Boundedness
// ---------------------------------------------------------------------------

/// With fan-out at most the threshold, the token count is non-increasing, so
/// the reachable term set is finite and the network is term-finite — which is
/// what exhaustive exploration and the quantum construction need.
#[test]
fn token_count_is_conserved_when_fanout_equals_threshold() {
    let spec = pair_spec(3);
    let (theory, term, _s) = rnn(&spec, |_, _| 1.0, None, 64);
    let cfg = Configuration::new(Space::install(&term), theory.initial_weights());
    let start = cfg.space.marking().total_tokens();

    let mut sim = Sim::new(&theory, &SimpleMatcher, cfg, 99);
    let trace = sim.run(2_000, f64::INFINITY);
    assert!(trace.len() > 1_000, "the network must stay live");
    assert_eq!(
        sim.config.space.marking().total_tokens(),
        start,
        "fan-out == threshold conserves the marking"
    );
}

/// A branching network is bounded by the capacity factor, which is where a
/// configuration-dependent constraint belongs: `g(k)` prices *where* an
/// interaction is, as against `w(φ)`, which prices what is transferred.
///
/// Note what a hard capacity does to a token amplifier: once every channel is
/// full, every enabled redex has factor zero, so `a₀ = 0` and the chain is
/// absorbing. That is the correct reading — a system that cannot afford any of
/// its available transitions has stopped — but a modeller who wants a *live*
/// saturating network wants a leak rule instead, which trades term-finiteness
/// for positive recurrence.
#[test]
fn capacity_bounds_a_branching_network() {
    let cap = 4;
    let spec = NetSpec {
        // Neuron 0 fans out to two targets while its threshold is 1, so the
        // token count would grow without bound.
        post: vec![vec![1, 2], vec![0], vec![0]],
        theta: vec![1, 1, 1],
        initial: vec![((1, 0), 1)],
    };
    let (theory, term, _s) = rnn(&spec, |_, _| 1.0, None, cap);
    let cfg = Configuration::new(Space::install(&term), theory.initial_weights());
    let mut sim = Sim::new(&theory, &SimpleMatcher, cfg.clone(), 12345);
    let trace = sim.run(3_000, f64::INFINITY);

    assert_eq!(
        trace.stop,
        rho_weighted::explore::Stop::Absorbing,
        "a saturated network stops rather than running out of step budget"
    );
    for c in sim.config.space.channels() {
        assert!(
            sim.config.space.occupancy(&c) <= cap,
            "channel {c} exceeded capacity: {}",
            sim.config.space.occupancy(&c)
        );
    }
    assert!(
        sim.config.space.marking().total_tokens() > 1,
        "the network did amplify before saturating"
    );
}

/// And the factor is load-bearing: without it, the same network grows past any
/// bound. A geometric factor is not decoration.
#[test]
fn without_capacity_the_same_network_grows_without_bound() {
    let spec = NetSpec {
        post: vec![vec![1, 2], vec![0], vec![0]],
        theta: vec![1, 1, 1],
        initial: vec![((1, 0), 1)],
    };
    let (mut theory, term, _s) = rnn(&spec, |_, _| 1.0, None, 4);
    theory.geometric = rho_weighted::theory::unit_geometry();
    let cfg = Configuration::new(Space::install(&term), theory.initial_weights());
    let mut sim = Sim::new(&theory, &SimpleMatcher, cfg, 12345);
    sim.run(200, f64::INFINITY);
    assert!(
        sim.config.space.marking().total_tokens() > 4,
        "an unbounded amplifier must exceed what the capped one reached"
    );
}

#[test]
fn a_bounded_network_has_a_finite_reachable_graph() {
    let spec = pair_spec(2);
    let (theory, term, _s) = rnn(&spec, |_, _| 1.0, None, 8);
    let cfg = Configuration::new(Space::install(&term), theory.initial_weights());
    let ex = exhaustive_graph(&theory, &cfg, &SimpleMatcher, 500);
    assert!(ex.is_complete(), "truncated: {:?}", ex.stop);
    // Two tokens distributed over two synapses: three markings.
    assert_eq!(ex.graph.node_count(), 3);
    assert!(ex.generator.stationary(50_000, 1e-12).is_some());
}

// ---------------------------------------------------------------------------
// Plasticity
// ---------------------------------------------------------------------------

/// Hebbian potentiation (note, Definition 44). The condition "pre and post were
/// jointly involved in an event" is not a correlation computed by an observer;
/// it is the firing of a single rewrite, which by construction requires both
/// parties. The event which transmits is the event which potentiates.
#[test]
fn a_used_synapse_potentiates_and_an_unused_one_does_not() {
    let eta = 0.1;
    let w_max = 5.0;
    // Neuron 2 exists but is never driven: its incoming synapse is a control.
    let spec = NetSpec {
        post: vec![vec![1], vec![0], vec![0]],
        theta: vec![1, 1, 1],
        initial: vec![((0, 1), 1)],
    };
    let (theory, term, syns) = rnn(&spec, |_, _| 1.0, Some((eta, w_max)), 16);
    let cfg = Configuration::new(Space::install(&term), theory.initial_weights());

    let used = synapse_class(&syns, 0, 1).unwrap();
    let control = synapse_class(&syns, 2, 0).unwrap();
    let w0_used = cfg.weights.rate(used);
    let w0_ctrl = cfg.weights.rate(control);

    let mut sim = Sim::new(&theory, &SimpleMatcher, cfg, 4242);
    sim.run(50, f64::INFINITY);

    let w1_used = sim.config.weights.rate(used);
    let w1_ctrl = sim.config.weights.rate(control);
    assert!(
        w1_used > w0_used,
        "the co-active synapse must potentiate: {w0_used} -> {w1_used}"
    );
    assert!(
        (w1_ctrl - w0_ctrl).abs() < 1e-12,
        "the control synapse must not move: {w0_ctrl} -> {w1_ctrl}"
    );
}

#[test]
fn potentiation_saturates_at_the_ceiling() {
    let w_max = 2.0;
    let spec = pair_spec(1);
    let (theory, term, syns) = rnn(&spec, |_, _| 1.0, Some((0.5, w_max)), 16);
    let cfg = Configuration::new(Space::install(&term), theory.initial_weights());
    let k = synapse_class(&syns, 0, 1).unwrap();

    let mut sim = Sim::new(&theory, &SimpleMatcher, cfg, 777);
    sim.run(500, f64::INFINITY);
    let w = sim.config.weights.rate(k);
    assert!(
        (w - w_max).abs() < 1e-12,
        "weight must saturate at {w_max}, got {w}"
    );
}

/// Theorem 47: learning and inference are two marginals of one transition
/// relation. There is no step that changes only the map, no training schedule,
/// and no separate learning clock.
#[test]
fn learning_and_inference_are_one_relation() {
    let spec = pair_spec(1);
    let (theory, term, _s) = rnn(&spec, |_, _| 1.0, Some((0.05, 100.0)), 16);
    let cfg = Configuration::new(Space::install(&term), theory.initial_weights());
    let mut sim = Sim::new(&theory, &SimpleMatcher, cfg, 2024);
    let trace = sim.run(40, f64::INFINITY);

    let mut term_changes = 0;
    let mut map_changes = 0;
    let mut map_only = 0;
    for w in trace.steps.windows(2) {
        let t = w[0].marking_key != w[1].marking_key;
        let m = w[0].weights_fingerprint != w[1].weights_fingerprint;
        if t {
            term_changes += 1;
        }
        if m {
            map_changes += 1;
        }
        if m && !t {
            map_only += 1;
        }
    }
    assert!(term_changes > 0 && map_changes > 0);
    assert_eq!(
        map_only, 0,
        "a step that changes only the map would be a separate learning process"
    );
}

/// The learning rate is a component of the theory, not a hyperparameter of an
/// outer loop: changing it is changing the refinement entry.
#[test]
fn the_learning_rate_lives_in_the_refinement_entry() {
    let spec = pair_spec(1);
    let mut finals = Vec::new();
    for eta in [0.01, 0.1] {
        let (theory, term, syns) = rnn(&spec, |_, _| 1.0, Some((eta, 1e9)), 16);
        let cfg = Configuration::new(Space::install(&term), theory.initial_weights());
        let k = synapse_class(&syns, 0, 1).unwrap();
        let mut sim = Sim::new(&theory, &SimpleMatcher, cfg, 31337);
        sim.run(100, f64::INFINITY);
        finals.push(sim.config.weights.rate(k));
    }
    assert!(
        finals[1] > finals[0],
        "a larger eta must potentiate further: {finals:?}"
    );
}

// ---------------------------------------------------------------------------
// Studies
// ---------------------------------------------------------------------------

#[test]
fn a_sweep_over_efficacy_shifts_the_stationary_distribution() {
    use rho_weighted::study::{sweep_1d, Study};

    let spec = pair_spec(2);
    let (theory, term, syns) = rnn(&spec, |_, _| 1.0, None, 8);
    let cfg = Configuration::new(Space::install(&term), theory.initial_weights());
    let k01 = synapse_class(&syns, 0, 1).unwrap();

    let grid = sweep_1d(k01, "w01", &[0.25, 1.0, 4.0]);
    let result = Study::new(&theory, &SimpleMatcher, cfg)
        .with_grid(grid)
        .with_state_budget(500)
        .run();

    assert_eq!(result.points.len(), 3);
    assert!(
        !result.any_truncated(),
        "the sweep must be complete, not silently cut short"
    );
    for p in &result.points {
        assert!(p.stationary.is_some(), "{}: no stationary", p.label);
        assert!(!p.dot.is_empty());
        assert!(p.provenance.to_json().contains("\"truncated\":false"));
    }

    // Raising w01 drains synapse (0,1) — traffic moves downstream.
    let occ = |p: &rho_weighted::study::PointResult| -> f64 {
        let key = synapse(0, 1).key();
        p.stationary
            .as_ref()
            .unwrap()
            .iter()
            .map(|(m, prob)| {
                let n = m
                    .find(&format!("{key}="))
                    .map(|i| {
                        let rest = &m[i + key.len() + 1..];
                        let e = rest.find(|c: char| !c.is_ascii_digit()).unwrap_or(rest.len());
                        rest[..e].parse::<f64>().unwrap_or(0.0)
                    })
                    .unwrap_or(0.0);
                n * prob
            })
            .sum()
    };
    let (lo, hi) = (occ(&result.points[0]), occ(&result.points[2]));
    assert!(
        hi < lo,
        "a stronger synapse should hold fewer pending spikes: {lo:.4} then {hi:.4}"
    );
}

#[test]
fn a_truncated_study_says_so() {
    let spec = NetSpec {
        post: vec![vec![1, 2], vec![0], vec![0]],
        theta: vec![1, 1, 1],
        initial: vec![((1, 0), 1)],
    };
    // Capacity high, state budget low: the exploration cannot complete.
    let (theory, term, _s) = rnn(&spec, |_, _| 1.0, None, 50);
    let cfg = Configuration::new(Space::install(&term), theory.initial_weights());
    let ex = exhaustive_graph(&theory, &cfg, &SimpleMatcher, 20);
    assert!(!ex.is_complete());
    assert_eq!(ex.stop, rho_weighted::explore::Stop::StateBudget);
}

// ---------------------------------------------------------------------------
// Ensembles
// ---------------------------------------------------------------------------

#[test]
fn an_ensemble_reports_its_seeds_and_its_spread() {
    let spec = pair_spec(2);
    let (theory, term, _s) = rnn(&spec, |_, _| 1.0, None, 8);
    let cfg = Configuration::new(Space::install(&term), theory.initial_weights());
    let e = Ensemble::run(&theory, &SimpleMatcher, &cfg, 5, 16, 400, f64::INFINITY);
    let sum = e.summary();
    assert_eq!(sum.runs, 16);
    assert_eq!(e.seeds.len(), 16);
    assert!(sum.sd_end_time > 0.0, "distinct seeds must give distinct runs");
    let occ: BTreeMap<String, f64> = e.mean_occupancy();
    let total: f64 = occ.values().sum();
    assert!((total - 1.0).abs() < 1e-9, "occupancy must be normalised");
}

#[test]
fn weights_are_rates_and_may_exceed_one() {
    let spec = pair_spec(1);
    let (theory, term, syns) = rnn(&spec, |_, _| 12.5, None, 8);
    let cfg = Configuration::new(Space::install(&term), theory.initial_weights());
    let k = synapse_class(&syns, 0, 1).unwrap();
    assert_eq!(cfg.weights.get(k), RateValue::Real(12.5));
    let p = propensities(&theory, &cfg, &SimpleMatcher, &mut b());
    assert!((p.total - 12.5).abs() < 1e-12);
    let _ = Term::Ground(Ground::Int(0));
}
