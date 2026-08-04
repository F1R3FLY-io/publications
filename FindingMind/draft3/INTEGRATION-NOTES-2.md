# Reconciling non-determinism-as-oracle into the hypercomputation chapters

Build: **372 pages**, three passes, zero errors, zero undefined references, zero
undefined citations, two overfull boxes book-wide. The `sec:gaps` duplicate is still
there — pre-existing, still out of scope.

## Files

| File | Status |
|---|---|
| `TheTurn/turn_ndo_ch01.tex` | new — *Not Every Cut Is an Aperture* (ch. 39) |
| `TheTurn/turn_ndo_ch02.tex` | new — *Where Does Agency Live?* (ch. 40) |
| `TheTurn/turn_chs_ch01.tex` | new — *Choice Principles are the Types; Schedulers are the Terms* (ch. 41) |
| `TheTurn/turn_hypercomp_ch01.tex` | revised — consciousness as a coordinate; organisation |
| `TheTurn/turn_hypercomp_ch02.tex` | revised — new closing section, *Two Things This Construction Gets Wrong* |
| `TheTurn/turn_hypercomp_ch05.tex` | revised — the ordinal remark and the comparison table |
| `TheTurn/turn_hypercomp_ch07.tex` | revised — Gap 2 superseded and replaced |
| `TheTurn/turn_rholife_ch04.tex` | revised — one forward pointer into ch. 39 |
| `finding_mind.tex`, `bibliography.tex` | revised |

Placement: all three new chapters sit immediately after *The Hypercomputational
Fibration* and before *Strictly Richer Physics at Each Stage*, in the order
mechanism (39), thesis (40), type theory (41).

## How the two papers were reconciled

Agency carries the thesis; cuts-apertures supplies the instrument. Mechanism first,
thesis second.

**Chapter 39 (mechanism)** takes interaction and the RSpace bridge from agency, the
choice-as-resource section from cuts-apertures (the fuller of the two, and the one
that already contains "a lattice, not a tower"), the definition of cut and aperture
and the governing principle from agency, the macro-component and the frontier
coincidence from cuts-apertures, and agency's argument that the islands are small.
A new closing section, *Choice types, and the terms that inhabit an aperture*, is
written from `choice-scheduling.tex`.

**Chapter 40 (thesis)** takes boundaries, Smith, the graded factorisation, the flat
ontology, the crux, the open problems and the Bell coda from agency, and
*Stratification versus the jumble* from cuts-apertures.

### The Smith disagreement

Cuts-apertures called the biosphere a macro-component with a determinate interior;
agency reproduces the same display and says the interior runs the deepest races
biology has, so the two factorisations **cross rather than nest**. Agency's verdict is
the book's.

Cuts-apertures' reading survives as a remark — *An earlier reading, and why it does
not survive* — because the mistake is instructive and worth keeping. The failure is
that **conformance constraints are not confluence**: an enzyme's specificity says
which pairings are admissible and says nothing about which admissible pairing occurs,
and it is the second that the rigidity criterion asks about. Two predators and one
prey is fully type-correct and a live race. Cuts-apertures' own material about the
constraint-sensitive interior is what makes the remark land, so its observation is
preserved while its conclusion is reversed.

Six labels collided between the papers (`def:macro`, `prin:main`, `prop:frontier`,
`q:crux`, `sec:choice`, `sec:macro`, plus two "two senses of agent" remarks under
different names). One of each survives; both chapters share a single `ndo:` namespace
so references across them resolve without redirection.

## What changed in the existing chapters

**Consciousness is a coordinate, not a threshold.** Per your position: every agent
occupies a point in the lattice, and the point *is* its consciousness. The ch05 remark
is rewritten under that title and makes two things explicit the ordinal reading
obscured — that positions need not be comparable, and that an agent's coordinate is
simultaneously the threshold at which the world islands for it, so its consciousness
and the granularity of the world it inhabits are one fact stated twice. The comparison
table changed four rows: Type (lattice-valued position), Location (which point in the
base), Governed by (choice strength affordable), Changes (by moving in the lattice),
plus Physical basis and Survives projection.

