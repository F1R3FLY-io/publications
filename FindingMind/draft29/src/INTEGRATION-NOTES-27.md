# Integration notes — draft29

**Section A: the Realization part. A new inner part of the Turn — *Realization: From the Calculus to the Machine* — between Machinery and Ecology, three chapters. 814 pp → 846 pp. 0 errors, 0 undefined references or citations, 0 multiply-defined labels. Same 9 overfull boxes as draft26–28 at identical widths.**

Built against the repo as committed (draft27 and draft28 under `draftN/src/`, byte-identical to what was delivered — verified before starting). Clean-room: repo baseline + draft2–draft28 (reading `draftN/` and `draftN/src/`) + draft29 → 846 pp.

---

## Manifest (11 files + this note + PDF)

| File | Destination | Status | Words | md5 (first 8) |
|---|---|---|---|---|
| `turn_real_ch01.tex` | `TheTurn/` | **new** — ch. 22, *The Store Is the Denotation* | 3043 | bf104e35 |
| `turn_real_ch02.tex` | `TheTurn/` | **new** — ch. 23, *Names Without Names: Combinators and the Matrix Machine* | 4213 | 33c0680b |
| `turn_real_ch03.tex` | `TheTurn/` | **new** — ch. 24, *What the Machine Prices* | 2094 | 6539a7c2 |
| `finding_mind.tex` | root | new `\part` + opener box + 3 `\input`s; Overarching Introduction: new Part II summary, Parts III–VII renumbered, figure, routes, confidence paragraph; Ecology opener box | 7688 | f87b2155 |
| `turn_gslt_ch11.tex` | `TheTurn/` | 1 sentence — closes **T4** (supersedes draft28's copy) | 8383 | d1ff5d2c |
| `turn_gslt_ch13.tex` | `TheTurn/` | 2 sentences — closes **T7** (supersedes draft28's copy) | 5374 | 7df556e0 |
| `turn_gslt_ch12.tex` | `TheTurn/` | 1 phrase — "the next part" → "the ecology part" (Instruments opener; would have been false) | 5653 | bb3a85c0 |
| `turn_conclusion_ch01.tex` | `TheTurn/` | "six parts" → seven; new paragraph for Part II; Grading/do sentence added to the Part I paragraph | 1502 | 54d10153 |
| `turn_conclusion_ch03.tex` | `TheTurn/` | "six parts" → seven | 355 | 07d2c009 |
| `prestige_apr_ch01.tex` | `ThePrestige/` | `ndo:sec:rspace` now points **backward** to ch. 22 instead of introducing RSpace cold; 2 spellings | 4320 | ecd2411a |
| `bibliography.tex` | root | +3 (287 → 290): `rhocomb`, `rhocomb3`, `rhocombmat` | 5283 | 41ed154e |
| `finding_mind.pdf` | — | rebuilt, 846 pp | | |

**Numbering.** Three chapters inserted at 22–24; everything from the Mortal Scientist on shifts +3 (Mortal Scientist 25, Spiking 29, Tubular Agents 30, Suffering 31, Complex Amplitudes 38, Kant 79). The Turn's inner parts: Machinery I, **Realization II**, Ecology III, Suffering IV, Situating V, Misled VI, Origins VII (prose numbering); the printed LaTeX part numbers shift by one as well (Realization prints as IV; Prestige's halves will print XI/XII). No hardcoded chapter numbers; `\ref`-safe.

---

## Decisions applied

- **Placement in the Turn** (your Nov 8 note: "from the point of view of the magic trick architecture, the implementation work goes in the Turn"), as a new inner part directly after Machinery — the recommended option.
- **Three chapters**, not two.
- **GPU claim in the width-thesis form**, as the matrix note states it, with the reconciliation you asked about made explicit in ch. 24 §24.2: near-linear scaling in processors is what the thesis predicts *where width is large*, and only there.
- **Saph1r3's admitted gaps reported in the book**, in the repository's own terms (ch. 23 §23.7 and `cb:rmk:nascent`); **RSpace reported as mature** and the formal model's limits reported too (`rs:rmk:gap`, `rs:sec:knows`). Your instruction — "perfectly reasonable to be honest that the Saph1r3 work is very nascent while the RSpace work is quite mature" — is the organizing principle of the part; both the part box and each chapter's closing remark separate the registers (implemented-mature / proved / implemented-nascent / conjectured-gating / direction).

---

## The three chapters

**ch. 22 *The Store Is the Denotation*** (`ch:rspace`, `rs:`). §Two senses of "runs" (the Pledge's promise; the two installments and their maturities; a word about voice — the book becomes a report, not a manual). §The store: `rs:def:table`, `rs:def:rspace`, **`rs:prop:hom`** with proof (counts are kept), reflection validated by the color-swap involution *described in one paragraph and not rehearsed*, `rs:def:zip` (context labels are RSpace puts), `rs:rmk:bialgebra` (store monad + zip; bialgebra laws filed as an obligation). §Two layers: $\pi$ from tables to behaviors, `rs:def:resbis`, the two full-abstraction theorems **stated at the source's confidence — sketches with obligations listed**; why two layers (the machine vs. the meaning; the multiplicity factor as this quotient refused); **`rs:rmk:gap`** (the formal model covers monadic single-channel input with no patterns; joins unmodelled). §The store as built: `rspace++`'s interface (the where-clause already reaches the matcher as a guard), matching as the zip with patterns and a *seeded* shuffle, hot store → Blake2b Merkle radix trie in LMDB (= the history monad with a hash), `ReplayRSpace`, `ChargingRSpace` (= cost monad running; order-independent charging had to exist before the book derived it; tied to `wt:prop:gate`), the merger (= the contention relation's first appearance in code). §What the machine knows that the calculus does not: **locked not lock-free** (the architecture doc's claim corrected against the code: 4096 striped mutexes), the blocked same-channel joins with their issue numbers, the measured bottleneck (replay pipeline, not disk), and accelerators listed as a non-goal — the hinge sentence from the matrix note. `rs:rmk:claims` sorts every claim into four registers.

**ch. 23 *Names Without Names*** (`ch:combinators`, `cb:`). §Status, first. §A calculus with no bound names: the nine atoms and the rules (Figure 23.1), **`cb:prop:defect`** with proof (the published calculus is unsound: drop conflated with the guard), reflection half-implemented and constructors completing it, the storage combinator as opacity, **`cb:prop:blowup`** (succinctness; $O(|P|)$), full abstraction relative to encoded contexts (restriction forced, as for π→rho; proofs sketched), the expressiveness lattice. §States are vectors, rules are a matrix: what an accelerator rewards vs. what a tuple-space interpreter does; the four structural facts; `cb:def:species`, **`cb:prin:free`** (= `rs:prop:hom` said a second way), `cb:def:operator`, **`cb:prop:massaction`** (a combinator program is a chemical reaction network; the weighted reading attaches at one diagonal factor — so the Gillespie reading and the matrix machine are one object), the self-indexing domain equation, `cb:def:intern`, **`cb:thm:three`** (the lattice's three invariants are the matrix's three finiteness conditions), `cb:rmk:evthrice` (one atom, three characterizations), *do not ask for* upper triangularity (right for model checking, wrong for execution). §What actually gets multiplied: join / second product / independent set; steps count causal depth; `cb:prin:reflection` (reflection is an intern table). §What has been measured, and the thesis: step counts and widths on four synthetic families; **`cb:prin:width`**; the three non-claims and the GATE in the source's voice. §The compiler, as it stands: the eight-crate pipeline, the wgpu target, the 48 tests described specifically, the software driver, no performance target; **the build's own findings against its specs** (hole indices, address supply, quoted units) reported as findings; **`cb:rmk:nascent`** — the store-backed machine unbuilt, no browser build, the curried scheme's reach, the dependency discrepancy, and the unmeasured width, closing: "a hypothesis has been installed as a guard, the assay has been designed, and no message has yet arrived on the probe channel." §Two realizations of one rule.

**ch. 24 *What the Machine Prices*** (`ch:price`, `pr:`). §The GPU does not speed up; it widens (depth vs. layers; the honest form of the datacenter claim). `pr:rmk:ledgers` (per-event vs. per-layer: one cost in two coordinate systems; the wall-clock bill). §Where the two philosophies disagree, priced: structural congruence, resolution (fairness of maximal progress is an open obligation; locks are what the store pays to be fair), reflection (unbounded vs. generated), state (the machine has no Merkle root yet — the one feature that makes f1r3node a shard). **§Execute, replay, and the three rungs** — the shard's execution model introduced in its own right (proposer/validators; order-then-execute; roots, cost, log), read as an experimenter/spectator split: proposer on rung two, validators on rung one, fork on rung three; and the payback — **one instrumentation serves validation, the Mortal Scientist's gradient (`sci:thm:score`) and `do`'s counterfactual replay (`do:thm:cf`)**. §The network is the computer, after all — the Pledge's telescope (§`sec:pledge-i`, §`sec:pledge-viii`): the general machine we had, the wide chip built for something else, and the chip built for this "if the width thesis survives its measurement" — explicitly a direction, not a description. `pr:rmk:claims`.

---

## Reconciliations

- **Overarching Introduction** (`finding_mind.tex`): new `\subsection*{Part II Realization}` summary; Parts III–VII renumbered in headers and in the four prose cross-references ("The learner of Part~III", "paying for Part~IV", "Part~V described the world", "the medium tower of Part~III"); dependency figure gains a `Part II Realization` box to the right of Machinery with arrow "the store, the matrix"; routes paragraph rewritten (an engineer's sentence added: "Part II is needed by none of the arguments and is the only part the engineer must read"); confidence paragraph gains "Part II is a report, and says on every page which of its two machines is mature and which is months old."
- **Ecology opener box**: "The preceding part built an apparatus and no one to use it" → "The first two parts built an apparatus, showed that it runs, and gave it no one to use it."
- **Instruments (ch. 20) opener**: "everything the next part builds on it" would now point at Realization; → "the ecology part."
- **Conclusion ch. 1**: "six parts" → seven; a new Part II paragraph; the Part I paragraph gains the Grading/do sentence it was missing since draft27/28 (a small pre-existing staleness).
- **Prestige ch. *Not Every Cut Is an Aperture*** `ndo:sec:rspace`: RSpace is now *recalled* from ch. 22 with refs to `rs:prop:hom` and `rs:def:zip`, instead of being introduced at p. ~640. The chapter's own argument is unchanged.
- The relative-position grep was run on the three new chapters and all neighbors; the only hit that had become false was the Instruments one above.

US spelling: the three new chapters are clean by construction; `towards` (Conclusion), `relativise(s/d)` ×2 (prestige_apr) converted.

---

## Tracked inconsistencies — status

| # | Item | Status |
|---|---|---|
| T4 | ch. 13 §13.6 RSpace promise | **closed** (→ `Part~\ref{part:realization}`) |
| T7 | ch. 14 §14.7 f1r3node execution model before its introduction | **closed** (→ Part II; §14.8's observation now also points at `ch:price` where it is developed) |
| T5 | front matter (`pledge_map`, `pledge_related`) | open → draft30. **Grew**: `pledge_map` says "The Turn is divided into six parts" (l. 57) and documents the printed/prose part-number mismatch as "III through VIII" / "X and XI" (ll. 137–141); now seven parts, IV–IX printed, XI/XII for the Prestige. Also the six-routes section wants an engineer's route, and the unproved list wants: `conj:global`, `conj:safe`, the recursion obligation (ch. 38), the bialgebra laws and eager confluence (ch. 22), the width thesis (ch. 23), fairness of maximal progress, the price of a `do` (ch. 14), the σ-rules off staging. |
| T8 | hand-off to the Inhabiting Choice chapter (ch. 14 §14.8, ch. 25 §17.1, and now ch. 24 §24.4 implicitly) | open → draft30 (B3) |
| T9 | ch. 26 *Learning to Play* l. 1212 "Chapter~\ref{ch:weight}" vocabulary | cosmetic → draft30 |
| **T10 (new)** | Hardcoded Roman part numbers in *body* prose that predate this iteration and were already inconsistent with the live numbering — `turn_gslt_ch02/03/04/05/10` say "Part~III" for the Machinery constructions, `turn_origins_*` and `turn_sciencebot_*` say "Part~I/II" in the arXiv paper's numbering. **Not touched.** They were stale before draft29 and the part insertion makes them no more wrong than they were; they want a `\ref{part:...}` sweep in draft30 or later. Listed so the grep does not read them as mine. |

---

## Three things you should look at

1. **ch. 24 §24.4, "a distributed ledger is, by design, a machine that separates the three rungs."** The proposer-resolves / validators-observe / fork-is-counterfactual reading is the interventions note's observation carried into the book and developed one step further (the "one instrumentation serves three masters" paragraph is mine). It is the most quotable thing in the part and should be checked against how order-then-execute actually records losers — the note says they are recomputable from the pre-state, and I have repeated that.
2. **ch. 22 §22.5, "locked, not lock-free."** The book now corrects the architecture document against the code. That is the right thing for the book's ethic, but it is a statement about a repository at a date; if the doc is fixed or the implementation changes, the sentence should go.
3. **ch. 23 §23.6, the third realization — the chip.** I wrote it as "a die on which the COMM rule is the instruction … for which the matrix machine is the software model and the store is the specification," and called it a direction. If the rho-ISA work has a public shape I should know about, this paragraph is where it would go; as written it commits the book to nothing.

Smaller: the Realization part prints as Part IV (LaTeX counter) while the prose calls it Part II — the same mismatch the map already documents for the other parts, to be re-documented in draft30. ch. 22's Definition 22.2 is set as a three-line `align*` after the first build produced a 47pt overfull box in the single-line form.

Next: draft30 = B3 *Inhabiting Choice* in the Prestige before Kant + front matter (T5) + site regeneration.
