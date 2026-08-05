# Draft 9 — folding in the scope note

570 pp. Clean compile: 0 errors, 0 undefined references, 0 undefined
citations, 0 multiply-defined labels, 4 overfull boxes — **all four
pre-existing**, verified against a pristine draft-8 rebuild (536 pp,
same 4).

Source: the fifth rho-life note, *Scope and Manufacture*.

## Build order

Overlay onto the draft2→draft8 tree as usual. Still delete
`TheTurn/turn_causality_ch02.tex`. Baseline was verified at exactly
536 pp before any edit.

## The split

Per instruction, the note went in as **two** chapters rather than one: the
scope machinery was promoted into Part I, where the rest of the book can
use it, and the counting-and-manufacture argument stayed in Part VI where
it is motivated.

| File | Chapter | Where |
|---|---|---|
| `turn_scope_ch01` | 16, *Scopes, and the Fixed Points that Generate Them* | Part I, p. 135; after `turn_causality_ch05a`, before `turn_gslt_ch10` |
| `turn_origins_ch09` | 59, *Scope and Manufacture* | Part VI, p. 489; after `turn_origins_ch08`, before Open Gaps |

Chapter 16 carries §3 of the note plus the definition of generator length:
scope as a predicate rather than a list, unique decomposition under
disjointness, generated scopes, decidability by descent, and the μ-not-ν
argument. Chapter 59 carries §§4–7 and the rholang: graded copy number,
the six floors, generator versus unfolding, the frontier, and the
four-skill worked composite.

Placing the machinery in Part I means it precedes everything that uses
namespaces, which is most of the book. It also means Chapter 16 forward-
references Part VI in three places; that direction is already established
in Part I (six existing forward references into the ecology chapters) and
is intended.

The note's abstract was dropped and replaced by a bridge on each chapter,
per the convention from the first rho-life integration. §8 ("what is shown
and not") was compressed into the chapters' own closings; §9 (open
problems) went to `turn_origins_ch06` rather than staying a chapter
section, so the Origins gaps live in one place. Appendix A (rholang)
became §59.5. Appendix B was dropped; its numbers already live in the
tables.

## Cross-reference re-pointing

Every one of the note's ~30 numbered citations into the companion notes
resolved **by name to a `\ref`**, never by transcribed number — the
failure mode from the draft-2 integration, where the game note's numbers
were off by one against a later draft. The mapping used:

| note cited | book label |
|---|---|
| `[scientist, Prop. 12]` | `sci:prop:confinement` |
| `[scientist, Def. 14]` | `sci:def:move` |
| `[scientist, §6.5]` | `sci:S6.5` |
| `[scientist, §11]` (the germ) | `sci:S12.2` — note §11 is book §12.2 |
| `[scientist, §4]` | `sci:S4` |
| `[compose, §5]` | `com:thm:factor` |
| `[compose, §9]` | `com:prop:root` |
| `[compose, §6]` | `com:S5` |
| `[compose, §12]` | `com:sec:worked` |
| `[engine, Def. 5.11]` | `eng:def:indiv` |
| `[engine, Prop. 5.12/5.14]` | `eng:prop:closuredeath` |
| `[engine, Prop. 5.18]` | `eng:prop:fragile` |
| `[engine, Thm. 6.3]` | `eng:thm:foster` |
| `[engine, Cor. 7.3]` | `eng:bridges-fail-twice` |
| `[engine, Rem. 5.24]` | `eng:rem:fixpoint` |
| `[engine, §7]` (individuation) | `eng:S8` — note §7 is book §8 |
| `[engine, §8]` (k is a sequence) | `eng:S8` |
| `[game, App. A]` | `gam:app:code` |
| `[book, Ch. 44]` | `sec:assembly-depth` |
| `[book, Thm. 44.4]` | `thm:ai-bound` |
| `[book, Rem. 44.2]` | `rmk:ai-loose` |

Two of these are off-by-section against the note as published and would
have been wrong if transcribed: the germ (note §11 → book §12.2) and the
individuation condition (note §7 → book §8).

