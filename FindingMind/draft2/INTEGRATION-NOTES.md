# Folding the rho-life series into *Finding Mind*

Files here mirror the repo layout. Drop `FindingMind/` over the existing
directory; nothing outside these seven files was touched.

| File | Status |
|---|---|
| `finding_mind.tex` | modified — preamble, new part, bibliography hook |
| `bibliography.tex` | new |
| `TheTurn/turn_rholife_ch01–04.tex` | new — the four notes as chapters |
| `TheTurn/code/noughts.rho` | new — copied from `rho-life/` so the listing resolves |
| `finding_mind.pdf` | rebuilt |

Build: three `pdflatex` passes. **339 pages** (was 170), zero errors, zero
undefined references, zero undefined citations, two overfull boxes book-wide
(was 37), none of them in the new chapters.

---

## Placement

New `\part{Ecology as Cognitive Architecture}` after *Origins of Life* and
before *Sentience, Consciousness, and the Hypercomputational Fibration*.
Chapters number 33–36.

---

## Stale cross-references found and corrected

This was the substantive editorial finding. The game note had been written
against an earlier draft of the scientist note, so nearly every reference it
made into that note pointed one result too low. The scientist note's own
appendix comments were stale by one to four. Compose and engine were accurate.

Every reference is now resolved by **name** rather than number — `\ref{sci:prop:confinement}`
rather than "Proposition 13" — so this cannot recur. Where a `\ref` was
impossible (inside `lstlisting`, which does not expand macros), the number was
replaced by the result's name in quotes.

**Chapter 34 (Learning to Play) → Chapter 33.** Every one of these was off by one:

| Was cited as | Actually | Result |
|---|---|---|
| Proposition 12 ×4 | 13 | Confinement |
| Theorem 24 ×2 | 25 | Predators are non-image contexts |
| Definition 25 ×2 | 26 | The game |
| Proposition 9 ×2 | 10 | The limit of inquiry is unaffordable |
| Remark 10 ×2 | 11 | When a finite theory is complete |
| Definition 8 | 9 | Stratified distance |
| Proposition 14 ×3 | 15 | Cost is monotone in distance |
| Proposition 27 | 28 | Foraging inequality |
| Remark 16 | 17 | Yield, not truth |
| Definition 13 | 14 | Revision move |
| Remark 19 | 20 | Science is entropy production |
| Remark 80 | 83 | The pragmatist's objection, and surplus |

Correct as written, and left alone: Proposition 6 (Asymmetry of the modalities)
and Proposition 5 (Trichotomy).

**Chapter 33 (The Mortal Scientist), Appendix A code comments:**

| Was | Now reads | Drift |
|---|---|---|
| `Def. 38` | `Def. "Source and prey"` | +1 |
| `Prop. 44` | `Prop. "Trophic succession"` | +1 |
| `Def. 20` | `Def. "Metabolic channel"` | +1 |
| `Rem. 21` | `Rem. "Metabolism is a race condition"` | +1 |
| `Thm. 24` | `Thm. "Predators are non-image contexts"` | +1 |
| `Prop. 40` | `Prop. "The two sciences"` | +1 |
| `Def. 46` | `Def. "Genome, phenotype"` | +1 |
| `Prop. 45` | `Prop. "Sex attacks the predator's hypothesis language"` | +1 |
| `Prop. 71` | `Prop. "Minimum viable endowment"` | +3 |
| `Def. 50` | `Def. "Homology"` | +2 |
| `Prop. 51` | `Prop. "Species is a namespace formula"` | +2 |
| `Def. 52` | `Def. "Crossover"` | +2 |
| `Sec. 12.4` | `Sec. "Crossover"` | — |

`Def. 4` (Assay) and `Prop. 5` (Trichotomy) were correct.

**Chapter 36 (Engines and Individuals):**

- `\cite[Prop.~12]{scientist}` → Confinement (was one low).
- `\cite[Def.~4.1]{compose}` → Definition "Typed namespaces". Compose has no
  section-numbered results, so `4.1` could not resolve; content fixes it.
- `\cite[Obs.~6.1]{compose}` appears twice with **different intended targets** —
  once "Cooperation is amortised perception", once "Tokens are conserved;
  formulae are not". Both now point where the surrounding sentence requires.
  Worth a glance: these are the two I am least certain of.

