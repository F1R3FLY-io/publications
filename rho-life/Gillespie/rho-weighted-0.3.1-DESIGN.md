---
title: rho-weighted — weighted GSLT simulator (as built)
version: 0.3.1
status: implements the note v2 except §9.1 surface syntax; logic hand-written behind its port
date: 2026-08-07
related-docs:
  - weighted-gslt-v2.pdf (the specification)
  - CHANGELOG.md (what changed, and what was verified)
  - docs/plans/weighted-gslt-dev-plan-v2.md (the plan this was built from)
---

# rho-weighted

> A **simulator** for weighted graph-structured lambda theories. Not the
> interpreter: the interpreter maintains the state of a running shard, and this
> answers *what would happen if*, across a family of hypothetical
> configurations, producing graphs rather than a history.

`cargo test --features quantum` → **91 tests**, no warnings, **no
dependencies**, stable Rust 1.75+.

---

## 1. Correspondence with the note

| Note | Implementation |
|---|---|
| Def. 1, base and context rules | `theory::{BaseRule, WeightedTheory}`; context rules carry `g`, never a propensity |
| Def. 2, redex as a *selection* | `redex::enumerate` — counts selections from the bag, not `≡`-classes |
| Def. 3–6, keys and admissibility | `logic::{Formula, Budget, Checkable}`; (P1)/(P2) in `logic::partition`, (R1)–(R3) in `Checkable` |
| Prop. 7, classification | `Partition::classify`, total by construction |
| Prop. 9 / Obs. 10, the dilemma | we take the **external** branch; §4 below |
| Lemma 11, locality | documented; the incremental scheme it licenses is not built (§6) |
| Def. 14–19, weighted rewriting | `theory::{WeightMap, Configuration}`, `UpdateFn` at the extended signature of Def. 60 |
| Def. 21, propensity | `propensity::propensities`, organised by redex per Prop. 23 |
| Def. 25, the SSA | `explore::ssa::Sim`, provenance recorded per run |
| Thm. 26 + Remark 27 | `explore::exhaustive` and `graph::Generator`; diagonal is `-Σ_{j≠i}` |
| Thm. 29, summation-free SPiM | `examples::channel_keyed` is the instantiation |
| Def. 32–34, finiteness | `theory::capacity` (term), saturating updates (map) |
| Def. 37–41, the quantum path | `quantum::{QctmcModel, Unravelling}` |
| Prop. 44, projectors | `QctmcModel::populations`, basis-diagonal by construction |
| §8, the spiking network | `examples::rnn`, `tests/rnn.rs` |
| Def. 62, the funding gate | `theory::Gate`, `χ` in the propensity |

Not implemented: §9.1 surface syntax (§6 below).

---

## 2. Layout

```
Cargo.toml         no dependencies; features quantum, workspace-matcher
README.md          build/test, the simulator/interpreter distinction, reading order
CHANGELOG.md       v0.3.0 corrections, v0.3.1 verification
DESIGN.md          this file
justfile           just | just verify | just sim MODEL | just lint | just doc
.github/workflows  fmt, build, clippy, both suites, empty-dependency assertion

src/
  syntax.rs        Term (Par-shaped), Name = quoted process, ordered List
  matching.rs      the Matching port; SimpleMatcher — the shared-semantics seam
  space.rs         immutable channel index, O(1) fork, non-destructive fire
  redex.rs         enumeration; multiplicity with persistence, peek, joins
  logic/           formula, check (satisfaction), partition; the OSLF port
  theory.rs        weight maps, updates, geometric factor, funding gate
  propensity.rs    a(r,φ,c) = w(φ) · Σₖ g(k)·χ
  explore/         ssa, exhaustive, ensemble
  graph.rs         LabelledTransitionGraph, Generator, DOT/CSV/JSON
  study.rs         grids, sweeps, provenance
  quantum/         jump operators, Lindbladian, unravelling, linalg
  faithfulness.rs  differential harness + the workspace adapter seam
  rng.rs           vendored SplitMix64
  bin/rho-sim.rs   emit graphs for a model

tests/             analytic · laws · logic · quantum · rnn · faithfulness
examples/          verify-claims.rs — prints the numbers behind the claims
```

Module paths are stable across v0.2 → v0.3.1. The note's §9.2 cites this API.

---

## 3. Why there are no dependencies

Three reasons, in descending order of importance.

A study's numbers must not move when a dependency bumps its algorithm. The PRNG
is vendored (`rng.rs`, SplitMix64) and so is the complex/matrix arithmetic
(`quantum/linalg.rs`) for exactly that reason: a result reproducible from
*(theory fingerprint, seed, budget)* stops being reproducible if the generator
underneath it changes.

