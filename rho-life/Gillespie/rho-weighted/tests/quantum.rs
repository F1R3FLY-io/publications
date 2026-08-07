//! The quantum reading, checked numerically.
//!
//! The headline is `the_ssa_is_the_quantum_jump_algorithm_at_zero_hamiltonian`:
//! "Gillespie-inspired" is a *degeneration*, not an analogy, and the two
//! hypotheses one has to drop to get there — a nonzero Hamiltonian and a
//! non-diagonal jump structure — are precisely the two features that make a
//! system quantum.
//!
//! Run with `cargo test --features quantum`.

#![cfg(feature = "quantum")]

use rho_weighted::examples::{birth_death, two_state};
use rho_weighted::explore::exhaustive_graph;
use rho_weighted::matching::SimpleMatcher;
use rho_weighted::quantum::linalg::{norm_sqr, Matrix, C};
use rho_weighted::quantum::{
    basis_index, interference, is_diagonal, QctmcModel, QuantumError, Unravelling,
};
use rho_weighted::space::Space;
use rho_weighted::syntax::{chan, Pattern, Term};
use rho_weighted::theory::Configuration;

fn model_for(theory: &rho_weighted::WeightedTheory, term: &Term, budget: usize) -> QctmcModel {
    let cfg = Configuration::new(Space::install(term), theory.initial_weights());
    let ex = exhaustive_graph(theory, &cfg, &SimpleMatcher, budget);
    QctmcModel::from_exploration(&ex).expect("complete exploration")
}

// ---------------------------------------------------------------------------
// Construction
// ---------------------------------------------------------------------------

#[test]
fn a_truncated_state_space_is_refused_rather_than_reported() {
    let (theory, term) = birth_death(2.0, 1.0, 40);
    let cfg = Configuration::new(Space::install(&term), theory.initial_weights());
    let ex = exhaustive_graph(&theory, &cfg, &SimpleMatcher, 5);
    assert!(!ex.is_complete());
    assert!(matches!(
        QctmcModel::from_exploration(&ex),
        Err(QuantumError::TruncatedStateSpace(_))
    ));
}

#[test]
fn amplitudes_are_the_square_roots_of_the_classical_rates() {
    let (r_ab, r_ba) = (4.0, 1.0);
    let (theory, term) = two_state(r_ab, r_ba);
    let m = model_for(&theory, &term, 100);
    assert_eq!(m.dimension(), 2);

    // Exactly one nonzero entry per jump operator here, and its modulus
    // squared is the classical rate.
    let mut rates: Vec<f64> = m
        .jumps
        .iter()
        .flat_map(|(_k, l)| {
            (0..l.n)
                .flat_map(move |i| (0..l.n).map(move |j| (i, j)))
                .filter_map(|(i, j)| {
                    let z = l.get(i, j);
                    if z.norm_sqr() > 0.0 {
                        Some(z.norm_sqr())
                    } else {
                        None
                    }
                })
        })
        .collect();
    rates.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert!((rates[0] - r_ba).abs() < 1e-12, "{rates:?}");
    assert!((rates[1] - r_ab).abs() < 1e-12, "{rates:?}");
}

#[test]
fn the_effective_hamiltonian_is_not_hermitian_and_that_is_the_point() {
    let (theory, term) = two_state(2.0, 1.0);
    let m = model_for(&theory, &term, 100);
    assert!(m.hamiltonian.is_hermitian(1e-12), "H = 0 is Hermitian");
    assert!(
        !m.effective_hamiltonian().is_hermitian(1e-9),
        "H_eff must have an anti-Hermitian part, or the norm would not decay"
    );
}

#[test]
fn a_non_hermitian_hamiltonian_is_refused() {
    let (theory, term) = two_state(1.0, 1.0);
    let m = model_for(&theory, &term, 100);
    let mut bad = Matrix::zeros(2);
    bad.set(0, 1, C::ONE); // not equal to its adjoint
    assert!(matches!(
        m.with_hamiltonian(bad),
        Err(QuantumError::NotHermitian)
    ));
}

// ---------------------------------------------------------------------------
// The Lindbladian
// ---------------------------------------------------------------------------

#[test]
fn lindblad_evolution_preserves_the_trace() {
    let (theory, term) = birth_death(2.0, 1.0, 5);
    let m = model_for(&theory, &term, 500);
    let rho0 = m.pure(0);
    for t in [0.1, 1.0, 5.0] {
        let rho = m.evolve(&rho0, t, 2000);
        let tr = rho.trace();
        assert!(
            (tr.re - 1.0).abs() < 1e-6 && tr.im.abs() < 1e-9,
            "t={t}: trace {tr:?}"
        );
        for p in m.populations(&rho) {
            assert!(p > -1e-9, "populations must stay non-negative: {p}");
        }
    }
}

