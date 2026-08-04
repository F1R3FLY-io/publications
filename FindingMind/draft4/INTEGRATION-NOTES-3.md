# Reworking the Pledge and the causality chapters against the cost, history, and virtual-token notes

Build: **391 pages** (was 372), three passes, zero errors, zero undefined
references, zero undefined citations, three overfull boxes — all three
pre-existing (the TOC line, `turn_origins_ch02`'s display, and the `turn_chs_ch01`
chapter title). The `sec:gaps` duplicate is still there; still out of scope.

## Files

Copy each into the tree at the indicated path.

| File | Destination | Status |
|---|---|---|
| `finding_mind.tex` | `FindingMind/` | revised — new macro block, `part:sciencebot` label |
| `bibliography.tex` | `FindingMind/` | revised — five new entries |
| `pledge_ch02.tex` | `ThePledge/` | revised — two insertions |
| `pledge_ch03.tex` | `ThePledge/` | revised — one insertion |
| `pledge_ch04.tex` | `ThePledge/` | revised — new closing section |
| `turn_causality_ch01.tex` | `TheTurn/` | revised — thesis, Rosetta table, organisation |
| `turn_causality_ch06.tex` | `TheTurn/` | **replaced** — *Decoration: One Map, Many Semirings* |
| `turn_causality_ch07.tex` | `TheTurn/` | **replaced** — *Conservation, and What It Costs to Have It* |
| `turn_causality_ch08.tex` | `TheTurn/` | revised — one decoration value; located accounts |
| `turn_causality_ch09.tex` | `TheTurn/` | **replaced** — *Complex Amplitudes, and an Obstruction* |
| `turn_causality_ch10.tex` | `TheTurn/` | revised — annotated dictionary, honest main theorem |
| `turn_causality_ch11.tex` | `TheTurn/` | revised — negative results; new open questions |
| `turn_sciencebot_ch01.tex` | `TheTurn/` | revised — one forward claim reworded |
| `turn_conclusion_ch01.tex` | `TheTurn/` | revised — one forward claim reworded |

The chapter count is unchanged: ch06 and ch07 were not merged into one file, they
were re-scoped. Old ch06 (weight map) and old ch07 (cost map) become new ch06
(the decoration, including the cost instance and affordability) and new ch07
(conservation). Nothing downstream renumbers.

## What changed, and why

### B — the decoration (ch06)

The weight map and the cost map had the same domain, the same indexing and the
same shape, and differed only in codomain. They are now one family
`\dec_r : HML(K) ⇀ R` parametric in a semiring, per `continued-gslt-cost-v2` §13:
Boolean gives bare nondeterminism, tropical gives cost, ℝ≥0 gives stochastic
dynamics, ℂ gives amplitudes.

Three consequences:

- **The Legendre problem dissolves.** ch11 had listed the invertibility of a
  Legendre transform between "the Lagrangian" and "the Hamiltonian" as an open
  question. There were never two objects to transform between. The question is
  withdrawn rather than answered, and said to be withdrawn.
- **Least action is recovered honestly**, as Prop. 6.1: in the tropical semiring
  the path weight is an additive action functional and the transition weight is
  its *minimisation over paths*. Least action is not an approximation to the sum
  over paths here; it **is** the sum over paths, computed where ⊕ = min. This is
  the one place the word "Lagrangian" is earned, and the text says so.
- **A well-definedness bug is fixed.** Both old chapters evaluated the map at
  "the most refined formula in the domain satisfied by the term," which
  presupposes a greatest element that need not exist. Now Def. 6.3,
  *refinement-complete decoration*, is an explicit hypothesis, and ch10's
  build recipe tells the implementer to check it — "if the most refined witness
  is not unique, the decoration is not well defined and the implementation will
  silently choose."

Also added: **dynamic decorations** (Def. 6.7), where each state carries a table
keyed by refining formulae and each rewrite updates it, per the stochastic
simulation note. The static map was too weak for what Parts V and VI need.

### C — conservation (ch07)

The old *Resource Conservation* theorem is retained as Prop. 7.1 under its real
name, **account bookkeeping**, with a remark (7.2) saying plainly that the proof
is its own critique: the backward rule was *defined* to credit what the forward
rule debited, so the proposition says only that the account is a function of the
extended state. It was previously billed as conservation of energy.

What replaces it, in order:

1. **No-arbitrage is the virtual token** (`virtual-token` §3). Energy is not a
   component of the account; it is a *valuation on the components*, and it exists
   exactly when conversion is path-independent. A system whose engines can be run
   round a cycle at a profit has no energy function at all.
2. **The Hodge split**, with `β₁` counting the degrees of arbitrage freedom, and
   the remark that arbitrage-removal is an *idempotent* reflection while the cost
   monad is *non*-idempotent — metering accumulates, arbitrage-removal saturates.
3. **First law** — ν conserved in the lossless regime.
4. **Second law** — friction makes ν a Lyapunov function, so the honest scalar is
   free energy, and starvation is an absorbing set. This is the join to the
   mortal-computation material and the book had no second law at all before.
5. **The ledger law** σ + κ = σ₀, which is what Prop. 7.1 was groping toward, with
   the crucial addition that it names the resource whose motion between two
   ledgers *constitutes* time. Keeps the label `thm:conservation` so nothing
   dangles.
6. **Conservation bounds erasure from below** (`history-endofunctor` §6) — the
   striking one. The naive expectation is that coarser forgetting buys
   conservation; the opposite holds. Conservation is a *lower bound on retained
   history*, set by the holonomy class.