The crate must never be able to reach the consensus path. An empty dependency
set is the cheapest possible guarantee, and CI asserts it.

And it builds anywhere a Rust toolchain exists — no `protoc`, no pinned
nightly, no network. That matters because the workspace this is destined for
does not currently build from a clean checkout (§6).

---

## 4. Decisions worth knowing

**The external branch of the partition dilemma.** Note §3.3 poses a genuine
choice: state (P1)/(P2) inside the generated logic and demand Boolean structure
of the target specification, or discharge them in the metatheory and forfeit
specification closure. We take the second and pay the stated price. The
partition check evaluates the conditions against a supplied set of witnesses, so
it is a **sound refuter and not a proof** — an overlap it reports is real, and
silence is not admissibility. Observation 10 says why that is a price and not a
defect.

**(R3) is enforced at elaboration, by construction.** Keys must be structural:
no `⟨K_j⟩`, no `[K_j]`, no `ν`. A rate that depends on what a redex can do next
cannot be recomputed locally, so Lemma 11 fails for modal keys and the
incremental scheme built on it would be unsound. `Partition::from_checked_keys`
is the only constructor and applies the check; `Checkable::try_new` still admits
modal formulae, because the restriction is on *keys* and not on *properties*,
and §8.9 uses `ν` for exactly that.

**Rates are `ℝ≥0`; amplitudes are unbounded.** `λ(z) = |z|²` is the
interpretation map, and it is the only relation between the two codomains
(Remark 12). The `|z| ≤ 1` bound of the first version is gone: on a
finite-dimensional space every operator is bounded outright, GKSL imposes no
norm condition, and the bound manufactured a dimensional inconsistency.

**Context rules contribute `g(k)` multiplicatively, never a propensity.**
A context rule is addressing, not an event; giving it an independent draw turns
one event into two.

**Two conventions on self-transitions, deliberately not mixed.** `Q` drops
self-loops and its diagonal is `-Σ_{j≠i} Q(i,j)` (Remark 27); the sampler keeps
them in `a₀` and fires them. A self-loop is a fictitious jump in the sense of
uniformisation, so the two agree on the distribution of the state at every time
and differ only on event counts and sojourns. `graph.rs` documents which is
which, and `law_self_transitions_are_fictitious_jumps` pins both halves.

**`capacity` checks post-firing occupancy.** The obvious reading — refuse a send
when the target is already full — blocks the *consumer* and deadlocks. Cost is
one `fire` per redex per propensity computation, affordable because `fire` is
pure. Relatedly, a hard capacity makes a token amplifier **absorbing**, not
live: once every channel is full every enabled redex has factor zero, so
`a₀ = 0`. That is a stop and not a deadlock; a network meant to saturate and
keep running wants the leak rule of §8.6 instead, which trades term-finiteness
for positive recurrence and thereby gives up §7.

---

## 5. Why acceptance is statistical

A wrong propensity produces perfectly plausible output. That is how the
multiplicity bug survived in the superseded prototype: nothing an eye could
catch distinguishes a chain running at the wrong rate from one running at the
right rate. So the load-bearing tests are numerical.

`birth_death(λ, μ, cap)` is the discriminating model. Its death propensity is
proportional to occupancy — a persistent receipt pairs with *every* pending
message — so the stationary occupancy is truncated Poisson(λ/μ). A
multiplicity-blind propensity gives a truncated **geometric** instead.

| Check | Test |
|---|---|
| exhaustive generator = analytic generator (no sampling) | `two_state_generator_and_stationary_match_the_closed_form` |
| stationary = truncated Poisson to 1e-6 | `birth_death_occupancy_is_poisson_because_multiplicity_is_counted` |
| the two laws are 0.5+ apart in total variation, so the test is not vacuous | `multiplicity_is_load_bearing` |
| sampled occupancy converges to the exhaustive stationary | `two_state_sampling_converges_to_the_generator` |
| `τ` is exponential with parameter `a₀` | `waiting_times_are_exponential_with_parameter_a0` |
| conservativity at `H = 0` | `populations_relax_to_the_classical_stationary_distribution` |
| degeneration without a diagonality hypothesis | `the_degeneration_holds_without_diagonality` |

Measured, at λ/μ = 3: P(0)=0.0498016, P(1)=0.1494049, P(2)=P(3)=0.2241073,
P(4)=0.1680805 — truncated Poisson(3) to seven places.

`examples/verify-claims.rs` prints these and the quantum figures.

### One gap in the suite, closed by the example

The conservativity test runs on the two-state chain, which is diagonal, where
the criterion cannot discriminate between root-of-sum and sum-of-roots. The
verification example checks it on a fixture with several derivations into one
target — the case the normalisation exists to settle — and the residual is
`5.6e-17`. Promoting that to a test would be a small and worthwhile addition.

