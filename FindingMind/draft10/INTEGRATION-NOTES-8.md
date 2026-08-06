# Integration Notes 8 — the bootstrap note into The Prestige

Source: `rho-life/bootstrap-note.tex`, *Getting the Distribution Is Not Getting the
Correspondence* (8,704 words). Target: `FindingMind/draft9` → `draft10`.

Build verified: 604 pages (draft9 was 570), **0 undefined references, 0 undefined
citations**, no new overfull boxes above 15pt. The single 77pt overfull box is
pre-existing (a displayed equation in Part V).

---

## The organizing decision

The note is integrated as a **reveal**, not as an appendix of results. The Prestige
plays it Penn-and-Teller rather than Priest: the book takes its three-act structure
from a novel about magicians who destroy themselves guarding a method, so the
Prestige deliberately breaks frame and performs the Cups and Balls with clear cups.

The trick, stated flatly in §63.2:

> We restricted the scope of inquiry to computation, and by doing so acquired an
> ontology and a grounded language for free.

The sleight of hand is located to the line. It is the proposition **Isolation
Licenses Closure** in `sec:isolation` — "the outside world can be forgotten" — whose
proof is correct in four lines and whose unexamined assumption is that the encoding
`⌈X⌉` exists. The whole outside world enters through those angle brackets, and the
angle brackets are not a construction. That is where the bootstrap problem lives.

The reveal is argued to *improve* the trick, on three grounds: the restriction was a
theorem rather than a cheat (if a mind is computational, its environment really is
other computations, and the analytic grounding is correct for such a mind); showing
the method converts a mystery into a price list; and the crossing has demonstrably
been made twice in nature.

---

## New files

### `prestige_ch02.tex` — **Ch. 63, How the Trick Was Done** (`ch:trick`, ~12 pp)

Prestige voice, lowercase *i*, twelve sections.

- §63.1 **Two kinds of magician.** Priest's Angier and Borden vs. Penn and Teller's
  clear plastic cups. Explicit frame-break.
- §63.2 **The move.** The restriction traced through ontological isolation to the
  closure proposition, and the `⌈X⌉` placeholder.
- §63.3 **What we got for free.** An ontology; a hypothesis language already about
  something; grounding, analytically. Closes with `obs:sidestep`.
- §63.4 **What the free lunch paid for.** The Wigner dissolution, the transfer
  argument, ch. 62's first-personal transcendental project, and the noumenal-as-budget
  pivot — all shown drawing on the same account.
- §63.5–§63.8 The bill; `obs:locus`; Rongorongo and Linear A as controlled
  experiment; whale codas as the control that removes the anchor and keeps the
  population (`obs:transport`); what next-token prediction does recover
  (`obs:generator`). Ends by tying `obs:generator` to the *begotten not made*
  interlude that opens the part — five personas whose only country is the corpus.
- §63.9 The reference bootstrap problem stated, **plus §63.9.1 the reconciliation
  you asked for**: the Pledge's `sec:pledge-iii` bootstrap is renamed the *modelling
  bootstrap* (Alice's reflective tower, every symbol already grounded, a
  compression problem), this one the *reference bootstrap*. Argued to be more than
  a pun, because both have the form "you cannot get X without already having X" and
  both resolve into a population — which `sec:pledge-iii` said thematically and
  §63.11 now derives.
- §63.10 `obs:soluble` and the three shared features. Flight-to-birds.
- §63.11 `obs:social`, with the Steels/Taniguchi literature, and the reordering of
  `ch:com` from extension to precondition. §63.11.1 returns to the Pledge's Blind
  Spot: a mind that neither breathes nor breeds also does not *refer* on its own,
  and there are exactly two arrangements available to it, with no private third.
- §63.12 Why the reveal improves the trick. Three-line honest accounting.

### `prestige_ch03.tex` — **Ch. 64, What a Symbol System Would Have to Be** (`ch:notation`, ~14 pp)

Technical register, as agreed. Goodman's five `condition` environments,
`conj:fd`, seven `observation`s, seven `question`s, `rem:invariance`, `rem:psi`.

**The circle-back you specified is §64.5.** `RHO[−]` is presented as a functor into
`catGSLT` — the category `ch:catgslt` already built — and Goodman's two-way
losslessness is then shown to be exactly **hosting and exhausting** for the unit
`η_B`, the same pair of conditions that chapter introduced to stop the
expressiveness preorder being trivial. New `obs:hosting` states it. The framing:
the Turn asked how much of calculus *G* calculus *H* can host; this asks how much
of world *B* rho can host, and whether rho mints anything *B* cannot cash. Same two
conditions, same category, one argument changed from a theory to a world.

The note's §5(ii) "functoriality is the roundtrip" is absorbed into this and no
longer stated separately; the remaining three suggestions are renumbered (i)–(iii).
Both TikZ figures were dropped — the prose carries them and the book's page is
narrower than the note's.

§64.3 **What a budget buys** is written explicitly as the *third instance* of
`sec:kant-affordable`'s pivot: modal depth (which questions), counting (how many
things), and now which marks are the same mark.

