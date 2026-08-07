# Pass A, part one: making the book stop contradicting itself

Responding to `draft9/five_perspectives.tex`, items **R1, R3, R4, R5** plus the
residual defect in the chapter draft12 rewrote. R2 (the semiring rewrite of
message fidelity) ships separately as draft15, per your split.

Build: **623 pages**, three passes, zero errors, zero undefined references, zero
undefined citations. Eight overfull boxes, byte-identical to the draft13
baseline — no new ones, and the page count is unchanged, so nothing downstream
reflowed.

## Files

| File | Chapter | What changed |
|---|---|---|
| `TheTurn/turn_hypercomp_ch01.tex` | 42 | R1 — the central-proposal paragraph, coordinate framing; R5 — organisation section |
| `TheTurn/turn_hypercomp_ch02.tex` | 44 | R5 — retitled, definition downgraded, `rmk:not-a-fibration`; R1 — new closing §44.5 |
| `TheTurn/turn_hypercomp_ch05.tex` | 50 | R1 — `rmk:coordinate` replaces the ordinal remark; six table rows; two reconciled remarks |
| `TheTurn/turn_hypercomp_ch08.tex` | 53 | residual — retracted amplitudes and the fibration claim |
| `TheTurn/turn_rholife_ch04.tex` | 22 | R1 — `eng:rmk:two-criteria`, the forward pointer into ch. 45/46 |
| `TheTurn/turn_sciencebot_ch06.tex` | 37 | R3 — demotion + two failure remarks; R4 — `rmk:stone` |
| `TheTurn/turn_conclusion_ch01.tex` | — | R1 — the consciousness sentence; R3 — the "sharpest result" claim |
| `TheTurn/turn_ndo_ch02.tex` | 46 | R5 — two stray uses of "fibration" for the tower |
| `TheTurn/turn_chs_ch01.tex` | 47 | R5 — one stray use |
| `ThePledge/pledge_map.tex` | — | R3, R5 — the gathered-gaps list reconciled with the body |

`finding_mind.tex` and `bibliography.tex` are **untouched**. No new bibliography
entries, no new macros, no new environments.

## R1 — the four lost revisions

One of the four turned out to be narrower than the review reported. The
*Organization* section of ch. 42 had already been repaired during the draft7
refactor — it names ch. 45/46/47, cites `rmk:two-ladders`, and already says the
ordinal presentation "is not how the thing actually works." Only the central
proposal paragraph was stale, and that is what has been rewritten.

The restored content, per `draft3/INTEGRATION-NOTES-2.md`:

**Ch. 42.** Consciousness is a coordinate: the strength of choice an agent can
afford, a position in a lattice, and every agent has one. The paragraph makes
explicit the thing the ordinal reading obscured — that an agent's coordinate is
simultaneously the threshold at which the world islands for it, so its
consciousness and the granularity of the world it inhabits are one fact reported
from two sides.

**Ch. 44, new §44.5 *Two things this construction gets wrong*.** The chapter now
names its own two defects before the reader can discover them three chapters on:
the index should not be an ordinal (ordinals are linear, the thing indexed is
not, two agents can each resolve what the other cannot), and the oracle should
not be bolted on (it is already there as the scheduler at every cut; the capacity
has to be found and priced, not adjoined). A third paragraph says what survives
both repairs — projections are lossy, going up reveals distinctions, each stage
is strictly richer — since none of that depends on the index being well-ordered.

**Ch. 50.** The ordinal remark becomes `rmk:coordinate`. The comparison table's
six stale rows are replaced (Type → a lattice-valued position; Location → which
point in the base; Governed by → choice strength affordable; Changes → by moving
in the lattice; Physical basis → what the budget buys at a cut; Survives
projection → moves, it does not vanish).

