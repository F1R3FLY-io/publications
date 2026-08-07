//! Statistical acceptance.
//!
//! A wrong propensity produces perfectly plausible-looking traces. That is how
//! the multiplicity bug survived in the v1 prototype, and it is why acceptance
//! here is numerical rather than by inspection.
//!
//! Three checks, deliberately independent:
//!
//! 1. the **generator** built by exhaustive exploration must equal the analytic
//!    generator — this tests enumeration with no sampling involved;
//! 2. the **stationary distribution** must equal the closed form;
//! 3. the **sampled** occupancy must converge to that stationary distribution —
//!    this cross-checks the sampler against the enumerator.
//!
//! `birth_death` is the discriminating case. Its death propensity is
//! proportional to occupancy, so it is *only* correct if multiplicity is
//! counted; a prototype-style propensity yields a geometric distribution where
//! the truth is Poisson. `multiplicity_is_load_bearing` asserts that the two
//! are actually distinguishable at the tolerances used, so the test cannot pass
//! vacuously.

use std::collections::BTreeMap;

use rho_weighted::examples::{birth_death, two_state};
use rho_weighted::explore::{exhaustive_graph, Ensemble};
use rho_weighted::matching::SimpleMatcher;
use rho_weighted::space::Space;
use rho_weighted::theory::Configuration;

const SEED: u64 = 0x5EED_1234;

fn setup(term: &rho_weighted::syntax::Term, theory: &rho_weighted::WeightedTheory) -> Configuration {
    Configuration::new(Space::install(term), theory.initial_weights())
}

/// Occupancy of `pool` read out of a marking key like `[@..pool=3][...]`.
fn pool_count(marking: &str, chan_key: &str) -> usize {
    let needle = format!("{chan_key}=");
    match marking.find(&needle) {
        Some(i) => {
            let rest = &marking[i + needle.len()..];
            let end = rest
                .find(|c: char| !c.is_ascii_digit())
                .unwrap_or(rest.len());
            rest[..end].parse().unwrap_or(0)
        }
        None => 0,
    }
}

#[test]
fn two_state_generator_and_stationary_match_the_closed_form() {
    let (r_ab, r_ba) = (2.0, 0.5);
    let (theory, term) = two_state(r_ab, r_ba);
    let cfg = setup(&term, &theory);
    let ex = exhaustive_graph(&theory, &cfg, &SimpleMatcher, 100);

    assert!(ex.is_complete(), "exploration truncated: {:?}", ex.stop);
    assert_eq!(ex.graph.node_count(), 2, "a two-state chain has two states");
    assert_eq!(ex.graph.edge_count(), 2);

    // Exit rates are the analytic ones.
    let mut rates: Vec<f64> = (0..2).map(|i| ex.generator.total_exit_rate(i)).collect();
    rates.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert!((rates[0] - r_ba).abs() < 1e-12, "got {rates:?}");
    assert!((rates[1] - r_ab).abs() < 1e-12, "got {rates:?}");

    // pi(a) = r_ba / (r_ab + r_ba)
    let pi = ex.generator.stationary(50_000, 1e-14).expect("stationary");
    let mut sorted = pi.clone();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let expect_small = r_ab / (r_ab + r_ba);
    let expect_large = r_ba / (r_ab + r_ba);
    let (lo, hi) = (sorted[0].min(sorted[1]), sorted[0].max(sorted[1]));
    assert!(
        (lo - expect_small.min(expect_large)).abs() < 1e-6
            && (hi - expect_large.max(expect_small)).abs() < 1e-6,
        "stationary {pi:?}, expected {{{expect_small}, {expect_large}}}"
    );
}

#[test]
fn two_state_sampling_converges_to_the_generator() {
    let (r_ab, r_ba) = (2.0, 0.5);
    let (theory, term) = two_state(r_ab, r_ba);
    let cfg = setup(&term, &theory);

    let ex = exhaustive_graph(&theory, &cfg, &SimpleMatcher, 100);
    let pi = ex.generator.stationary(50_000, 1e-14).unwrap();
    let mut analytic: BTreeMap<String, f64> = BTreeMap::new();
    for (i, p) in pi.iter().enumerate() {
        *analytic.entry(ex.graph.node_labels[i].clone()).or_insert(0.0) += p;
    }

    let e = Ensemble::run(&theory, &SimpleMatcher, &cfg, SEED, 8, 20_000, f64::INFINITY);
    let emp = e.mean_occupancy();

    for (k, v) in &analytic {
        let got = emp.get(k).copied().unwrap_or(0.0);
        assert!(
            (got - v).abs() < 0.02,
            "state {k}: sampled {got:.4} vs stationary {v:.4}"
        );
    }
}

