//! # Language Extension: Augmented Rewrite Syntax
//!
//! Documents the proposed syntax for augmenting `language!` macro rewrite
//! rules with weights, refinement update functions, and context fold functions.
//!
//! ## Proposed Syntax
//!
//! ```text
//! rules {
//!     // Base rule: weight + refinement entries with (weight, update_fn)
//!     COMM . (PPar {(PInput n ^x.p), (POutput n q), ...rest})
//!            ~> (PPar {(subst ^x.p (NQuote q)), ...rest})
//!            [0.7] {
//!                comm(n, n) => (0.5, scale(0.8)),
//!                local(n)   => (0.3, id),
//!            };
//!
//!     // Another base rule with no refinements (defaults to null => (1.0, id))
//!     DROP . (PDrop (NQuote P)) ~> P [1.0] {};
//!
//!     // Context rule: weight + fold function
//!     PAR_CTXT . if S ~> T
//!                then (PPar {S, ...rest}) ~> (PPar {T, ...rest})
//!                [1.0] {
//!                    merge   // fold: merge context maps with rule map
//!                };
//! }
//! ```
//!
//! ## Built-in Update Functions
//!
//! - `id` — identity, map unchanged
//! - `scale(f)` — multiply all rates by factor `f`
//! - `remove(behavior)` — remove the entry for a spatial behavior
//! - `set(behavior, value)` — set/replace an entry
//!
//! ## Built-in Fold Functions
//!
//! - `merge` — additive merge of context maps with rule map
//! - `product` — multiplicative composition
//! - `replace` — ignore context, keep only rule map
//!
//! ## Defaults
//!
//! - Base rules without refinement blocks get `{ null => (1.0, id) }`
//! - Context rules without fold blocks get `{ merge }`
//! - Rules without explicit weight get `[1.0]`

use crate::augmented_rule::*;
use crate::rate_map::RateMap;
use crate::rate_value::RateValue;
use crate::spatial_behavior::SpatialBehavior;

/// Builder for constructing base rules with a fluent API.
pub struct BaseRuleBuilder {
    name: String,
    lhs: Option<TermRef>,
    rhs: Option<TermRef>,
    weight: f64,
    refinements: Vec<(SpatialBehavior, f64, UpdateFn)>,
}

impl BaseRuleBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        BaseRuleBuilder {
            name: name.into(),
            lhs: None,
            rhs: None,
            weight: 1.0,
            refinements: Vec::new(),
        }
    }

    pub fn lhs(mut self, id: u64, sort: &str, display: &str) -> Self {
        self.lhs = Some(TermRef::new(id, sort, display));
        self
    }

    pub fn rhs(mut self, id: u64, sort: &str, display: &str) -> Self {
        self.rhs = Some(TermRef::new(id, sort, display));
        self
    }

    pub fn weight(mut self, w: f64) -> Self {
        self.weight = w;
        self
    }

    /// Add a refinement with a custom update function.
    pub fn refinement(
        mut self,
        behavior: SpatialBehavior,
        weight: f64,
        update: impl Fn(&RateMap) -> RateMap + Send + Sync + 'static,
    ) -> Self {
        self.refinements
            .push((behavior, weight, std::sync::Arc::new(update)));
        self
    }

    /// Add a refinement with the identity update.
    pub fn refinement_id(self, behavior: SpatialBehavior, weight: f64) -> Self {
        self.refinement(behavior, weight, |map| map.clone())
    }

    /// Add a refinement with a scale update.
    pub fn refinement_scale(
        self,
        behavior: SpatialBehavior,
        weight: f64,
        scale_factor: f64,
    ) -> Self {
        self.refinement(behavior, weight, move |map| {
            let mut new_map = RateMap::new();
            for (sb, rv) in map.entries() {
                let scaled = match rv {
                    RateValue::Real(r) => RateValue::Real((r * scale_factor).min(1.0)),
                    RateValue::Complex(z) => RateValue::Complex(
                        z * num_complex::Complex64::new(scale_factor, 0.0),
                    ),
                };
                new_map.insert(sb.clone(), scaled);
            }
            new_map
        })
    }

    pub fn build(self) -> Result<BaseRule, String> {
        let lhs = self.lhs.ok_or("LHS not set")?;
        let rhs = self.rhs.ok_or("RHS not set")?;

        let mut rule = BaseRule::new(self.name, lhs, rhs, self.weight);

        if self.refinements.is_empty() {
            // Default: null => (1.0, id)
            rule = rule.add_refinement(RefinementEntry::new(
                SpatialBehavior::Null,
                1.0,
                |map| map.clone(),
            ));
        } else {
            for (behavior, weight, update) in self.refinements {
                rule = rule.add_refinement(RefinementEntry {
                    behavior,
                    weight,
                    update,
                });
            }
        }

        Ok(rule)
    }
}

