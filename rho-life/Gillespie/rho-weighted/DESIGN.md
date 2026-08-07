---
title: rho-weighted — weighted GSLT simulator (as built)
status: v0.2.0, P0–P5 and P7 landed; P6 pending
author: claude-session
date: 2026-08-06
related-docs:
  - docs/plans/weighted-gslt-dev-plan-v2.md
  - docs/cost-accounting/transpiler.md
  - publications/weighted-gslt/weighted-gslt.pdf
---

# rho-weighted

> A **simulator** for weighted graph-structured lambda theories. Not the
> interpreter: the interpreter maintains the actual state of a running shard,
> and this answers *what would happen if*, producing graphs across a family of
> hypothetical configurations.

`cargo test --features quantum` → **81 tests**, no warnings, no dependencies.

---

## 1. What was built

| Plan phase | Status | Where |
|---|---|---|
| P0 Foundations | done | `logic/{formula,check,partition}.rs` |
| P1 The `Space` | done | `space.rs`, `redex.rs` |
| P2 Behavioural connective | done | `logic/formula.rs` (`Dia`/`Boxm`/`Nu`), `logic/check.rs` |
| P3 Propensity + exploration | done | `propensity.rs`, `explore/{ssa,exhaustive}.rs` |
| P4 Studies and graphs | done | `study.rs`, `graph.rs`, `bin/rho-sim.rs` |
| P5 Dynamic maps + RNN | done | `theory.rs` (updates), `examples.rs` |
| P6 Surface syntax | **pending** | needs `rholang-rs`; see §7 |
| P7 Quantum | done | `quantum/{mod,linalg}.rs` |

### Test inventory

| Suite | n | Establishes |
|---|---|---|
| `analytic` | 5 | statistical acceptance against closed forms |
| `laws` | 17 | conformance laws, `oslf.rs` idiom |
| `logic` | 14 | the behavioural connective and its budget |
| `rnn` | 19 | the note's §8 end to end |
| `quantum` | 14 | Lindbladian, the degeneration theorem, interference |
| `faithfulness` | 6 | differential matching, `install`, regressions |
| unit + doc | 6 | matcher, quick-start |

---

## 2. Deviation from the plan: the build environment

The plan assumed the crate would be a workspace member of `f1r3node-rust` and
would call `Matcher::get` directly (F2/DR-W9). **That workspace does not build
in a fresh clone**: the toolchain pin is `nightly-2026-02-09`, `protoc` is a
build-script requirement of `models`/`node`/`comm`, and the root `Cargo.toml`
`[patch]` points at `../rholang-rs-cost-accounting-transpiler/`, a sibling
worktree that is not part of any checkout.

So `rho-weighted` was built **dependency-free and stable-compatible**, with the
matching semantics behind the `Matching` port the plan already specified. The
consequences are contained and reversible:

* `syntax::Term` is a `Par`-shaped AST standing in for `models::rhoapi::Par`.
  Only `matching` and `space` pattern match on it.
* `matching::SimpleMatcher` stands in for `Matcher::get`.
* `faithfulness::workspace` is the adapter, gated behind the
  `workspace-matcher` feature, with the three-line body documented.

**Re-integration checklist**

1. Add `rho-weighted` to `[workspace] members`.
2. Add `models`, `rholang`, `rspace_plus_plus`, `rho-pure-eval` as path deps.
3. Replace `syntax::Term` with `models::rhoapi::Par`; `syntax` becomes a
   conversion adapter.
4. Enable `workspace-matcher`; the differential in `tests/faithfulness.rs`
   then runs against the interpreter instead of `ReferenceMatcher`.
5. Assert the §9 invariant in CI: `node` and `casper` must not depend on
   `rho-weighted`, directly or transitively.

Nothing in `rholang`, `rspace++` or `reduce.rs` changes. That is the dividend of
the rev-2 correction.

---

## 3. Architecture as built

```
syntax      Term (Par-shaped), Name = quoted process, ordered List
matching    the Matching port; SimpleMatcher; the shared-semantics seam
space       Space: immutable channel index, O(1) fork, non-destructive fire
redex       enumeration; multiplicity with persistence, peek, joins
logic       Formula (structural + behavioural), satisfaction, Budget/Checkable,
            partition checking; SpatialBehavioralLogic port + RhoLogic instance
theory      WeightedTheory, WeightMap, update functions, geometric factor, gate
propensity  a(r,φ,c) = w(φ) · Σ_k g(k)·χ
explore     ssa (Gillespie), exhaustive (reachable graph + generator), ensemble
graph       LabelledTransitionGraph, Generator, DOT/CSV/JSON
study       grids, sweeps, provenance
quantum     jump operators, Lindbladian, quantum-jump unravelling
faithfulness differential harness + the workspace adapter seam
```