**The tower stays as the introduction.** Ch01 now says so in as many words: the
capacity index is presented first as Turing's ordinals because that is how the
intuition is had, then replaced by the lattice it shadows. Ch02 gains a closing section
naming its own two errors — that the index should not be an ordinal, and that the
oracle should not be bolted on — and handing over.

**Gap 2 is superseded.** The old gap asked for a construction of the oracle as a
persistent receive. It is replaced by the metatheorem that actually matters: does the
term assignment satisfy subject reduction, or can a scheduler climb the stalk merely by
running? `choice-scheduling.tex` supplies the identification, and it is a good one —
the crux of agency and the metatheory of the correspondence are the same lemma, which
is the strongest evidence in the three papers that the correspondence is the right one.

## Cross-references to the Ecology part

Four, all load-bearing rather than decorative:

- **The trophic seam.** Chapter 40's Smith section now notes that the ecology chapters
  derived the same energy-eater/organism-eater distinction as an *access profile*, and
  that the agreement is not coincidence: a linear send receivable by either of two
  harvesters *is* a contended channel, and the foraging race is the aperture.
- **Confinement is a threshold.** The graded-factorisation section reads
  "poverty is confining" through the decoration — a learner whose budget affords only
  strength `C` sees a `C`-islanding and cannot formulate a hypothesis requiring finer
  distinctions. The ecology part's escape (move the unit from individual to lineage to
  composite) becomes raising `C` by acquiring a stronger scheduler.
- **Two criteria for a boundary.** A remark in chapter 39 puts the engine chapter's
  economic criterion (close where repair is cheaper than exposure) beside this part's
  semantic one (close where no internal cut carries observable choice) and observes
  that neither implies the other — being worth owning and being determinate are
  independent, and most organisms have the first without the second.
- A matching forward pointer in the engine chapter.

## Open items

1. **`\Cut` collided.** The engine chapter uses it for the cycle-space cut; the two new
   papers used it for parallel composition. The new chapters now use `\Par`.
2. **The Overarching Introduction and the Turn's Conclusion still describe consciousness
   as ordinal-valued** and still describe The Turn as four parts. Untouched, as before.
   This is now the only place in the book where the old framing survives.
3. **`consensus` is cited but not in the repo** — *From Coalition Logic to Rho
   Calculus*, which chapter 41 leans on for the pattern it instantiates and for the
   three-valued enabledness reading. A bibliography entry stands in for now.
4. `engine.rho` still outstanding, as you noted.

## Chapter 41 — choice-scheduling as a chapter

Added on the second pass. Sections: the two theories of nondeterminism (choice
declaratively, fairness operationally), the bridge that already exists in fragments
(Martin-Löf, Spector, Berardi–Bezem–Coquand, Krivine, Escardó–Oliva), the gap
(proofs-as-processes is confluent by design, so linear logic types the communication
skeleton and is silent on its resolution), the correspondence and its dictionary,
subject reduction as the no-bootstrap law, the synthesis pipeline, connections, and
seven open problems.

Consequential edits: the compact *Choice types* section written into chapter 39 on the
first pass has been trimmed to a two-consequence lead-in that hands off to 41 — the
proof-irrelevance reading of the macro-component, which chapter 39 needs locally, and
a pointer to the metatheorem. Chapter 40's crux subsection and the rewritten Gap 2 in
ch07 now point at chapter 41 rather than at a section.

Cross-reference repairs: the source note cited `agency` throughout, but a third of
those references are to material that now lives in chapter 39 (the races, the
macro-component, "capacity lives at the apertures") or in the fibration chapter (the
stalk). Eleven were re-pointed individually.

Key collision: this note's `abramsky` is Abramsky's *Proofs as processes* (1994),
while agency's `abramsky` is Abramsky–Brandenburger on contextuality. Renamed to
`abramskyPaP`. Twelve bibliography entries added.
