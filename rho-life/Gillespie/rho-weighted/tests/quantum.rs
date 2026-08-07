//! The quantum reading, checked numerically.
//!
//! The headline is `the_ssa_is_the_quantum_jump_algorithm_at_zero_hamiltonian`:
//! "Gillespie-inspired" is a *degeneration*, not an analogy. It now holds with
//! `H = 0` as the only hypothesis; the diagonality of the jump structure that an
//! earlier version also required was an artefact of the old normalisation, and
//! `indistinguishable_reactants_do_not_interfere` together with
//! `sum_of_roots_would_have_given_m_squared` is the pair that establishes it.
//!
//! Which leaves exactly one feature that makes a model quantum: a nonzero
//! Hamiltonian, supplied by equations the presentation withholds from the
//! quotient.
//!
//! Run with `cargo test --features quantum`.

#![cfg(feature = "quantum")]

use rho_weighted::examples::{birth_death, two_state};
use rho_weighted::explore::exhaustive_graph;
use rho_weighted::matching::SimpleMatcher;
use rho_weighted::quantum::linalg::{norm_sqr, Matrix, C};
use rho_weighted::quantum::{
    basis_index, degeneracy, is_diagonal, QctmcModel, QuantumError, Unravelling,
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
    assert!(degeneracy(&ex.graph).is_empty());
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
    // Recorded, not required. Under root-of-sum the theorem holds whether or
    // not the jump structure is diagonal; see
    // `the_degeneration_holds_without_diagonality` below.
    assert!(is_diagonal(&ex.graph), "this particular model happens to be");

    let m = QctmcModel::from_exploration(&ex).unwrap();
    assert_eq!(
        m.hamiltonian.trace(),
        C::ZERO,
        "the hypothesis: no coherent part"
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

/// Build the `m`-degenerate model: one persistent receipt on `a` and `m`
/// indistinguishable pending sends, so there are `m` derivations and one
/// contractum.
fn degenerate_model(m_count: usize, rate: f64) -> (rho_weighted::explore::Exploration, f64) {
    let mut parts = vec![Term::recv_persistent(
        chan("a"),
        vec![Pattern::Wildcard],
        Term::send(chan("b"), vec![Term::Zero]),
    )];
    for _ in 0..m_count {
        parts.push(Term::send(chan("a"), vec![Term::Zero]));
    }
    let term = Term::par(parts);
    let theory = rho_weighted::examples::channel_keyed(
        &[(chan("a"), rate)],
        rho_weighted::theory::unit_geometry(),
        rho_weighted::theory::open_gate(),
    );
    let cfg = Configuration::new(Space::install(&term), theory.initial_weights());
    let ex = exhaustive_graph(&theory, &cfg, &SimpleMatcher, 200);
    let classical = ex.generator.total_exit_rate(0);
    (ex, classical)
}

/// Indistinguishable reactants give several derivations and one target, and
/// they do **not** interfere.
///
/// This inverts a test that used to assert the opposite. Under the earlier
/// normalisation — a square root per derivation, summed — the transition weight
/// came out `m²|z|²` against a classical rate of `m|z|²`, and that
/// superlinearity was reported as an open question about the physics. It was a
/// normalisation artefact. `|c⟩` is one normalised basis vector for a
/// configuration whose parallel composition is a multiset, so the `m`
/// derivations are one route with degeneracy `m`; summing roots charges for the
/// degeneracy twice. Aggregating rates and taking one root gives `√m·|z|` and
/// hence `m|z|²`, which is the classical answer and the bosonic one
/// (`a|m⟩ = √m|m-1⟩`).
#[test]
fn indistinguishable_reactants_do_not_interfere() {
    for m_count in 2..=4usize {
        let rate = 0.5;
        let (ex, classical) = degenerate_model(m_count, rate);

        assert!(
            (classical - rate * m_count as f64).abs() < 1e-12,
            "classical exit rate should be m*rate"
        );

        // The degeneracy is real and is what the classical multiplicity factor
        // `h` is counting. It is simply not interference.
        assert!(!is_diagonal(&ex.graph), "m={m_count}");
        assert!(degeneracy(&ex.graph).values().any(|c| *c == m_count));

        let qm = QctmcModel::from_exploration(&ex).unwrap();
        let l = &qm.jumps[0].1;
        let mut psi = vec![C::ZERO; qm.dimension()];
        psi[0] = C::ONE;
        let quantum = norm_sqr(&l.apply(&psi));
        assert!(
            (quantum - classical).abs() < 1e-9,
            "m={m_count}: quantum weight {quantum:.6} must equal the classical rate {classical:.6}"
        );

        // And the single nonzero entry is √(m·rate), not m·√rate. Resolve the
        // target rather than assuming its index: the fixture has one successor
        // of node 0, but the test should say so rather than rely on it.
        let targets: std::collections::BTreeSet<usize> = ex
            .graph
            .edges
            .iter()
            .filter(|e| e.from == 0 && e.rate > 0.0)
            .map(|e| e.to)
            .collect();
        assert_eq!(targets.len(), 1, "m={m_count}: one contractum, m derivations");
        let to = *targets.iter().next().unwrap();
        let entry = l.get(to, 0).norm_sqr().sqrt();
        assert!(
            (entry - (m_count as f64 * rate).sqrt()).abs() < 1e-9,
            "m={m_count}: entry {entry:.6}, expected sqrt(m*rate) {:.6}",
            (m_count as f64 * rate).sqrt()
        );
    }
}

/// The negative control for the test above.
///
/// A regression that quietly restored sum-of-roots would leave
/// `indistinguishable_reactants_do_not_interfere` as the only guard, and a test
/// asserting that two numbers are equal cannot say how far apart the wrong
/// answer would have been. This computes the old construction by hand and
/// checks that it differs by the factor `m` — so the assertion above is known
/// to be discriminating rather than vacuous.
#[test]
fn sum_of_roots_would_have_given_m_squared() {
    for m_count in 2..=4usize {
        let rate = 0.5;
        let (ex, classical) = degenerate_model(m_count, rate);

        // The old normalisation, reconstructed from the same graph: one root
        // per derivation, summed into the matrix entry.
        let mut old_entry = 0.0f64;
        for e in &ex.graph.edges {
            if e.rate > 0.0 && e.from == 0 {
                old_entry += e.rate.sqrt();
            }
        }
        let old_weight = old_entry * old_entry;

        assert!(
            (old_weight - (m_count as f64) * classical).abs() < 1e-9,
            "m={m_count}: sum-of-roots gives {old_weight:.6}, which is m times the classical \
             rate {classical:.6} — the factor this crate no longer carries"
        );
        assert!(
            old_weight > classical + 1e-9,
            "and the two normalisations must actually disagree, or this control proves nothing"
        );
    }
}

/// The degeneration theorem, on a model whose jump structure is **not**
/// diagonal.
///
/// This is the hypothesis the normalisation change bought back. Under
/// sum-of-roots the total jump rate exceeded the classical exit rate here, so
/// the sampled waiting times were wrong and the theorem had to exclude the
/// case. Under root-of-sum `Σ‖L|c⟩‖² = a₀` identically.
#[test]
fn the_degeneration_holds_without_diagonality() {
    let (ex, classical) = degenerate_model(3, 0.7);
    assert!(!is_diagonal(&ex.graph), "the point of the fixture");

    let m = QctmcModel::from_exploration(&ex).unwrap();
    let mut psi = vec![C::ZERO; m.dimension()];
    psi[0] = C::ONE;
    let total: f64 = m.jumps.iter().map(|(_k, l)| norm_sqr(&l.apply(&psi))).sum();
    assert!(
        (total - classical).abs() < 1e-9,
        "total jump rate {total:.6} must be the classical a0 {classical:.6}"
    );

    // Which is to say the norm still decays at exactly the classical rate.
    let damping = m.total_jump_operator();
    assert!(
        (damping.get(0, 0).re - classical).abs() < 1e-9,
        "(ΣL†L)_00 must be a0"
    );
}

/// Applying an overall phase to a class leaves the populations untouched.
///
/// This test has not changed, but what it is evidence *for* has. It used to sit
/// under a claim that a phase "is what makes cancellation possible," which it
/// contradicted: a global phase on a jump operator cancels in `LρL†`, so the
/// only phase knob the crate offers was already provably inert, and no relative
/// phase exists anywhere in the formalism for a cancellation to come from.
///
/// Read now as what it is: a carrier check. Phases survive construction rather
/// than being dropped, which matters because `with_hamiltonian` needs them.
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
