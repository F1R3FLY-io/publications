//! The quantum reading.
//!
//! Everything in the refinement layer is unchanged: the same keys, the same
//! partition discipline, the same classification. What changes is what one does
//! with the numbers.
//!
//! # The construction
//!
//! The configuration Hilbert space is `ℓ²(B)` for `B` the reachable
//! configurations modulo structural congruence — finite only for a term-finite
//! model, which is why [`crate::theory::capacity`] exists. Each refinement
//! entry becomes a **jump operator**
//!
//! ```text
//! L_{r,φ} = z · Σ_P Σ_{k ∈ K(r,φ,P)} √(g(k)·χ) |k[Rσ]⟩⟨P|
//! ```
//!
//! and the generator is the Lindbladian
//!
//! ```text
//! ℒ(ρ) = -i[H, ρ] + Σ (L ρ L† - ½{L†L, ρ}).
//! ```
//!
//! Amplitudes are the square roots of the per-redex classical rates, which is
//! the normalisation under which `‖L_{r,φ}|P⟩‖²` recovers the classical
//! propensity in the diagonal case — and fails to, instructively, when it is
//! not diagonal (see [`interference`] and `Remark` below).
//!
//! # Where the coherent part comes from
//!
//! A GSLT already carries two kinds of relation on terms: symmetric, cost-free
//! *equations* and directed, costed *rewrites*. A quantum reading has exactly
//! two slots to fill. Rewrites are irreversible, so they are naturally
//! dissipative and become the jumps; equations are symmetric, so they are
//! naturally unitary and become the Hamiltonian. Most presentations quotient
//! every equation into the basis — as [`crate::syntax::Term::canonical`] does —
//! so `H = 0` by default, and [`QctmcModel::with_hamiltonian`] is how a
//! modeller puts an equation back as coherent hopping instead.
//!
//! # Gillespie as a degenerate quantum jump
//!
//! The trajectory sampler ([`Unravelling`]) is the quantum-jump or
//! Monte-Carlo-wavefunction method: deterministic non-Hermitian evolution under
//! `H_eff = H - (i/2)ΣL†L`, punctuated by jumps, with the waiting time drawn
//! from the *decaying norm* `‖ψ̃(τ)‖²`. At `H = 0` with a diagonal jump
//! structure the norm decays as `e^{-a₀τ}` and the whole thing collapses to the
//! stochastic simulation algorithm. That is the precise sense in which the
//! quantum simulator is Gillespie-*inspired*: not an analogy, a degeneration.
//! `tests/quantum.rs` checks it numerically.

pub mod linalg;

use std::collections::BTreeMap;

use crate::explore::Exploration;
use crate::graph::LabelledTransitionGraph;
use crate::redex::RuleId;
use crate::rng::Rng;
use linalg::{norm_sqr, normalize, Matrix, C};

/// A quantum continuous-time Markov chain over a reachable configuration set.
#[derive(Clone, Debug)]
pub struct QctmcModel {
    /// Basis labels — the markings, in index order.
    pub basis: Vec<String>,
    /// One jump operator per refinement class that has any transition.
    pub jumps: Vec<((RuleId, usize), Matrix)>,
    /// The coherent part. Zero unless a modeller supplies equations as
    /// amplitudes.
    pub hamiltonian: Matrix,
}

impl QctmcModel {
    pub fn dimension(&self) -> usize {
        self.basis.len()
    }

    /// Build the model from an exhaustive exploration.
    ///
    /// The exploration must be complete: a truncated reachable set gives a
    /// generator that is not trace-preserving, and silently so.
    pub fn from_exploration(ex: &Exploration) -> Result<QctmcModel, QuantumError> {
        if !ex.is_complete() {
            return Err(QuantumError::TruncatedStateSpace(ex.states_visited));
        }
        Ok(Self::from_graph_unchecked(&ex.graph))
    }

