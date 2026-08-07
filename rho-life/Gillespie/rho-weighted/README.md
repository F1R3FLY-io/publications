# rho-weighted

A **simulator** for weighted graph-structured lambda theories, implementing
*Weighted Graph-Structured Lambda Theories* (v2, August 2026).

A rewrite rule's left-hand side is a type. Refine it with formulae of the
generated spatial–behavioural logic, attach a weight to each refinement, and the
enabled redexes of a configuration become a distribution. With weights in `ℝ≥0`
that gives a Gillespie simulator; with weights in `ℂ`, read as amplitudes, a
quantum continuous-time Markov chain of which the Gillespie algorithm is the
zero-Hamiltonian unravelling.

Two things distinguish this from the stochastic π-calculus literature. **Keys
are formulae, not channel names** — channel identity is one refinement among
many, and a key may inspect a name's own structure, so one key can cover a
growing population of channels. And **weight maps are state, not declaration**,
so a rewrite may update them; synaptic plasticity is then an ordinary transition
rather than an outer loop.

## Not the interpreter

The interpreter maintains the actual state of a running shard: one history,
authoritative, consensus-bound. This crate answers *what would happen if*,
across a family of hypothetical configurations, and its product is **graphs** —
labelled transition systems, generators, trajectory ensembles, parameter
sweeps.

|  | Interpreter | Simulator |
|---|---|---|
| Question | what *is* the state | what *would* happen |
| History | one, authoritative | many, hypothetical, branching |
| Nondeterminism | resolved by the tuple space, unobserved | enumerated, weighted, sampled *or* exhausted |
| Budget | phlogiston | steps, states, inference tokens |
| Output | new state | graphs |

They share exactly one thing, and it is the important one: **the semantics of a
single step**. See `src/matching.rs` for that seam and `tests/faithfulness.rs`
for the obligation it creates.

This crate must never become a dependency of `node` or `casper`.

## Build and test

Requires a Rust toolchain, 1.75 or newer. Nothing else: **no dependencies, no
`protoc`, no pinned nightly, no network access at build time.**

```bash
cargo build --all-targets
cargo test                      # classical suites,  ~4 s
cargo test --features quantum   # adds the complex codomain, ~8 s
```

With [`just`](https://github.com/casey/just):

```bash
just            # fmt-check + build + both test suites
just verify     # print the numbers behind the note's quantitative claims
just sim rnn    # emit graphs for a model
just doc
```

### Emitting graphs

```bash
cargo run --release --bin rho-sim -- birth-death out
```

writes `out/birth-death.dot` (the labelled transition system),
`out/birth-death-generator.csv` (the CTMC generator `Q`), and
`out/birth-death.json` (the study with its provenance: theory fingerprint, seed,
budgets, crate version, and whether the budget was exhausted). Models:
`two-state`, `birth-death`, `rnn`.

### Seeing the claims

`cargo run --release --features quantum --example verify-claims` prints the
measured numbers rather than a green tick — the amplitude normalisation, the
conservativity residual on a non-diagonal fixture, the configuration-keyed
basis, the self-transition convention, and the birth–death stationary law
against truncated Poisson.

## A five-minute tour

```rust
use rho_weighted::examples::two_state;
use rho_weighted::matching::SimpleMatcher;
use rho_weighted::space::Space;
use rho_weighted::study::Study;
use rho_weighted::theory::Configuration;

let (theory, term) = two_state(2.0, 1.0);
let cfg = Configuration::new(Space::install(&term), theory.initial_weights());
let result = Study::new(&theory, &SimpleMatcher, cfg).run();
assert!(!result.any_truncated());
```

Reading order for the source:

| Module | Note § | What it is |
|---|---|---|
| `syntax`, `matching` | 2.1 | the object language, and the shared-semantics seam |
| `space`, `redex` | 2.2, 5 | the simulator's own state; enumeration and multiplicity |
| `logic` | 2.3, 3 | formulae, satisfaction, budgets, the partition discipline |
| `theory` | 4 | weight maps, update functions, geometric factor, funding gate |
| `propensity` | 5 | `a(r,φ,c) = w(φ) · Σₖ g(k)·χ` |
| `explore` | 6 | Gillespie sampling and exhaustive graph construction |
| `graph`, `study` | 9 | generators, sweeps, provenance, export |
| `quantum` | 7 | jump operators, the Lindbladian, the unravelling |
| `examples` | 8 | the spiking network, and the analytic fixtures |

## Status

Everything the note specifies is implemented except the surface syntax of §9.1,
which needs grammar changes in a separate repository; theories are built
programmatically. The generated logic is hand-written behind the port an
OSLF-generated instance would implement, so the genericity claimed in §1 is
argued and not yet demonstrated. See `DESIGN.md` §6 for the full list, and
`CHANGELOG.md` for what changed in v0.3.

91 tests. `DESIGN.md` §5 explains why acceptance is statistical rather than
observational.
