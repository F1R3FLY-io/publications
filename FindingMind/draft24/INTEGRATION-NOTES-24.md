# Integration notes — draft24

**Both `developing-ideas` research notes integrated. 749 pp → 766 pp. 0 errors, 0 undefined references, 0 multiply-defined labels, 0 missing citations.**

Baseline verified before any edit: base tree + draft2–draft23 overlaid reproduces draft23's 749 pages exactly.

---

## Files in this delivery (6 + this note)

| File | Status |
|---|---|
| `turn_rholife_ch06.tex` | **new** — Ch. 26, *Tubular Agents* (Part IV closer) |
| `prestige_lie_ch01.tex` | **new** — Ch. 66, *On Lying as the Origin of Meaning* (Part XI) |
| `prestige_sim_ch01.tex` | modified — one sentence in the opening |
| `finding_mind.tex` | modified — 2 `\input`s, 4 prose reconciliations |
| `bibliography.tex` | modified — 18 new entries (242 → 260) |
| `finding_mind.pdf` | rebuilt, 766 pp |

Destinations: `TheTurn/turn_rholife_ch06.tex`, `ThePrestige/prestige_lie_ch01.tex`, `ThePrestige/prestige_sim_ch01.tex`.

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