    /// Build without the completeness check. Only for models whose finiteness
    /// is known by construction.
    pub fn from_graph_unchecked(g: &LabelledTransitionGraph) -> QctmcModel {
        let n = g.node_count();
        let mut per_class: BTreeMap<(RuleId, usize), Matrix> = BTreeMap::new();
        for e in &g.edges {
            if e.rate <= 0.0 {
                continue;
            }
            let m = per_class
                .entry((e.rule, e.class))
                .or_insert_with(|| Matrix::zeros(n));
            // The amplitude of one redex is the square root of its classical
            // rate. Amplitudes of distinct redexes with the same contractum
            // ADD — which is the entire difference from the classical case.
            m.add_to(e.to, e.from, C::real(e.rate.sqrt()));
        }
        QctmcModel {
            basis: g.node_labels.clone(),
            jumps: per_class.into_iter().collect(),
            hamiltonian: Matrix::zeros(n),
        }
    }

    /// Supply a coherent part. `H` must be Hermitian.
    pub fn with_hamiltonian(mut self, h: Matrix) -> Result<QctmcModel, QuantumError> {
        if h.n != self.dimension() {
            return Err(QuantumError::DimensionMismatch {
                expected: self.dimension(),
                got: h.n,
            });
        }
        if !h.is_hermitian(1e-9) {
            return Err(QuantumError::NotHermitian);
        }
        self.hamiltonian = h;
        Ok(self)
    }

    /// Apply an overall phase to a class's jump operator. Phases are invisible
    /// classically and are exactly what makes cancellation possible, so this is
    /// the knob that turns a classical-looking model into one that interferes.
    pub fn with_phase(mut self, key: (RuleId, usize), theta: f64) -> QctmcModel {
        let z = C::new(theta.cos(), theta.sin());
        for (k, m) in self.jumps.iter_mut() {
            if *k == key {
                *m = m.mul_c(z);
            }
        }
        self
    }

    /// `Σ L†L`, the operator whose expectation is the total jump rate.
    pub fn total_jump_operator(&self) -> Matrix {
        let n = self.dimension();
        let mut acc = Matrix::zeros(n);
        for (_k, l) in &self.jumps {
            acc = acc.add(&l.dagger().mul(l));
        }
        acc
    }

    /// `H_eff = H - (i/2) Σ L†L`. Non-Hermitian by construction: its
    /// anti-Hermitian part is what makes the norm decay, and the decaying norm
    /// is what the waiting time is drawn from.
    pub fn effective_hamiltonian(&self) -> Matrix {
        let damping = self.total_jump_operator().scale(0.5);
        self.hamiltonian
            .sub(&damping.mul_c(C::I))
    }

    /// The Lindbladian applied to a density operator.
    pub fn lindblad(&self, rho: &Matrix) -> Matrix {
        let comm = self
            .hamiltonian
            .mul(rho)
            .sub(&rho.mul(&self.hamiltonian))
            .mul_c(-C::I);
        let mut out = comm;
        for (_k, l) in &self.jumps {
            let ld = l.dagger();
            let feed = l.mul(rho).mul(&ld);
            let lld = ld.mul(l);
            let anti = lld.mul(rho).add(&rho.mul(&lld)).scale(0.5);
            out = out.add(&feed).sub(&anti);
        }
        out
    }

    /// `ρ(t) = e^{tℒ}ρ(0)`, by fourth-order Runge--Kutta. Small dense
    /// matrices, so this is cheap and its error is controllable by `steps`.
    pub fn evolve(&self, rho0: &Matrix, t: f64, steps: usize) -> Matrix {
        let h = t / steps as f64;
        let mut rho = rho0.clone();
        for _ in 0..steps {
            let k1 = self.lindblad(&rho);
            let k2 = self.lindblad(&rho.add(&k1.scale(h / 2.0)));
            let k3 = self.lindblad(&rho.add(&k2.scale(h / 2.0)));
            let k4 = self.lindblad(&rho.add(&k3.scale(h)));
            let inc = k1
                .add(&k2.scale(2.0))
                .add(&k3.scale(2.0))
                .add(&k4)
                .scale(h / 6.0);
            rho = rho.add(&inc);
        }
        rho
    }

    /// The measurement in the basis: `Tr(Π_i ρ)` for each basis state.
    ///
    /// The projectors of a checked partition are mutually orthogonal and sum to
    /// the identity, so a refinement class *is* a projective measurement; that
    /// is what makes the atomic propositions of quantum CTMC model checking
    /// available for free rather than something a modeller has to invent.
    pub fn populations(&self, rho: &Matrix) -> Vec<f64> {
        (0..self.dimension()).map(|i| rho.get(i, i).re).collect()
    }