7. **Landauer** — the history you may erase for free is exactly the history whose
   erasure preserves energy. The Hodge threshold and the Landauer threshold index
   one and the same cohomology class.
8. **A charge, not a substance**, and the generator demoted to two conjectures
   (`vtok` Conj. 8.1 and `History` §9), stated as conjectures and labelled as
   such. The framework's distinctive offer is that the two hats can be held
   apart — charge proven, generator open.
9. **Located authority.** The old `⟨P, A⟩` with one global account *is* the
   ambient-authority defect corrected in `finrho` §2.2, and the book inherited it
   verbatim. Fixed, and then Remark 7.14 draws out the payoff: locating a stack
   ties a temporal resource to a spatial surface, so the two can no longer be
   varied independently — a spacetime rather than a space × a time. The text flags
   this as the first point in the book where the Pledge's "rods and clocks must be
   inside the model" is discharged by a mechanism rather than asserted.

### D — repairs

- **ch01 / ch10 tables** now carry a **Status** column: `CON` construction,
  `THM` theorem with a hypothesis that can fail, `CNJ` conjecture. Per your call
  the rows are kept as claims rather than demoted wholesale; the conjectural ones
  are marked. ch10 adds a paragraph on what counting the column shows — the
  constructions are numerous and cheap, the theorems are the reason to care, and
  two of the three conjectures concern the same missing object.
  ch10's dictionary is split into three tables; as one table it overflowed a page.
- **ch09** kept and rewritten around the obstruction, and marked as unfinished in
  its opening remark ("the least finished thing in this part of the book…
  everything after §9.2 is a research programme, not a result"). The obstruction
  is `continued-gslt-cost-v2` §13's: interference *cancels* contributions to a
  shared outcome, and bisimulation is branching-time and refuses to pool runs, so
  the quantum reading pulls toward linear time — a reformulation, not a
  relabelling. Three routes out are named without choosing among them. The
  density-matrix section is retracted explicitly: a resource account is an ordered
  monoid tracking what may be spent, a density matrix is a state tracking what may
  be observed.
- **ch08** now carries one decoration value plus its reversal instead of
  `⟨K, w⁺, w⁻, c, A⟩`; where both an amplitude and a charge are wanted, take a
  product semiring. Its *Local vs. Global Accounts* section is retired — the
  global/local distinction is about where the arithmetic happens, and the question
  that matters is who is entitled to do it.
- **ch11** gains a *What This Part Does Not Establish* section (four negative
  results) and a `sec:discussion` label. The symplectic question is upgraded: σ
  against κ is a much more concrete conjugate pair than the old "current term +
  history = position + momentum".

### A — the Pledge

Light touch as agreed: four insertions, no equations, no new jargon left
unexplained.

- **ch02, Agency** — a coda on the difference between *what may be spent* and
  *who may spend it*, using Eisenhower and the House. Names ambient authority,
  promises the fix, and previews that the price of the fix is that space and time
  stop being independent.
- **ch02, Bootstrapping** — the section previously ended on "there is a canonical
  way to compress this information" and stopped. It now cashes that: Alice is
  carrying a *history*, compression means folding it, and the surprise is that
  conservation sets a *floor* on how much she must keep. Ends on what her
  aggressive forgetting actually costs her, which lands the point back on Bob.
- **ch03, The Blind Spot** — the "no perpetual motion, so there's a breathing
  protocol and a breeding protocol" passage now gets the second law: the conserved
  quantity is only conserved frictionlessly, friction turns it into free energy,
  and starvation follows. Closes on Lewis: whatever the angelic mode of being is,
  nothing that computes is going to have it.
- **ch04, new §4.2 *Which Computer?*** — the promise to "update your version of
  computation" is made repeatedly in the Pledge and never kept. Three generations:
  function, interaction, reflection; bisimulation arriving with the second;
  reflection making the fractal picture available and letting the model hold its
  own measuring apparatus. Includes the terminology rule — rho as r.h.o., the pun
  on coming after pi, never the Greek letter.

## Bibliography

Five new entries: `Noether`, `Hodge` (Lim's graph Hodge Laplacians), `Bennett`,
`Miller2006` (the ocap thesis), `TwoErasures`. `CE`, `vtok`, `finrho`, `CARho`,
`MC`, `History`, `Landauer` and `StochSim` were already present.

Note a pre-existing duplication left alone: `CostMonad` and `CE` are the same
note under two keys, as are `History` and (in the rho-life chapters) some of the
history citations. The new material cites `CE` and `History`.

## Flagged for you

1. **`two-erasures` is under-used.** It supplies the forgetting-vs-internalising
   distinction that ch08 now leans on and that the Pledge's rods-and-clocks
   complaint is really asking for, and it has the CCS/λ/rho probe profile that
   justifies the choice of rho. Right now it is cited twice. It may deserve its
   own chapter in Part I rather than being spread thin.
2. **The Overarching Introduction to The Turn** still previews Part I in the old
   terms. Out of the scope you set, but it is now the last place in the book that
   describes this material as a Lagrangian and a Hamiltonian.
3. **One typo fixed in passing**: `pledge_ch03`, "replication protocol
   (breading)" → "(breeding)", inside a passage that was being rewritten anyway.
   Others left alone, since you did not ask for a copy-edit: "peal back the
   narrative" (ch02), "the ones whe are privileged" and "a beginning, a middle,
   and and end" and "humanity's position is exhalted" (ch03), "Piraha" (ch03).
4. **`Semi` and `dec`** are new macros in the master preamble. `\dec` renders as
   `d`; if that collides with anything in a part I have not read closely, it is a
   one-line change.
