//! Print the numbers behind the note's quantitative claims.
//!
//! The test suite asserts these; this prints them, so a reader can see the
//! magnitudes rather than take a green tick on faith. Run with
//! `just verify`, or `cargo run --release --features quantum --example
//! verify-claims`.

use std::collections::BTreeSet;

use rho_weighted::examples::{birth_death, channel_keyed, rnn, NetSpec};
use rho_weighted::explore::exhaustive_graph;
use rho_weighted::matching::SimpleMatcher;
use rho_weighted::quantum::linalg::{norm_sqr, C};
use rho_weighted::quantum::{degeneracy, QctmcModel};
use rho_weighted::space::Space;
use rho_weighted::syntax::{chan, Pattern, Term};
use rho_weighted::theory::{open_gate, saturating_add, unit_geometry, Configuration, RateValue};
use rho_weighted::Budget;

fn rule(title: &str) {
    println!("\n\x1b[1m{title}\x1b[0m\n{}", "-".repeat(title.len()));
}

/// A configuration with `m` indistinguishable pending messages on one channel
/// and a persistent receipt: `m` derivations, one contractum. Deliberately
/// non-diagonal.
fn degenerate(m: usize, rate: f64) -> (rho_weighted::WeightedTheory, Configuration) {
    let mut parts = vec![Term::recv_persistent(
        chan("a"),
        vec![Pattern::Wildcard],
        Term::send(chan("b"), vec![Term::Zero]),
    )];
    for _ in 0..m {
        parts.push(Term::send(chan("a"), vec![Term::Zero]));
    }
    let theory = channel_keyed(&[(chan("a"), rate)], unit_geometry(), open_gate());
    let cfg = Configuration::new(Space::install(&Term::par(parts)), theory.initial_weights());
    (theory, cfg)
}

