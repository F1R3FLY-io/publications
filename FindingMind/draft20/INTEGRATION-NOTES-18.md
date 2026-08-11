# INTEGRATION-NOTES-18 — draft20

**The structural pass: hypercomputation moves to the Prestige, sentience is rebuilt
as internalized homeostasis, and the simulation hypothesis gets a chapter.**

Applies on top of draft19. Also carries the 44-file draft18 nomenclature sweep,
reconstructed, since it was never committed and every file this pass touches would
otherwise have reverted it.

Build: repo baseline + draft2..draft19 + this delta, three `pdflatex` passes.
**704pp** (was 680), 0 errors, 0 undefined references, 0 undefined citations,
8 overfull boxes — the same 8 as the baseline, at identical widths, four with line
numbers shifted by the 16 lines inserted into `prestige_ch01`. Verified in a clean
room assembled from the repository, not from the working tree.

**A PDF ships this time.** draft19 shipped none, because a build from the repository
lacked the 46 missing draft18 files and would have printed a correct title page over
chapters still saying *engine* and *colour*. That is no longer true — see §1.

**51 files.** 35 in The Turn, 10 in The Prestige, 2 in The Pledge, 2 Interludes, and
`finding_mind.tex`.

**11 deletions.** These are moved originals, not casualties, and they must come out of
`TheTurn/` or the tree will hold two copies of each chapter under different names:

```
TheTurn/turn_hypercomp_ch01.tex   TheTurn/turn_hypercomp_ch05.tex   TheTurn/turn_ndo_ch01.tex
TheTurn/turn_hypercomp_ch02.tex   TheTurn/turn_hypercomp_ch06.tex   TheTurn/turn_ndo_ch02.tex
TheTurn/turn_hypercomp_ch03.tex   TheTurn/turn_hypercomp_ch07.tex   TheTurn/turn_chs_ch01.tex
TheTurn/turn_hypercomp_ch04.tex   TheTurn/turn_hypercomp_ch08.tex
```

Nothing `\input`s them after this delta, so a build succeeds either way; leaving them
is a trap for the next `grep`, not for the compiler.

---

## Decisions carried in

| # | decision | effect |
|---|---|---|
| 1 | keep machinery close to its payoff | **all** of the old Part V moves, not just the interpretation — entropy's choice half, apertures, agency, choice types, the tower, consciousness |
| 2 | Kant stays last | `prestige_ch03` unmoved; the hypercomputation block is explicitly framed as foreshadowing, and `ch:kant` gains the simulation instance as a fourth |
| 3 | sentience opens Part V | new chapter is the Turn's ch45, not a Part II close |
| 4 | vector setpoint | `\suff` is signed and indexed by flavor; incommensurability becomes value conflict |
| 5 | the price is structural | internalization is priced through **selection**, not through a learner's decision; Buss supplies the template |
| 6 | combined | one delta, not four |
| 7 | nomenclature consistent as you go | the draft18 sweep reconstructed and applied first, so no file reverts |

---

## 1. The recovered sweep

Reconstructed from the rules in INTEGRATION-NOTES-16 rather than from the lost
delivery. **44 files, 461 lines, 680pp unchanged, same 8 boxes** — applied and verified
*before* any structural work, so the diffs below are structural only.

**engine → enzyme**, 28 technical sites outside `turn_rholife_ch04` (which was one of
the five files that did land): `turn_causality_ch07` ×12, `turn_gslt_ch09` ×8 including
the chapter title, `turn_entropy_ch01` ×3, `turn_origins_ch09` ×3, `turn_causality_ch10`,
`turn_rholife_ch05`. Applied per-file, in the six files where *every* occurrence is the
technical object, rather than by a global rule.

**Seven left alone, deliberately** — every one ordinary English, matching the list in
notes 16: *engineer/engineering* (`turn_rholife_ch01` ×2, `prestige_ch02`), *reduction
engine* (`turn_gslt_ch02`), *the shared engine* (`prestige_chs_ch01`), and the two
literary uses in *The Permitted Say*. The dozen occurrences inside
`eng:rem:nomenclature` also stay: that remark discusses the word on purpose.

