//! # Example: Stochastic Rho-Calculus with Update/Fold Semantics
//!
//! Demonstrates the corrected rule structure:
//! - Base rules with weighted refinements and update functions
//! - Context rules with fold functions
//! - The full pipeline: select rule → select refinement → update map → fold with context

use mettail_gillespie::augmented_rule::*;
use mettail_gillespie::gillespie;
use mettail_gillespie::language_ext::*;
use mettail_gillespie::rate_map::RateMap;
use mettail_gillespie::rate_value::RateValue;
use mettail_gillespie::spatial_behavior::SpatialBehavior;

fn main() {
    println!("═══════════════════════════════════════════════════════════════");
    println!("  MeTTaIL Gillespie — Update/Fold Semantics Demo");
    println!("═══════════════════════════════════════════════════════════════\n");

    // ── Build the rewrite system ──────────────────────────────────────

    // COMM rule: weight 0.7, two refinements with different update functions
    let comm = BaseRuleBuilder::new("COMM")
        .lhs(1, "Proc", "{ x?(y).P | x!(Q) | rest }")
        .rhs(2, "Proc", "{ P{Q/y} | rest }")
        .weight(0.7)
        // Refinement 1: communication on channel x, scales rates by 0.8
        .refinement_scale(SpatialBehavior::interaction("x", "x"), 0.5, 0.8)
        // Refinement 2: local activity, identity update
        .refinement_id(SpatialBehavior::local("x"), 0.3)
        .build()
        .unwrap();

    // DROP rule: weight 1.0, default refinement (null => (1.0, id))
    let drop_rule = BaseRuleBuilder::new("DROP")
        .lhs(3, "Proc", "*(@(P))")
        .rhs(4, "Proc", "P")
        .weight(1.0)
        .build()
        .unwrap();

    // PAR context rule: weight 1.0, merge fold
    let par_ctxt = ContextRuleBuilder::new("PAR_CTXT")
        .lhs(10, "Proc", "{ S | rest }")
        .rhs(11, "Proc", "{ T | rest }")
        .weight(1.0)
        .condition("if S ~> T")
        .fold_merge()
        .build()
        .unwrap();

    let system = RewriteSystem::new()
        .add_base_rule(comm)
        .add_base_rule(drop_rule)
        .add_context_rule(par_ctxt);

    println!("Rules:");
    for rule in &system.rules {
        println!("  {}", rule);
    }

    // ── Initial state ─────────────────────────────────────────────────

    let initial_term = TermRef::new(1, "Proc", "{ x?(y).P | x!(Q) | rest }");
    let mut initial_map = RateMap::new();
    initial_map.insert(
        SpatialBehavior::interaction("x", "x"),
        RateValue::real(0.8).unwrap(),
    );
    initial_map.insert(
        SpatialBehavior::local("x"),
        RateValue::real(0.5).unwrap(),
    );

    println!("\nInitial term: {}", initial_term);
    println!("Initial map:  {}", initial_map);

    // ── Run classical simulation ──────────────────────────────────────

    println!("\n━━━ Classical Simulation (10 steps) ━━━\n");

    let mut sim = gillespie::Simulator::new(
        initial_term.clone(),
        initial_map.clone(),
        system.clone(),
    );

    // Add a context map (from the "rest" sub-terms)
    let mut rest_map = RateMap::new();
    rest_map.insert(
        SpatialBehavior::local("y"),
        RateValue::real(0.4).unwrap(),
    );
    sim.context_maps = vec![rest_map];

    let trace = sim.run(10);
    gillespie::print_trace(&trace);

    println!("\nFinal time: {:.6}", sim.time);
    println!("Final map:  {}", sim.current_map);

    // ── Demonstrate update function effect ─────────────────────────────

    println!("\n━━━ Update Function Demo ━━━\n");

    let comm_rule = &system.base_rules_for_sort("Proc")[0];
    println!("Rule: {}", comm_rule.name);

    let mut demo_map = RateMap::new();
    demo_map.insert(
        SpatialBehavior::interaction("x", "x"),
        RateValue::real(1.0).unwrap(),
    );
    demo_map.insert(
        SpatialBehavior::local("x"),
        RateValue::real(0.6).unwrap(),
    );

    println!("Before update: {}", demo_map);

    // Apply refinement 0 (scales by 0.8)
    let updated = comm_rule.refinements[0].apply_update(&demo_map);
    println!("After refinement[0] (scale 0.8): {}", updated);

    // Apply refinement 1 (identity)
    let updated2 = comm_rule.refinements[1].apply_update(&demo_map);
    println!("After refinement[1] (identity):  {}", updated2);

    // ── Demonstrate fold function effect ───────────────────────────────

    println!("\n━━━ Fold Function Demo ━━━\n");

    let ctx_rule = &system.context_rules_for_sort("Proc")[0];

    let mut rule_map = RateMap::new();
    rule_map.insert(
        SpatialBehavior::local("x"),
        RateValue::real(0.3).unwrap(),
    );

    let mut ctx_map1 = RateMap::new();
    ctx_map1.insert(
        SpatialBehavior::local("y"),
        RateValue::real(0.5).unwrap(),
    );

    let mut ctx_map2 = RateMap::new();
    ctx_map2.insert(
        SpatialBehavior::local("z"),
        RateValue::real(0.2).unwrap(),
    );

    println!("Rule map:    {}", rule_map);
    println!("Context[0]:  {}", ctx_map1);
    println!("Context[1]:  {}", ctx_map2);

    let folded = ctx_rule.apply_fold(&[&ctx_map1, &ctx_map2], &rule_map);
    println!("After fold:  {}", folded);
}
