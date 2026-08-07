//! # Augmented Rewrite Rules (Corrected Semantics)
//!
//! This module implements the augmented rewrite rule system where:
//!
//! ## Base Rules
//!
//! ```text
//! <base-rule-name> . <rule> [<weight>] {
//!     <type-refinement-of-LHS>_i => (<weight>_i, <update-function>_i)
//! }
//! ```
//!
//! Each base rule has:
//! - A **weight** governing how likely the rule is to be selected.
//! - A map of **refinement entries**, keyed by spatial behavior terms that
//!   refine the type of the LHS. Each entry carries:
//!   - A **weight** specific to that refinement.
//!   - An **update function** that takes the entire rules annotation map
//!     and produces a (possibly new) annotation map. This is how firing
//!     a rule transforms the stochastic/quantum state.
//!
//! ## Context Rules
//!
//! ```text
//! <ctxt-rule-name> . <rule> [<weight>] {
//!     <fold-function>
//! }
//! ```
//!
//! Each context rule (structural/congruence) has:
//! - A **weight** governing selection probability.
//! - A **fold function** that takes the annotation map(s) from the context
//!   (i.e., the sub-derivation where a base rule was applied) and combines
//!   them with the updated map from the applied rule.
//!
//! ## Flow
//!
//! 1. A base rule fires on a redex, selected by its weight.
//! 2. Within the base rule, a refinement is selected by its weight.
//! 3. The selected refinement's **update function** transforms the
//!    annotation map, producing a new map.
//! 4. If the rewrite occurs inside a context (e.g., under parallel
//!    composition), the context rule's **fold function** combines
//!    the updated map with the maps from the surrounding context.
//!
//! This produces the annotation map that governs the next step of
//! the Gillespie simulator.

use crate::rate_map::RateMap;
use crate::rate_value::RateValue;
use crate::spatial_behavior::SpatialBehavior;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::Arc;

// ─── Term References ───────────────────────────────────────────────────────

/// An abstract term reference.
///
/// In the full MeTTaIL integration, this would be parameterized over the
/// generated AST type from the `language!` macro.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TermRef {
    pub id: u64,
    pub sort: String,
    pub display: String,
}

impl TermRef {
    pub fn new(id: u64, sort: impl Into<String>, display: impl Into<String>) -> Self {
        TermRef {
            id,
            sort: sort.into(),
            display: display.into(),
        }
    }
}

impl fmt::Display for TermRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display)
    }
}

// ─── Update and Fold Functions ─────────────────────────────────────────────

/// An update function transforms an annotation map into a (possibly new) one.
///
/// This is the core semantic action of a refinement entry in a base rule.
/// When a base rule fires with a particular spatial behavior refinement,
/// the update function determines how the annotation map evolves.
///
/// In the classical (stochastic π-machine) case, this might renormalize
/// rates after a communication event.
///
/// In the quantum (CTMC) case, this might apply a unitary transformation
/// to the amplitude map.
pub type UpdateFn = Arc<dyn Fn(&RateMap) -> RateMap + Send + Sync>;

/// A fold function combines context maps with the map from an applied rule.
///
/// When a rewrite occurs inside a larger term (e.g., a parallel composition),
/// the context rule's fold function takes:
/// - The annotation maps from the context (surrounding sub-terms)
/// - The updated annotation map from the applied base rule
///
/// And produces a single combined annotation map for the whole term.
///
/// For classical simulation, this is typically addition or product of rates.
/// For quantum simulation, this could be a tensor product of amplitude maps.
pub type FoldFn = Arc<dyn Fn(&[&RateMap], &RateMap) -> RateMap + Send + Sync>;

// ─── Refinement Entry ──────────────────────────────────────────────────────

/// A single refinement entry in a base rule's annotation block.
///
/// Maps a spatial behavior (type refinement of the LHS) to a weight
/// and an update function.
#[derive(Clone)]
pub struct RefinementEntry {
    /// The spatial behavior that refines the type of the LHS.
    pub behavior: SpatialBehavior,
    /// The weight for this refinement (used in Gillespie selection).
    pub weight: f64,
    /// The update function: takes the current annotation map,
    /// produces the updated annotation map after this refinement fires.
    pub update: UpdateFn,
}