**colour → flavor** in the token sense (`turn_causality_ch07`, `turn_gslt_ch09`,
`turn_origins_ch09`); **colour → color** in ch22/ch23's board-game sense
(`turn_rholife_ch02`, `turn_rholife_ch03`). `hypercomp_ch03`'s *color charge* was
already US and is physics.

**US spelling**, from a word list built from forms actually present. `analogue` and
`dialogue` kept per decision 2 of notes 16; `catalogue → catalog`. Masked during
substitution: the arguments of `\label \ref \cite \input \texttt \verb`, all
`lstlisting`/`verbatim` blocks, every LaTeX colour command. `ThePledge/` excluded
entirely except the two files this pass edits for content; `bibliography.tex` untouched.

The three retained oddities from notes 16 survive and are still not misses:
`\label{eng:coloured-learner}` names "The flavor of a learner",
`\label{eng:def:engine}` names Definition 24.2 *Enzyme*, and
`\label{sci:prop:sexdefence}` keeps its British spelling.

---

## 2. The Turn's Part V, split

`turn_entropy_ch01` was doing two jobs under one title and they part cleanly.

**Stays in the Turn** — ch46, *"Entropy, and the Cost of Staying Level"*,
`\label{ch:homeo-ent}` (new). Carries the dissipation section verbatim and
`rmk:two-erasures` verbatim, plus new material tying both to homeostasis:

- `ent:prop:self-charge` — regulation is charged to the account it regulates. There is
  no second account to fund it from, since by `rmk:ambient` an adjacent account is
  ambient authority and the framework has already refused that. So holding a setpoint
  costs strictly more inflow than merely running.
- `ent:rmk:thermostat` — the thermostat pays for the thermometer, and this is why the
  previous chapter could not simply assert that internalizing is better. Also the
  chapter's most unsentimental line: an organism that felt nothing would be cheaper, and
  in a placid world it would win.
- The two-erasures remark now has something that *sits in the gap it describes*. A
  setpoint is cost information; a drift is history information; neither reduces to the
  other. **That is why suffering could not be a component of the account and had to be a
  formula about one** — which was not obvious when the remark was written and is the
  best argument in the chapter.

**Moves to the Prestige** — ch57, *"Everything So Far Was Turing Complete"*, and it
**keeps `\label{ch:ent}`**, along with `rmk:ent-not-a-defect` and
`conj:dissipation-choice`. Keeping the label is what makes the move ref-safe: every
existing citation resolves without edit.

Part title is now **"Suffering: Homeostasis, Dissipation, and the Learner's Own
Account"**; `\label{part:choice}` retained, since three files point at it.

---

## 3. `turn_suffering_ch01.tex` — ch45, ~3,500 words, `suf:` namespace

The chapter the pass exists for. `\label{ch:suffer}`.

**§`suf:S1` The move.** Three pieces already built and never put together: the vectorial
account (`def:affordable`) whose components are flavors (`ch:eng`); reflection
(`sci:def:genome`, `sci:sec:logic`, `sci:tab:access` — reflection prices are the
cheapest access the calculus sells); and instruments (`ch:obsext`).
`suf:def:internal` says a learner may aim that apparatus **inward**, and that when it
does it pays the same tariff as when it aims it outward. `suf:rem:notnew` states that
no primitive is added, and that a framework in which introspection were free would
prove anything one liked.

**§`suf:S2` Homeostasis.** `suf:def:setpoint` ($\setpt$, tolerance, comfort region,
$\phi_{\mathrm{home}}$) and `suf:def:drift`. The bathtub is stated as a picture that
carries content: holding a level is not achieved by any act, it is achieved by matching
one rate to another with the level as the only evidence.

**§`suf:S3` Suffering.** `suf:def:suffering` — the assayed drift, signed, vectorial,
indexed by flavor. Three corrections to what it replaces, each stated as such:

