//! The quantum reading.
//!
//! Everything in the refinement layer is unchanged: the same keys, the same
//! partition discipline, the same classification. What changes is what one does
//! with the numbers.
//!
//! # The construction
//!
//! The configuration Hilbert space is `ℓ²(B)` for `B` the reachable
//! **configurations** modulo structural congruence — terms paired with weight
//! maps, not terms alone. That is what keeps the generator fixed: after
//! trajectories bifurcate into histories carrying different maps, no single
//! Lindbladian on `ℓ²(Terms)` generates the evolution, and "`e^{tℒ}` with `ℒ`
//! rebuilt at each jump" is a history-dependent switching process rather than
//! the solution of a master equation. `B` is finite only when the model is
//! term-finite *and* map-finite, which is why [`crate::theory::capacity`] and
//! [`crate::theory::saturating_add`] both exist and why the worked example uses
//! both.
//!
//! Each refinement entry becomes a **jump operator**
//!
//! ```text
//! L_{r,φ} = e^{iθ} · Σ_c Σ_{c''} √( Σ_{k ∈ K(c,c'',r,φ)} λ(z)·g(k)·χ ) |c''⟩⟨c|
//! ```
//!
//! and the generator is the Lindbladian
//!
//! ```text
//! ℒ(ρ) = -i[H, ρ] + Σ (L ρ L† - ½{L†L, ρ}).
//! ```
//!
//! # The normalisation, and why it is this one
//!
//! Note the placement of the square root: rates are **aggregated over
//! derivations first**, and the root is taken once, per jump channel. An
//! earlier version of this crate summed the roots instead, which for `m`
//! indistinguishable reactants gave an amplitude `m√λ` and a transition weight
//! `m²λ` where the classical rate is `mλ`. That superlinearity was reported as
//! an open question — feature of the physics, or artefact of the presentation?
//! It is an artefact. `|c⟩` is a single *normalised* basis vector for a
//! configuration whose parallel composition is a multiset; the `m` redexes are
//! not `m` distinguishable routes but one route with degeneracy `m`, and
//! summing roots charges for the degeneracy twice. Second quantisation fixes
//! the factor for exactly this case: `a|m⟩ = √m |m-1⟩`, so the matrix element
//! carries `√m` and the rate carries `m`.
//!
//! The criterion that selects the normalisation is **conservativity**: at
//! `H = 0` the populations must solve the forward equation of the classical
//! chain. Root-of-sum is conservative, sum-of-roots is not, and
//! `tests/quantum.rs` checks it rather than asserting it.
//!
//! The consequence is that no coherence arises from the rewrite relation at
//! all: within a channel the amplitude is fixed by the aggregate rate, and
//! distinct classes give distinct jump operators whose dissipators add as
//! operations rather than as amplitudes. All coherence is carried by `H`.
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
//! modeller puts an equation back as coherent hopping instead. Combined with
//! the normalisation above, this gives the slogan: *a weighted GSLT is quantum
//! exactly to the extent that its presentation withholds equations from the
//! quotient.*
//!
//! # Gillespie as a degenerate quantum jump
//!
//! The trajectory sampler ([`Unravelling`]) is the quantum-jump or
//! Monte-Carlo-wavefunction method: deterministic non-Hermitian evolution under
//! `H_eff = H - (i/2)ΣL†L`, punctuated by jumps, with the waiting time drawn
//! from the *decaying norm* `‖ψ̃(τ)‖²`. At `H = 0` the norm decays as `e^{-a₀τ}`
//! and the whole thing collapses to the stochastic simulation algorithm. That
//! is the precise sense in which the quantum simulator is Gillespie-*inspired*:
//! not an analogy, a degeneration. No hypothesis on the jump structure is
//! needed — the diagonality the earlier version required is exactly what the
//! normalisation above made unnecessary. `tests/quantum.rs` checks it
//! numerically.

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
    /// Basis **identity** — one entry per reachable configuration, in index
    /// order, keyed by marking *and* weight-map fingerprint.
    ///
    /// This must be the configuration key and not the marking. Under a plastic
    /// theory several configurations share a marking, and an earlier version of
    /// this crate stored the marking here, so [`QctmcModel::index_of`] silently
    /// returned whichever of them happened to come first.
    pub basis: Vec<String>,
    /// Display labels — the marking of each configuration. Not unique under a
    /// plastic theory; use [`QctmcModel::index_of_term`] to resolve one.
    pub labels: Vec<String>,
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

        // Aggregate the CLASSICAL RATES of every derivation belonging to one
        // jump channel `(rule, class, source, target)`, and only then take a
        // single square root.
        //
        // The order matters and is the whole of the difference from the earlier
        // version, which took a root per derivation and summed those. For `m`
        // indistinguishable reactants that gave `m√λ`, hence a weight `m²λ`
        // against a classical rate of `mλ`. `|c⟩` is one normalised vector for
        // a configuration whose parallel composition is a multiset, so the `m`
        // derivations are one route with degeneracy `m`, and summing roots
        // charges for the degeneracy twice. Root-of-sum is the bosonic
        // normalisation (`a|m⟩ = √m|m-1⟩`) and it is the unique choice under
        // which `‖L|c⟩‖²` is the classical propensity — with no hypothesis on
        // the jump structure, which is why the degeneration theorem no longer
        // needs one.
        let mut rates: BTreeMap<(RuleId, usize), BTreeMap<(usize, usize), f64>> = BTreeMap::new();
        for e in &g.edges {
            if e.rate <= 0.0 {
                continue;
            }
            *rates
                .entry((e.rule, e.class))
                .or_default()
                .entry((e.to, e.from))
                .or_insert(0.0) += e.rate;
        }

        let mut per_class: BTreeMap<(RuleId, usize), Matrix> = BTreeMap::new();
        for (key, channel) in rates {
            let m = per_class.entry(key).or_insert_with(|| Matrix::zeros(n));
            for ((to, from), rate) in channel {
                m.add_to(to, from, C::real(rate.sqrt()));
            }
        }

        // Self-transitions are kept, unlike in the classical `Generator`, where
        // they cannot appear off-diagonal. A self-loop is a real jump channel:
        // it damps the norm and it can fire. This is the same convention the
        // sampler uses, and the reason the two agree on `a₀` while `Q` does not.
        let basis = if g.node_keys.len() == n {
            g.node_keys.clone()
        } else {
            // A graph assembled without configuration keys — a term projection,
            // for instance. Markings are then the identity, correctly.
            g.node_labels.clone()
        };
        QctmcModel {
            basis,
            labels: g.node_labels.clone(),
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

    /// Apply an overall phase to a class's jump operator.
    ///
    /// This is a **carrier check**, not an interference knob, and an earlier
    /// version of this comment claimed otherwise — that it "turns a
    /// classical-looking model into one that interferes." It does not, and the
    /// suite has always said so: `an_overall_phase_leaves_populations_invariant`
    /// asserts that the populations are untouched, which is elementary, since a
    /// global phase on a jump operator cancels in `LρL†`.
    ///
    /// Nor could a per-class phase ever produce cancellation. Within one
    /// `L_{r,φ}` every derivation carries the same `z`, while `g` and `χ` are
    /// non-negative reals; distinct classes are distinct operators whose
    /// dissipators add as operations. There is no relative phase in the
    /// formalism for a cancellation to come from. Obtaining one needs
    /// amplitudes indexed by *derivations* rather than by refinement classes,
    /// which is a strictly larger construction and not a repair of this one.
    ///
    /// What this is good for: confirming that phases are carried through the
    /// pipeline rather than dropped on construction, and supplying a nonzero
    /// off-diagonal to [`QctmcModel::with_hamiltonian`] experiments.
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

    /// Resolve a **configuration key** (marking and weight fingerprint) to its
    /// basis index. Unique by construction.
    pub fn index_of(&self, configuration_key: &str) -> Option<usize> {
        self.basis.iter().position(|b| b == configuration_key)
    }

    /// Resolve a **marking** to every basis index carrying it.
    ///
    /// Returns a set rather than an option because it genuinely is one: under a
    /// plastic theory the same term is reached with different weight maps, and
    /// those are different configurations with different futures. A caller that
    /// wants a single index wants a configuration key, so it should be using
    /// [`QctmcModel::index_of`]; a caller that wants the term-marginal
    /// population should sum over what this returns.
    pub fn index_of_term(&self, marking: &str) -> Vec<usize> {
        self.labels
            .iter()
            .enumerate()
            .filter(|(_, l)| l.as_str() == marking)
            .map(|(i, _)| i)
            .collect()
    }

    /// The population of a *term*, summed over the configurations carrying it.
    /// The honest term-marginal of a plastic model.
    pub fn term_population(&self, rho: &Matrix, marking: &str) -> f64 {
        self.index_of_term(marking)
            .into_iter()
            .map(|i| rho.get(i, i).re)
            .sum()
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
/// derivations out of a state reach distinct targets.
///
/// This is a *diagnostic*, and no longer a hypothesis of anything. Under the
/// earlier sum-of-roots normalisation the degeneration theorem needed it,
/// because without it `‖L|c⟩‖²` was not the classical propensity. Under
/// root-of-sum the identity is exact regardless, so what this now reports is a
/// structural fact about the model — how much multiset degeneracy the
/// presentation carries — and not a precondition for using it.
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

/// The **degeneracy** map: for each class, the number of distinct derivations
/// reaching each target, where that number exceeds one.
///
/// Formerly called an interference budget, on the view that `m`
/// indistinguishable reactants give an amplitude `m·z` and hence a weight `m²|z|²` against a
/// classical rate of `m|z|²`. They do not, under the normalisation this crate
/// now uses: the amplitude is `√m·|z|`, the weight is `m|z|²`, and the two
/// readings agree. See the module documentation for why summing roots was the
/// error rather than the discovery.
///
/// What the map is still worth reporting: it is exactly the multiset degeneracy
/// of the presentation, so a nonempty result tells a modeller that the
/// classical multiplicity factor `h` is doing real work at these transitions —
/// which is the place where omitting it would silently halve a rate, and the
/// hardest kind of error to see in a trace.
pub fn degeneracy(g: &LabelledTransitionGraph) -> BTreeMap<(usize, usize, usize), usize> {
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

/// Former name of [`degeneracy`]. Kept so external callers break loudly at the
/// name rather than quietly at the semantics.
#[deprecated(note = "renamed to `degeneracy`: these counts are not interference")]
pub fn interference(g: &LabelledTransitionGraph) -> BTreeMap<(usize, usize, usize), usize> {
    degeneracy(g)
}
