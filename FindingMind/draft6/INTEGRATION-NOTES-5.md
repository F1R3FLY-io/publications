# Integration Notes 5 — the Kantian positioning in The Prestige

Draft 5 → draft 6. Baseline verified at **435 pp** before any edit; result is
**455 pp**, 0 errors, 0 undefined references, 0 undefined citations,
2 overfull boxes (both pre-existing, identical to baseline).

## What was added

### 1. `ThePrestige/prestige_ch01.tex` — *Any Future Metaphysics* (ch. 49, pp. 425–438)

The first chapter of The Prestige. Source: the Substack essay *From Compressed
Causal Graphs to a Future Metaphysics*, sections V–XII plus the closing
paragraphs, massaged for the book.

Structure:

- unnumbered opening bridge (new)
- 49.1 Kant's question — essay §V
- 49.2 The vindication: forms of cognition as constitutive — §VI
- 49.3 The refinement: the synthetic a priori as compression — §VII
- 49.4 The contestation: the closure of the list — §VIII
- 49.5 The reframing: phenomena, noumena, and bisimulation — §IX
  - 49.5.1 **Characterized, and unaffordable** (new)
- 49.6 The radicalization: predictive existence — §X
- 49.7 The methodological inversion — §XI
- 49.8 **A constructive metaphysics** (new)
- 49.9 Coda: transcendental philosophy made internal — §XII + close

**The opening bridge** does three jobs. It collects the borrowed word from
`pledge_ch01` (*Prolegomena to Finding Mind*), which the book has not touched
since page one; it re-frames the Turn's third-person mathematics as a
first-person question; and it compresses essay §§I–IV — which the Turn already
contains — into a six-item recapitulation with `\ref`s instead of restatement.

**§49.5.1 is the load-bearing new material.** The essay's §IX claims the
noumenal is positively characterizable via Hennessy–Milner adequacy, and does
not notice that `turn_sciencebot_ch08` (*The Apparent Ontology*) and
Proposition `sci:prop:limit` (*The limit of inquiry is unaffordable*) appear to
say the opposite. Unrepaired this reads as overclaiming. The repair is the
distinction between characterization and affordability: the agent's own logic
determines the noumenal, and the agent cannot buy it. Kant's *thinkable but not
knowable* survives — as a budget constraint rather than a categorial
prohibition, i.e. as a frontier rather than a wall, with the consequence that it
can be financed (`ch:gam`), pooled (`ch:com`), or moved (the fibration), none of
which repeal the limit.

**§49.8 is new** and states the claim you asked for: GSLT plus the identified
monads and functors — the reversibility construction, the history endofunctor
and the cost monad, the derivative and the event sub-bundle, the OSLF functor,
the fibration, the semantic category — is the proposal for a *constructive*
metaphysics. Each is a construction with a universal property rather than a
posit; the unbuilt parts show up as gaps in a construction rather than as
disagreements between schools. It also names the three that are gaps: OSLF is
described but not exhibited, the metric is not derived, the complex case is
obstructed.

Three places where the essay was corrected against the book rather than copied:

- **Ordinals → coordinates.** The essay indexes hypotheses by ordinal fiber
  level. The chapter says "a coordinate rather than a rank" and points at
  `ch:apr` / `ch:agy`, so the Prestige does not become the third place still
  describing consciousness as ordinal-valued. (The Overarching Introduction and
  the Turn Conclusion still do; unchanged, still flagged.)
- **The quantum aside.** The essay's parenthetical (real weights → classical,
  complex → quantum) is softened into a caution that names
  `sec:quantum`'s obstruction and its own unfinished status.
- **Phlogiston.** The book uses the word twice, in passing, in imported
  chapters. The chapter introduces it once, as the source programme's name for
  the currency, and otherwise talks about cost and budget in the book's terms.

### 2. `TheTurn/turn_causality_ch05a.tex` — *What Goes Into a Logic* (ch. 10, pp. 85–90)

New chapter in the causality part, placed **after** `turn_causality_ch05` (so
the `Context` definition and context-decorated HML are already available) and
before `turn_causality_ch06`.

Per your instruction, this is the chapter where the OSLF inputs will be
gathered. It opens with `rmk:oslf-status`, which says exactly that and says what
is missing, so its brevity does not read as a claim about the size of the OSLF
construction. For now it holds essay §II:

- 10.1 The agent needs a *where* — space cannot be pre-given
- 10.2 One-holed contexts and the derivative of a type — Huet's zipper,
  McBride's derivative, `def:dT`
- 10.3 A context is a shape; it is not a place — the `λx.[-]` argument,
  `def:splitting`, `rmk:dedekind` (Dedekind and Conway)
- 10.4 Fire: the events available now — `def:fire`, the discrete tangent space,
  the program/environment cut as a splitting
- 10.5 Space and time from one object — the departure from the pre-given arena
- 10.6 What is still missing — the metric, and the functor itself

This is what lets §49.3's reading of the **First Analogy** stand as the essay
wrote it, on `∂T × T`, rather than being paraphrased around a gap: what persists
is the context side of the cut, and what changes is the subterm at it. The
chapter's closing paragraph points forward to `ch:kant` by name, since one of
the arguments there rests on a construction the book describes but has not yet
exhibited.

## What was modified

| File | Change |
|---|---|
| `finding_mind.tex` | inputs for both new chapters; `\Split` `\Fire` `\Plug` macros; Prestige part frontispiece rewritten (the literal `[Placeholder]` is gone) |
| `bibliography.tex` | +5: `Kant1783`, `Kant1781`, `Huet97`, `McBride08`, `spacecalc` (96 → 101 entries) |
| `ThePrestige/prestige_placeholder.tex` | retitled *What the Prestige Still Owes*; four provisional chapters cut to three; item 1 now cross-references `A View from Inside the Framework` and §49.5.1 and names the missing piece as the theorem |
| `TheTurn/turn_rholife_ch01.tex` | one sentence after "a Kantian conclusion with a ledger attached", pointing at `ch:kant` |
| `TheTurn/turn_causality_ch01.tex` | added `\label{sec:causality-intro}` (it had none) |
| `TheTurn/turn_causality_ch10.tex` | pre-existing bug: prose said "the preview table of Chapter~1", which has been wrong since the Pledge chapters were numbered ahead of it. Now a `\ref`. |

## Things to know

**Renumbering.** Inserting a chapter into the causality part shifts every
subsequent chapter number by one (Decoration 10 → 11, and so on to the end).
This is safe: the whole book resolves chapter numbers by `\ref`, and the one
hardcoded number in the text was the ch10 bug fixed above. But any external
notes or slides that cite chapters by number are now off by one from ch. 10.

**Still multiply defined:** `sec:gaps` and `sec:discussion`, both pre-existing.
Neither new chapter uses either; the Prestige opening points at `sec:synthesis`
rather than `sec:discussion` for exactly this reason.

**Not done, deliberately:** Substack Parts VIII and IX still have no home;
`ThePledge/posts/` stops at VII. The Overarching Introduction and the Turn
Conclusion are untouched and remain the last two places describing consciousness
as ordinal-valued. The OSLF construction proper, the metric, and the refactor
that puts the ecology of mind ahead of the physics are the next three pieces.

**One judgement call to check.** Question 4 was an or-question and the answer was
"yes", so the placeholder is both kept after the new chapter *and* rewritten as a
short "still owes" note. If you wanted only one of those, say which.