fn main() {
    // -----------------------------------------------------------------
    rule("§7.2  A jump channel's amplitude is the root of its aggregate rate");
    println!("  Root-of-sum, not sum-of-roots. For m degenerate derivations of");
    println!("  common rate λ the entry is √(mλ) and the weight mλ — the classical");
    println!("  answer. Sum-of-roots would give m√λ and m²λ.\n");
    println!("   m   entry      √(mλ)      ‖L|c⟩‖²   classical  degeneracies");
    for m in [2usize, 3, 5, 8] {
        let rate = 0.5;
        let (theory, cfg) = degenerate(m, rate);
        let ex = exhaustive_graph(&theory, &cfg, &SimpleMatcher, 500);
        let qm = QctmcModel::from_exploration(&ex).unwrap();
        let l = &qm.jumps[0].1;
        let mut psi = vec![C::ZERO; qm.dimension()];
        psi[0] = C::ONE;
        let entry = (0..l.n).map(|i| l.get(i, 0)).find(|z| z.norm_sqr() > 0.0).unwrap();
        let degs: BTreeSet<usize> = degeneracy(&ex.graph).values().copied().collect();
        println!(
            "  {m:2}   {:.6}   {:.6}   {:.6}   {:.6}   {:?}",
            entry.abs(),
            (m as f64 * rate).sqrt(),
            norm_sqr(&l.apply(&psi)),
            ex.generator.total_exit_rate(0),
            degs
        );
    }

    // -----------------------------------------------------------------
    rule("Definition 36  Conservativity, on a NON-diagonal fixture");
    println!("  The criterion that selects the normalisation: at H = 0 the");
    println!("  populations must solve the classical forward equation. Checked");
    println!("  where it could fail — several derivations into one target.\n");
    let (theory, cfg) = degenerate(4, 0.7);
    let ex = exhaustive_graph(&theory, &cfg, &SimpleMatcher, 500);
    let qm = QctmcModel::from_exploration(&ex).unwrap();
    let t = 0.9;
    let steps = 40_000;
    let pops = qm.populations(&qm.evolve(&qm.pure(0), t, steps));
    let n = ex.generator.n;
    let mut p = vec![0.0; n];
    p[0] = 1.0;
    let h = t / steps as f64;
    let f = |p: &Vec<f64>| -> Vec<f64> {
        (0..n)
            .map(|j| (0..n).map(|i| ex.generator.q[i][j] * p[i]).sum())
            .collect()
    };
    for _ in 0..steps {
        let k1 = f(&p);
        let k2 = f(&(0..n).map(|i| p[i] + h / 2.0 * k1[i]).collect());
        let k3 = f(&(0..n).map(|i| p[i] + h / 2.0 * k2[i]).collect());
        let k4 = f(&(0..n).map(|i| p[i] + h * k3[i]).collect());
        for i in 0..n {
            p[i] += h / 6.0 * (k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + k4[i]);
        }
    }
    let err = (0..n).map(|i| (pops[i] - p[i]).abs()).fold(0.0, f64::max);
    println!("  max |quantum − classical| over {n} configurations at t={t}: {err:.3e}");

    // -----------------------------------------------------------------
    rule("Remark 13  Amplitudes are unbounded; λ(z) = |z|² is the bridge");
    for (re, im) in [(0.5, 0.5), (5.0, 0.0), (0.0, 3.0)] {
        let v = RateValue::complex(re, im).unwrap();
        println!("  z = {re} + {im}i   λ(z) = {:.4}", v.rate());
    }
    println!(
        "  non-finite refused: {}",
        RateValue::complex(f64::INFINITY, 0.0).is_err()
    );

    // -----------------------------------------------------------------
    rule("§7.1  The basis is keyed by configuration, not by marking");
    println!("  Under a plastic theory several basis vectors share a marking.");
    println!("  Storing markings would collapse them and make index_of ambiguous.\n");
    let spec = NetSpec {
        post: vec![vec![1], vec![0]],
        theta: vec![1, 1],
        initial: vec![((0, 1), 1)],
    };
    let (mut th, term, _s) = rnn(&spec, |_, _| 1.0, Some((0.5, 3.0)), 4);
    th.rules[0].entries[0].update = saturating_add(0.5, 3.0);
    let cfg = Configuration::new(Space::install(&term), th.initial_weights());
    let ex = exhaustive_graph(&th, &cfg, &SimpleMatcher, 500);
    let qm = QctmcModel::from_exploration(&ex).unwrap();
    let ub: BTreeSet<_> = qm.basis.iter().collect();
    let ul: BTreeSet<_> = qm.labels.iter().collect();
    println!(
        "  dimension {}   distinct configuration keys {}   distinct markings {}",
        qm.dimension(),
        ub.len(),
        ul.len()
    );
    println!(
        "  index_of_term(\"{}\") → {} indices",
        &qm.labels[0][..qm.labels[0].len().min(28)],
        qm.index_of_term(&qm.labels[0]).len()
    );

    // -----------------------------------------------------------------
    rule("Remark 27  Self-transitions are in a₀ and absent from Q");
    let t = Term::par(vec![
        Term::recv_persistent(
            chan("a"),
            vec![Pattern::Wildcard],
            Term::send(chan("a"), vec![Term::Zero]),
        ),
        Term::send(chan("a"), vec![Term::Zero]),
    ]);
    let theory = channel_keyed(&[(chan("a"), 2.0)], unit_geometry(), open_gate());
    let cfg = Configuration::new(Space::install(&t), theory.initial_weights());
    let ex = exhaustive_graph(&theory, &cfg, &SimpleMatcher, 100);
    let a0 = rho_weighted::propensities(&theory, &cfg, &SimpleMatcher, &mut Budget::default()).total;
    let rowsum: f64 = (0..ex.generator.n).map(|j| ex.generator.q[0][j]).sum();
    println!(
        "  a₀ = {a0:.3} (self-loop counted)   Q[0][0] = {:.3} (dropped)   row sum = {rowsum:.1e}",
        ex.generator.q[0][0]
    );
    println!("  A self-loop is a fictitious jump: same state distribution, different event counts.");

    // -----------------------------------------------------------------
    rule("§5  Multiplicity is the factor that is invisible in a trace");
    let (bd, term) = birth_death(3.0, 1.0, 10);
    let cfg = Configuration::new(Space::install(&term), bd.initial_weights());
    let ex = exhaustive_graph(&bd, &cfg, &SimpleMatcher, 2000);
    let pi = ex.generator.stationary(200_000, 1e-14).unwrap();
    let pool = chan("pool").key();
    let mut got = vec![0.0; 11];
    for (i, p) in pi.iter().enumerate() {
        let lbl = &ex.graph.node_labels[i];
        if let Some(j) = lbl.find(&format!("{pool}=")) {
            let rest = &lbl[j + pool.len() + 1..];
            let e = rest.find(|c: char| !c.is_ascii_digit()).unwrap_or(rest.len());
            if let Ok(k) = rest[..e].parse::<usize>() {
                got[k] += p;
            }
        } else {
            got[0] += p;
        }
    }
    let r = 3.0f64;
    let mut want = vec![0.0; 11];
    let mut term_k = 1.0;
    for (k, w) in want.iter_mut().enumerate() {
        if k > 0 {
            term_k *= r / k as f64;
        }
        *w = term_k;
    }
    let z: f64 = want.iter().sum();
    println!("  Birth–death with λ/μ = 3, capacity 10. Death rate ∝ occupancy,");
    println!("  so the law is Poisson only if multiplicity is counted.\n");
    println!("   k   stationary   truncated Poisson(3)");
    for k in 0..=6 {
        println!("  {k:2}   {:.7}    {:.7}", got[k], want[k] / z);
    }
    println!("\n  A multiplicity-blind propensity gives a geometric law instead;");
    println!("  the two are 0.5+ apart in total variation, so the test is not vacuous.");
    println!();
}