---

## 6. What is not done

**Surface syntax (§9.1).** A design, not a grammar. Needs parser and tree-sitter
changes in `rholang-rs`, on a separate release cadence. Theories are built
programmatically; nothing else depends on it landing.

**The logic is hand-written.** The connectives of §2.3 are implemented by hand
behind a port whose interface an OSLF-generated instance would implement, so the
swap is at the boundary rather than through the simulator. Until it lands, the
genericity over GSLTs claimed in §1 is **argued and not demonstrated**, and that
is the largest gap in the note.

**Faithfulness against the interpreter.** The simulator and the interpreter
share semantics but not code paths, so their agreement is a permanent
obligation, not a theorem. What runs today is a differential against an
independently written reference matcher over a corpus of several hundred cases,
plus a negative control establishing that the corpus can detect a broken
matcher. The differential against `Matcher::get` is written, is gated behind the
`workspace-matcher` feature, and **has not been run**, because
`f1r3node-rust` does not build from a clean checkout: the toolchain pin is
`nightly-2026-02-09`, `protoc` is a build-script requirement of
`models`/`node`/`comm`, and the root `[patch]` points at
`../rholang-rs-cost-accounting-transpiler/`, a sibling worktree not part of any
checkout. `the_real_obligation_is_still_open` keeps the gap in the test output
rather than only here.

**Incremental propensity (§9.3).** Lemma 11 licenses it and (R3) now makes it
sound, but the Fenwick maintenance is not built; propensity is recomputed per
step.

**Performance.** `Sep` enumerates all `2ⁿ` splits of a term's components and
refuses above 20; `capacity` fires per redex per step. Fine at the scales
tested, unprofiled beyond them.

### Re-integration checklist

1. Add `rho-weighted` to `[workspace] members`.
2. Add `models`, `rholang`, `rspace_plus_plus`, `rho-pure-eval` as path deps.
3. Replace `syntax::Term` with `models::rhoapi::Par`; `syntax` becomes a
   conversion adapter. Only `matching` and `space` pattern match on `Term`.
4. Enable `workspace-matcher`; `tests/faithfulness.rs` then differentials
   against the interpreter instead of `ReferenceMatcher`.
5. Keep the CI dependency assertion, retargeted at `node` and `casper`.

Nothing in `rholang`, `rspace++` or `reduce.rs` changes.

---

## 7. Verification

```bash
cargo build --all-targets --features quantum
cargo test                                        # ~4 s
cargo test --features quantum                     # ~8 s
cargo run --release --features quantum --example verify-claims
cargo run --release --bin rho-sim -- birth-death out
```

`cargo fmt --check` and `cargo clippy -- -D warnings` are in `justfile` and CI;
neither tool was available in the environment this revision was verified in, so
both are **unrun**. Compile and test results above are real.

---

## 8. Scientific ledger

| Date | Step | Hypothesis | Result |
|---|---|---|---|
| 2026-08-06 | Rev 1 plan review | simulator as an interpreter scheduling policy | **rejected** — separate execution path; it gets its own state |
| 2026-08-06 | Build environment | the crate can be built against the workspace | **refuted** — toolchain pin, missing protoc, dangling `[patch]`; built standalone behind the `Matching` port |
| 2026-08-06 | Redex enumeration | multiplicity must be counted or the chain is wrong | **confirmed** — Poisson vs geometric, TV distance 0.5+ |
| 2026-08-06 | Structured names | a reflective index can ride on parallel composition | **refuted** — `Par` is a multiset; needs an ordered former, which then supplies the `ListAt` connective namespace keys want |
| 2026-08-06 | Capacity as a geometric factor | a current-occupancy check suffices | **refuted** — deadlocks the consumer; must check post-firing |
| 2026-08-06 | Theorem sharpness | `H ≠ 0` implies a non-exponential sojourn | **refuted** — scalar damping stays exponential; the note now says *unequal exit rates* (Remark 43) |
| 2026-08-07 | v0.3.0 normalisation | sum-of-roots gave `m²λ`; root-of-sum gives `mλ` | **confirmed** — exact for m = 2, 3, 5, 8 |
| 2026-08-07 | v0.3.0 conservativity | holds on a *non-diagonal* fixture, where it can fail | **confirmed** — residual `5.6e-17` |
| 2026-08-07 | v0.3.0 (R3) enforcement | "a modal key cannot reach a theory by any route" | **refuted** — `Partition`'s public fields bypassed it; fields privatised in v0.3.1 and the claim now holds |
| | P6 grammar spike | weight-block tokens lex without conflict | *pending* |
| | Workspace differential | `SimpleMatcher` agrees with `Matcher::get` | *pending* |
