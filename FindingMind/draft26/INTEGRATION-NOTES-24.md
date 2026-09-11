# Integration notes — draft24

**Both `developing-ideas` research notes integrated, plus the Goodman retrospect. 749 pp → 779 pp. 0 errors, 0 undefined references, 0 multiply-defined labels, 0 missing citations.**

Baseline verified before any edit: base tree + draft2–draft23 overlaid reproduces draft23's 749 pages exactly.

---

## Files in this delivery (6 + this note)

| File | Status |
|---|---|
| `turn_rholife_ch06.tex` | **new** — Ch. 26, *Tubular Agents* (Part IV closer) |
| `prestige_lie_ch01.tex` | **new** — Ch. 66, *On Lying as the Origin of Meaning* (Part XI) |
| `prestige_sim_ch01.tex` | modified — one sentence in the opening |
| `prestige_ch02.tex` | modified — one forward-pointing paragraph in `sec:failures` |
| `finding_mind.tex` | modified — 2 `\input`s, 4 prose reconciliations |
| `bibliography.tex` | modified — 32 new entries (242 → 274) |
| `finding_mind.pdf` | rebuilt, 779 pp |

Destinations: `TheTurn/turn_rholife_ch06.tex`, `ThePrestige/prestige_lie_ch01.tex`, `ThePrestige/prestige_sim_ch01.tex`, `ThePrestige/prestige_ch02.tex`.

---

## Placement

**Ch. 26, *Tubular Agents*** (`\label{ch:tub}`) — closes Part IV, after ch. 25 (spiking), pp. 387–394.

**Ch. 66, *On Lying as the Origin of Meaning*** (`\label{ch:lie}`) — Part XI, immediately after ch. 65 (*What a Symbol System Would Have to Be*), before ch. 67 (*If This Is a Simulation*), pp. 675–682.

Renumbering: everything after ch. 25 shifts +1, then everything after ch. 65 shifts +1 again. Kant is now ch. 70. No hardcoded chapter numbers exist in the source, so this is `\ref`-safe throughout.

---

## Decisions applied

1. Chapter after ch. 64 (now 65), not a section inside it.
2. Titles as given: *Tubular Agents*, *On Lying as the Origin of Meaning*.
3. `§Filing` and `§What to write down formally` dropped as research-note apparatus. The tube note's formal-targets section is kept but rewritten as §26.5 *What would have to be written down*, which is forward-looking rather than filing instructions.
4. All citations added — see below.
5. Keys disambiguated: `lewis1969` (David Lewis, against the existing `lewis1938`, C. S. Lewis); `wheelerbc2009/2010/2013` (B. C. Wheeler, against the existing `wheeler1990`, John Archibald Wheeler).
6. Lowercase `i` throughout the Prestige chapter; `we` in the Turn chapter, matching Part IV.

---

## Register

Per your scope note, both chapters state most of their content as **conjecture** rather than proposition, using the book's existing `conjecture` environment, and both close with an explicit *What is claimed / Not claimed* section separating the two. Ch. 26 argues two propositions and six conjectures; ch. 66 argues two observations and seven conjectures. Nothing is dressed up as settled.

## The new material in ch. 26

Your direction — autotroph/heterotroph reread as communities of learners, communities serving the boundary namespaces — is §26.3 *Organs are communities*, and it is the chapter's centre rather than an addition to it. Three things came out of it that the source note did not have:

- **Observation 26.4 (The boundary namespace can be let).** A population occupying `Chn_∂(B)` is neither inside `B` nor foreign to it. A digestive organ is a community tenanted in a boundary namespace, and the gradient of conversions along a gut is a succession running in space rather than time.
- **It dissolves the Coase objection ch. 24 records.** A market inside a firm is awkward; a market in the lumen is not, because by the lumen observation the lumen is not the inside. The firm has an interior with no market and a lumen with one, and they are different namespaces.
- **The mycorrhizal network becomes the exact complement of the gut** rather than its analogue — the same rented community in the same ownership relation, with the topology turned inside out. A leaf and a root system are what an autotroph buys instead of a tube.

