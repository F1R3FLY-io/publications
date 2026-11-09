# Integration notes — draft27

**The grading pass, part one (Phases 0 + 1 of the four-delta plan): weight maps become the on-ramp to the graded where-clause; the Complex Amplitudes chapter is rebuilt on the v6 construction. 779 pp → 794 pp. 0 errors, 0 undefined references, 0 undefined citations, 0 multiply-defined labels. Same 9 overfull boxes as draft26 at identical widths (one line-number shift, 252→262, in `finding_mind.tex` from inserted lines).**

Baseline verified before any edit: base tree + draft2–draft26 overlaid reproduces draft26's 779 pages, 9 overfull boxes, 0 undefined.

---

## Manifest (13 files + this note + PDF)

Diff this table against the directory after committing. Word counts are `wc -w` on the delivered file.

| File | Destination | Status | Words | md5 (first 8) |
|---|---|---|---|---|
| `turn_gslt_ch11.tex` | `TheTurn/` | **rewritten** — ch. 13, now *Grading: From Weight Maps to the Where-Clause* | 8360 | 7f87a4b1 |
| `turn_causality_ch09.tex` | `TheTurn/` | **rewritten** — ch. 34, *Complex Amplitudes, and Where the Coherence Is* | 3776 | f3e2059c |
| `turn_rholife_ch05.tex` | `TheTurn/` | re-expressed — ch. 25, *A Network That Learns by Rewriting* | 4047 | 902a38d4 |
| `turn_causality_ch06.tex` | `TheTurn/` | re-expressed — ch. 31, *Decoration* | 2377 | 30ff9671 |
| `turn_causality_ch08.tex` | `TheTurn/` | 3 sites — ch. 33, *Extended Modal Operators* | 1093 | 108f9130 |
| `turn_causality_ch10.tex` | `TheTurn/` | 1 site — ch. 35, *Synthesis* | 1116 | 8afb3a73 |
| `turn_gslt_ch07.tex` | `TheTurn/` | 1 site — ch. 12, *The History Monad* | 797 | b1659c86 |
| `turn_origins_ch06.tex` | `TheTurn/` | 2 gap items — ch. 55, *Open Gaps* | 864 | ad336168 |
| `turn_causality_ch01.tex` | `TheTurn/` | 2 sites — Part IV opener table + organization | 1636 | 376a426e |
| `turn_suffering_ch01.tex` | `TheTurn/` | 1 site | 4205 | d6cebef2 |
| `prestige_cons_ch01.tex` | `ThePrestige/` | 2 sites — ch. 66, *Consciousness* | 2501 | c4f9f85f |
| `finding_mind.tex` | root | 8 new macros; Part I summary paragraph; input comment | 7088 | 68967e58 |
| `bibliography.tex` | root | +6 entries (274 → 280) | 5091 | cda35fa8 |
| `finding_mind.pdf` | — | rebuilt, 794 pp | | |

No files deleted. No chapter moved. No `\input` order changed. Chapter numbering is **unchanged** throughout (the rewrite is in place).

---

## Decisions applied (your answers, Nov 8)