/// Builder for constructing context rules.
pub struct ContextRuleBuilder {
    name: String,
    lhs: Option<TermRef>,
    rhs: Option<TermRef>,
    weight: f64,
    condition: String,
    fold: Option<FoldFn>,
}

impl ContextRuleBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        ContextRuleBuilder {
            name: name.into(),
            lhs: None,
            rhs: None,
            weight: 1.0,
            condition: String::new(),
            fold: None,
        }
    }

    pub fn lhs(mut self, id: u64, sort: &str, display: &str) -> Self {
        self.lhs = Some(TermRef::new(id, sort, display));
        self
    }

    pub fn rhs(mut self, id: u64, sort: &str, display: &str) -> Self {
        self.rhs = Some(TermRef::new(id, sort, display));
        self
    }

    pub fn weight(mut self, w: f64) -> Self {
        self.weight = w;
        self
    }

    pub fn condition(mut self, c: impl Into<String>) -> Self {
        self.condition = c.into();
        self
    }

    /// Set a custom fold function.
    pub fn fold(
        mut self,
        f: impl Fn(&[&RateMap], &RateMap) -> RateMap + Send + Sync + 'static,
    ) -> Self {
        self.fold = Some(std::sync::Arc::new(f));
        self
    }

    /// Use the merge fold (default).
    pub fn fold_merge(self) -> Self {
        self.fold(|context_maps, rule_map| {
            let mut result = rule_map.clone();
            for ctx in context_maps {
                result = result.merge(ctx);
            }
            result
        })
    }

    /// Use the product fold.
    pub fn fold_product(self) -> Self {
        self.fold(|context_maps, rule_map| {
            let mut result = rule_map.clone();
            for ctx in context_maps {
                if let Ok(composed) = result.compose(ctx) {
                    result = composed;
                }
            }
            result
        })
    }

    /// Use the replace fold (ignore context).
    pub fn fold_replace(self) -> Self {
        self.fold(|_, rule_map| rule_map.clone())
    }

    pub fn build(self) -> Result<ContextRule, String> {
        let lhs = self.lhs.ok_or("LHS not set")?;
        let rhs = self.rhs.ok_or("RHS not set")?;

        let fold = self.fold.unwrap_or_else(|| {
            // Default: merge fold
            std::sync::Arc::new(|context_maps: &[&RateMap], rule_map: &RateMap| {
                let mut result = rule_map.clone();
                for ctx in context_maps {
                    result = result.merge(ctx);
                }
                result
            })
        });

        Ok(ContextRule {
            name: self.name,
            lhs,
            rhs,
            weight: self.weight,
            condition: self.condition,
            fold,
        })
    }
}

/// Codegen reference for the proc macro implementation.
pub fn codegen_reference() -> &'static str {
    r#"