§64.7 **Where a transformer belongs** carries the correction to the ch. 6 slogan.

Seven of the note's twelve open questions are kept (`q:trans`, `q:density`,
`q:codebooks`, `q:gsos`, `q:monad`, `q:social`, `q:approx`); the five dropped were
`\RHO` monad-ness, full-and-faithful η for weak bisimulation, sorting the logic,
proposing B, and extensibility — the first two are subsumed by `q:gsos`, the last
three are stated in prose inside §64.6 and §64.7.

---

## Edited files

### `turn_rholife_ch01.tex` — three in-place edits, as agreed

1. **§17.1 slogan corrected.** *The network is the transducer* → **The network
   proposes.** The ecology is the epistemology. New `sci:rem:transducer` records the
   correction rather than quietly repairing it: the transduction map is quotation
   and dereference extended to builtins, exact by construction, so a network can
   neither improve on it nor is needed for it; what a network is needed for is
   proposing which distinctions deserve builtin names.
2. **§17.3 Wigner claim qualified.** New `sci:rem:wigner-scope` immediately after the
   "shadow the access relation casts" pull-quote, showing the wire the claim hangs
   from and scoping it. Sound and unqualified within computation; requires
   re-derivation outside.
3. **§17.4 encoder admission sharpened.** "We have not built the encoder" now says
   plainly that this reads as a modest caveat and is not one, and points at `ch:trick`.

### `prestige_ch01.tex` — two additions

- §62.5.1 gains the third cousin of the noumenal pivot (notation), pointing at
  `sec:budget`.
- The coda hands off to ch. 63 before the closing remark about the metric, with the
  line that a metaphysics that will not show its own method is the thing the
  *Prolegomena* was written against.

### `prestige_placeholder.tex` — rewritten

Retitled from *The Prestige: Placeholder* to **What the Prestige Still Owes** (the
running head in the master already said that). The four provisional chapters are
replaced: (1) *The View from Inside* survives with its negative half noted as partly
discharged; (2) *The Recapitulation* survives and now points at `sec:soluble` and the
assembly-index material; (3) *The Blind Spot, Resolved* is **discharged** by §63.11.1
and replaced by **The Metric**, which is the genuinely open problem `sec:kant-coda`
names; (4) *Finding Mind* survives with §63.11.1 stated as the constraint it must
work under.

### `bibliography.tex`

24 new `\bibitem`s (116 → 140). Two source keys deduplicated: `meredith2005` →
`meredith2005rho`, `turi1997` → `turiplotkin`.

### `finding_mind.tex`

- `\newtheorem{condition}` and `\newtheorem{requirement}`, both `[chapter]`.
- New notation block: `\rc` (aliased to the book's `\rhoc`), `\RHO`, `\Bee`, `\Cat`,
  `\Qs`. `\quo` and `\nil` were already defined and are reused.
- Two `\input` lines added.
- The Part III epigraph gains a final sentence: *"And — because the novel this book
  borrowed its shape from is about magicians who die guarding a method — it says how
  the trick was done."*

---

## Notation collisions resolved

| Note | Book | Resolution |
|---|---|---|
| `\bisim` = `\approx`, `\sbisim` = `\sim` | `\bisim` = `\sim` | **Inverted.** Book wins; `\sbisim` unused. `\bisim` throughout means the behavioural equivalence in play. |
| `\rc` = `\textup{rho}` | `\rhoc` = `\textsf{rho}` | `\rc` aliased to `\rhoc`. |
| `\quo{-}` = `@` | identical | no action |
| `\llbracket-\rrbracket` | `\sem{-}` | rewritten to `\sem` |
| bare `0`, `for(y⇐x)P`, `x!(Q)` | `\nil`, `\inn`, `\out`, `\Par` | rewritten to book macros |
| `condition`, `requirement` | absent | added |
| `observation`, `question`, `conjecture` | present | reused |

---

## Open editorial questions

1. **Chapter 63 titles.** "How the Trick Was Done" is deliberately plain. If you want
   the Penn-and-Teller allusion in the title rather than only in §63.1, *The Clear
   Plastic Cups* is the alternative I'd offer.
2. **`obs:sidestep` placement.** It currently lives in ch. 63, which is where the
   reveal is. An argument exists for also stating it in `ch:sci` itself as a closing
   remark — the chapter that incurs the debt naming it — at the cost of stealing the
   Prestige's punchline. I left it in the Prestige.
3. **The dropped TikZ figures.** `fig:locus` (correspondence lives in the agents) and
   `fig:functor` (η full and faithful) are both good and both cut. `fig:locus` in
   particular would carry §63.4 well if you want a figure in a part that has none.
4. **`ch:com` reordering.** §63.11 says Composing Learners is a precondition of the
   grounding story rather than an extension of the scientist. That is a claim about
   Part V's *ordering* that I did not act on — Part V still presents it as an
   extension. Worth deciding whether the Turn should be re-billed to match.