1. **One rewritten chapter**, weight maps as the on-ramp. ✔
2. **ch. 34 adopts v6's construction and the reviewer's modest claim; `unitary-equations` stated as a view beside it**, not reconciled. ✔
3–4. Phase 2 (Mortal Scientist's hypotheses as clauses throughout; PLN fibre results) — **deferred to draft28** as agreed. See "Tracked inconsistencies".
5. `process-molecules`, catamorphisms — not touched. ✔
6. Paper III out of scope; the Grading chapter draws on v4 (resolution algebra, conservativity, DNF, plasticity), v6 (contention sets, completion, fragment) and the split-plan directly. ✔
7. Four-delta sequence, grading first. This is delta 1 of 4.
Also applied: **Section A goes in the Turn** (your mid-turn note) — bears on draft29, no effect here.

---

## Phase 0 — ch. 13, *Grading: From Weight Maps to the Where-Clause* (`\label{ch:weight}` kept, `\label{ch:grading}` added)

**Structure.** §§13.1–13.5 are the weight-map construction, kept nearly verbatim with every `wt:` label in place (33 labels, ~110 inbound `\ref`s across ten chapters all resolve). §13.1 was rewritten to announce the two constructions and the order; small forward pointers were added at the three places the first half imposes a discipline the second half dissolves. §§13.6–13.9 are new, namespace `gw:`:

- **§13.6 Where the choice is** — the race equation; RSpace as a resolution mechanism; the resolver as a *parameter*; `gw:prin:aperture` (contexts create apertures; they do not read them); the separation frame (Resolvers × Terms → Distributions); `gw:def:support` (support adequacy, one-sided on purpose); `gw:rmk:noncompositional`.
- **§13.7 The graded where-clause** — `gw:def:algebra` (resolution algebra) + Table 13.1 of instances; `gw:prin:or` (interference is a false disjunction with true disjuncts); two sorts, one slot (`gw:rmk:oneslot`); the syntax as a rholang listing; `gw:def:cand` (candidates by *occurrence*); `gw:def:val`; the graded modality as transfer operator (`gw:eq:onestep`); `gw:def:denotation` with the budget (`gw:rmk:budget` ties it to the ω of ch:tower and the token budget of ch:sci); `gw:thm:conservativity`; `gw:cor:adequacy`.
- **§13.8 Weight maps are the linear graded clauses** — `gw:thm:dnf` (the hinge); `gw:cor:nopartition` (closes the seam of `wt:obs:closure`); `gw:prop:plastic`; **`gw:rmk:state-is-term`** — see below; `gw:rmk:retired` (what of the weight map survives: $\gfac$, $\chi$, and $\Wgt_\Semi$ as the name of the linear fragment).
- **§13.9 Contention, and what is normalized** — `gw:def:contention` (conflict, components, maximal matchings = the RSpace zip); `gw:prop:eventfact`; `gw:rmk:costs` (nonlocality; permanent-shaped sums). Kept short; it exists so ch. 34 and the forthcoming `do` chapter have the vocabulary.
- **§13.10 Where this is used** — now names four consumers including ch:sci ("a hypothesis is a clause") — which is a *promise against draft28*.

**The quantum section (`wt:sec:quantum`) left ch. 13 entirely.** Its six labels (`wt:def:hilbert`, `wt:def:jump`, `wt:def:hamiltonian`, `wt:prin:slogan`, `wt:prop:interference`, `wt:thm:degeneration`) now live in ch. 34, so the 14 inbound refs from ch. 33/35/66/55 still resolve. Nothing refers to `wt:sec:quantum` itself.

**`gw:rmk:state-is-term` is the one piece of new argument in the chapter, and it is mine.** `gw:prop:plastic` (from v4) has a hypothesis — trace-dependence expressible in the current term — and `wt:ex:nonmarkov` (non-commuting updates) is a case where it fails. The weight map's answer was Markov-on-configurations. The remark says the clause's answer is rho's answer: put the history in the term as a datum the clause reads; the chain is then on terms. This is what lets ch. 25 keep "the same rewrite that infers also learns" without a table, and it is what ch. 31 and ch. 35 now quote. It is a reading of v4's proposition plus the book's own ethic rather than a result in any source note; it wants your eye.

---

## Phase 1 — ch. 34, *Complex Amplitudes, and Where the Coherence Is* (`\label{sec:quantum}` kept; `sec:coherence`, `rmk:obstruction`, `rmk:two-obstructions` kept, all inbound refs resolve)

Rebuilt in six sections:

1. **Status remark** rewritten: what each of three versions changed; the claim stated as v6 states it — *a finite, contractive, staged fragment of graded rho admits a candidate quantum interpretation; the extension to unrestricted reflective concurrency remains open.*
2. **The question** (cancellation), now also `gw:prin:or`.
3. **The first construction and its two obstructions** — the linear-fragment apparatus (`wt:def:hilbert`, `wt:def:jump`, `wt:def:hamiltonian`, `def:conservativity`, `wt:prop:interference`, `wt:thm:degeneration`) kept compactly *because other chapters cite it and because the degeneration theorem is true*; `rmk:obstruction` (semantic) kept verbatim; the constructive obstruction given its own label `rmk:constructive`; `rmk:two-obstructions` restated — the constructive one is now **specific to the linear fragment**.
4. **The second construction: amplitudes on candidates** — the clause is the derivation-indexed decoration the previous version said would be "a strictly larger construction"; conservativity is not violated on clauses in normal form; **the semantic obstruction answered by `gw:prin:aperture`** (the resolver is a parameter no context reads; bisimulation is on the unweighted relation; support adequacy holds, its converse fails, and the failure is the interference). **The Part VI (sciencebot) ontology concession is retracted again, for a cleaner reason than draft16's** — the resolver decides races, it identifies no terms. Then `prop:completion` (fibre sum over maximal matchings per contention component; the three measured wrong conventions named), `rmk:causal` (n! inflation, tied back to ch:reversibility's tree), the two costs restated for ℂ, `conj:global`.
5. **When is the denotation a quantum state?** — `prop:absorb` (termination is not unitary), halt tagging, `def:staging` (spectators/staging), `conj:safe` with the finite-search evidence described as evidence and the diagnosis named; `rmk:linear` — **"the quantum reading is credible for the fragment of rho that looks like a circuit, and the part of rho that makes it rho is the part that is open."**
6. **Two views of where unitarity lives** (`sec:twoviews`, aliased `sec:coherence`) — view one (reductions carry coherent amplitude; built; bounded) and view two (unitary in the equations, measurement in the reductions; `UnitaryEq`; a design session, not a semantics); where they agree (bisimulation survives on both; `wt:prin:slogan` true of two, vacuous of one) and where they differ (the status of a firing: coherent step vs measurement) — **with the observation that the example separating them has not been written.** `wt:prin:slogan` kept and restated with the qualification. `rmk:coherence-gaps` rewritten to three items.
7. **Simulation, in the meantime** — stochastic loop on terms; Born loop on the fragment; `wt:thm:degeneration` true of the linear fragment.