impl RefinementEntry {
    pub fn new(
        behavior: SpatialBehavior,
        weight: f64,
        update: impl Fn(&RateMap) -> RateMap + Send + Sync + 'static,
    ) -> Self {
        RefinementEntry {
            behavior,
            weight,
            update: Arc::new(update),
        }
    }

    /// Apply the update function to an annotation map.
    pub fn apply_update(&self, map: &RateMap) -> RateMap {
        (self.update)(map)
    }
}

impl fmt::Debug for RefinementEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RefinementEntry")
            .field("behavior", &self.behavior)
            .field("weight", &self.weight)
            .field("update", &"<fn>")
            .finish()
    }
}

// ─── Base Rule ─────────────────────────────────────────────────────────────

/// A base rewrite rule with weight and refinement entries.
///
/// ```text
/// COMM . (PPar {(PInput n ^x.p), (POutput n q), ...rest})
///        ~> (PPar {(subst ^x.p (NQuote q)), ...rest})
///        [0.7] {
///            comm(n, n) => (0.5, update_comm),
///            local(n)   => (0.3, update_local),
///        }
/// ```
#[derive(Clone)]
pub struct BaseRule {
    /// Name of the rule (e.g., "COMM", "DROP").
    pub name: String,
    /// The LHS pattern term.
    pub lhs: TermRef,
    /// The RHS result term.
    pub rhs: TermRef,
    /// The rule-level weight (for selecting among competing rules).
    pub weight: f64,
    /// Refinement entries: spatial behaviors with weights and update functions.
    pub refinements: Vec<RefinementEntry>,
}

impl BaseRule {
    pub fn new(
        name: impl Into<String>,
        lhs: TermRef,
        rhs: TermRef,
        weight: f64,
    ) -> Self {
        BaseRule {
            name: name.into(),
            lhs,
            rhs,
            weight,
            refinements: Vec::new(),
        }
    }

    /// Add a refinement entry.
    pub fn add_refinement(mut self, entry: RefinementEntry) -> Self {
        self.refinements.push(entry);
        self
    }

    /// Total refinement weight (sum of all refinement entry weights).
    pub fn total_refinement_weight(&self) -> f64 {
        self.refinements.iter().map(|r| r.weight).sum()
    }

    /// Effective propensity = rule weight × total refinement weight.
    pub fn propensity(&self) -> f64 {
        self.weight * self.total_refinement_weight()
    }

    /// Select a refinement by weighted random choice.
    /// Returns the index of the selected refinement.
    pub fn select_refinement(&self, uniform_sample: f64) -> Option<usize> {
        let total = self.total_refinement_weight();
        if total < 1e-15 {
            return None;
        }
        let threshold = uniform_sample * total;
        let mut cumulative = 0.0;
        for (i, entry) in self.refinements.iter().enumerate() {
            cumulative += entry.weight;
            if cumulative >= threshold {
                return Some(i);
            }
        }
        Some(self.refinements.len() - 1)
    }
}

impl fmt::Debug for BaseRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BaseRule")
            .field("name", &self.name)
            .field("lhs", &self.lhs)
            .field("rhs", &self.rhs)
            .field("weight", &self.weight)
            .field("refinements", &self.refinements)
            .finish()
    }
}

impl fmt::Display for BaseRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} . {} ~> {} [{}] {{",
            self.name, self.lhs, self.rhs, self.weight
        )?;
        for (i, entry) in self.refinements.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{} => ({}, <update>)", entry.behavior, entry.weight)?;
        }
        write!(f, "}}")
    }
}

// ─── Context Rule ──────────────────────────────────────────────────────────

/// A context (structural/congruence) rule with weight and fold function.
///
/// ```text
/// PAR_CTXT . if S ~> T then (PPar {S, ...rest}) ~> (PPar {T, ...rest})
///            [1.0] {
///                fold_par  // combines maps from rest with updated map from S~>T
///            }
/// ```
#[derive(Clone)]
pub struct ContextRule {
    /// Name of the rule (e.g., "PAR_CTXT").
    pub name: String,
    /// The LHS pattern term (the context).
    pub lhs: TermRef,
    /// The RHS result term.
    pub rhs: TermRef,
    /// The rule-level weight.
    pub weight: f64,
    /// Description of the condition (e.g., "if S ~> T").
    pub condition: String,
    /// The fold function: combines context maps with the applied rule's map.
    pub fold: FoldFn,
}