/// With no Hamiltonian and a diagonal jump structure the Lindbladian's
/// populations obey the classical master equation, so the long-time
/// distribution is the classical stationary one.
#[test]
fn populations_relax_to_the_classical_stationary_distribution() {
    let (r_ab, r_ba) = (3.0, 1.0);
    let (theory, term) = two_state(r_ab, r_ba);
    let cfg = Configuration::new(Space::install(&term), theory.initial_weights());
    let ex = exhaustive_graph(&theory, &cfg, &SimpleMatcher, 100);
    let classical = ex.generator.stationary(50_000, 1e-14).unwrap();

    let m = QctmcModel::from_exploration(&ex).unwrap();
    let rho = m.evolve(&m.pure(0), 40.0, 20_000);
    let pops = m.populations(&rho);

    for i in 0..m.dimension() {
        assert!(
            (pops[i] - classical[i]).abs() < 1e-4,
            "state {i}: quantum {:.6} vs classical {:.6}",
            pops[i],
            classical[i]
        );
    }
}

// ---------------------------------------------------------------------------
// Theorem: the SSA is the quantum-jump algorithm at H = 0
// ---------------------------------------------------------------------------

#[test]
fn the_jump_structure_of_a_simple_chain_is_diagonal() {
    let (theory, term) = two_state(2.0, 1.0);
    let cfg = Configuration::new(Space::install(&term), theory.initial_weights());
    let ex = exhaustive_graph(&theory, &cfg, &SimpleMatcher, 100);
    assert!(is_diagonal(&ex.graph));
    assert!(interference(&ex.graph).is_empty());
}

/// (i) The norm decays as `exp(-a₀ s)`, so (ii) the waiting time is exponential
/// with parameter `a₀` — which is the Gillespie draw. (iv) The post-jump state
/// is again a basis state, so the reduction propagates.
#[test]
fn the_ssa_is_the_quantum_jump_algorithm_at_zero_hamiltonian() {
    let (r_ab, r_ba) = (3.0, 1.0);
    let (theory, term) = two_state(r_ab, r_ba);
    let cfg = Configuration::new(Space::install(&term), theory.initial_weights());
    let ex = exhaustive_graph(&theory, &cfg, &SimpleMatcher, 100);
    assert!(is_diagonal(&ex.graph), "hypothesis of the theorem");

    let m = QctmcModel::from_exploration(&ex).unwrap();
    assert_eq!(
        m.hamiltonian.trace(),
        C::ZERO,
        "the other hypothesis: no coherent part"
    );

    // (i) ‖ψ̃(s)‖² = exp(-Γ s) with Γ = a₀, the classical exit rate.
    let start = 0usize;
    let gamma = ex.generator.total_exit_rate(start);
    let u = Unravelling::new(&m, start, 1);
    let mut psi = vec![C::ZERO; m.dimension()];
    psi[start] = C::ONE;
    for s in [0.05f64, 0.2, 0.5] {
        let mut v = psi.clone();
        let steps = (s / 1e-4).round() as usize;
        for _ in 0..steps {
            v = integrate_public(&u, &v, 1e-4);
        }
        let got = norm_sqr(&v);
        let want = (-gamma * s).exp();
        assert!(
            (got - want).abs() < 1e-6,
            "s={s}: ‖psi‖² {got:.8} vs exp(-a0 s) {want:.8}"
        );
    }

    // (ii) and (iv): sampled waiting times are exponential with parameter equal
    // to the exit rate of the state they were drawn from, and every post-jump
    // state is a basis state.
    let mut per_state: Vec<Vec<f64>> = vec![Vec::new(); m.dimension()];
    let mut at = start;
    let mut un = Unravelling::new(&m, start, 0xA11CE).with_dt(1e-3);
    for _ in 0..4000 {
        let before = at;
        let step = un.step(1e6).expect("the chain is live");
        let idx = step
            .collapsed_to
            .expect("a diagonal jump collapses to a basis state");
        per_state[before].push(step.tau);
        at = idx;
    }
    for i in 0..m.dimension() {
        let taus = &per_state[i];
        if taus.len() < 500 {
            continue;
        }
        let a0 = ex.generator.total_exit_rate(i);
        let mean = taus.iter().sum::<f64>() / taus.len() as f64;
        assert!(
            (mean - 1.0 / a0).abs() < 0.06 / a0,
            "state {i}: mean tau {mean:.5}, expected {:.5}",
            1.0 / a0
        );
    }
}

