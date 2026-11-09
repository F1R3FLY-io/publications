# Integration notes — draft28

**The grading pass, part two (Phase 2 + B2 of the four-delta plan): the Mortal Scientist's hypotheses become graded clauses throughout, with the fibre and its gradient; and the new chapter *Interventions at the Cut* lands directly after Grading. 794 pp → 814 pp. 0 errors, 0 undefined references, 0 undefined citations, 0 multiply-defined labels. Same 9 overfull boxes as draft26/27 at identical widths.**

Built against draft27 as delivered (optimistic concurrency). Clean-room verified: repo baseline + draft2–draft26 + draft27 + draft28 → 814 pp. If your review changes any draft27 file, tell me which and I will rebase; the only draft27 files draft28 also touches are `turn_gslt_ch11.tex`, `finding_mind.tex` and `bibliography.tex`, and the changes to each are small and listed below.

---

## Manifest (8 files + this note + PDF)

| File | Destination | Status | Words | md5 (first 8) |
|---|---|---|---|---|
| `turn_gslt_ch13.tex` | `TheTurn/` | **new** — ch. 14, *Interventions at the Cut* | 5379 | f678fd79 |
| `turn_rholife_ch01.tex` | `TheTurn/` | **restated** — ch. 22, *The Mortal Scientist*: hypotheses as clauses throughout; new §22.6.2 | 24624 | 3372a6e8 |
| `turn_gslt_ch11.tex` | `TheTurn/` | 1 paragraph — ch. 13 §*Where this is used* now names ch. 14 (supersedes draft27's copy) | 8390 | 04845a94 |
| `turn_gslt_ch01.tex` | `TheTurn/` | §*Organization of this part* repaired (see below) | 1379 | 6e1b8c7e |
| `turn_rholife_ch03.tex` | `TheTurn/` | 5 sites — ch. 24, *Composing Learners* | 11317 | 2535dff2 |
| `turn_origins_ch05.tex` | `TheTurn/` | 1 site | 1467 | d559e1e3 |
| `finding_mind.tex` | root | +10 macros; 1 `\input`; Part I summary (supersedes draft27's copy) | 7170 | b539bb2b |
| `bibliography.tex` | root | +7 entries (280 → 287) (supersedes draft27's copy) | 5219 | 94697c4e |
| `finding_mind.pdf` | — | rebuilt, 814 pp | | |

**Numbering:** one chapter inserted at position 14. Everything from *Cost-Accounted Rho* onward shifts +1: the Mortal Scientist is now **ch. 22** (was 21), Spiking 26, Tubular Agents 27, Suffering 28, Complex Amplitudes 35, Kant 76. No hardcoded chapter numbers exist in the source, so this is `\ref`-safe; INTEGRATION-NOTES-25's chapter numbers are now off by one from ch. 14 on.

---

## Phase 2 — ch. 22, *The Mortal Scientist*

Twelve sites, in reading order. Every `sci:` label is unchanged; four new labels (`sci:def:gradhyp`, `sci:rem:onecla`, `sci:prop:hypfib`, `sci:fig:fibration`) and the new subsection `sci:sec:fibre` (§22.6.2) with `sci:prin:twosearch`, `sci:thm:score`, `sci:thm:natgrad`, `sci:prin:metric`, `sci:rem:trap`, `sci:prop:nobase`, `sci:prin:division`.

1. **§1.3 (R3)** — the where-clause is now Chapter 13's graded clause, not `\cite[§9]{Omnibus}`; "a hypothesis is a pair — a formula and a value — and the two are revised by different kinds of move."
2. **§1.4** — the claim about learning now says the space of *formulae* has no gradient and the space of *graded hypotheses* is a fibration whose fibre does.
3. **§4.1** — the general form carries a graded $\psi$; **`sci:def:gradhyp`** (shape + $\theta\in\Rnn^d$); the chapter fixes $\Semi=\Rnn$ and says why (support adequacy is an equality there, so nothing cancels and a hypothesis has a likelihood); a Boolean guard is the special case and constructions mentioning only the crisp condition are unchanged.
4. **§4.2** — **`sci:rem:onecla`**, *The pair becomes one clause*: the complementary guard pair's dependence on $\neg$ in the hypothesis language is removed by valuing the guard at $(\gstr,\gconf)$; confirm/refute/$\bot$ are the corners of the unit square and $\bot$ is a coordinate; with the caution that confidence cancels unless the learner has an outside option. `sci:prop:trichotomy` itself unchanged.
5. **§5** — **`sci:prop:hypfib`**, *The hypothesis space is a fibration*, + TikZ figure (`sci:fig:fibration`, from the PLN paper, caption rewritten to point at `sci:prop:confinement`).
6. **§6.1** — `sci:rem:nogradient` retitled *There is no gradient in the base*; body qualified; `ch02`, `ch03`, `ch04`, `ch06`, `origins_ch09`, `prestige_agy` all cite `sci:prop:confinement` and none cites the remark, so nothing downstream reads differently.
7. **§6.2 NEW — *The fibre, and the gradient that lives there*** (~1,900 words). The objective is a likelihood (over $\Rnn$ only — the structural reason the learning instance is real and the quantum instance is not a learner); `sci:thm:score` with proof (one number per firing; the gradient is a message, not a pass; likelihood-ratio estimator / eligibility trace / policy gradient named as the other vocabularies, with the point being the direction of derivation; the objective is metabolic); the naive step diagnosed; `sci:thm:natgrad` with its two-line proof (revision = implicit natural-gradient step at the posterior count; the sharpness note — $0.46$ discrepancy at $n$, machine precision at $n+1$); `sci:prin:metric` (confidence is the metric, not a second value — and this is why `sci:rem:onecla`'s caution holds); `sci:rem:trap` (the bootstrap trap is a vanishing gradient; initialization and exploration-as-a-disjunct both inside the clause language); `sci:prop:nobase`; the forager example from the PLN paper; `sci:prin:division`; the three base operators named (recombination, composition, death) with pointers.
8. **§6.3** — strategies labeled as *base* policies; the fibre runs continuously between base moves.
9. **§13.3 Sampling** — weight map → clauses/transfer operator, with the linear-fragment objects named as such.
10. **§14 audit table** — "the weight map" → "the clause values".
11. **§15** — "weight maps" → "assignments of clause values" (×2); `sci:prop:section` (the withdrawn canonical section) re-pointed to ch. 13 and reworded for clauses; a sentence identifying $\Dec(G)$ with the fibre of `sci:prop:hypfib` taken over every receipt.
12. **§17.1** — "Nothing … is a learning rule in the gradient sense" qualified: nothing *that moves in the base*; the one gradient the scientist has runs over its own clause values and by `sci:prop:nobase` never changes what it proposes to test. The slogan *The network proposes. The ecology is the epistemology* is untouched.
13. **§18** — `ch:weight` → `ch:grading`.

**One pre-existing inconsistency found and fixed in passing.** §15's `sci:prop:fibration` (ecologies as a fibration) already existed; my new hypothesis-space proposition collided with it on first build and was renamed `sci:prop:hypfib`. The two fibrations are different objects (ecologies over theories vs. graded hypotheses over shapes) and the new sentence in §15 says how they relate.

**ch. 24 *Composing Learners*** — five sites. Two of them were stale **before** this pass: l. 159 said the fibration has "a canonical section given by taking cost as the weight map", and l. 287 asked whether "the canonical section … is monoidal" — but ch. 22 §15.2 withdrew canonicity in draft17 (`sci:prop:section` is a Remark titled *Cost is not already a weighting*). Both now say there is no canonical section and that laxity fails for *every* monotone cost-to-value section. The other three are vocabulary.

**ch. 55 origins_ch05** — "broken symmetry in the weight map" → in the clause values, glossed.

US spelling: *artefact* ×3, *modeller* ×2, *regulariser*, *Equalises* ×2 converted in ch. 22/24 (pre-existing residue; `lgrey` is a color name and stays).

---

## B2 — ch. 14, *Interventions at the Cut* (`\label{ch:do}`, namespace `do:`, "we" register)

From `fuzzyware/interventions-at-the-cut.tex`, cut to book length (10.5k → 5.4k words). Sections:

1. **What is being built, and why here** — the program already carries a causal model; the operator; why the cut (aperture is a property of the cut; the halting argument makes it a policy conditional on an event); conservative by `gw:thm:dnf`; why directly after Grading (first thing a clause can do that a table cannot; two downstream consumers).
2. **Three rungs** — Pearl introduced for the first time in the book: the ladder, `do:def:cbn`, SCM vs CBN, and the observation that the ladder is a statement about *time* (trace / branching / replay), which is why the chapter is in Machinery.
3. **Interventions have one carrier** — `do:def:intervention`, `do:prop:transparent`, `do:prop:nocontext` (the aperture principle in the forcing direction), the timing qualification, `do:prin:metered`.
4. **The construct** — `do:def:do`, listing, `do:def:membrane`, `do:def:lowering`, `do:prop:lowering`, `do:obs:restrict`, `do:prop:dnf` (compressed do = weight map; composition), specification-into-guard.
5. **The embedding of Pearl's calculus** — fan-out, `do:def:gadget` with the rholang listing, `do:lem:staging`, `do:thm:adequacy`, `do:rmk:shor` (stochastic twin of the one-qubit gadget of ch. 35), `do:thm:surgery` with proof, `do:rmk:positivity` (positivity is starvation — and $\bot$ of `sci:prop:trichotomy`), `do:cor:rules` (the three rules derived), **§Counterfactuals** (`do:def:record`, `do:thm:cf` with proof; the history monad read as the third rung waiting to be used), the dictionary table with σ-interventions and conditioning glossed.
6. **What it costs, and the fragment that tames it** — the three sources of expense + vacuity as reachability; the four disciplines (staged / flat / finite-domain / confined) each identified with where the book already drew it; `do:prop:practical`; what is lost outside, ending on "an experiment rather than a surgery, which is what the scientist does to its environment all day."
7. **Execution, briefly** — four facts about f1r3node (lowering in the elaboration pass; the membrane as twin of the cost membrane; pricing; counterfactuals from the validator's record), and the experimenter/spectator reading of execute-vs-replay.
8. **What this does for the scientist** — rung one-and-a-half; the do gives rung two over owned code; a clause under a do is a hypothesis with its experiment; a germ carrying both is an inherited hedge — then the hand-off to the Prestige chapter (B3, draft30): confluent systems state but cannot inhabit; vending machines; "This one is where the thing it leans on is built."
9. **What is owed** — five items (d-separation as a behavioral formula; rules off staging; the price of a do; identifiability across namespaces; continuous domains and cycles).

**Dropped from the note as research apparatus:** the full σ-calculus section (kept as one glossed table row + one paragraph), the four-fragment × four-discipline table (kept as prose), the complexity section's numbered fragments $\mathcal F_0$–$\mathcal F_3$ (kept as the practical fragment's hypotheses in words), the GPU kernel paragraph, the Hygiene remark, the cross-site exclusion remark.

**Chosen placement:** ch. 14, between Grading and Cost-Accounted Rho. The alternative (Part IV beside the causality chapters) was declined on the ground that the chapter is a *construct and an embedding theorem*, not a physics; it cites `ch:carho` forward for the cost membrane (3 refs), which is fine since `ch:carho` is the very next chapter.

---

## Also in this delivery

**ch. 8 `turn_gslt_ch01` §*Organization of this part* was stale on three counts before this pass** — it omitted Grading (since draft16), Scopes, and Instruments, and said Hypercube "closes the part" (false since draft17). Rewritten: Grading and Interventions described, Scopes added, Instruments named as the closer.

**`finding_mind.tex`** — +5 fibre macros (`\gstr` $=s$, `\gev` $=n$, `\gconf` $=c$, `\Fish`, `\Score`) and +5 do macros (`\dok`, `\ink`, `\fork`, `\where`, `\dsep`), all checked for collisions (`\str` exists as $\mathrm{str}$ — hence `\gstr`); `\input{TheTurn/turn_gslt_ch13}` after ch11; Part I summary gains a sentence on the do chapter.

**Bibliography +7:** `Pearl95`, `Pearl09`, `CorreaBareinboim`, `ShpitserPearl`, `VaraccaVolzerWinskel`, `AbbesBenveniste`, `Interventions` (the working note). `GradedPLN` (added in draft27) is now cited three times in ch. 22.

---

## Tracked inconsistencies — status

| # | Item | Status |
|---|---|---|
| T1 | ch. 22 Mortal Scientist in weight-map vocabulary | **closed** (this delta) |
| T2 | ch. 24 Composing Learners "cost as the weight map" | **closed** (this delta), plus the two pre-existing canonicity stalenesses |
| T3 | Part I summary "an intervention is a clause" with no do chapter | **closed** (this delta) |
| T4 | ch. 13 §13.6 "the store … to which this book returns" — no Realization part yet | open → draft29 |
| T5 | front matter (`pledge_map`, `pledge_related`) not updated for Grading/do/v6 | open → draft30 |
| T6 | ch. 55 origins_ch05 weight-map mention | **closed** (this delta) |
| **T7 (new)** | ch. 14 §14.7 *Execution, briefly* and §14.8 describe f1r3node's elaboration pass, membrane, and execute/replay model before the book has introduced f1r3node | open → draft29 (Section A will carry the introduction; §14.7's first sentence already promises it) |
| **T8 (new)** | ch. 14 §14.8 and ch. 22 §17.1 hand off to a Prestige chapter on inhabiting choice that does not exist yet | open → draft30 (B3) |
| **T9 (new)** | ch. 23 *Learning to Play* l. 1212 still says "Gillespie construction of Chapter~\ref{ch:weight}" — resolves correctly (label kept) and is true; vocabulary only | cosmetic; sweep in draft30 |

---

## Three things you should look at

1. **§22.6.2's placement of the PLN results in the Mortal Scientist rather than in Grading** (your answer 4). It works, but it makes ch. 22 — already the longest chapter at 24.6k words — about 1.9k words longer, and the fibre material is the first time the chapter proves a theorem with a Fisher information in it. If it reads as a change of register, the whole of §22.6.2 lifts out as a short chapter between Grading and Interventions without touching anything else.
2. **ch. 14 §14.8's claim** that "a germ carrying [a clause under a do] is a hedge that is inherited, since a do is a term and crossover on quoted code applies to it unchanged" is the interventions note's claim carried over, and ch. 22 §12 (Reproduction) has not been edited to receive it. It is true as stated — a `do` is syntax and `@` quotes syntax — but §12 should eventually say so in its own voice. Not done here; flagged.
3. **The ladder-is-time paragraph in ch. 14 §14.2** ("association is linear time, intervention is branching time, counterfactual is replay") is stated as the alignment the inhabiting-choice note makes "by argument," and I have written it as an observation rather than a result. The note itself calls the alignment "exact"; the survey flagged that as asserted rather than shown. The book's wording is the cautious one.