Labels namespaced `sco:` in Chapter 16 and `scm:` in Chapter 59. No
collisions with existing labels.

## Edits to existing chapters

- **`turn_origins_ch02`** — new `rmk:finiteness-provisional` after the
  namespace definition (finiteness is a convenience and does not survive);
  `def:copy-number` kept verbatim as the flat, full-resolution case and
  followed by `rmk:cn-not-effective`; the biosignature remark gains the
  two qualifications it is owed; the Gap 1 paragraph now says the failure
  is ill-formedness rather than intractability, so the repair is to supply
  the argument rather than approximate.
- **`turn_origins_ch04`** — the joint-rise remark gets `rmk:joint-rise` as
  a label and a closing paragraph upgrading the prediction from a
  direction to a rate.
- **`turn_origins_ch08`** — **the retraction.** "Copy number does not
  extend the depth bound; it does something orthogonal" became "At first
  sight it does something orthogonal", followed by three sentences saying
  where the orthogonality fails and pointing at the frontier.
  `rmk:depth-width` now closes on the stronger observation that buying
  depth obliges you to buy copies. First open gap narrowed to the
  non-disjointly-rooted case; third open gap now says the choice of a
  solution is the choice of a generator, and that the two circularities
  are one.
- **`turn_origins_ch06`** — four new gaps: non-well-founded scopes, the
  copy series under merger, affordable scope versus the individuation
  fixed point, and the survey-is-an-assay third column.
- **`turn_origins_ch07`** — "The present framework addresses both" softened
  to "addresses both, though not in one step", with the undecidability and
  the observer-relativity spelled out.
- **`turn_origins_ch01`** — organization paragraph.
- **`turn_causality_ch05a`** — closing paragraph handing off to Chapter 16.
- **`turn_rholife_ch04`** — `eng:rem:fixpoint` now says *which* fixed
  point, pointing at `sco:rmk:munu`.
- **`turn_conclusion_ch01`** — copy number is "count within a namespace,
  taken at the resolution the counter can afford"; the recap gains the
  read-backwards form of the bound.
- **`finding_mind.tex`** — two `\input`s; macros `\Ext`, `\str`, `\gen`;
  Part I and Part VI summaries in the Overarching Introduction; the Origins
  part frontispiece.
- **`bibliography.tex`** — two new entries (`kozen`, `sangiorgi`). Four of
  the note's keys remapped onto entries the book already had:
  `assembly`→`cronin2023assembly`, `HM`→`HennessyMilner1985`,
  `caires`→`CairesCardelli`, `foster` unchanged.

## The Prestige wiring

`prestige_ch01` §`sec:kant-affordable` — the pivot section — gains two
paragraphs before "That is the pivot", giving the budget constraint a
third thing that can be done to it: it can be *measured*. The argument is
that copy number is the mundane cousin of the noumenal, since a count
needs a region a learner can pay to visit and a criterion it was assembled
to afford. The sentence that does the work: a learner does not fail to see
the distinctions it cannot afford — it fails to see that they are there,
and reports a world with fewer kinds in it than the world has.

Chapters 16 and 59 both point back at `sec:kant-affordable`, so the
connection is wired in both directions.

## Notation

Kept both words rather than sweeping. **Namespace** is the object;
**scope** is the role it plays when something is being located or counted
in it. Stated once in Chapter 16 §2 and used consistently after.

## What is not done

- The ecology chapters (Part II) still use the extensional notion
  throughout. Rewriting them in terms of generated scopes — so that
  rooting, typed namespaces and the medium tower are stated in the
  language of Chapter 16 — is the natural next refactor and is a
  substantially larger job than this one.
- `prop:nesting-bound` still identifies namespace nesting depth with modal
  nesting depth; `scm:rmk:direction` flags the related question about
  characteristic-formula depth under composition, which nothing here
  depends on but anyone extending it would.
- The note's estimate of biosignature inflation in actual
  mass-spectrometric practice is a mechanism without a number, and is
  flagged as such in `scm:rmk:falsepos`.
