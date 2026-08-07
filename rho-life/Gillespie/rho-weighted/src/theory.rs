//! Weighted rewrite systems.
//!
//! A weight map instance is *part of a state specification*. That single
//! sentence is the design: the map lives in the configuration, not in the
//! declaration, so a rewrite may update it, and the process stays a
//! time-homogeneous CTMC on configurations rather than becoming a semi-Markov
//! process on terms (note, Theorem 21).
//!
//! Rates are in `ℝ≥0` (DR-W8), not the unit interval. A Gillespie waiting time
//! is `a₀⁻¹ ln(1/u)` and therefore carries units of time; a weight is a rate.
//! The unit interval is available as a derived presentation (uniformisation),
//! not as the specification.

use std::collections::BTreeMap;
use std::sync::Arc;

use crate::logic::{Budget, Formula, Partition};
use crate::matching::Bindings;
use crate::redex::{Redex, RuleId};
use crate::space::Space;

// ---------------------------------------------------------------------------
// Rates
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum RateValue {
    /// A rate in `ℝ≥0`, units of inverse time.
    Real(f64),
    /// An amplitude `z`, for the quantum reading. Dimensionless; the rate it
    /// denotes is `λ(z) = |z|²`. **Unbounded** — see [`RateValue::complex`].
    Complex(f64, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum RateError {
    Negative(f64),
    NotFinite(f64),
}

impl RateValue {
    pub fn real(r: f64) -> Result<RateValue, RateError> {
        if !r.is_finite() {
            Err(RateError::NotFinite(r))
        } else if r < 0.0 {
            Err(RateError::Negative(r))
        } else {
            Ok(RateValue::Real(r))
        }
    }

    /// An amplitude. No bound is imposed on `|z|`, and an earlier version of
    /// this crate imposed `|z| ≤ 1`.
    ///
    /// That bound was imported from discrete quantum-operation intuition and
    /// belongs to neither of the two things it was supposed to protect. On the
    /// finite-dimensional space `ℓ²(B)` every operator is bounded outright, so
    /// nothing has to be assumed to make it so; and the GKSL theorem imposes no
    /// norm condition on jump operators, scaling one by ten being an increase
    /// of a dissipation rate and not a violation. It also manufactured a
    /// dimensional inconsistency, since classical weights are unbounded
    /// inverse-time rates while `|z| ≤ 1` made amplitudes bounded dimensionless
    /// numbers with no stated relation between them. [`RateValue::rate`] is
    /// that relation.
    pub fn complex(re: f64, im: f64) -> Result<RateValue, RateError> {
        if !re.is_finite() {
            Err(RateError::NotFinite(re))
        } else if !im.is_finite() {
            Err(RateError::NotFinite(im))
        } else {
            Ok(RateValue::Complex(re, im))
        }
    }

    /// The classical rate an entry denotes.
    ///
    /// This is the interpretation map `λ : ℂ → ℝ≥0`, `λ(z) = |z|²`, of the note
    /// (Remark 12). The two codomains are *not* interchangeable instantiations
    /// of one semiring parameter: a real weight is a rate with units of inverse
    /// time, a complex weight is a dimensionless amplitude, and `λ` is what
    /// relates them. Everything in the classical layer is stated for a rate and
    /// reads a complex entry through here.
    pub fn rate(&self) -> f64 {
        match self {
            RateValue::Real(r) => *r,
            RateValue::Complex(re, im) => re * re + im * im,
        }
    }
}

// ---------------------------------------------------------------------------
// Weight maps
// ---------------------------------------------------------------------------

/// A key into the weight map: which rule, which refinement.
pub type Key = (RuleId, usize);

#[derive(Clone, Debug, Default, PartialEq)]
pub struct WeightMap {
    entries: BTreeMap<Key, RateValue>,
}

impl WeightMap {
    pub fn new() -> WeightMap {
        WeightMap::default()
    }

    pub fn set(&mut self, k: Key, v: RateValue) {
        self.entries.insert(k, v);
    }

    pub fn get(&self, k: Key) -> RateValue {
        self.entries.get(&k).copied().unwrap_or(RateValue::Real(0.0))
    }

    pub fn rate(&self, k: Key) -> f64 {
        self.get(k).rate()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Key, &RateValue)> {
        self.entries.iter()
    }

    /// A stable key for the map, so a configuration can be identified.
    pub fn fingerprint(&self) -> String {
        self.entries
            .iter()
            .map(|((r, i), v)| format!("{}:{}={:.6}", r.0, i, v.rate()))
            .collect::<Vec<_>>()
            .join(",")
    }
}

// ---------------------------------------------------------------------------
// Update functions
// ---------------------------------------------------------------------------

/// Everything an update function may see.
///
/// The signature is the note's *extended* one from the start, so
/// spike-timing-dependent plasticity does not need a later breaking change.
/// `TraceSummary` must be finite-dimensional and deterministically updated, or
/// the process stops being Markov and exact simulation is off the table — which
/// is exactly why the eligibility-trace formulation of STDP is the standard
/// device.
pub struct UpdateCtx<'a> {
    pub key: Key,
    pub bindings: &'a Bindings,
    pub position: &'a str,
    pub time: f64,
    pub trace: &'a TraceSummary,
}

#[derive(Clone, Debug, Default)]
pub struct TraceSummary {
    /// One eligibility value per key, decaying.
    pub eligibility: BTreeMap<Key, f64>,
    pub last_fired: BTreeMap<Key, f64>,
}

pub type UpdateFn = Arc<dyn Fn(&WeightMap, &UpdateCtx) -> WeightMap + Send + Sync>;

/// Identity: the SPiM case, where rates are fixed at declaration.
pub fn id_update() -> UpdateFn {
    Arc::new(|w, _| w.clone())
}

/// Multiply the fired class's rate.
pub fn scale(f: f64) -> UpdateFn {
    Arc::new(move |w, ctx| {
        let mut out = w.clone();
        out.set(ctx.key, RateValue::Real(w.rate(ctx.key) * f));
        out
    })
}

/// Hebbian potentiation with a ceiling (note, Definition 44): the event which
/// transmits is the event which potentiates. Saturating, which also keeps the
/// reachable weight set finite when the increment is quantised.
pub fn saturating_add(eta: f64, w_max: f64) -> UpdateFn {
    Arc::new(move |w, ctx| {
        let mut out = w.clone();
        out.set(
            ctx.key,
            RateValue::Real((w.rate(ctx.key) + eta).min(w_max)),
        );
        out
    })
}

/// Timing-dependent potentiation: uses the clock and the eligibility trace, so
/// it exercises the extended signature.
pub fn stdp(eta: f64, tau: f64, w_max: f64) -> UpdateFn {
    Arc::new(move |w, ctx| {
        let last = ctx.trace.last_fired.get(&ctx.key).copied();
        let bump = match last {
            Some(t0) => eta * (-(ctx.time - t0).abs() / tau).exp(),
            None => eta,
        };
        let mut out = w.clone();
        out.set(ctx.key, RateValue::Real((w.rate(ctx.key) + bump).min(w_max)));
        out
    })
}

// ---------------------------------------------------------------------------
// Geometric factor and funding gate
// ---------------------------------------------------------------------------

/// `g(k)` — where the interaction is, priced.
///
/// Context rules contribute a multiplicative factor and **never an independent
/// propensity**: a context rule is addressing, not an event, so giving it its
/// own draw turns one event into two. Default `1`, which is "addressing is
/// free".
///
/// This is also the honest home of a spatial bias, and of any constraint that
/// depends on the surrounding configuration rather than on the redex's own
/// shape — a channel capacity bound, for instance.
pub type GeometricFactor = Arc<dyn Fn(&Space, &Redex) -> f64 + Send + Sync>;

pub fn unit_geometry() -> GeometricFactor {
    Arc::new(|_, _| 1.0)
}

/// A saturating channel: a firing that would push a capped channel past `cap`
/// contributes nothing.
///
/// The check is on the *result* of the firing, not on the current occupancy,
/// because the current occupancy cannot distinguish a redex that fills a
/// channel from one that drains it — a naive check deadlocks the consumer at
/// exactly the point it is most needed. Firing to decide costs one `fire` per
/// redex per propensity computation, which is affordable precisely because
/// `fire` is non-destructive.
///
/// Bounding occupancy bounds the reachable state space, which is what
/// exhaustive exploration and the quantum construction need.
pub fn capacity(cap: usize, channels: Vec<String>) -> GeometricFactor {
    Arc::new(move |space, r| {
        let after = space.fire(r);
        for c in &channels {
            if after.occupancy(c) > cap {
                return 0.0;
            }
        }
        1.0
    })
}

/// `χ` — whether the interaction can be afforded.
///
/// Wires to the branch's decidable funding judgment `funds Σ Δ := Δ ≤ Σ`.
/// Decidability matters: a Boolean gate keeps the simulator exact, where a
/// real-valued penalty would make propensity depend on an optimisation.
pub type Gate = Arc<dyn Fn(&Space, &Redex) -> bool + Send + Sync>;

pub fn open_gate() -> Gate {
    Arc::new(|_, _| true)
}

// ---------------------------------------------------------------------------
// The theory
// ---------------------------------------------------------------------------

pub struct RefinementEntry {
    pub weight: RateValue,
    pub update: UpdateFn,
    pub label: String,
}

pub struct BaseRule {
    pub id: RuleId,
    pub name: String,
    pub lhs_shape: Formula,
    pub partition: Partition,
    pub entries: Vec<RefinementEntry>,
}

pub struct WeightedTheory {
    pub rules: Vec<BaseRule>,
    pub geometric: GeometricFactor,
    pub gate: Gate,
}

impl WeightedTheory {
    pub fn rule(&self, id: RuleId) -> Option<&BaseRule> {
        self.rules.iter().find(|r| r.id == id)
    }

    /// The initial weight map, read off the declared entries.
    pub fn initial_weights(&self) -> WeightMap {
        let mut w = WeightMap::new();
        for r in &self.rules {
            for (i, e) in r.entries.iter().enumerate() {
                w.set((r.id, i), e.weight);
            }
        }
        w
    }

    /// A content fingerprint, recorded in every study's provenance so a graph
    /// can be traced to the theory that produced it.
    pub fn fingerprint(&self) -> String {
        let mut s = String::new();
        for r in &self.rules {
            s.push_str(&format!("{}:{}[", r.id.0, r.name));
            for (i, e) in r.entries.iter().enumerate() {
                s.push_str(&format!(
                    "{}={}:{:.6};",
                    i,
                    r.partition.keys()[i].render(),
                    e.weight.rate()
                ));
            }
            s.push(']');
        }
        s
    }
}

/// A configuration: the state of the simulated system. The weight map is *in*
/// here, which is the whole point.
#[derive(Clone, Debug)]
pub struct Configuration {
    pub space: Space,
    pub weights: WeightMap,
    pub trace: TraceSummary,
}

impl Configuration {
    pub fn new(space: Space, weights: WeightMap) -> Configuration {
        Configuration {
            space,
            weights,
            trace: TraceSummary::default(),
        }
    }

    /// Identifies the configuration as a node of a transition graph. Term and
    /// map both contribute, because two runs reaching the same term with
    /// different maps have different futures (note, Example 25).
    pub fn key(&self) -> String {
        format!("{}|{}", self.space.marking().key(), self.weights.fingerprint())
    }

    /// The term-marginal key, for studies that model check the term only.
    pub fn term_key(&self) -> String {
        self.space.marking().key()
    }
}

/// Classify a redex: which refinement of the rule's left-hand side it inhabits.
/// Total and single-valued by the partition discipline.
pub fn classify(
    theory: &WeightedTheory,
    space: &Space,
    r: &Redex,
    b: &mut Budget,
) -> Option<usize> {
    let rule = theory.rule(r.rule)?;
    let local = crate::redex::local_term(space, r);
    rule.partition.classify(&local, b)
}