The chapter also makes explicit that Proposition 26.2 (autotrophs are not tubes) and Corollary 21.x (intelligence is a heterotroph's expense) are one sentence about one access profile: *plants do not need brains* and *plants are not tubes* for one reason, not two.

## Citations added (18)

**Ch. 26 (6).** `amdahl1967` · `stevenshume1995` · `yaguchi2024` · `duquecorrea2021` · `karasov2013` · `hirakawa2001`

Two are worth flagging as better than expected:

- **Yaguchi et al., *Nat. Commun.* 2024.** In sea urchin larvae the pylorus and anus are under light-modulated neural control by *different wavelengths through different transmitter systems*, so that it is very rare for both to be open at once. That is Proposition 26.1's mutual exclusion implemented literally, and the chapter says so: the framework predicts an exclusion on a shared name and predicts that a through-gut must pay to approximate one — a sphincter pair is what paying looks like.
- **Duque-Correa et al., *Proc. R. Soc. B* 2021.** The gut-length-tracks-diet claim is corroborated across 519 mammal species but is *messier* than the note assumed: the effect sits in the large intestine, is inseparable from phylogeny, and the authors decline to call it a fixed law. The chapter reports this against itself rather than for itself, and notes that the scatter is what Proposition 21.x (the overlap is populated by composites) would lead one to expect.

**Ch. 66 (12).** `benedikt1991` · `burroughs1962` · `dawkinskrebs1978` · `flower2011` · `flower2014` · `lewis1969` · `msharper2003` · `munn1986` · `skyrms2010` · `wheelerbc2009` · `wheelerbc2010` · `wheelerbc2013`

---

## Prose reconciliations (the part a green build does not catch)

1. **Part IV opener box** — a sixth chapter added to the "four sizes / a fifth" enumeration.
2. **Part IV opener essay** — a paragraph added for the closer, describing it as deliberately more suggestion than proof.
3. **Part IV opener essay, pre-existing miss repaired.** It read *"the tower of media built in the last chapter is what Part VIII bounds by the assembly index."* The tower of media is ch. 24; the last chapter of Part IV has been the spiking chapter since draft16, so this had been stale for eight drafts and the new closer would have made it worse. Now a `\ref` to `ch:eng`.
4. **Part XI opener box** — a clause added for the new chapter.
5. **`prestige_sim_ch01.tex` opening** — *"The last chapter asked what a symbol system would have to be"* → `Chapter~\ref{ch:notation}`, since the last chapter is now ch. 66.

Verified untouched and still true: ch. 25's *"The four preceding chapters built learners at four sizes"* (the closer follows it, which is why this placement was cheaper); ch. 68's *"the last thing the Prestige does before it goes back for Kant"*; ch. 68 → ch. 69 → Kant. Grepped all six adjacent chapter files for relative-position language.

---

## Two things you should look at

**1. Two open questions in the source note referenced material that is not in the book.** The gap note's last two questions turned on open-source game theory and on the collapse result for two open-source bots that are extensionally identical on the decidable fragment. Neither appears anywhere in *Finding Mind* — they belong to the wider research corpus. I merged them into a single question stated entirely in the book's own vocabulary (the three grades of access, Table 1 of ch. 21), keeping the payoff: *deception is engineering the opponent into the undecided region and harvesting their default*. If you want the bot collapse in the book, it needs a home of its own first.

**2. Ch. 65's closing hands off to ch. 66 better than planned.** Its final *Not claimed* item is that the reference bootstrap problem is soluble by construction — "nature's two solutions were achieved by populations, under selection, over long timescales… and this book has no argument that any of those conditions can be dispensed with." Ch. 66 now opens directly onto that. No edit was needed; worth knowing the seam is load-bearing if either chapter moves again.

## Terminology conversions applied

British → US throughout (≈8 forms per note). `engine` → `enzyme` (7 occurrences, ch. 26). `colour` → `flavor` (2 occurrences, ch. 26). Note titles *The Tube* / *The Gap* retired in favour of the chapter titles you chose.


---

# Addendum — §66.4 *Goodman, read backwards*

New section in `prestige_lie_ch01.tex`, placed after *Three stages, and a re-typing* and before *Statements to attempt*, per the retrospective framing. Chapter 66 is now 4,324 words; the book is 772 pp. Six subsections:

- **§*A perfect notation that means nothing*.** Runs Goodman's five conditions on a discrete reflexive population. All five hold; unambiguity holds maximally, because the weld *is* what Condition 65.3 asks for. Observation 66.3: notationality is maximised where reference is absent. Stated as sharpening `rem:invariance` rather than contradicting it — that remark says the conditions cannot *select* a compliance relation; this says they are best satisfied where none was selected, because the world wired it.
- **§*Compliance requires possible non-compliance*.** The stronger reading. A trigger cannot fail, so there is no compliance relation at Stage 0, only causation described twice. Observation 66.4 gives the arrow-count version: Goodman's roundtrip needs two arrows and Stage 0 has one, so the lie is what makes the roundtrip a roundtrip.
- **§*Deception attacks what the budget cannot repair*.** Table 66.1 sets the five conditions against Stage 0, against the post-lie state, and against who repairs them. `conj:fd` covers the two finite-differentiation conditions; the lie and mimicry cover unambiguity and the two disjointness conditions. The division is exact and neither account touches the other's. Mimicry is flagged as a *syntactic* attack — a drongo emitting a babbler's call has put one mark into two characters.
- **§*What the ethologists have been measuring*.** The perception criterion for functional reference is Condition 65.3 tested from the receiver's side; the counterdeception results are a measurement of it failing. Second hit: the quantitative vervet reanalysis separates leopard/eagle/snake calls cleanly but finds overlap once *aggressive-context* calls are included — syntactic disjointness failing exactly where strategic use enters.
- **§*The objection, which i cannot dispose of*.** The deflationary reading at full strength, with the reply, and the admission that the reply rests on a claim currently carrying one paragraph of intuition.
- **§*A prediction the pairing generates*.** Conjecture 66.1, deception as a force for digitality.

**One departure from what I proposed, flagged for your call.** I said the digitality prediction would move out into the conjecture list. Writing it, keeping it as the section's capstone read better — it is the claim the *pairing* generates, so divorcing it from the derivation costs more than the tidiness gains, and ending on a forward-looking prediction beats ending on the objection. The cost is numbering: it becomes **Conjecture 66.1**, ahead of the seven in *Statements to attempt* that it forward-references. Moving it is a one-block cut-and-paste if you disagree.

**Edits elsewhere.** A new `\paragraph{A fourth failure mode, deferred.}` at the end of `sec:failures` in ch. 65, so a reader who stops there is told the finding exists and where it is. That is the only change to ch. 65. A three-sentence lead-in now opens *Statements to attempt*, since the section before it ends on a conjecture.

**Three citations added:** `macedonia1993` (the two criteria for functional reference), `seyfarth1980` (the classic vervet result), `price2015` (the quantitative reanalysis).

Build: 0 errors, 0 undefined references, 0 multiply-defined labels, 0 missing citations; all 57 cross-references in the new section render with real numbers.


---

# Addendum 2 — ch. 26 rewritten against the 11 September note

`TubularAgents.tex` grew from ~1,870 to ~4,900 body words, with two new sections and eleven new citations. The chapter is rewritten to match: **5,022 words, pp. 387–404** (was 8 pp.), book now **779 pp**. Build clean; 103 cross-references in the chapter, none broken.

## What changed, and why it matters

**The chapter now has a criterion instead of an intuition.** Definitions 26.1 (occupancy ratio Λ = τ_proc/τ_enc) and 26.2 (separability μ) and Proposition 26.2, *the tube criterion*: a tube is forced when the agent is phagotrophic **and** Λ ≳ 1 **and** μ ≈ 0; failure of any one clause is sufficient for a bag. This subsumes *Two ends, not one* rather than replacing it — the mutual exclusion is what the agent pays, Λ is how much of the cycle it consumes, μ is whether it can be evaded at the aperture. It also connects to the serialization conjecture concretely: σ = Λ/(1+Λ), so the Amdahl bound bites only as Λ passes unity.

**Respiration is now handled, and it strengthens the argument.** Unidirectional pulmonary flow in alligators and the savannah monitor, with Farmer's inference that the trait is ancestral for diapsids and therefore *not* a high-rate adaptation, kills the rate explanation cleanly. Breathing is a bag because μ ≈ 1: the mammalian lung tolerates six-parts-spent-to-one-fresh re-mixing every cycle and works anyway.

**§26.7 *The test set*** — Mike Stay's seven proposed counterexamples, attributed in text (the book has no acknowledgements section; it credits in place). Table 26.1 sorts them: one factually wrong, four predicted, two scope corrections.

**Two things in that section are worth your eye.** Ctenophores were the proposed counterexample and they *have* a through-gut; since they branch very deep in Metazoa, that turns a threatened counterexample into the chapter's strongest support — the construction is arrived at from an access profile, not inherited from the bilaterian body plan. And Pelagia ephyrae fill the gut in ~15 minutes against ~18 hours of digestion, read by the authors as forcing diel periodicity: that is *Two ends, not one* observed in one organism with the ratio measured.

**A correction propagated.** The old Proposition *Autotrophs are not tubes* is now **Non-phagotrophs are not tubes**, with the chapter saying plainly that the earlier statement was too narrow and that the test set is what forced it. Cestodes are the case that did it: animal, bilaterian, heterotroph, descended from through-gut ancestors, no gut at all — unaccommodatable on any heterotrophy formulation.

**Observation 26.5, *Osmotrophs evert the lumen*,** now discharges something §26.3 could only gesture at. Last draft, §*Organs are communities* claimed the mycorrhizal network is the gut's exact complement. That claim now has a mechanism — tenancy without enclosure, versus enclosure plus tenancy — so §26.3 ends by naming the question it cannot yet answer and pointing at 26.5, and 26.5 answers it. Internalising buys exclusivity over a partially extracted harvest; externalising surrenders it and never transports residue. An osmotroph took the other side of that trade and should be predictably vulnerable to theft of extracellular product.

**Priority stated.** Hejnol and Martín-Durán already note in passing that a one-way gut permits uptake while still digesting. The chapter says so and narrows its claim to the derivation and the threshold rather than the observation.

**Two new open questions** (osmotroph/phagotroph as a framework-native split; a second critical point at high Λ, and whether the framework predicts ruminant chamber count), and one new formal target flagged as the most valuable on the list: recovering Λ from the harvest term, at which point the criterion becomes an inequality between two quantities already metered.

## Register

*What is claimed* now has three tiers rather than two, because the criterion needed one. Observations and *Two ends* are claimed; the criterion is claimed **with a different kind of support** — not derived, backed by a test set it survived; the six conjectures are not claimed. That seemed the honest way to record a claim whose evidence is that the first attempt to kill it failed.

## Citations added (11)

`farmer2015` · `farmersanders2010` · `schachner2014` · `presnell2016` · `giribet2016` · `ishii2001` · `gordoa2013` · `hejnol2015` · `jangoux1982` · `leake1994` · `bidartondo2005`

## Prose reconciliation

The Part IV opener essay described the closer as "deliberately more suggestion than proof," which was accurate last draft and is no longer. Rewritten to say the chapter is mostly conjecture but states its condition sharply enough to predict bags, and puts it to a test set. The opener box clause and ch. 25's *"four preceding chapters"* are unaffected.

Build: 0 errors, 0 undefined references, 0 multiply-defined labels, 0 missing citations.