Two further remarks in that chapter had to move with it, because they encoded
the threshold reading in prose rather than in the table. The definition's
"sentient but not conscious" clause now describes an agent at the *floor* of the
lattice rather than outside the structure, and the remark that followed is
retitled *Sentience and position are independent* — nothing about being at the
floor makes an agent numb, and nothing about being high in the lattice makes it
feel anything.

**Ch. 22.** `eng:rmk:two-criteria` at the end of §`eng:S8`. Puts the engine
chapter's economic criterion (close where repair is cheaper than exposure)
beside ch. 45's semantic one (close where no internal cut carries observable
choice) and states that neither implies the other — being worth owning and being
determinate are independent, and most organisms have the first without the
second. Read forward it says the individuation fixed point has a companion it
does not know about; read backward it says the semantic criterion has an
economic partner already worked out. Named in both places so a reader does not
have to notice the gap alone.

**The Conclusion.** "the first continuous, the second not" is gone. Replaced
with the coordinate reading and the observation that the two quantities vary
independently, linked by cycles that run in either direction.

## R5 — the tower is not a fibration

Ch. 44 is now **The Tower of Oracle Extensions**.

`\label{sec:fibration}` is **kept**, with `\label{ch:tower}` added alongside it.
Eight existing references point at `sec:fibration` (in ch. 42, 43, 46, 47 and
`prestige_ch03`); all still resolve, and new material can use the honest name.
This was the ref-safe way to do the retitle without touching five other files.

`\begin{definition}[The oracle tower]`, labelled `def:oracle-tower`, replaces
`\begin{definition}[Hypercomputational Fibration]`. Immediately after it,
`rmk:not-a-fibration` states the objection in full: a functor out of a product
is an indexed family, from which the Grothendieck construction would *build* a
fibration; the variance is overdetermined because both `\proj{\alpha}` and
`\iota_\alpha` are given and `Ord` is a poset; the intended reading is
`\iota_\alpha ⊣ \proj{\alpha}`, whose triangle identities are **not** checked
here; and the object the part is really about is `p: Hyp → GSLT` from open
problem (ii) of §`ndo:sec:open`, where the base is all of **GSLT** and the
Lawvere-adjoint question can even be asked.

Three stray sentences elsewhere called Φ a fibration and now say "tower"
(`ndo_ch02` ll. 212 and 319, `chs_ch01` l. 314). The *correct* bifibration
references in `ndo_ch02` §"Both pictures at once" and open problem (ii) are left
alone — those are about the right object.

## R3 — the compactness repair does not go through

I attempted it. Both routes fail, and how they fail is the useful part, so it is
written into the chapter rather than into this file.

**The argument as given.** φₙ holds of an agent that has completed *at least* n
oscillation steps, so satisfying φₙ₊₁ entails satisfying φₙ and the family is
**decreasing**. Hence ⋃_{n≤N}⟦φₙ⟧ = ⟦φ₁⟧ for every N, and an agent that has
oscillated more than N times lies *inside* that union. No failure of finite
subcovering, no contradiction.

**Repair 1, the reindexing.** Take ψₙ = ¬φₙ, "fewer than n steps", giving an
increasing family. That family covers X_pop precisely when every agent
oscillates finitely often — which is the conclusion. Circular.

**Repair 2, finite intersection.** The extensions are clopen (see R4 below), so
the family is closed as well as open and has the FIP whenever some agent has
oscillated N times for each N. Compactness then gives ⋂ₙ⟦φₙ⟧ ≠ ∅: an agent
satisfying every φₙ, which is an infinite oscillator. **This argument is correct
and it establishes the opposite of the theorem.** Compactness is a condition
guaranteeing that limits are attained, and a perpetual disagreement is exactly
such a limit.

So: demoted to `\begin{conjecture}`, label `thm:compact-consensus` retained so
nothing breaks. Two remarks added — `rmk:consensus-attempt` (the three arguments
above) and `rmk:consensus-missing` (what the conjecture needs).

