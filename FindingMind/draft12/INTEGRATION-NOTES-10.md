# Integration Notes 10 — the map, related work, and the placeholder

Adds two unnumbered chapters at the end of The Pledge, removes *What the Prestige
Still Owes*, and retires the stale related-work section in ch. 52.

Build verified: **619 pages** (from 606), **0 undefined references, 0 undefined
citations**, no new overfull boxes. Chapter numbering is completely unchanged —
both new chapters are `\chapter*` with `\addcontentsline`, so nothing renumbers.

| | |
|---|---|
| `ThePledge/pledge_map.tex` | *How to Read This Book* — p. 57, ~6 pp, one figure |
| `ThePledge/pledge_related.tex` | *Related Work, and Where This Differs* — p. 63, ~11 pp, one table |
| `ThePrestige/prestige_placeholder.tex` | **deleted**; `\input` removed from the master |
| `TheTurn/turn_hypercomp_ch08.tex` | ch. 52 §1 trimmed to a back-pointer |
| `bibliography.tex` | 140 → 160 entries |

---

## What each section does that nothing else in the book was doing

**The map.** The Turn already carries a long Overarching Introduction with part-by-part
summaries, so the map deliberately avoids that ground and does four things instead:
covers all three acts, gives reading routes with an explicit statement of what each
route costs you, grades the difficulty honestly, and gathers the load-bearing unproved
claims in one place.

That last section is the one I'd draw your attention to. It names, with references:
`thm:compact-consensus` (stated as a theorem, proof doesn't work as given, and the
Conclusion calls it the sharpest result in its part); the fibration *p*: Hyp → GSLT,
unbuilt; `conj:conjugacy` and `conj:reversal`, demoted in draft 4; `conj:dissipation-choice`;
`conj:fd`; and `q:gsos`. It closes with a sentence I think earns its place: *"a reader
who discovers them one at a time will reasonably wonder what else is hiding. Nothing
else is hiding."*

The map also fixes two things that were reader-hostile and cheap to repair. It states
up front that the Prestige audits the book's method, which changes how the middle reads
and costs nothing to give away. And it explains the two ladders — the subcategory ladder
in *Situating the Learner* and the oracle ladder in *Entropy, Choice* are perpendicular,
which `rmk:two-ladders` says once, in passing, on p. 319.

**A numbering quirk I documented rather than fixed.** LaTeX numbers every `\part` in one
sequence, so the Turn's six internal parts print as Parts III–VIII while the prose calls
them Parts I–VI of the Turn. The map names this explicitly and recommends referring to
parts by name. Fixing it properly would mean either renumbering the prose or suppressing
part numbers, and both are larger changes than this pass.

**Related work.** Organised by what each neighbour is a theory *of*, with four things
said about each: the claim, what this book takes, where they part, and whether the
parting is checkable. The through-line: of every programme discussed, this is the only
one in which the knower's limits are *priced*.

---

## The scored table, since you wanted to see it

Rows: IIT, free energy, global workspace, constructor theory, ruliad, AIXI, assembly
theory, this book. Columns: *theory of* / *primitive* / *observer inside?* / *what would
falsify it* / *anything priced?*

The pricing column reads: **no, no, no, boolean only, no, no, depth-not-access, yes.**

Two cells carry argument rather than description, and the chapter defends both after the
table:

- **Constructor theory scores "Boolean only" rather than "no."** A two-valued possibility
  measure is still a measure, and on the reading proposed in §3 it is precisely the
  Boolean fibre of the semiring-parametric construction in ch. 25 — with possible-at-a-cost
  as the tropical one. This is the single claim in the chapter I'd most want you to check.
  If it survives being made carefully it is a real result, and I don't believe it has been
  stated before.
- **AIXI's falsification cell is a dash.** AIXI is optimal by construction relative to its
  own prior, so nothing refutes it; what is refutable is that it models anything
  realisable, and `sci:prop:limit` is one form of that refutation. The chapter says the
  dash records this rather than scoring it.

Assembly theory scores partially because it prices *construction* and says nothing about
the cost of *access* — which is exactly the gap `scm:prop:geom` exploits.

---

## Concessions the related-work chapter makes on the book's behalf

These are written in, not just noted here, because a related-work section that only
scores wins is not worth reading:

- IIT and FEP have experimental programmes; assembly theory has a mass spectrometer and a
  published protocol. This book has Fermi estimates.
- The book's ς is a budget line, not an affect — one drive, no valence, nothing *about*
  anything. Ch. 21's token colours are named as the partial answer.
- The attention-schema paragraph concedes that the germ **is** a self-model and the book
  uses it almost only for reproduction — and that §62.5.1 applied reflexively derives the
  introspective illusion in about three sentences that are not in the book.
- No distributive law is stated for Cost ∘ Hist though both are applied together;
  `rmk:two-erasures` comes close to saying they interchange only laxly and stops short.
- Hyperon/MeTTa is called the most conspicuous omission in the chapter, and the comparison
  owed — MeTTa's self-modification versus reproduction-as-recombination-on-quoted-code,
  and Hyperon's attention allocation versus the metering here — is described as a chapter
  that isn't written.
- The Chalmers paragraph says plainly that the book's move relocates the hard problem
  rather than dissolving it, and that the book does rather more making-tractable than
  answering.

---

## Mechanics

- Both chapters use `\chapter*` + `\addcontentsline` + `\markboth`, matching the
  convention already used by *A Note on the Interludes*.
- The map's figure and the table would otherwise have numbered as **4.1** — inheriting
  the chapter counter from *Finding the Shape* — which reads as though they belong to
  ch. 4. Each file now locally redefines its counter to `\Alph`, so they print as
  **Figure A** and **Table A**, and restores `\thechapter.\arabic` at the end of the
  file. Verified: downstream numbering still runs 16.1, 18.1, 18.2, …
- The figure uses only `arrows.meta` and `positioning`, both already loaded.
- 20 new bibitems: Baars, Chalmers, Coecke–Kissinger, Cronin (Marshall et al.), Dehaene,
  Deutsch, Fong–Spivak, Goertzel, Graziano, Hutter, Lloyd, Marletto, Maturana–Varela,
  Nagel, Rosen, Solomonoff, Walker (Sharma et al.), Wheeler, Wolfram ×2.

**One to check.** The Goertzel bibitem is the weakest entry — the Hyperon paper's exact
venue and year I could not verify offline, so I've given a preprint form plus *The General
Theory of General Intelligence*. Worth correcting before this goes anywhere.

---

## Still open

Unchanged from Notes 8 and 9: the alternative title *The Clear Plastic Cups* for ch. 62;
the two dropped TikZ figures from the bootstrap note; and whether Part V should re-bill
Composing Learners as a precondition rather than an extension.

New from this pass:

1. **The part-numbering mismatch** (III–VIII versus I–VI) is now documented but not fixed.
2. **The constructor-theory identification** wants checking before it stands as written.
3. **`thm:compact-consensus`** is now named in the front matter as a gap. That makes
   fixing or demoting it more urgent than it was when it was buried on p. 381.