1. **Not a component of the account, a formula about it.** The distinguished component
   $A_{\mathrm{s}}$ is withdrawn.
2. **The punishment is not administered.** `suf:prop:derived` — the sign and magnitude
   of $\suff$ are fixed by $\vect{A}$, $\setpt$ and the instrument, with no further
   parameter. $\delta^+$ and $\delta^-$ are gone; they were the weakest joint in the old
   argument and nothing was ever said about where they came from.
   `suf:rem:sees` is careful about the word *punished*: there is no punisher, the
   learner holds a goal and reads itself receding from it, and that is the whole
   mechanism.
3. **Efficacious without being money.** `suf:prop:not-poverty` — a rich learner far from
   its setpoint suffers, a poor learner at its setpoint does not. Trivial to prove and
   **unstatable** under the old definition, which is the point of proving it.

**§`suf:S4` Why this is affect.** An explicit audit against the four things the
five-perspective review said $\varsigma$ was not. Multi-dimensional (a vector over
non-interconvertible flavors); about something (indexed by a signature, hence a
community); modulates policy at fixed wealth (`suf:prop:policy` — two learners equal in
wealth and unequal in drift price the same assay differently); and has a set point by
construction. `suf:rem:attention` does **not** claim an attention mechanism: it says the
admissible policies are constrained, which is weaker and is what the framework can
actually support. `suf:rem:conflict` — two deficits with no arbitrage-free conversion is,
in `eng:S10`'s exact sense, a conflict of values; where conversion *is* arbitrage-free
the virtual token supplies a rate and the conflict is merely a computation. **The line
between a tractable trade-off and a genuine dilemma is a property of the enzyme graph,
not of the learner's psychology.**

**§`suf:S5` The price, and who pays it.** `suf:rem:no-decision` — the trade cannot be
evaluated by the one taking it, since the deliberation needs the model whose acquisition
is at issue, and there is no order in which it could be carried out. So internalization
is inherited or it is not.

`suf:S5.1` is Buss, with only the apparatus he needs. Von Neumann's dual use as the
historical framing; `sci:def:genome` supplies the object; `sci:rem:provision` supplies
what grammar cannot. **We do not claim a Weismann barrier** — `sci:rem:noble` already
declined that and nothing here needs it. `suf:prop:buss`: in a free theory a faster
subterm invades without bound; under metering a subterm drawing more than it earns runs
for a number of cycles bounded by its endowment and then deadlocks. Mortality is
selected *because* a bounded life is the precondition for a stable individual.

`suf:S5.2` runs the same argument one level up. `suf:conj:selected` — a threshold in the
variance of inflow, increasing in the regulation cost. The mechanism is anticipation:
a merely-surviving learner corrects after it cannot afford a step, an internalizing one
corrects while it still has tokens to commit. **Stated as a conjecture, and the reason
is stated too:** the framework has no stochastic inflow process to run it against.
`ch:weight` supplies the machinery. The experiment has not been run.

`suf:rem:nobody-chose` also answers the question a reader will be holding since
`suf:def:setpoint`: **where does $\setpt$ come from?** Not from the learner — it is a
heritable parameter carried in $@P$, subject to `sci:prop:sexdefence`'s recombination,
and selected the way body sizes are.

**§`suf:S6`** is explicit that phenomenality is not claimed, and that the word is
heavier than the structure. *"Negative valence signal" would be a way of not saying what
we mean* — but hold us to the structure.

---

## 4. The Prestige, restructured

Two internal parts, printing as X and XI:

**Part X — The Method, and What It Left Out** (chs 56–63)
`prestige_ch01` (trick) → `prestige_break_ch01` → `prestige_tower_ch01` →
`prestige_apr_ch01` → `prestige_agy_ch01` → `prestige_chs_ch01` →
`prestige_cons_ch01` → `prestige_gaps_ch01`

**Part XI — Reference, Simulation, and a Future Metaphysics** (chs 64–66)
`prestige_ch02` (symbol system) → `prestige_sim_ch01` → `prestige_ch03` (Kant)