impl ContextRule {
    pub fn new(
        name: impl Into<String>,
        lhs: TermRef,
        rhs: TermRef,
        weight: f64,
        condition: impl Into<String>,
        fold: impl Fn(&[&RateMap], &RateMap) -> RateMap + Send + Sync + 'static,
    ) -> Self {
        ContextRule {
            name: name.into(),
            lhs,
            rhs,
            weight,
            condition: condition.into(),
            fold: Arc::new(fold),
        }
    }

    /// Apply the fold function.
    pub fn apply_fold(&self, context_maps: &[&RateMap], rule_map: &RateMap) -> RateMap {
        (self.fold)(context_maps, rule_map)
    }
}

impl fmt::Debug for ContextRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ContextRule")
            .field("name", &self.name)
            .field("lhs", &self.lhs)
            .field("rhs", &self.rhs)
            .field("weight", &self.weight)
            .field("condition", &self.condition)
            .field("fold", &"<fn>")
            .finish()
    }
}

impl fmt::Display for ContextRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} . {} {} ~> {} [{}] {{ <fold> }}",
            self.name, self.condition, self.lhs, self.rhs, self.weight
        )
    }
}

// ─── Rule Enum ─────────────────────────────────────────────────────────────

/// A rule in the augmented rewrite system: either a base rule or a context rule.
#[derive(Clone, Debug)]
pub enum Rule {
    Base(BaseRule),
    Context(ContextRule),
}

impl Rule {
    pub fn name(&self) -> &str {
        match self {
            Rule::Base(r) => &r.name,
            Rule::Context(r) => &r.name,
        }
    }

    pub fn weight(&self) -> f64 {
        match self {
            Rule::Base(r) => r.weight,
            Rule::Context(r) => r.weight,
        }
    }

    pub fn lhs(&self) -> &TermRef {
        match self {
            Rule::Base(r) => &r.lhs,
            Rule::Context(r) => &r.lhs,
        }
    }

    pub fn rhs(&self) -> &TermRef {
        match self {
            Rule::Base(r) => &r.rhs,
            Rule::Context(r) => &r.rhs,
        }
    }

    pub fn lhs_sort(&self) -> &str {
        &self.lhs().sort
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rule::Base(r) => write!(f, "{}", r),
            Rule::Context(r) => write!(f, "{}", r),
        }
    }
}

// ─── Rewrite System ────────────────────────────────────────────────────────

/// A collection of augmented rewrite rules.
#[derive(Clone, Debug)]
pub struct RewriteSystem {
    pub rules: Vec<Rule>,
}

impl RewriteSystem {
    pub fn new() -> Self {
        RewriteSystem { rules: Vec::new() }
    }

    pub fn add_base_rule(mut self, rule: BaseRule) -> Self {
        self.rules.push(Rule::Base(rule));
        self
    }

    pub fn add_context_rule(mut self, rule: ContextRule) -> Self {
        self.rules.push(Rule::Context(rule));
        self
    }

    /// Get all base rules matching a given sort.
    pub fn base_rules_for_sort(&self, sort: &str) -> Vec<&BaseRule> {
        self.rules
            .iter()
            .filter_map(|r| match r {
                Rule::Base(br) if br.lhs.sort == sort => Some(br),
                _ => None,
            })
            .collect()
    }

    /// Get all context rules matching a given sort.
    pub fn context_rules_for_sort(&self, sort: &str) -> Vec<&ContextRule> {
        self.rules
            .iter()
            .filter_map(|r| match r {
                Rule::Context(cr) if cr.lhs.sort == sort => Some(cr),
                _ => None,
            })
            .collect()
    }

    /// Total weight of all rules.
    pub fn total_weight(&self) -> f64 {
        self.rules.iter().map(|r| r.weight()).sum()
    }

