# Draft 7 — the mind-first restructure

536 pp. Clean compile: 0 errors, 0 undefined references, 0 multiply-defined labels.
(Draft 6 was 455 pp.)

## What changed

The Turn was reordered so the ecology-as-mind cognitive architecture is
developed **after the machinery and before everything else**. New part order:

| | Part | pp |
|---|---|---|
| I | The Machinery: From GSLTs to Cost-Accounted Theories | 71–140 |
| II | Ecology as Cognitive Architecture | 141–302 |
| III | Situating the Learner: A Variety of Possible Physics | 303–352 |
| IV | Why Scientists Get Misled | 353–384 |
| V | Entropy, Choice, and the Break from Turing | 385–450 |
| VI | Origins of Life | 451–482 |

The learner is **situated before it is shown to be misled**: build the
knower, describe the world it is in, then ask how much of that world a
population of knowers can find. Prose touched by that ordering: the
sciencebot part frontispiece (it no longer opens by referring back to the
ecology part), the three forward flags at the end of the ecology preface,
the Part III/IV summaries and the dependency diagram in the Overarching
Introduction, the recap order in `turn_conclusion_ch01`, and the
narrative listing in `turn_origins_ch07`.

Forward cross-part references after the reorder: 6 from Part I into the
ecology chapters and 4 into the physics chapters (the machinery pointing
ahead to where its constructions get used, which is intended), plus two
into the Prestige. No part references a part that follows it otherwise.

The Pledge, the four interludes, the Conclusion and the Prestige are
unmoved. The Pledge is untouched entirely.

## Build order

Overlay onto the draft2→draft6 tree as usual. **Delete
`TheTurn/turn_causality_ch02.tex`** — it is superseded by
`turn_gslt_ch02.tex`.

## New files (12)

| File | Chapter | Source |
|---|---|---|
| `turn_gslt_ch01` | What a Language Presentation Is, and Why It Comes First | new |
| `turn_gslt_ch02` | Graph-Structured Lambda Theories | ported, `GSLT-intro/omnibus` §2 |
| `turn_gslt_ch03` | Interactive GSLTs, and the Site of Interaction | ported, omnibus §3 |
| `turn_gslt_ch04` | Continued Interactive GSLTs | ported, omnibus §4 |
| `turn_gslt_ch05` | The Category of GSLTs | ported, omnibus §5 |
| `turn_gslt_ch06` | The Cost Monad | ported, omnibus §6 |
| `turn_gslt_ch07` | The History Monad | ported, omnibus §7 |
| `turn_gslt_ch08` | Cost-Accounted Rho | distilled, `cost-accounting` + `cost-accounting-as-monad` |
| `turn_gslt_ch09` | Conversion, Engines, and the Virtual Token | distilled, `virtual-tokens` |
| `turn_gslt_ch10` | Generating Type Systems: OSLF and the Hypercube | ported, omnibus §8 |
| `turn_entropy_ch01` | Entropy, and Why Determinism Is Not Enough | new; draws on `history-and-cost/two-erasures` |
| `turn_origins_ch08` | Assembly Index and the Depth of the Fractal Learner | new |

Omnibus §9 (Rholang 1.4) was **not** ported — it is implementation
material and sits outside the book's argument. The `where` clause it
documents is covered in `turn_gslt_ch08` instead.

### Notation

The omnibus writes a theory as `G = (Σ, E, R)` and reserves boldface for
the categories; the book writes `\GSLT = (\terms, \eqs, \rules)`. Both
spellings are kept and reconciled in a new remark
(`ob:rmk:notation-bridge`, in `turn_gslt_ch02`). New preamble macros:
`\catGSLT`, `\catiGSLT`, `\catciGSLT`, `\Lcat`, `\Hist`, `\hypK`,
`\SigCat`, `\Nm`, `\Stk`, `\Surf`, `\Tw`, `\rsq`, `\compute`, `\near`,
`\cf`, `\angles`, `\fv`, `\Zc`, `\ctxbar`, `\AI`, `\CN`. Added packages:
`mathpartir`. Added environment: `nonexample`. 13 bibliography entries
imported; 7 omnibus citation keys remapped onto entries the book already
had.