### Decisions that survived contact

* **`fire` is non-destructive** and `fork` is copy-on-write (`Arc::make_mut`).
  This is what makes exhaustive graph construction possible and what lets the
  capacity factor be implemented by firing-and-checking.
* **Propensity is computed by redex, not by key.** Safe exactly because the key
  set is a partition; `law_propensity_by_redex` checks the two agree.
* **Context rules contribute `g(k)` multiplicatively, never a propensity.**
* **Rates are `ℝ≥0`.** `law_rates_are_nonnegative_reals_not_probabilities`.
* **The weight map is in the configuration.** `law_map_in_state` exhibits two
  runs at the same term with different futures.

### Decisions that changed

* **`capacity` checks the post-firing occupancy, not the current one.** The
  naive version blocks the *consumer* at exactly the point it is most needed and
  deadlocks. Cost is one `fire` per redex per propensity computation, affordable
  because `fire` is pure.
* **A hard capacity makes a token amplifier absorbing, not live.** Once every
  channel is full, every enabled redex has factor zero, so `a₀ = 0`. That is the
  correct reading — a system that cannot afford any available transition has
  stopped — but a modeller wanting a live saturating network wants a *leak* rule
  instead, trading term-finiteness for positive recurrence.
  (`capacity_bounds_a_branching_network`.)

---

## 4. Two bugs found by building, and what they cost

**Names rebuilt from key strings.** `local_term` and `Space::to_term`
reconstructed a channel's `Name` from its canonical key string. Every name
predicate then failed and every redex fell through to the synthesised `default`
class, which has weight zero — so the simulator ran, produced traces, and
reported `a₀ = 0` everywhere. A name predicate must see into a name's structure;
a key string has already thrown that away. Fixed by keeping the original `Name`
on `Datum`. Regression: `installation_preserves_name_structure`.

**Ordered tuples encoded as parallel composition.** `structured("syn",[j,i])`
built `@(Par[syn, j, i])`. Parallel composition is a multiset, so `@[syn,0,1]`
and `@[syn,1,0]` canonicalised to the *same channel*: the recurrent pair
collapsed to one node and its propensity silently doubled. An ordered tuple
needs an ordered former. Fixed by adding `Term::List` and, with it, the
structural connective OSLF would emit for that former, `Formula::ListAt`.
Regression: `ordered_structured_names_stay_distinct`.

`ListAt` turned out to be what makes the namespace-logic claim operative rather
than decorative: `comm_in_namespace_at(2, Eq(Int(i)))` is **one key for every
synapse onto neuron `i`**, recovering the endpoint from the name rather than
from traffic. That is namespace logic doing the work the note claims for it, and
it was not expressible before the bug forced the former to exist.

Both bugs share a shape — a judgment that looks right and quietly disagrees with
itself on structured data — and both are invisible in a trace. That is the
argument for §5.

---

## 5. Why acceptance is statistical

A wrong propensity produces perfectly plausible output. That is how the
multiplicity bug survived in the v1 prototype, and it is why the load-bearing
tests are numerical.

`birth_death(λ, μ, cap)` is the discriminating model. Its death propensity is
proportional to occupancy — a persistent receipt pairs with *every* pending
message — so the stationary occupancy is Poisson(λ/μ), truncated. A
multiplicity-blind propensity gives a truncated **geometric** instead.

| check | test |
|---|---|
| exhaustive generator = analytic generator (no sampling) | `two_state_generator_and_stationary_match_the_closed_form` |
| stationary = truncated Poisson to 1e-6 | `birth_death_occupancy_is_poisson_because_multiplicity_is_counted` |
| the two models are 0.5 apart in total variation, so the test is not vacuous | `multiplicity_is_load_bearing` |
| sampled occupancy converges to the exhaustive stationary | `two_state_sampling_converges_to_the_generator` |
| `τ` is exponential with parameter `a₀` | `waiting_times_are_exponential_with_parameter_a0` |

Verified output for `birth-death 3.0 1.0 10`: P(2)=P(3)=0.2241, P(4)=0.1681,
P(1)=0.1494 — Poisson(3) to four places.

---

## 6. The quantum path, and one thing the paper understates

Jump operators are read straight off the exhaustive graph: the amplitude of a
single redex is `√(rate)`, and amplitudes of distinct redexes with the same
contractum **add**. That one line is the whole difference from the classical
case, and both headline results fall out of it.