Four chapters moved essentially verbatim, renamed from `turn_*` to `prestige_*` with
**all labels unchanged** (`ndo:`, `chs:`, and the tower's `sec:fibration`/`ch:tower`
pair). Three prose repairs only: two "this part" phrases in the tower chapter and one in
the apertures chapter.

### `prestige_break_ch01.tex` — ch57

`rmk:why-here` is the chapter that justifies the whole pass, and it is written to be
read by a skeptic: *the Turn's business is construction, all of it done inside the
restriction, and doing it inside the restriction is what made it work. What follows is
not more construction — it is the audit of the restriction.* It then concedes, in the
same remark, that this **puts the book's thinnest ice under its loudest claims**, and
says that is the honest arrangement rather than the flattering one.

`rmk:ent-handoff` picks up the promissory note `ent:rmk:handoff` left in the Turn.
`rmk:ladders-again` re-states the perpendicular-ladders warning at the point it is now
needed.

### `prestige_cons_ch01.tex` — ch62, *"Consciousness, and the World It Comes With"*

Merges the old ch03 (richer physics), ch05 (consciousness) and ch06 (the two cycles).
`sec:consciousness`, `def:consciousness`, `rmk:coordinate`, `sec:richer`,
`prop:strict-bisim`, `prop:new-noether`, `prop:resolution`, `sec:relationship` all
retained.

**Every result restated for the lattice order $v < w$ rather than the ordinal successor
$\alpha \to \alpha+1$.** The old chapter said the index was wrong and then used it
anyway. `prop:strict-hml` is new (the HML expansion had no label before).

**`sec:shared` is the new section and is what you asked for.** `prop:shared` — two
agents at the same lattice point resolve the same class and distinguish to the same
quotient, because both are functions of $w$ alone. `rmk:shared` draws the distinction
the chapter exists for: **sentience is owned and consciousness is occupied.**
`rmk:ecology-couldnt` says why the Turn could not have hosted this: the ecology's only
notion of sharing is a *reservoir*, which is rival and exhaustible, and a lattice point
is neither — a crowd at $w$ does not make $w$ poorer.

`rmk:composite` is flagged in-text as the fact `ch:sim` will need: an agent below does
not see a world with pieces missing, it sees a world of the right size whose contents
happen to be featureless.

`rmk:descent` is the least comfortable thing in the chapter and is not softened: a
learner that cannot afford its position does not merely see its world worse, it comes to
be in a smaller world and loses the vocabulary in which the loss could be noticed.

**One R2 site closed for free.** `def:approx` used $|\langle \mathtt{halt} \mid A \inter
\lceil (M,x)\rceil\rangle|^2$ — a squared amplitude on the complex instance withdrawn in
`sec:quantum`. It is now stated on the stochastic instance, and `rmk:approx-status`
concedes what the correction costs: a rate grades **reliability at a position** and does
not place an agent strictly between two positions, which the amplitude version claimed
too easily. Whether approximate resolution defines a genuine intermediate point is now
an open question rather than an assumed answer.

### `prestige_sim_ch01.tex` — ch65, *"If This Is a Simulation, Which Kind"*, ~2,400 words

New writing. Register matches `prestige_ch01`/`ch02` (lowercase *i*).

The premise: the useful variable is not *whether* but the **relation between
$\mathrm{pos}(\mathcal{S})$ and $\mathrm{pos}(\Bee)$**, and the criterion separating the
cases is `obs:hosting` — Goodman's two-way losslessness as hosting-and-exhausting.

- **Flat** (`prop:sim-flat`): equal positions give an encoding that is both hosting and
  exhausting, so bootstrapping is re-encoding and **nothing follows from the
  hypothesis**. `rmk:sim-brackets` connects this to `sec:the-move`: the flat hypothesis
  is precisely the assertion that the book's own angle brackets were free, which is the
  weakest thing one could believe on the subject.