    /// A pure state as a density operator.
    pub fn pure(&self, index: usize) -> Matrix {
        let mut rho = Matrix::zeros(self.dimension());
        rho.set(index, index, C::ONE);
        rho
    }

    pub fn index_of(&self, marking: &str) -> Option<usize> {
        self.basis.iter().position(|b| b == marking)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum QuantumError {
    /// A truncated reachable set gives a generator that is not
    /// trace-preserving. Refusing is better than reporting one.
    TruncatedStateSpace(usize),
    DimensionMismatch {
        expected: usize,
        got: usize,
    },
    NotHermitian,
}

impl std::fmt::Display for QuantumError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuantumError::TruncatedStateSpace(n) => write!(
                f,
                "the reachable set was truncated at {n} states; a Lindbladian over a \
                 truncated basis is not trace-preserving"
            ),
            QuantumError::DimensionMismatch { expected, got } => {
                write!(f, "expected a {expected}x{expected} matrix, got {got}x{got}")
            }
            QuantumError::NotHermitian => write!(f, "the Hamiltonian must be Hermitian"),
        }
    }
}

// ---------------------------------------------------------------------------
// The trajectory sampler
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct QuantumStep {
    pub time: f64,
    pub tau: f64,
    pub jumped: (RuleId, usize),
    /// The basis state after the jump, when the post-jump state is a basis
    /// state; `None` when it is a superposition.
    pub collapsed_to: Option<usize>,
    /// `‖ψ̃(τ)‖²` at the moment of the jump — the value the uniform was
    /// matched against.
    pub norm_at_jump: f64,
}

/// The quantum-jump (Monte-Carlo wavefunction) unravelling.
pub struct Unravelling<'a> {
    pub model: &'a QctmcModel,
    pub psi: Vec<C>,
    pub time: f64,
    pub rng: Rng,
    /// Integration step for the deterministic evolution between jumps.
    pub dt: f64,
    heff: Matrix,
}

