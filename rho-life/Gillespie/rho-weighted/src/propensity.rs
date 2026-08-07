//! Propensity.
//!
//! ```text
//! a(r, φ, c) = w(φ) · Σ_{k ∈ K(r,φ,P)} g(k) · χ(r,k,σ)
//! ```
//!
//! Four independent factors that the literature usually presents fused:
//!
//! * `w(φ)` — the rate constant. What kind of interaction this is, priced.
//!   Dynamic: this is the component update functions rewrite.
//! * `g(k)` — the geometric factor. Where the interaction is, priced. Default 1.
//! * the cardinality of the sum — **multiplicity**. How many ways this
//!   interaction is available. This is Gillespie's `h_j`, and it is the factor
//!   the v1 prototype omitted entirely, with the consequence that a term with
//!   fifty available redexes of a kind fired them no faster than a term with
//!   one. A wrong multiplicity produces perfectly plausible traces, which is
//!   how the bug survived; `tests/analytic.rs` is what catches it.
//! * `χ` — the funding gate.

use crate::logic::Budget;
use crate::matching::Matching;
use crate::redex::{enumerate, Redex, RuleId};
use crate::theory::{classify, Configuration, WeightedTheory};

/// One enabled redex with its class and its contribution to `a₀`.
#[derive(Clone, Debug)]
pub struct Weighted {
    pub redex: Redex,
    pub class: usize,
    /// `w(φ) · g(k) · χ`, the redex's own contribution.
    pub rate: f64,
}

#[derive(Clone, Debug, Default)]
pub struct Propensities {
    pub items: Vec<Weighted>,
    pub total: f64,
}

impl Propensities {
    /// Aggregate by class, for reporting and for the by-key / by-redex
    /// agreement law.
    pub fn by_class(&self, rule: RuleId) -> Vec<(usize, f64)> {
        let mut out: std::collections::BTreeMap<usize, f64> = Default::default();
        for w in &self.items {
            if w.redex.rule == rule {
                *out.entry(w.class).or_insert(0.0) += w.rate;
            }
        }
        out.into_iter().collect()
    }

    /// Multiplicity of a class: how many redexes inhabit it.
    pub fn multiplicity(&self, rule: RuleId, class: usize) -> usize {
        self.items
            .iter()
            .filter(|w| w.redex.rule == rule && w.class == class)
            .count()
    }

    pub fn is_absorbing(&self) -> bool {
        self.total <= 0.0
    }
}

/// Compute propensities for a configuration.
///
/// Organised **by redex**, not by key (note, Prop. 20): walk the enabled
/// redexes, classify each, add its rate. That organisation is safe exactly
/// because the key set is a partition — without exclusivity the walk
/// over-counts, without exhaustiveness it under-counts.
pub fn propensities<M: Matching>(
    theory: &WeightedTheory,
    cfg: &Configuration,
    m: &M,
    b: &mut Budget,
) -> Propensities {
    let mut items = Vec::new();
    let mut total = 0.0;
    for r in enumerate(&cfg.space, m) {
        let class = match classify(theory, &cfg.space, &r, b) {
            Some(c) => c,
            // Unreachable under a checked partition; a redex with no class is
            // inert rather than silently defaulted.
            None => continue,
        };
        let w = cfg.weights.rate((r.rule, class));
        if w == 0.0 {
            items.push(Weighted {
                redex: r,
                class,
                rate: 0.0,
            });
            continue;
        }
        let g = (theory.geometric)(&cfg.space, &r);
        let chi = if (theory.gate)(&cfg.space, &r) { 1.0 } else { 0.0 };
        let rate = w * g * chi;
        total += rate;
        items.push(Weighted {
            redex: r,
            class,
            rate,
        });
    }
    Propensities { items, total }
}