// === PROC MACRO CODE GENERATION REFERENCE ===
//
// For each rule in the `rules { ... }` block:
//
// BASE RULES:
//   Parse: <name> . <lhs> ~> <rhs> [<weight>] { <refinements> }
//   Generate:
//     BaseRule::new("<name>", lhs_ref, rhs_ref, <weight>)
//       .add_refinement(RefinementEntry::new(
//           <spatial_behavior>,
//           <refinement_weight>,
//           <update_fn>,  // closure: &RateMap -> RateMap
//       ))
//       ...
//
// CONTEXT RULES:
//   Parse: <name> . if <cond> then <lhs> ~> <rhs> [<weight>] { <fold> }
//   Generate:
//     ContextRule::new("<name>", lhs_ref, rhs_ref, <weight>, "<cond>",
//       <fold_fn>,  // closure: (&[&RateMap], &RateMap) -> RateMap
//     )
//
// The generated ascent rules should include the annotation map:
//   relation rewrite(TermId, TermId, RateMap);
// with update/fold applied during rule firing.
"#
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_rule_builder() {
        let rule = BaseRuleBuilder::new("COMM")
            .lhs(1, "Proc", "x?(y).P | x!(Q)")
            .rhs(2, "Proc", "P{Q/y}")
            .weight(0.7)
            .refinement_scale(SpatialBehavior::interaction("x", "x"), 0.5, 0.8)
            .refinement_id(SpatialBehavior::local("x"), 0.3)
            .build()
            .unwrap();

        assert_eq!(rule.name, "COMM");
        assert_eq!(rule.refinements.len(), 2);
        assert!((rule.weight - 0.7).abs() < 1e-10);
    }

    #[test]
    fn test_base_rule_default_refinement() {
        let rule = BaseRuleBuilder::new("DROP")
            .lhs(1, "Proc", "*(@P)")
            .rhs(2, "Proc", "P")
            .build()
            .unwrap();

        // Default: one refinement, null => (1.0, id)
        assert_eq!(rule.refinements.len(), 1);
        assert!((rule.refinements[0].weight - 1.0).abs() < 1e-10);
        assert!(rule.refinements[0].behavior.is_null());
    }

    #[test]
    fn test_context_rule_builder() {
        let rule = ContextRuleBuilder::new("PAR_CTXT")
            .lhs(10, "Proc", "{S, ...rest}")
            .rhs(11, "Proc", "{T, ...rest}")
            .weight(1.0)
            .condition("if S ~> T")
            .fold_merge()
            .build()
            .unwrap();

        assert_eq!(rule.name, "PAR_CTXT");
        assert_eq!(rule.condition, "if S ~> T");
    }

    #[test]
    fn test_context_default_fold() {
        let rule = ContextRuleBuilder::new("CTXT")
            .lhs(1, "P", "C[S]")
            .rhs(2, "P", "C[T]")
            .condition("if S ~> T")
            .build()
            .unwrap();

        // Default fold is merge
        let mut map1 = RateMap::new();
        map1.insert(SpatialBehavior::local("x"), RateValue::real(0.3).unwrap());
        let mut map2 = RateMap::new();
        map2.insert(SpatialBehavior::local("y"), RateValue::real(0.4).unwrap());

        let result = rule.apply_fold(&[&map2], &map1);
        assert!(result.get(&SpatialBehavior::local("x")).is_some());
        assert!(result.get(&SpatialBehavior::local("y")).is_some());
    }

    #[test]
    fn test_update_then_fold_pipeline() {
        // Build a base rule with a scaling update
        let base = BaseRuleBuilder::new("COMM")
            .lhs(1, "Proc", "x?(y).P | x!(Q)")
            .rhs(2, "Proc", "P{Q/y}")
            .weight(0.7)
            .refinement_scale(SpatialBehavior::interaction("x", "x"), 0.5, 0.5)
            .build()
            .unwrap();

        // Build a context rule with merge fold
        let ctx = ContextRuleBuilder::new("PAR")
            .lhs(10, "Proc", "{S|rest}")
            .rhs(11, "Proc", "{T|rest}")
            .condition("if S ~> T")
            .fold_merge()
            .build()
            .unwrap();

        // Initial map
        let mut map = RateMap::new();
        map.insert(
            SpatialBehavior::interaction("x", "x"),
            RateValue::real(0.8).unwrap(),
        );

        // Step 1: base rule fires, refinement 0 selected → update scales by 0.5
        let updated = base.refinements[0].apply_update(&map);
        let rate = updated
            .get(&SpatialBehavior::interaction("x", "x"))
            .unwrap()
            .probability();
        assert!((rate - 0.4).abs() < 1e-10);

        // Step 2: context rule folds with a context map
        let mut ctx_map = RateMap::new();
        ctx_map.insert(
            SpatialBehavior::local("y"),
            RateValue::real(0.6).unwrap(),
        );
        let folded = ctx.apply_fold(&[&ctx_map], &updated);

        // Result should have both entries
        assert!(folded.get(&SpatialBehavior::interaction("x", "x")).is_some());
        assert!(folded.get(&SpatialBehavior::local("y")).is_some());
    }
}