**What I think is actually missing.** Compactness in the logical topology is a
*richness* condition: by Stone duality it says every finitely satisfiable set of
formulae is realised by some point of the closure, so the population omits no
consistent type. Nothing about richness makes a process stop. Termination is not
topological, it is **budgetary** — every revision is a paid assay, so if each
alternation costs a bounded-below quantity from a bounded-above endowment, the
alternation terminates for reasons having nothing to do with the topology. On
that reading compactness supplies the *second* half of the statement: the bound
on convergence time, once termination is available from the budget.

Two routes are stated in the chapter. Add the cost hypothesis and prove it as a
result about priced revision — in which case it belongs in the ecology part, not
here. Or keep it topological and weaken the conclusion from "the alternation
terminates" to "the alternation is confined to a subspace of bounded logical
depth", which may be provable as stated and is weaker than what the rest of the
part uses. The first looks likelier to be both true and useful.

The two downstream results — coherence clusters as maximal compact
subpopulations, and non-compactness as permanent disagreement — are flagged as
depending on it. The *Non-compact populations* remark is restated in the
conditional, and now notes the unreconciled tension: under Stone duality
non-compactness says the population omits a consistent type, which is a
statement about **gaps** in the population rather than deadlock within it.

The Conclusion and `pledge_map` both updated.

## R4 — Stone, not Scott, and it unifies two parts

`\HML(\ctx)` has negation (Definition `def:cdHML`), so ⟦¬φ⟧ is the complement of
⟦φ⟧, every subbasic open is clopen, and the space is zero-dimensional. By
Hennessy–Milner, distinct bisimulation classes are separated by a formula, hence
by a clopen set — so the space is Hausdorff with closed points. The
specialisation order, which carries the content in a Scott topology, is discrete
here and carries none; "sober, T₀" understates the separation badly.

The payoff is the one Spivak predicted. Compact X_pop is the Stone space of the
Boolean algebra of `\HML(\ctx)`-definable sets: a **profinite limit of finite
quotients**, each of which is the population as seen at a bounded modal depth.
That is the same inverse system §`sci:sec:lattice` already puts on the hypothesis
space, where the ultrametric comes from agreement up to depth and the balls are
its levels. The logical metric d_HML is the ultrametric that induces this
topology, and the metric balls are exactly the basic clopens.

**Flagged for your eye.** This identification — that the two parts have been
using one object under two descriptions — is mine, not something the book states
anywhere. It is the most substantive new claim in this delta and it should be
checked before it ships as fact.

## Residual in ch. 53

Draft12's rewrite fixed the staleness the review found but left two things.
§"The Broader Picture" credited the physics chapters with grounding "quantum
amplitudes" — the apparatus ch. 29 retracts. It now reads as the
semiring-parametric path weight, with the complex instance named as withdrawn
and the stochastic instance named as what everything downstream uses. And the
Turing paragraph called the construction a fibration; it now describes the move
off the chain in three steps, ending at the Weihrauch coordinate.

## Open items

1. **The Stone/ultrametric identification** (R4 above). Wants checking.
2. **Coherence clusters now rest on a conjecture.** `sciencebot_ch07`,
   `sciencebot_ch01` and `origins_ch04` all use "maximal compact subpopulation"
   as though the consensus result were settled. The new remark says so in one
   place rather than chasing it through three chapters — that is a bigger edit
   than this pass's remit, and it may resolve itself if the budgetary route in
   R3 works out.
3. **`rmk:not-a-fibration` asserts the intended adjunction without checking the
   triangle identities**, and says so. If ι ⊣ π does hold, the tower is a tower
   of adjunctions and the remark should be upgraded to a proposition.
4. **Ch. 50's approximate oracle access is still a squared amplitude.** That is
   R2 and it goes in draft15 with the message-fidelity rewrite, so the two uses
   of the retracted apparatus are repaired together.
5. The `sec:fibration` / `ch:tower` double label is deliberate. If you would
   rather have one name, the rename is mechanical across eight call sites.