#[test]
fn birth_death_occupancy_is_poisson_because_multiplicity_is_counted() {
    // Birth rate lambda (one tick, always multiplicity 1); death rate mu*k
    // where k is the pool occupancy, since a persistent receipt pairs with
    // every pending message. Stationary occupancy is Poisson(lambda/mu),
    // truncated at the capacity.
    let (lambda, mu, cap) = (3.0, 1.0, 12usize);
    let (theory, term) = birth_death(lambda, mu, cap);
    let cfg = setup(&term, &theory);

    let ex = exhaustive_graph(&theory, &cfg, &SimpleMatcher, 1000);
    assert!(ex.is_complete(), "truncated: {:?}", ex.stop);
    assert_eq!(
        ex.graph.node_count(),
        cap + 1,
        "occupancies 0..=cap are the states"
    );

    let pool_key = rho_weighted::syntax::chan("pool").key();
    let pi = ex.generator.stationary(200_000, 1e-14).expect("stationary");

    let mut got = vec![0.0; cap + 1];
    for (i, p) in pi.iter().enumerate() {
        let k = pool_count(&ex.graph.node_labels[i], &pool_key);
        got[k] += p;
    }

    // Truncated Poisson(lambda/mu).
    let r = lambda / mu;
    let mut want = vec![0.0; cap + 1];
    let mut term_k = 1.0;
    for k in 0..=cap {
        if k > 0 {
            term_k *= r / k as f64;
        }
        want[k] = term_k;
    }
    let z: f64 = want.iter().sum();
    for w in want.iter_mut() {
        *w /= z;
    }

    for k in 0..=cap {
        assert!(
            (got[k] - want[k]).abs() < 1e-6,
            "occupancy {k}: got {:.8}, truncated Poisson {:.8}\nfull: {got:?}",
            got[k],
            want[k]
        );
    }
}

/// The test above must not be able to pass with a broken propensity. Here we
/// compute what a prototype-style, multiplicity-blind simulator would produce —
/// a geometric distribution — and assert it is far outside the tolerance used.
#[test]
fn multiplicity_is_load_bearing() {
    let (lambda, mu, cap) = (3.0, 1.0, 12usize);
    let r = lambda / mu;

    let poisson = {
        let mut w = vec![0.0; cap + 1];
        let mut t = 1.0;
        for k in 0..=cap {
            if k > 0 {
                t *= r / k as f64;
            }
            w[k] = t;
        }
        let z: f64 = w.iter().sum();
        w.into_iter().map(|x| x / z).collect::<Vec<_>>()
    };
    // Multiplicity-blind: constant death rate mu, so birth/death ratio is r at
    // every level — a truncated geometric.
    let geometric = {
        let mut w = vec![0.0; cap + 1];
        for (k, item) in w.iter_mut().enumerate() {
            *item = r.powi(k as i32);
        }
        let z: f64 = w.iter().sum();
        w.into_iter().map(|x| x / z).collect::<Vec<_>>()
    };

    let tv: f64 = poisson
        .iter()
        .zip(geometric.iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<f64>()
        / 2.0;
    assert!(
        tv > 0.5,
        "the two propensity models must be far apart or the test is vacuous; total variation {tv:.4}"
    );
}

#[test]
fn waiting_times_are_exponential_with_parameter_a0() {
    // In the two-state chain each state has a single exit rate, so the sampled
    // sojourn in a state of exit rate `a` must have mean 1/a. Anderson--Darling
    // is overkill for a check this sharp; the mean and the second moment
    // together pin the exponential.
    let (r_ab, r_ba) = (4.0, 4.0);
    let (theory, term) = two_state(r_ab, r_ba);
    let cfg = setup(&term, &theory);
    let e = Ensemble::run(&theory, &SimpleMatcher, &cfg, SEED, 4, 50_000, f64::INFINITY);
    let taus = e.taus();
    assert!(taus.len() > 10_000, "not enough samples: {}", taus.len());

    let n = taus.len() as f64;
    let mean = taus.iter().sum::<f64>() / n;
    let second = taus.iter().map(|t| t * t).sum::<f64>() / n;

    // Exponential(a): mean 1/a, E[T^2] = 2/a^2.
    let expected_mean = 1.0 / r_ab;
    assert!(
        (mean - expected_mean).abs() < 0.01 * expected_mean + 1e-3,
        "mean tau {mean:.6}, expected {expected_mean:.6}"
    );
    let ratio = second / (mean * mean);
    assert!(
        (ratio - 2.0).abs() < 0.1,
        "E[T^2]/E[T]^2 = {ratio:.4}, expected 2 for an exponential"
    );
}