The density-matrix remark (ρ as resource account) was dropped as superseded — nothing referenced it.

**ch. 31 Decoration** — three in-place edits: the opening paragraph places the decoration as *the linear fragment, on purpose* (the semiring is the only thing left to choose there); the partition sentence gains the `gw:cor:nopartition` qualification; `def:refcomplete`'s "overlapping keys … a different semantics, and it should be said which" now says which (sum, by `gw:thm:dnf`; most-specific is a third thing the clause language does not supply); the complex row of the table re-pointed; and §Decorations move gets a paragraph giving the two readings of "constants respond to what has happened" (table beside the term vs. read off the term) and saying the second is the one the part should be heard in.

**ch. 25 Spiking** — the substantive re-expression. The synapse is now a graded clause reading a **weight register** $w_{ji}$ (a channel carrying one quantized datum), `spk:def:keys` is retitled *Synaptic clause* (label kept), `spk:prop:admissible` restated as "normal form, depth 2, one lookup"; `spk:prop:sum` reproved from candidates-by-occurrence; §25.4 retitled *Plasticity is a rewrite of the register* (`spk:sec:plastic`), `spk:def:hebb` is now a join that consumes and re-emits the register; `spk:thm:one` restated with marginals *markings / registers* and a one-line proof; `spk:rmk:stdp` repaired via an eligibility register (the hypothesis of `gw:prop:plastic` failing, and `gw:rmk:state-is-term` saying what to do); §Uniformity: one clause schema over structured names; **`spk:rmk:bosonic` rewritten** — the $m$ spikes do not interfere because the clause reads the register, not the spike, hence is in normal form, hence distinguishes no candidates, and they share a receipt so completion normalizes over them; "negative instance of both views". The design-decision section gained the paragraph that faces the subtlety: with no table, the efficacy must be in the term, and *being weighed by* a number is not *computing with* it.

**Lighter sites** (ch. 12, 33, 35, 55, Part IV opener, Suffering, ch. 66): each re-pointed to `ch:grading` / the clause with a clause gloss; ch. 55's gradient-coupled-rules gap now reads as **closed in two steps** (dynamic rates are the ordinary case; the externally-driven parameter comes inside as traffic the clause reads — "what remains is to write the gradient as that traffic, which is modeling and not framework"). ch. 66 `rmk:approx-status` notes the candidate-indexed instance is credible only on a staged fragment a scheduling problem does not in general inhabit.

---

## Macros, environments, bibliography

- `finding_mind.tex` +8 macros after `\Hilb`: `\crisp`, `\Cand`, `\Out`, `\sumv`, `\tensorv`, `\supp`, `\Reach`, `\Bool`. Checked: none collide with chapter-local definitions.
- No new environments. `principle`, `observation`, `conjecture`, `construction` and the `rholang` listing language already existed.
- Bibliography +6: `GradedV4`, `GradedQuantum` (v6, "Races Decided by the Born Rule"), `GradedPLN`, `UnitaryEq`, `Winskel`, `AaronsonArkhipov`. The three graded-where entries are typed as F1R3FLY.io working notes with the paper-I/II-of-three designations from `split-plan.md`; adjust titles if the notes are retitled on release.