- **Descending** (`prop:sim-shadow`): cannot be exhausting, so bootstrapping is
  approximation — a shadow. `rmk:sim-undetectable` is stronger than the usual
  undetectability arguments, which turn on the simulator being careful: **nothing here
  turns on care.** The simulated cannot notice because noticing is a distinction their
  position does not resolve. `rmk:sim-noumenon` hands this to `ch:kant` as a fourth
  instance of the budget move.
- **Ascending** (`prop:sim-noup`): ruled out. `rmk:sim-notspeed` answers the obvious
  objection — a position is not a rate, and no amount of running realizes a stronger
  choice principle. **This depends on Gap 2 (subject reduction), and the remark says so
  and names what fails with it.**
- **Skew** (`prop:sim-skew`): incomparable positions, partial in both directions.
  Flagged as mine: every argument I know assumes the two are comparable, and once the
  coordinate is a lattice point that assumption is a substantive hypothesis nobody has
  been asked to defend.

`sec:sim-evidence` refuses the glitch argument and says why, then offers a real if modest
payoff: if every physical aperture carries only finite contention, $\mathrm{pos}(\Bee)$
is at the floor, ascending and skew are vacuous, and the hypothesis collapses to two
cases. **"Are we simulated?" is not tractable; "how much contention does a physical
aperture carry?" is a question about physics, and it decides which versions of the first
are available to ask.**

### `prestige_gaps_ch01.tex` — ch63

Merges the old gaps and discussion chapters; keeps `sec:hyper-gaps` and
`sec:hyper-discussion`. Nine gaps. Two are new:

- **Gap 5, the standing charge of a position.** `sec:relationship` asserts that occupying
  a lattice point is recurring rather than one-time, and `rmk:descent` turns on it.
  `ch:agy` prices individual resolutions and does not price maintaining the capacity.
  **Nothing in the book supplies that quantity.**
- **Gap 8**, `suf:conj:selected` — the cheapest item on the list and the most
  embarrassing to have left open, since it could be closed by running something rather
  than by proving something.

Gap 3 is noted to have got **wider**: restating `prop:strict-bisim` for a general pair
$v<w$ removes the obvious candidate problem a successor supplied.

The discussion section states the sentience change as a **trade** rather than a win: a
derived quantity resting on `ch:obsext`'s two unproved hypotheses, against a posited one
resting on nothing. We think that is the right trade; the reader should see it as one.

---

## 5. Reconciliation across the rest of the book

**`turn_origins_ch08`** — the passage read "Chapter `ch:ent` left an obligation, and this
is the place to discharge it." The chapter is now ahead of it, in the Prestige, so the
direction is reversed: the Turn *supplies* what the Prestige will need, and
`cor:ai-choice` is cited forward from `prestige_break_ch01` rather than backward.

**`turn_rholife_ch04`** — `eng:rmk:two-criteria` now says `ch:apr` is "in the Prestige,
and a long way ahead", since a reader hitting that remark at p345 would otherwise expect
the chapter within fifty pages.

**`turn_conclusion_ch01`** — the Part V paragraph rewritten entire, and it now states the
structural change in plain terms: the argument that a mind can do something a Turing
machine cannot, and the account of consciousness that follows from it, **are no longer in
the Turn**, because both belong to the audit of the restriction the Turn was built
inside.

**`turn_conclusion_ch02`** — the level-by-level walkthrough keeps the ordinal tower as
the chain-shaped shadow but says so, and the $\fiber{0}$ agent is now "at the floor of
the lattice, not outside the structure". The recursive-picture paragraph's "sentience
component governed by predictive accuracy" becomes "learners that may hold a setpoint and
suffer their distance from it".

**`turn_conclusion_ch03`** — the hard-problem paragraph restated; the closing paragraph
now says "the six parts of the Turn, and the first half of the Prestige which audits
them".