## Rewritten

- **Overarching Introduction** (in `finding_mind.tex`). Six new part
  summaries, new dependency diagram, new statement of why the order was
  reversed. This retires the last Lagrangian/Hamiltonian language and the
  last "ordinal-valued consciousness" framing in the book.
- **`turn_causality_ch01`** → *The Learner's World Already Has a Physics*.
  Now the Part IV opener. Carries the ladder-of-subcategories table and
  the Rosetta Stone, plus `rmk:two-ladders` distinguishing this ladder
  (down, adding axioms) from the hypercomputational one (up, adding
  power).
- **`turn_conclusion_ch01`** — six parts instead of four.
- **Ecology preface** ("Why the Unit Keeps Moving") — no longer claims the
  preceding parts helped themselves to an agent, since only the machinery
  precedes it now. Gains three forward flags to Parts III, IV and VI.
- **Organization sections** in `turn_sciencebot_ch01`, `turn_origins_ch01`,
  `turn_hypercomp_ch01`.

## Edited

- **`turn_causality_ch07`** left whole and in Part IV, per instruction.
  Gains `rmk:vtok-twice` — *The same theorem, twice, on purpose* — which
  carries the intuition rather than merely pointing: energy is the shadow
  the conversion structure casts when it has no free lunches, and the size
  of the free lunches is how badly the shadow fails to be defined.
- **`turn_sciencebot_ch02` / `ch03`** kept, per instruction, with new
  openings saying why the repetition is deliberate: the population
  argument works *even for ideal agents*, and removing the budget is what
  makes that visible. `ch03` gains `rmk:two-agents`.
- **`turn_sciencebot_ch04`** → *Behavioral Primes*. Its §1 (Interactive
  GSLTs) was promoted into `turn_gslt_ch03`; the chapter now opens by
  pointing back there.
- **`turn_causality_ch05a`**, **`prestige_ch01`** — references repointed
  from `sec:gslt`/`sec:combinators` to `ch:gslt`/`ch:igslt`.
- **`turn_hypercomp_ch07`/`ch08`** — labels disambiguated to
  `sec:hyper-gaps` / `sec:hyper-discussion`; `sec:gaps` and
  `sec:discussion` were each defined twice in draft 6.
- **`turn_conclusion_ch03`**, **`turn_origins_ch07`** — stale "four parts"
  and hard-coded Part~I/II/III/IV references replaced with `\ref`s.

## The two new results in `turn_origins_ch08`

1. **`prop:tower-ai`** — tower height ≤ assembly index. Each level of the
   medium tower needs a medium constructed, and by no-self-code that
   construction is an edge of the open synchronization tree.
2. **`thm:ai-bound`** — an ecology-as-mind of assembly index *n* cannot
   distinguish environments that are *n*-step bisimilar. Via
   `prop:nesting-bound` (affordable nesting ≤ tower height) and the
   Hennessy–Milner depth characterisation.
3. **`cor:ai-choice`** — aperture classes ≤ AI − 1, which discharges the
   obligation left by `turn_entropy_ch01`.

`rmk:power-caveat` states plainly that this is **not** a Turing-degree
bound: rho is Turing complete at every assembly index. The bound is on
resolution, not reach.

## Open, and flagged in-text

- `prop:tower-ai` does not show that the joining steps for distinct levels
  are *distinct edges of a single minimum-length path*. Believed to follow
  from no-self-code; not written.
- `prop:nesting-bound` identifies namespace nesting depth with modal
  nesting depth. That identification is exactly what the OSLF manufacture
  should settle, and OSLF is still described rather than exhibited.
- `AI(E)` presumes a solution to the individuation fixed point of
  `ch:eng`. Different solutions give different indices.
- `conj:dissipation-choice` (entropy chapter) is one-directional at best:
  the Weihrauch lattice is not linearly ordered and θ is a single real.