---

## Prose reconciliations (the part a green build does not catch)

Grepped the 13 shipped files and their neighbours (ch. 11, 12, 14, 26, 32) for relative-position language. All true: "the two preceding chapters" in ch. 13 (Cost, History), "the four preceding chapters" in ch. 25, "the next chapter" in ch. 31 and ch. 12. Two forward-position statements I had written were caught and fixed before shipping: ch. 13 no longer says the RSpace part is "the next part of the book" (it will be after draft29; it now says "to which this book returns when it turns from the calculus to the machine"), and ch. 34 no longer says "three later chapters refer to it" (ch. 33 is earlier).

Part I summary in `finding_mind.tex` gained four sentences on the dissolution. The TikZ dependency figure was **not** changed: Grading still sits in the Machinery box and no arrow changes.

US spelling: the 13 shipped files are sweep-clean. Fourteen residual British forms were found in pre-existing text of five of them (ch. 31 ×8 incl. *tropicalisation*, *minimisation*; ch. 33 *internalising* ×2; ch. 35 *normalised*, *optimisation*; ch. 12 *Internalisation*, *behaviour*; ch. 55 *factorisation*, *generalisation*) and converted, with `\label`/`\ref`/`\cite` arguments masked. Page-neutral.

---

## Tracked inconsistencies — to be closed by draft28 (you asked that these be kept track of)

| # | Where | What | Closes in |
|---|---|---|---|
| T1 | ch. 21 *The Mortal Scientist* (`turn_rholife_ch01.tex`), ll. ~1958–1963, 2019, 2076–2099, 2405 | Still speaks of "a weight map" and `wt:def:coalgebra`/`wt:thm:markov` as the learner's machinery; its hypothesis language is not yet the clause. ch. 13 §13.10 and `finding_mind.tex`'s Part I summary now *promise* "a hypothesis is a clause". | draft28 (Phase 2) |
| T2 | ch. 23 *Composing Learners* (`turn_rholife_ch03.tex`), ll. 159, 179, 287, 306, 1135 | "cost as the weight map" — the canonical-section discussion is in weight-map vocabulary. | draft28 |
| T3 | `finding_mind.tex` Part I summary | says "an intervention is a clause" — the `do` chapter does not exist yet. | draft28 (B2) |
| T4 | ch. 13 §13.6 | RSpace described as "the store that realizes rho in f1r3node, to which this book returns" — the Realization part does not exist yet. | draft29 (Section A) |
| T5 | `pledge_map.tex` l. 329, `pledge_related.tex` | Front matter still describes the Weighting chapter as weight maps only; unproved-claims list lacks `conj:global`, `conj:safe`, the recursion obligation. Front matter is draft30 by plan. | draft30 |
| T6 | ch. 24 *Enzymes and Individuals*, ch. 53 origins_ch05 l. 23 ("broken symmetry in the weight map") | Weight-map vocabulary in passing; harmless until Phase 2 renames. | draft28 |

T1 is the one you accepted explicitly. T3 and T4 are promises I introduced; both are one-line reversions if draft28/29 slip.

---

## Three things you should look at

1. **`gw:rmk:state-is-term`** (ch. 13 §13.8), as above — mine, load-bearing for ch. 25/31/35/55.
2. **ch. 34 §34.3.1, the retraction of the Part VI concession.** draft16 retracted it on the ground that coherence sited in the equations never asks bisimulation to pool runs. This draft retracts it on the ground that the resolver is a parameter no context reads (`gw:prin:aperture`), which holds on *both* views. If you think the aperture principle is doing too much work here, the sentence to weaken is "A theory may have a Born resolver and keep every bisimulation class it had."
3. **ch. 34 §34.5, the separating example.** I wrote that the example telling view one from view two "has not been written" — view one predicts a which-path record destroys interference only if read, view two predicts the reduction *is* the reading. That is my characterisation of the difference, not the source's; if it is wrong the paragraph should go.

Smaller: ch. 25's opening still says "four preceding chapters" though Tubular Agents now *follows* it — still true, left alone. The `principle` environment prints green like theorems; `wt:prin:slogan` and `gw:prin:aperture` render as Principle 34.1 / 13.1 — fine but worth knowing the numbering restarted when the slogan moved chapters.