    /// Total propensity (for base rules, weight × refinement weights).
    pub fn total_propensity(&self) -> f64 {
        self.rules
            .iter()
            .map(|r| match r {
                Rule::Base(br) => br.propensity(),
                Rule::Context(cr) => cr.weight,
            })
            .sum()
    }
}

impl Default for RewriteSystem {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Standard Update Functions ─────────────────────────────────────────────

/// Identity update: returns the map unchanged.
pub fn update_identity() -> UpdateFn {
    Arc::new(|map: &RateMap| map.clone())
}

/// Scale all rates in the map by a constant factor.
pub fn update_scale(factor: f64) -> UpdateFn {
    Arc::new(move |map: &RateMap| {
        let mut new_map = RateMap::new();
        for (sb, rv) in map.entries() {
            let scaled = match rv {
                RateValue::Real(r) => RateValue::Real((r * factor).min(1.0)),
                RateValue::Complex(z) => {
                    RateValue::Complex(z * num_complex::Complex64::new(factor, 0.0))
                }
            };
            new_map.insert(sb.clone(), scaled);
        }
        new_map
    })
}

/// Remove the entry for a specific spatial behavior (channel consumed).
pub fn update_remove(behavior: SpatialBehavior) -> UpdateFn {
    Arc::new(move |map: &RateMap| {
        let entries: Vec<_> = map
            .entries()
            .iter()
            .filter(|(sb, _)| sb != &behavior)
            .cloned()
            .collect();
        RateMap::from_entries(entries)
    })
}

/// Add or replace an entry in the map.
pub fn update_set(behavior: SpatialBehavior, value: RateValue) -> UpdateFn {
    Arc::new(move |map: &RateMap| {
        let mut new_map = map.clone();
        new_map.insert(behavior.clone(), value);
        new_map
    })
}

/// Compose two update functions: apply first, then second.
pub fn update_compose(first: UpdateFn, second: UpdateFn) -> UpdateFn {
    Arc::new(move |map: &RateMap| {
        let intermediate = first(map);
        second(&intermediate)
    })
}

// ─── Standard Fold Functions ───────────────────────────────────────────────

/// Merge fold: merge all context maps with the rule map using rate addition.
/// This is the natural fold for parallel composition in the stochastic case.
pub fn fold_merge() -> FoldFn {
    Arc::new(|context_maps: &[&RateMap], rule_map: &RateMap| {
        let mut result = rule_map.clone();
        for ctx_map in context_maps {
            result = result.merge(ctx_map);
        }
        result
    })
}

/// Product fold: multiply rates from context with rates from the rule.
/// This is appropriate when context constrains the rule's rates.
pub fn fold_product() -> FoldFn {
    Arc::new(|context_maps: &[&RateMap], rule_map: &RateMap| {
        let mut result = rule_map.clone();
        for ctx_map in context_maps {
            if let Ok(composed) = result.compose(ctx_map) {
                result = composed;
            }
        }
        result
    })
}

/// Replace fold: ignore context maps, use only the rule's updated map.
pub fn fold_replace() -> FoldFn {
    Arc::new(|_context_maps: &[&RateMap], rule_map: &RateMap| rule_map.clone())
}

/// Custom fold with a user-provided combining function applied pairwise.
pub fn fold_pairwise(
    combine: impl Fn(&RateMap, &RateMap) -> RateMap + Send + Sync + 'static,
) -> FoldFn {
    let combine = Arc::new(combine);
    Arc::new(move |context_maps: &[&RateMap], rule_map: &RateMap| {
        let mut result = rule_map.clone();
        for ctx_map in context_maps {
            result = combine(&result, ctx_map);
        }
        result
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_comm_rule() -> BaseRule {
        BaseRule::new(
            "COMM",
            TermRef::new(1, "Proc", "x?(y).P | x!(Q)"),
            TermRef::new(2, "Proc", "P{Q/y}"),
            0.7,
        )
        .add_refinement(RefinementEntry::new(
            SpatialBehavior::interaction("x", "x"),
            0.5,
            |map| {
                // After comm on x, scale remaining rates by 0.8
                let mut new_map = RateMap::new();
                for (sb, rv) in map.entries() {
                    let scaled = match rv {
                        RateValue::Real(r) => RateValue::Real(r * 0.8),
                        other => *other,
                    };
                    new_map.insert(sb.clone(), scaled);
                }
                new_map
            },
        ))
        .add_refinement(RefinementEntry::new(
            SpatialBehavior::local("x"),
            0.3,
            |map| map.clone(), // identity update for local
        ))
    }

    fn make_par_context_rule() -> ContextRule {
        ContextRule::new(
            "PAR_CTXT",
            TermRef::new(10, "Proc", "{S, ...rest}"),
            TermRef::new(11, "Proc", "{T, ...rest}"),
            1.0,
            "if S ~> T",
            |context_maps, rule_map| {
                // Merge: combine context maps with the applied rule's map
                let mut result = rule_map.clone();
                for ctx_map in context_maps {
                    result = result.merge(ctx_map);
                }
                result
            },
        )
    }

    #[test]
    fn test_base_rule_propensity() {
        let rule = make_comm_rule();
        // propensity = rule_weight * (0.5 + 0.3) = 0.7 * 0.8 = 0.56
        assert!((rule.propensity() - 0.56).abs() < 1e-10);
    }

    #[test]
    fn test_refinement_selection() {
        let rule = make_comm_rule();
        // Total refinement weight = 0.8
        // sample near 0 → first refinement (weight 0.5, cumulative 0.5/0.8 = 0.625)
        assert_eq!(rule.select_refinement(0.1), Some(0));
        // sample near 1 → second refinement
        assert_eq!(rule.select_refinement(0.9), Some(1));
    }

    #[test]
    fn test_update_function() {
        let rule = make_comm_rule();
        let mut initial_map = RateMap::new();
        initial_map.insert(
            SpatialBehavior::local("y"),
            RateValue::real(0.6).unwrap(),
        );

        // Apply first refinement's update (scales by 0.8)
        let updated = rule.refinements[0].apply_update(&initial_map);
        let y_rate = updated
            .get(&SpatialBehavior::local("y"))
            .unwrap()
            .probability();
        assert!((y_rate - 0.48).abs() < 1e-10); // 0.6 * 0.8
    }

    #[test]
    fn test_context_fold() {
        let ctx_rule = make_par_context_rule();

        let mut rule_map = RateMap::new();
        rule_map.insert(
            SpatialBehavior::local("x"),
            RateValue::real(0.3).unwrap(),
        );

        let mut ctx_map = RateMap::new();
        ctx_map.insert(
            SpatialBehavior::local("y"),
            RateValue::real(0.5).unwrap(),
        );

        let folded = ctx_rule.apply_fold(&[&ctx_map], &rule_map);
        // Should contain both x and y entries
        assert!(folded.get(&SpatialBehavior::local("x")).is_some());
        assert!(folded.get(&SpatialBehavior::local("y")).is_some());
    }

    #[test]
    fn test_rewrite_system() {
        let system = RewriteSystem::new()
            .add_base_rule(make_comm_rule())
            .add_context_rule(make_par_context_rule());

        assert_eq!(system.rules.len(), 2);
        assert_eq!(system.base_rules_for_sort("Proc").len(), 1);
        assert_eq!(system.context_rules_for_sort("Proc").len(), 1);
    }

    #[test]
    fn test_standard_update_scale() {
        let mut map = RateMap::new();
        map.insert(SpatialBehavior::local("x"), RateValue::real(0.5).unwrap());
        let update = update_scale(0.6);
        let result = update(&map);
        let p = result.get(&SpatialBehavior::local("x")).unwrap().probability();
        assert!((p - 0.3).abs() < 1e-10);
    }

    #[test]
    fn test_standard_fold_merge() {
        let mut map1 = RateMap::new();
        map1.insert(SpatialBehavior::local("x"), RateValue::real(0.3).unwrap());
        let mut map2 = RateMap::new();
        map2.insert(SpatialBehavior::local("y"), RateValue::real(0.4).unwrap());

        let fold = fold_merge();
        let result = fold(&[&map2], &map1);
        assert!(result.get(&SpatialBehavior::local("x")).is_some());
        assert!(result.get(&SpatialBehavior::local("y")).is_some());
    }
}