---

## Notation

One harmonised block in the master preamble, commented as such. The book won
every collision, as agreed:

- `\GSLT` — the book's `\mathcal{S}` is preserved. The scientist note used the
  same name for the *category*; its two use sites now say `\mathbf{GSLT}` explicitly.
- `\bisim` — no conflict in the end. The notes never used the macro.
- `\rhoc` / `\rh` / `\rt` — three spellings of the same thing, now aliases.
- `\Nsp`, `\Met`, `\Src`, `\Prb`, `\Mat` — unified on the engine note's sans forms
  (compose had `\mathcal{N}`, which collides visually with the book's `\nerve`).
- `\sep` — a false alarm. The two definitions differ in delimiter sizing only,
  not argument order.
- Added `lemma`, `corollary`, `observation` in the book's mdframed house style;
  `listings`, `microtype`, `xspace`, and four tikz libraries added.

All 460-odd labels are namespaced (`sci:`, `gam:`, `com:`, `eng:`), which
resolved seven collisions between the notes. Cross-note citations became
`\chSci` / `\chGame` / `\chComp` / `\chEng`, so the chapters can be reordered
without touching prose.

---

## Compression

- Abstracts dropped from all four, per your call.
- "What is imported" → "What this chapter carries forward" in all three later
  chapters, with the framing sentence rewritten to point at the preceding
  chapter rather than at a citation.
- Chapter 35's recap lost two paragraphs (the cut / three grades of access, and
  sources and prey) that now restate material twenty pages earlier. Nothing
  load-bearing went with them — both had already been trimmed of their
  notation-introducing content.

## Transitional prose

- Part frontispiece in house style.
- Part introduction, "Why the Unit Keeps Moving," arguing that the earlier parts
  each required a knower and each declined to supply one.
- A bridge opening each chapter, replacing the dropped abstract.

Register is the middle ground: formal enough for the Turn, first person with
your lowercase *i*, reaching back to the Pledge twice — bisimulation as "anything
Bob can do Alice can do" now carrying a price tag, and the speed-breath-chess
point about power being the thing a machine cannot hold.

---

## Bibliography

One consolidated list, 62 entries, at the back with a TOC entry.

- 44 from the notes, after deduplication. Eleven pairs were the same work under
  two keys (`knots`/`KnotsNote`, `rho`/`RHO`, `smith`/`SmithMorowitz`,
  `nslogic`/`NamespaceLogic`, `classifier`/`ClassifierNote`, `gslt`/`Omnibus`,
  `probing`/`universality`/`Probing`, `stochastic`/`gillespie-note`/`StochSim`,
  `cleaveland`/`Cleaveland`, `gillespie`/`Gillespie`, `caires`/`CairesCardelli`).
  Keys were normalised in the chapter text.
- 18 the book had been citing without defining, lifted from
  `TheTurn/arxiv/computation-causality-consciousness-2026-03-15.tex`. This
  clears the 20 undefined-citation warnings the book had before this work.
- `scientist`, `game`, `compose` were removed as bibliography entries — they are
  now chapters.

---

## Open items

1. **`engine.rho` is not in the repo.** Chapter 36's `\lstinputlisting{engine.rho}`
   had nothing to include; a bracketed note stands in its place, pointing at the
   sections that give the construction. Drop the file into `TheTurn/code/` and
   restore the listing when you have it.
2. **`sec:gaps` is multiply defined** — `turn_sciencebot_ch09` and
   `turn_hypercomp_ch07` both claim it, so one of the book's own cross-references
   points at the wrong chapter. Pre-existing, one character to fix, left alone
   under the narrow-scope instruction.
3. **Part numbering.** The new part prints as "Part VI" because the Pledge is
   Part I. The Overarching Introduction and the Conclusion both describe four
   parts and number them differently again from the document body. Untouched, as
   agreed — but the new part makes the mismatch more visible, and the Conclusion
   now closes a Turn that has five strands rather than four.
4. **Cosmetic.** Over-wide tables and figures are wrapped in a `\fitwidth` helper
   that shrinks only when necessary; `\emergencystretch` is set to 2.5em, which
   also cleared most of the book's pre-existing overfull boxes.