**`finding_mind.tex`** — Part V summary in the Overarching Introduction rewritten;
dependency figure retitled with a new **the account** arrow from Ecology to Part V
(rerouted after the first attempt put its label on top of the Part I box — worth knowing
if you edit that figure); reading-route paragraph and confidence paragraph updated; the
Prestige framing box extended; three macros added (`\setpt`, `\drift`, `\suff`) with a
comment noting that `\sent` is retained only for the withdrawn account it names and that
nothing in the body uses it.

**`pledge_map`** — figure box retitled and a second Prestige box added showing its two
halves; the two-ladders paragraph re-pointed; the spine paragraph notes the Prestige is
now about a third again as long; philosopher's and mathematician's routes rewritten
(**the mathematician's route now ends at the Prestige, which is where the material that
route wants has gone**); the difficulty gradient split for the Prestige's two halves; the
part-numbering note extended to Parts X and XI; **two new entries in the load-bearing
unproved list** (`suf:conj:selected` and `prop:sim-noup`); and the closing "not a theory
of phenomenal consciousness" paragraph restated for suffering.

**`pledge_related`** — the "$\varsigma$ is a budget line, not a feeling" concession is
now the record of that concession *having moved*: what is conceded is smaller and named
(no arousal dimension, an inherited setpoint, a conjectural price). *This is a theory of
affect now, in the sense that it has the right moving parts. It is not a good one yet.*
The attention-schema paragraph is repaired the same way — the absence is partly filled,
and `suf:rem:attention` gets as far as constraining the policy space and declines to pick
one. **The three sentences on the introspective illusion are still not in the book.**

---

## Reference safety

No label was renamed, removed, or repointed. `ch:ent`, `sec:consciousness`,
`def:consciousness`, `rmk:coordinate`, `sec:richer`, `prop:strict-bisim`,
`prop:new-noether`, `prop:resolution`, `sec:relationship`, `sec:hyper-gaps`,
`sec:hyper-discussion`, `conj:dissipation-choice`, `rmk:two-erasures` and the whole
`ndo:` and `chs:` namespaces all travel with their chapters or stay with the half of the
split chapter that earns them.

`conj:survival` and `\sent` are **retired**: the conjecture went with the withdrawn
account and nothing refers to it; the macro is left defined and unused.

New labels: `ch:homeo-ent`, `ent:prop:self-charge`, `ent:rmk:thermostat`,
`ent:rmk:handoff`; the `suf:` namespace; `rmk:why-here`, `rmk:ent-handoff`,
`rmk:ladders-again`; `prop:shared`, `rmk:shared`, `rmk:ecology-couldnt`,
`prop:strict-hml`, `rmk:composite`, `rmk:independent`, `rmk:descent`, `def:approx`,
`rmk:approx-status`; `ch:sim` and the `sim:` results; `part:method`, `part:reference`.

No new bibliography entries. No new environments. Three new macros.

---

## Open, and flagged for your eye

1. **`prop:sim-noup` leans on Gap 2.** If subject reduction fails for schedulers, a
   running scheduler can come to realize a stronger choice principle, and the ascending
   case reopens. Said in the chapter, in the gaps list, and in `pledge_map` — but it is
   the load I would least like to be wrong about.
2. **The skew case is mine.** `prop:sim-skew` and `rmk:sim-skew-inside` are not in any
   source note and I have not seen the observation made elsewhere. Worth your eye before
   it stands.
3. **`suf:conj:selected` has never been run.** It is stated in the form
   `ch:weight`'s simulator could check. Closing it would be the single highest-value
   small piece of work left in the book.
4. **Gap 5 is new and real.** The unfavorable cycle in `sec:relationship` needs a
   standing charge for occupying a position, and nothing prices one.
5. **`\drift` is `\boldsymbol{\delta}`**, which makes LaTeX request a bold stmaryrd that
   does not exist. One harmless warning; the glyph sets correctly (checked by rendering).
   Swap to plain `\delta` if you would rather not carry it.
6. **Two "Conclusion" entries in the ToC**, pre-existing and untouched.
7. **The verso running header still prints on only ~10 front-matter pages.**
   Pre-existing, noted in draft19, untouched.