/// End to end: the unravelling's empirical occupancy matches the classical
/// stationary distribution on the same theory.
#[test]
fn unravelling_occupancy_matches_the_classical_chain() {
    let (theory, term) = two_state(3.0, 1.0);
    let cfg = Configuration::new(Space::install(&term), theory.initial_weights());
    let ex = exhaustive_graph(&theory, &cfg, &SimpleMatcher, 100);
    let classical = ex.generator.stationary(50_000, 1e-14).unwrap();
    let m = QctmcModel::from_exploration(&ex).unwrap();

    let mut un = Unravelling::new(&m, 0, 20_260_806).with_dt(1e-3);
    let mut dwell = vec![0.0; m.dimension()];
    let mut at = 0usize;
    for _ in 0..6000 {
        let step = un.step(1e6).expect("live");
        dwell[at] += step.tau;
        at = step.collapsed_to.unwrap();
    }
    let total: f64 = dwell.iter().sum();
    for i in 0..m.dimension() {
        let got = dwell[i] / total;
        assert!(
            (got - classical[i]).abs() < 0.02,
            "state {i}: unravelling {got:.4} vs classical {:.4}",
            classical[i]
        );
    }
}

// ---------------------------------------------------------------------------
// Where the two readings part company
// ---------------------------------------------------------------------------

/// Indistinguishable reactants give several redexes and one target, so their
/// *amplitudes* add before being squared. The transition weight is `m²|z|²`
/// where the classical rate is `m|z|²`: the quantum model is **superlinear in
/// multiplicity**, and the enhancement comes entirely from the reactants living
/// in a bag rather than a list.
#[test]
fn indistinguishable_reactants_interfere() {
    for m_count in 2..=4usize {
        let mut parts = vec![Term::recv_persistent(
            chan("a"),
            vec![Pattern::Wildcard],
            Term::send(chan("b"), vec![Term::Zero]),
        )];
        for _ in 0..m_count {
            parts.push(Term::send(chan("a"), vec![Term::Zero]));
        }
        let term = Term::par(parts);
        let rate = 0.5;
        let theory = rho_weighted::examples::channel_keyed(
            &[(chan("a"), rate)],
            rho_weighted::theory::unit_geometry(),
            rho_weighted::theory::open_gate(),
        );
        let cfg = Configuration::new(Space::install(&term), theory.initial_weights());
        let ex = exhaustive_graph(&theory, &cfg, &SimpleMatcher, 200);

        // Classically: m redexes to one target, rates add.
        let classical = ex.generator.total_exit_rate(0);
        assert!(
            (classical - rate * m_count as f64).abs() < 1e-12,
            "classical exit rate should be m*rate"
        );

        // The jump structure is NOT diagonal, which is exactly the hypothesis
        // the degeneration theorem needs and does not have here.
        assert!(!is_diagonal(&ex.graph), "m={m_count}");
        let inter = interference(&ex.graph);
        assert!(inter.values().any(|c| *c == m_count));

        // Quantum: amplitudes add, so the weight is m^2 |z|^2.
        let qm = QctmcModel::from_exploration(&ex).unwrap();
        let l = &qm.jumps[0].1;
        let start = 0usize;
        let mut psi = vec![C::ZERO; qm.dimension()];
        psi[start] = C::ONE;
        let quantum = norm_sqr(&l.apply(&psi));
        let expected = (m_count as f64).powi(2) * rate;
        assert!(
            (quantum - expected).abs() < 1e-9,
            "m={m_count}: quantum weight {quantum:.6}, expected m^2*rate {expected:.6}"
        );
        assert!(
            quantum > classical + 1e-9,
            "and it must exceed the classical rate {classical:.6}"
        );
    }
}

/// A phase is invisible classically and is what makes cancellation possible.
/// Applying an overall phase to a class leaves the populations untouched —
/// which is the sanity check that phases are being carried, not dropped.
#[test]
fn an_overall_phase_leaves_populations_invariant() {
    let (theory, term) = two_state(2.0, 1.0);
    let cfg = Configuration::new(Space::install(&term), theory.initial_weights());
    let ex = exhaustive_graph(&theory, &cfg, &SimpleMatcher, 100);
    let m = QctmcModel::from_exploration(&ex).unwrap();
    let key = m.jumps[0].0;
    let phased = m.clone().with_phase(key, std::f64::consts::FRAC_PI_3);

    let a = m.populations(&m.evolve(&m.pure(0), 2.0, 4000));
    let b = phased.populations(&phased.evolve(&phased.pure(0), 2.0, 4000));
    for i in 0..a.len() {
        assert!((a[i] - b[i]).abs() < 1e-9, "phase changed populations at {i}");
    }
}

