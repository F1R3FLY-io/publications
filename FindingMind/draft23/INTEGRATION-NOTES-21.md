# INTEGRATION-NOTES-21 — draft23

**The psychology-of-bots material into The Prestige.**
Source: `psychology-of-bots/reception-and-lapse.{tex,pdf}`,
"Reception and Lapse: Models of Computation as Evidence about the
Communities that Transmit Them" (14,386 words raw; ~11.5k body + 84
bibitems).

Baseline: repo base tree + draft2..draft22, 708pp.

---

## MANIFEST — 6 .tex files + this file + a preview PDF

| file | destination | status |
|---|---|---|
| `prestige_psy_ch01.tex` | `ThePrestige/` | **NEW** — ch66 |
| `prestige_psy_ch02.tex` | `ThePrestige/` | **NEW** — ch67 |
| `prestige_ch03.tex` | `ThePrestige/` | revised (Kant opening) |
| `finding_mind.tex` | root | revised (part title, inputs, two boxes) |
| `bibliography.tex` | root | revised (+70 entries) |
| `pledge_map.tex` | `ThePledge/` | revised (route, gradient, two paragraphs) |
| `finding_mind-draft23-PREVIEW.pdf` | — | preview only |

Diff this table against the directory after committing. The last four
deltas each declared more files than landed; this one is small enough
that it should arrive intact, and if it does not, the shortfall is
visible in six rows.

---

## BUILD

**749pp** (was 708). 0 errors. 0 undefined references. 0 undefined
citations. 0 multiply-defined labels.

**8 overfull boxes — the same 8 as the baseline, at identical
widths.** One line-number shift: `detected at line 251` → `line 252`,
which is the pre-existing box in `pledge_map.tex` displaced by the
thirteen lines this delta inserts above it. No new boxes.

Verified in a **clean room**: repo base tree, then draft2..draft22
overlaid in order, then this delta, three `pdflatex` passes.
Not from the working tree.

---

## NUMBERING

Part XI now runs:

| ch | title | pp |
|---|---|---|
| 64 | What a Symbol System Would Have to Be | 651–666 |
| 65 | If This Is a Simulation, Which Kind | 667–674 |
| **66** | **What It Is Like to Be a Bot** | **675–698** |
| **67** | **How Life-Like Is Life-Like Enough?** | **699–708** |
| 68 | Any Future Metaphysics | 709–724 |

Kant moved by two. Nothing else in the book shifted — Part X is
untouched and the Turn is untouched. No hardcoded chapter numbers
exist anywhere in the source, so the insertion was reference-safe by
construction; this was re-verified rather than assumed.

Part XI retitled **"Reference, Simulation, Reception, and a Future
Metaphysics"**. `\label{part:reference}` retained.

---

## THE SEVEN DECISIONS, AS IMPLEMENTED

### (1) Just before Kant

Placed after `ch:sim` and before `ch:kant`. The reason recorded in
the plan is now visible in the text: `ch:kant`'s opening already
asked *what is it like to be an agent under existential pressure to
work out its own situation?* and posed it cold. It now arrives with
two chapters of answers behind it, and §`ch:kant` says so in a new
paragraph.

### (2) Two chapters, split at *The present*

The split falls exactly where the source note broke. Chapter 66 is
the survey; chapter 67 is the argument. Both use the `psy:`
namespace and the lowercase-i personal register, matching the four
native Prestige chapters rather than the seven that moved in from
the Turn.

**Chapter 66 spine.** Opening (the promise) · §`psy:sec:method` the
instrument, with `psy:prin:lapse` (Selection and lapse) and
`psy:rem:lapse-hosting` · §`psy:sec:founding` Turing / Post / Church
· §`psy:sec:vn` von Neumann, with `psy:obs:isolation` (Ontological
isolation) · §`psy:sec:chomsky` Chomsky and cybernetics, with the
nine-row summary table · §`psy:sec:milner` Milner and Petri, with
`psy:rem:fixpoints` · §`psy:sec:girard` Girard, with `psy:rem:bang`
and the Rust/session-types reception · §`psy:sec:quantum` Feynman
and Deutsch, with `psy:obs:three`, `psy:rem:angelic`,
`psy:rem:two-quantum` · §`psy:sec:reflection` Gödel / McCarthy /
Smith / Rosette / rho · §`psy:sec:reception-of-reflection` ·
§`psy:sec:nn` neural networks · §`psy:sec:delivered`.