impl<'a> Unravelling<'a> {
    pub fn new(model: &'a QctmcModel, start: usize, seed: u64) -> Unravelling<'a> {
        let mut psi = vec![C::ZERO; model.dimension()];
        psi[start] = C::ONE;
        Unravelling {
            heff: model.effective_hamiltonian(),
            model,
            psi,
            time: 0.0,
            rng: Rng::seeded(seed),
            dt: 1e-3,
        }
    }

    pub fn with_dt(mut self, dt: f64) -> Self {
        self.dt = dt;
        self
    }

    /// `dψ/ds = -i H_eff ψ`, one RK4 step. The norm is *not* conserved — that
    /// is the point.
    fn integrate(&self, psi: &[C], h: f64) -> Vec<C> {
        let f = |v: &[C]| -> Vec<C> {
            self.heff
                .apply(v)
                .into_iter()
                .map(|z| z * (-C::I))
                .collect()
        };
        let add = |a: &[C], b: &[C], k: f64| -> Vec<C> {
            a.iter().zip(b.iter()).map(|(x, y)| *x + y.scale(k)).collect()
        };
        let k1 = f(psi);
        let k2 = f(&add(psi, &k1, h / 2.0));
        let k3 = f(&add(psi, &k2, h / 2.0));
        let k4 = f(&add(psi, &k3, h));
        let mut out = psi.to_vec();
        for i in 0..out.len() {
            let inc = (k1[i] + k2[i].scale(2.0) + k3[i].scale(2.0) + k4[i]).scale(h / 6.0);
            out[i] = out[i] + inc;
        }
        out
    }

    /// One jump. `None` when the state has no outgoing rate — the quantum
    /// analogue of an absorbing configuration.
    pub fn step(&mut self, max_time: f64) -> Option<QuantumStep> {
        let target = self.rng.unit();
        let mut psi = self.psi.clone();
        let mut s = 0.0;

        // Evolve until the norm drops to the drawn uniform, bisecting the last
        // step for accuracy. This *is* the Gillespie waiting-time draw,
        // generalised: at H = 0 with diagonal jumps the norm decays as
        // exp(-a0 s) and inverting it gives tau = ln(1/u)/a0 exactly.
        loop {
            if norm_sqr(&psi) <= target {
                break;
            }
            if s > max_time {
                return None;
            }
            let next = self.integrate(&psi, self.dt);
            if norm_sqr(&next) <= target {
                let (mut lo, mut hi) = (0.0, self.dt);
                for _ in 0..60 {
                    let mid = 0.5 * (lo + hi);
                    if norm_sqr(&self.integrate(&psi, mid)) <= target {
                        hi = mid;
                    } else {
                        lo = mid;
                    }
                }
                psi = self.integrate(&psi, hi);
                s += hi;
                break;
            }
            psi = next;
            s += self.dt;
        }

        // Select a jump with probability proportional to ‖L ψ̃‖².
        let weights: Vec<((RuleId, usize), Vec<C>, f64)> = self
            .model
            .jumps
            .iter()
            .map(|(k, l)| {
                let v = l.apply(&psi);
                let w = norm_sqr(&v);
                (*k, v, w)
            })
            .collect();
        let total: f64 = weights.iter().map(|(_, _, w)| *w).sum();
        if total <= 0.0 {
            return None;
        }
        let mut u = self.rng.unit() * total;
        let mut chosen = weights.last().cloned().unwrap();
        for w in &weights {
            u -= w.2;
            if u <= 0.0 {
                chosen = w.clone();
                break;
            }
        }

        let mut post = chosen.1;
        normalize(&mut post);
        let collapsed_to = basis_index(&post);
        self.psi = post;
        self.time += s;

        Some(QuantumStep {
            time: self.time,
            tau: s,
            jumped: chosen.0,
            collapsed_to,
            norm_at_jump: target,
        })
    }

    pub fn run(&mut self, max_steps: usize, horizon: f64) -> Vec<QuantumStep> {
        let mut out = Vec::new();
        while out.len() < max_steps && self.time < horizon {
            match self.step(horizon) {
                Some(s) => out.push(s),
                None => break,
            }
        }
        out
    }
}

/// Whether a state is (numerically) a basis state, and which.
pub fn basis_index(psi: &[C]) -> Option<usize> {
    let mut idx = None;
    for (i, z) in psi.iter().enumerate() {
        if z.norm_sqr() > 1e-12 {
            if idx.is_some() {
                return None;
            }
            idx = Some(i);
        }
    }
    idx
}

/// Whether the jump structure is **diagonal**: within each class, distinct
/// redexes out of a state reach distinct targets.
///
/// This is one of the two hypotheses of the degeneration theorem, and it is
/// exactly the condition under which no interference can occur. A model that
/// fails it is not the classical one with phases attached.
pub fn is_diagonal(g: &LabelledTransitionGraph) -> bool {
    let mut seen: BTreeMap<(usize, usize, RuleId, usize), usize> = BTreeMap::new();
    for e in &g.edges {
        if e.rate <= 0.0 {
            continue;
        }
        *seen
            .entry((e.from, e.to, e.rule, e.class))
            .or_insert(0) += 1;
    }
    seen.values().all(|c| *c == 1)
}

/// The interference budget at a state: for each class, the number of distinct
/// redexes reaching each target.
///
/// `m` indistinguishable reactants give `m` redexes and one target, so the
/// amplitude is `m·z` and the transition weight `m²|z|²` where the classical
/// rate is `m|z|²`. The quantum network is superlinear in multiplicity, and the
/// enhancement comes entirely from the reactants living in a bag rather than a
/// list. Whether that is a feature of the physics or an artefact of the
/// presentation is open — tagging the reactants would break the degeneracy and
/// restore linearity.
pub fn interference(g: &LabelledTransitionGraph) -> BTreeMap<(usize, usize, usize), usize> {
    let mut out: BTreeMap<(usize, usize, usize), usize> = BTreeMap::new();
    for e in &g.edges {
        if e.rate <= 0.0 {
            continue;
        }
        *out.entry((e.from, e.to, e.class)).or_insert(0) += 1;
    }
    out.retain(|_, v| *v > 1);
    out
}