**The degeneration theorem, executably.** At `H = 0` with a diagonal jump
structure, `‖ψ̃(s)‖² = e^{-a₀s}` to 1e-6, sampled waiting times have mean `1/a₀`,
and every post-jump state is a basis state — so the unravelling *is* the SSA.
`the_ssa_is_the_quantum_jump_algorithm_at_zero_hamiltonian`.

**Interference is superlinear in multiplicity.** `m` indistinguishable pending
messages give `m` redexes and one target, so the amplitude is `m·z` and the
weight `m²|z|²` where the classical rate is `m|z|²`. Checked for `m = 2,3,4` in
`indistinguishable_reactants_interfere`. The enhancement comes entirely from the
reactants living in a bag rather than a list; tagging them would restore
linearity. Open question, not a settled one.

**What the paper understates.** The theorem's hypotheses are sufficient but not
necessary. If `Σ L†L` is a scalar multiple of the identity — every configuration
has the same total exit rate — the damping commutes with everything and the norm
decays as a pure exponential *no matter what the Hamiltonian is*. Coherent
evolution redistributes amplitude among states that all decay at the same rate,
and the norm never notices. So a nonzero Hamiltonian alone does not buy a
non-exponential sojourn: the *variation in exit rate across the states the
coherence connects* is what does.
`a_scalar_damping_stays_exponential_even_with_a_hamiltonian` pins this, and
§7 of the note should be amended to say so.

---

## 7. What is not done

**P6, surface syntax.** Needs grammar and AST changes in `rholang-rs`
(`[w]` rate annotations, `{ φ => (v, u) }` refinement blocks, `default`), a
`LANGUAGE_VERSION` bump and a `rev` bump. Nothing in P0–P5 or P7 depends on it;
theories are built programmatically today (`examples.rs`).

**The real faithfulness obligation.** DR-W9 asks for a differential against the
*interpreter*. What runs today is a differential against an independently
written `ReferenceMatcher` over a 360-case corpus, plus a negative control
proving the corpus can catch a broken matcher. That is weaker, and
`the_real_obligation_is_still_open` exists so the gap shows up in the suite
output rather than only here.

**OSLF-generated logic.** `RhoLogic` is hand-written, behind the
`SpatialBehavioralLogic` port. Swapping in a generated instance is
interface-level, per DR-W4.

**Performance.** `Sep` enumerates all `2^n` splits of a term's components and
refuses above 20; `capacity` fires per redex per step. Both are fine at the
scales tested and neither has been profiled.

---

## 8. Verification

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings      # not run here: clippy unavailable
cargo test
cargo test --features quantum
cargo run --bin rho-sim -- birth-death out     # emits .dot, .csv, .json
```

`rho-sim <two-state|birth-death|rnn> [outdir]` writes the transition graph as
DOT, the generator as CSV, and the study as JSON with provenance — theory
fingerprint, seed, budgets, crate version, and whether the budget was exhausted.
A truncated exploration says so; `a_truncated_study_says_so` checks it.

---

## 9. Scientific ledger

| Date | Step | Hypothesis | Result |
|---|---|---|---|
| 2026-08-06 | Rev 1 review | simulator as an interpreter scheduling policy | **rejected** — separate execution path; rev 2 gives it its own state |
| 2026-08-06 | Build environment | the crate can be built against the workspace | **refuted** — toolchain pin, missing protoc, dangling `[patch]`; built standalone behind the `Matching` port instead |
| 2026-08-06 | Redex enumeration | multiplicity must be counted or the chain is wrong | **confirmed** — Poisson vs geometric, TV distance 0.5+ |
| 2026-08-06 | Structured names | a reflective index can ride on parallel composition | **refuted** — `Par` is a multiset; needs an ordered former, which then supplies the `ListAt` connective namespace keys want |
| 2026-08-06 | Capacity as a geometric factor | current-occupancy check suffices | **refuted** — deadlocks the consumer; must check post-firing |
| 2026-08-06 | Degeneration theorem | SSA = quantum jump at `H=0`, diagonal | **confirmed** numerically to 1e-6 |
| 2026-08-06 | Theorem sharpness | `H ≠ 0` implies non-exponential sojourn | **refuted** — scalar damping stays exponential; the note should say *unequal exit rates* |
| | P6 grammar spike | weight-block tokens lex without conflict | *pending* |
| | Workspace differential | `SimpleMatcher` agrees with `Matcher::get` | *pending* |