**Chapter 67 spine.** §`psy:sec:present` with `psy:prin:loop` (No
loop without a path) and `psy:rem:same-split` · §`psy:sec:conditions`
with `psy:req:structural`, `psy:req:runtime`, `psy:req:priced`,
`psy:req:social`, `psy:req:contact`, `psy:req:closed` ·
§`psy:sec:objections` · §`psy:sec:threshold` with
`psy:prin:lifelike` and `psy:rem:assembly` ·
§`psy:sec:openchallenges` · §`psy:sec:questions` with `psy:q:metering`,
`psy:q:concurrency`, `psy:q:learner`, `psy:q:counterexample` ·
§`psy:sec:psy-claims`.

### (3) The promise, redeemed explicitly

Chapter 66 opens by naming §`sec:pledge-viii` (*Which Computer?*) as
a promissory note — three generations named, the third asserted as
the destination, nothing argued because the machinery did not yet
exist — and says these two chapters redeem it.

§`psy:sec:delivered` discharges it and then states the limit
precisely: what has been shown is that the third generation was not
chosen for its power, since every ingredient was available and was
passed over for operational reasons. What has **not** been shown is
that it is right. *"A history of what a field could not bring itself
to say is not an argument that the thing it could not say is true."*

This is the same shape as ch56 redeeming §`sec:pledge-iii`, and
deliberately so.

### (4) The forecast, redone as questions

§`psy:sec:questions` states the withdrawn forecast in full (the
metering travels, the reflection lapses; the concurrency travels, the
namespace discipline does not; the weights travel, the learner does
not), then says why it is being withdrawn: running a historical
instrument forwards converts it into a prophecy, in the one case
where the author has the least standing to be believed and the most
reason to be self-serving — *a forecast of one's own neglect is the
cheapest form of insurance a writer can buy.*

The content survives as four numbered Questions, each checkable:

- `psy:q:metering` — does the metering travel without the reflection?
  Requirement `psy:req:priced` says the two are separable, so the
  book has named its own exposure.
- `psy:q:concurrency` — does concurrency travel without namespace
  discipline? The check is whether anything downstream treats *what
  a channel is made of* as a question at all.
- `psy:q:learner` — do the weights travel without the learner?
  Flagged as the sharpest available test, because here the two
  halves are unusually easy to separate.
- `psy:q:counterexample` — is there a reception that went the other
  way? Names the gap: `psy:prin:lapse` is a claim about a tendency
  and nothing in the book bounds it.

Closing line: *four questions with checkable answers are a better
countermeasure than one prediction with none.*

### (5) The two quantum questions, separated

`psy:rem:two-quantum` is the load-bearing piece of this decision and
is new writing, not in the source note.