/// With a coherent part the amplitude moves between basis states before any
/// jump occurs, so the sojourn distribution is no longer a simple exponential.
/// This is the respect in which the quantum sampler is Gillespie-*inspired*
/// rather than Gillespie: the waiting time is drawn from a decaying norm, and
/// that norm is only an exponential in the degenerate case.
///
/// Note what the model needs for this to bite: **unequal exit rates**. See
/// `a_scalar_damping_stays_exponential_even_with_a_hamiltonian` below.
#[test]
fn a_hamiltonian_makes_the_norm_decay_non_exponential() {
    let (theory, term) = two_state(6.0, 0.5);
    let m = model_for(&theory, &term, 100);
    let mut h = Matrix::zeros(2);
    h.set(0, 1, C::real(4.0));
    h.set(1, 0, C::real(4.0));
    let m = m.with_hamiltonian(h).expect("Hermitian");

    let u = Unravelling::new(&m, 0, 5);
    let mut v = vec![C::ZERO; 2];
    v[0] = C::ONE;
    // ln‖psi‖² at equal spacings: exponential decay would make the successive
    // differences equal.
    let mut logs = Vec::new();
    for _ in 0..3 {
        for _ in 0..2000 {
            v = integrate_public(&u, &v, 1e-4);
        }
        logs.push(norm_sqr(&v).ln());
    }
    let d1 = logs[1] - logs[0];
    let d2 = logs[2] - logs[1];
    assert!(
        (d1 - d2).abs() > 1e-2,
        "with H != 0 and unequal exit rates the decay must not be a plain \
         exponential: {d1:.6} vs {d2:.6}"
    );
}

/// The degeneration theorem's hypotheses are sufficient but not necessary, and
/// this is the sharpest way to see it.
///
/// If `Σ L†L` is a scalar multiple of the identity — every configuration has
/// the *same* total exit rate — then the damping commutes with everything, so
/// `‖ψ̃(s)‖² = e^{-Γs}` exactly no matter what the Hamiltonian is. Coherent
/// evolution redistributes amplitude among states that all decay at the same
/// rate, and the total norm never notices. So a nonzero Hamiltonian alone does
/// not buy a non-exponential sojourn: the *variation* in exit rate across the
/// states the coherence connects is what does.
#[test]
fn a_scalar_damping_stays_exponential_even_with_a_hamiltonian() {
    // Equal rates in both directions, so the exit rate is the same everywhere.
    let (theory, term) = two_state(1.0, 1.0);
    let m = model_for(&theory, &term, 100);
    let mut h = Matrix::zeros(2);
    h.set(0, 1, C::real(3.0));
    h.set(1, 0, C::real(3.0));
    let m = m.with_hamiltonian(h).expect("Hermitian");

    let u = Unravelling::new(&m, 0, 5);
    let mut v = vec![C::ZERO; 2];
    v[0] = C::ONE;
    let mut logs = Vec::new();
    for _ in 0..3 {
        for _ in 0..2000 {
            v = integrate_public(&u, &v, 1e-4);
        }
        logs.push(norm_sqr(&v).ln());
    }
    let (d1, d2) = (logs[1] - logs[0], logs[2] - logs[1]);
    assert!(
        (d1 - d2).abs() < 1e-6,
        "scalar damping must give a pure exponential: {d1:.8} vs {d2:.8}"
    );
    // And the rate is the common exit rate.
    assert!((-d1 / 0.2 - 1.0).abs() < 1e-6, "decay rate {:.6}", -d1 / 0.2);
}

#[test]
fn basis_index_detects_superpositions() {
    assert_eq!(basis_index(&[C::ONE, C::ZERO]), Some(0));
    let s = 1.0 / 2f64.sqrt();
    assert_eq!(basis_index(&[C::real(s), C::real(s)]), None);
}

// A tiny shim so the tests can drive the deterministic evolution directly.
fn integrate_public(u: &Unravelling, psi: &[C], h: f64) -> Vec<C> {
    let heff = u.model.effective_hamiltonian();
    let f = |v: &[C]| -> Vec<C> { heff.apply(v).into_iter().map(|z| z * (-C::I)).collect() };
    let add = |a: &[C], b: &[C], k: f64| -> Vec<C> {
        a.iter().zip(b.iter()).map(|(x, y)| *x + y.scale(k)).collect()
    };
    let k1 = f(psi);
    let k2 = f(&add(psi, &k1, h / 2.0));
    let k3 = f(&add(psi, &k2, h / 2.0));
    let k4 = f(&add(psi, &k3, h));
    let mut out = psi.to_vec();
    for i in 0..out.len() {
        out[i] = out[i]
            + (k1[i] + k2[i].scale(2.0) + k3[i].scale(2.0) + k4[i]).scale(h / 6.0);
    }
    out
}