- **The technical question** — can a weighted, cost-accounted
  calculus carry a quantum presentation? — is pointed back at
  §`sec:quantum` and `rmk:two-obstructions`. Your view that later
  versions of that machinery will settle it is recorded as a view
  and not a result, with the linear-fragment route named (every
  bound name used exactly once; an interpretation of that fragment
  only; the exponential marking the boundary where the fragment
  meets the ambient non-linear world, which is where information is
  destroyed and Landauer's charge falls due). The remark says
  plainly that nothing in the book establishes it.
- **The chapter's own question** — what is it like to be a *quantum*
  bot, and what does that say about a computational model for
  general intelligence? — is answered by `psy:obs:three` and
  `psy:rem:angelic`, and the remark states that settling the first
  question would not settle this one: *a calculus that can express a
  unitary evolution has not thereby made its agents any less
  angelic; it has acquired the vocabulary to describe a kind of
  agent for which improvement is structurally unavailable, which is
  a gain in expressiveness and not a gain in life-likeness.*
- Closes by noting that existing quantum process calculi are dualist
  and that this repeats, one level up, the defect
  `psy:obs:isolation` names.

`pledge_map`'s "It does not derive quantum mechanics" paragraph now
points at this remark.

### (6) Part XI retitled

Both the act-opener box (Part IX) and the Part XI box were extended,
since each enumerates what the act does and each was now wrong.

### (7) Sequenced first

Ahead of the parked weight-maps-before-the-physics reorder. Nothing
in this delta touches the Turn, so that pass starts from an
unchanged base.

---

## WHAT THIS CONNECTS TO IN THE BOOK

The source note is a standalone essay. The chapters are not: eleven
internal connections were built that the note could not have.

**`psy:rem:lapse-hosting` — the best of them.** A reception is a map
from one generation's practice to the next, and the pair of
conditions `obs:hosting` calls *hosting* and *exhausting* is exactly
the pair deciding whether such a map loses anything. **Lapse is
hosting without exhausting.** The remark ends on why lapse is
invisible from inside: the hosting condition is never violated —
nothing the receiving generation can say has become unsayable. This
makes the note's borrowed textual-transmission metaphor into a
statement in the book's own vocabulary, and it is the reason the
chapters read better after `ch:notation` than before it.

**`psy:rem:fixpoints`.** The least/greatest fixed-point framing now
says why *this book* is coinductive: the criterion of identity
throughout is a greatest fixed point, and `ch:suffer`'s learner is
defined by what it maintains rather than by what it terminates in.

**`psy:rem:same-split`.** The training/inference separation is the
metalevel/agent split arrived at architecturally rather than
institutionally, and the ecology of Part II is what a model looks
like when the split is unavailable. `spk:thm:one` (learning and
inference are one relation) is named as the sharpest version, and
§`psy:sec:nn` calls that theorem the exact negation of the regime it
describes.

**`psy:obs:three`.** The free/priced/prohibited trichotomy places
`ch:carho` at the middle position and says why that position is the
only one at which the criterion of improvement becomes internal to
the agent. §`psy:sec:threshold` then uses it to carry
`psy:prin:lifelike`.

Smaller ones: §`psy:sec:church` marks the λ-calculus as the ecology
with the trophic seam and nothing else; §`psy:sec:chomsky` names the
subset construction as the founding generation's decision that the
place `ch:tower` locates does not exist; §`psy:sec:girard` puts
`ch:chs` downstream of the sentence about who schedules;
§`psy:sec:smith` reads lazy instantiation as the first appearance of
the thought `ch:obsext` prices; §`psy:sec:rho` credits `ch:sci`'s
quoted-behavior hypotheses; §`psy:sec:objections` gives Smith's
pre-individuation objection the book's direct answer (`ch:sci`) and
its incompleteness.

---

## FLAGGED FOR YOUR EYE

1. **`psy:rem:assembly` is mine and it is load-bearing.** The note
   proposed assembly index as a candidate quantity for "how life-like
   is enough". The book *has* `thm:ai-bound`, so the remark credits
   it — and then concedes two things: the number bounds
   **resolution**, not reach (per `rmk:power-caveat`), and **nothing
   connects it to the six conditions.** A bound on what an ecology
   can discriminate is not a bound on whether the improvement loop
   closes inside it. If that concession is too strong or too weak,
   this is the paragraph to change.

2. **The closing paragraph of ch67 claims the pair for the pivot.**
   It argues that the missing path is not a prohibition either: it is
   *the accumulated cost of a great many decisions each of which was
   locally cheap*, which is the book's price-not-prohibition move one
   level up, and it hands that to `ch:kant` as the last instance.
   `ch:kant` §`sec:kant-affordable` currently gathers three
   instances. **It has not been updated to gather a fourth** — I did
   not want to touch the gathering without your say-so, since it is
   the book's pivot. One paragraph if you want it.

3. **The sceptic's route** was added to `pledge_map` and the section
   retitled *Six routes through*. It claims the two chapters stand
   alone and can be read cold by someone deciding whether the rest is
   worth six hundred pages. That is a real claim about them and you
   should test it. Spelled British to match its neighbours, per the
   standing decision that The Pledge is yours.

4. **Chapter 66 is now the longest chapter in the book** at ~10.3k
   source words / 24pp, against Kant's 16pp. The two-way split was
   your instruction and I think it is right, but the imbalance
   between 66 and 67 (24pp vs 10pp) is visible in the table of
   contents. A second cut is available at §`psy:sec:reflection` if
   you ever want three.

5. **§`psy:sec:rho` discounts itself in its first sentence** — "the
   one place where the method is being used by an interested party,
   and the reader should discount accordingly." Not in the note.
   Delete it if it reads as false modesty.

---

## BIBLIOGRAPHY

**+70 entries**, appended under a dated comment banner. Book total
172 → 242.

**Twelve duplicates caught and remapped rather than merged.** The
note's keys were re-pointed at the book's existing entries in the
chapter sources, so the bibliography grew by 70 and not 82:

| note key | → book key | |
|---|---|---|
| `landauer1961` | `Landauer` | same paper |
| `hebb1949` | `hebb` | same book |
| `sharma2023` | `walker2023` | **same assembly-theory paper** |
| `meredith2005` | `RHO` | same paper |
| `abramsky1994` | `abramskyPaP` | same paper |
| `wadler2012` | `wadler` | same paper, JFP version |
| `mcculloch1943` | `mcp` | same paper |
| `hinton2022` | `Hinton` | same paper |
| `bellinscott1994` | `bellinscott` | same paper |
| `cairespfenning2010` | `cairespfenning` | same paper |
| `danos2004` | `danos2004reversible` | same paper |
| `nagel1974`, `vonneumann1966` | unchanged | already identical |

`bennett1973` (logical reversibility, 1973) and `Bennett` (the
thermodynamics review, 1982) are **genuinely different papers** and
both stand. The book had no Girard entry; `girard1987` is new.

Cited titles in the new entries keep their authors' spelling, per
the standing rule.

---

## SPELLING

All six delivered files verified against the draft22 word list plus
the false-friend list. **Residual in the two new chapters: zero.**
One `towards` in ch67 was normalised to `toward` to match the book's
19-to-4 preference.

`pledge_map.tex` retains its British forms — yours by decision, not
backlog.

---

## REPO COMMIT FAILURE — FOURTH OCCURRENCE

Measured on the clean-room overlay before this delta was applied.

INTEGRATION-NOTES-20 declares **40 files**; committed draft22 holds
**18 .tex**. The overlay carries **191 British-spelling hits across
34 files** in the Turn, the Prestige and the Interludes, where
NOTES-20 records residual-outside-The-Pledge as zero.

Worst uncommitted: `turn_ndo_ch02` (38), `turn_chs_ch01` (27),
`turn_ndo_ch01` (16), `turn_gslt_ch05` (16), `turn_gslt_ch11` (11).

**And it is again not purely a commit failure.** `artefact` survives
in files draft22 *did* commit — `turn_rholife_ch01`, `ch02`, `ch04`,
`prestige_agy_ch01`, `prestige_ch01`. That is the third word-list gap
in a row (after the draft18 list, then the draft20 list). The next
sweep should be run off a list generated from the files rather than
one built by hand; the hand-built list has now been wrong three
times, each time in a different place.

## SOURCE NOTE BUG

`reception-and-lapse.tex` line 19:

```latex
\newcommand{\parr}{\rotatebox[origin=c]{180}{\newcommand{\rhoc}}}
```

A `\newcommand` nested inside a `\rotatebox`. The macro is used once,
at line 582. The chapters sidestep it by naming the connectives in
words rather than symbols, which reads better in this register
anyway, but the standalone note will not compile that line.
