# Alternating-Layer ciGSLTs: Cuts, Decoherence, Reflection, and a Bisimulation Metric — Context Note II

*A companion to the "Interaction, Cost, and Physical Correspondences" note. Where
the first note opened the GR and QFT correspondences as curiosities, this one
records a single line of thinking that ties them together: that the
complex/real weight distinction is a **reflection** in the exact sense of the rho
calculus, that this reflection organises the decoherence story, the phase
structure, and a recursive (alternating $n$-level) construction, and that the
General Relativity reading becomes a concrete, real-layer optimal-transport
programme once surfaces are treated as processes. As before, nothing here is
claimed as theorem; the register is "curious correspondence worth investigating,"
in the spirit of Curry–Howard and of Girard's Geometry of Interaction. Citations
introduced here are flagged for verification.*

---

## 0. The throughline

One organising idea and one recurring obstruction run through everything below.

**The idea.** The split between *complex-weighted* dynamics (coherent, reversible,
interference-bearing) and *real-weighted* dynamics (classical, irreversible,
probabilistic) is not a parameter set on a single calculus. It is a **reflection**
between two layers, structurally identical to rho's reflection between processes
and names. The complex layer plays the role of the *quote* (the name); the real
layer plays the role of the *process*. Decoherence is the dynamics that crosses
this boundary, and it is exactly the cost endofunctor.

**The obstruction (the same shape, every time).** At each stage, the property that
makes an object *well-defined* is the property that *kills its interesting part*:

- built-in decoherence makes the LTS bisimulation-sound — and forbids the sharp
  Wigner's friend (§4);
- the interaction constructor's rigidity makes the reversible/irreversible cut
  immovable — and that immovability is what the sharp paradox needs to violate
  (§2, §4);
- the discount factor makes the bisimulation metric well-defined — and pushes the
  geometry toward an ultrametric with degenerate curvature (§7).

The resolution is the same each time: route the irreversibility / contraction onto
the **temporal** monoid ($\mathord{::}$, the clock) and leave the **spatial** /
coherent monoid ($K$, nearness) free. This is the single move that recurs.

---

## 1. Feynman diagrams and cut rules: two senses of "cut"

The slogan "Feynman diagrams coincide with cut rules" is right, but "cut" is
overloaded, and the precise version is stronger than the slogan.

- The **interaction cut** of the cost paper, $K(\Kp(I,C_p),\Ke(J,C_e))
  \rightsquigarrow K'(C_p',C_e')$, is a *redex* — one reduction step, one local
  contraction $\mathrm{compute}(I,J,C_p,C_e)$.
- The **Gentzen cut** is *composition*: joining two proofs along a shared formula.

A Feynman diagram needs both, mapped to different parts of the diagram:

- **Vertices = interaction cuts.** A vertex is where two introductions meet along
  matched surfaces and their continuations combine.
- **Internal lines / propagators = Gentzen cuts.** A propagator is a residual
  produced by one vertex that becomes the surface consumed by another — concretely
  a **shared (located) surface** $S(I,\,\cdot\,)$.

So the defensible statement is: *a Feynman diagram is a morphism built by composing
interaction cuts, and evaluating the diagram is cut elimination.* This is the
GoI / categorical-quantum-mechanics identity; its sharpest published form is the
Baez–Stay **Rosetta Stone**, which tabulates Feynman diagrams, proofs, and
programs as arrows in monoidal categories with composition $=$ cut. (Note: Mike
Stay is a co-author — the frame is already inside the house.)

**The payoff: the internal line is the single locus of everything hard.** A
propagator is a *shared* surface, i.e. the contended located stack under an AC
$K$. Therefore:

- **No internal lines = a tree** = the $\lambda$ / rigid regime: each residual
  consumed once (linearity), surface structural and unforgeable, object-capability
  ideal. Tree-level $=$ tropical/cost reading $=$ $\hbar\to 0$ saddle point $=$
  classical $=$ where bisimulation is sound.
- **Internal lines / loops = the rho / AC regime**: a surface shared, competition
  admitted, histories recombine.

Hence **interference, entanglement, the qCCS congruence failure, and the
ambient-authority leak are all "there is an internal line," seen four ways.** The
"local move, global reach" knot of the first note is not a knot but a *location*:
it is the propagator. **Loop order = surface-sharing = departure from the ocap
ideal.** In linear-logic terms the multiplicative fragment is tree-level (MLL
proof nets are essentially acyclic; the correctness criterion forbids vicious
cycles), and the **exponentials** ($!$, replication, recursion) are where loops
live.

Two further identifications:

- **The signature requirement is a vertex selection rule.** Needing the matching
  signature resource to fire a cut is charge conservation at the vertex: the
  signature is a gauge charge, distinct token kinds are distinct flavours, and the
  resource-sufficiency condition is the Feynman rule that licenses the vertex.
- **Confluence is microcausality.** The diamond commuting independent redexes is
  "space-like vertices commute"; the cases where reduction order matters are the
  *contended* (time-like) surfaces. The amplitude's invariance under vertex
  reordering is confluence; the sum-over-diagrams arises from **non-deterministic**
  resource choices (the deterministic $\lambda$-regime has one diagram, no sum,
  tree-level, classical).

---

## 2. Factoring the interaction DAG: reversible vs irreversible loops

The DAG inside interaction is not "loops" undifferentiated. There are **Feynman
loops** (virtual, coherent — the integral one performs to evaluate a vertex) and
**control-flow loops** (recursion, replication — a server cycling to its ready
state). They separate cleanly, and the factorisation is the key refinement:

> **All reversible/unitary loops fold inside interaction; irreversible loops need
> not.**

Read as physics this is the **GKSL / Stinespring split realised in syntax**: the
generator of an open system factors into a reversible part $-i[H,\cdot]$ and an
irreversible dissipator $\mathcal D$; equivalently, every channel dilates to
*unitary on a larger space, then partial trace*. Here the reversible core is the
dilated unitary (the annihilation rule's internal compute supplying the ancilla),
and the **cell-pop is the partial trace** — the net interaction firing is where
the which-path bit is written to the consumed stack and the off-diagonals die.

**Criterion: net phlogiston around the cycle.** Zero net consumption (the stack
returns to its configuration — borrow-and-return, the Fock ladder up then down) is
reversible and absorbable into a vertex; nonzero net consumption is irreversible
and stays at the LTS. This is computable from the trace — the same instrument the
first note nominates as the decoherence detector. **The factorisation is read off
the metering, not laid on top of it.**

This **corrects the tree-vs-DAG framing.** The obstruction to bisimulation is not
loops, it is *reversible* loops. Bisimulation handles irreversible cyclic LTSs
natively — coinduction is built for exactly that; a recursive process is a cyclic
LTS and bisimulation is its home. What bisimulation cannot represent is coherent
recombination with cancellation. So decoherence is the passage
DAG $\to$ *DAG-with-no-reversible-loops*, **not** DAG $\to$ tree; acyclicity
(termination) is a *separate axis* the first note conflated with decoherence.

The linear-logic gloss makes "non-reversible" concrete: **weakening is
decoherence** (discarding a resource $=$ dropping the phlo record $=$ the trace-out
$=$ "forgetting the stack"). **Contraction** (unbounded reuse) is where
irreversible loops come from — the bang, replication, recursion. So: irreversible
loops $=$ the exponential; reversible loops $=$ the multiplicative-but-cyclic core;
the cell-pop where weakening happens is the boundary. This refines §1's "internal
line = where interference lives": only *reversible* internal lines carry
interference. A net-consumed shared surface is a classical race, not an
interference channel. **Interference needs both sharing and reversibility — two
axes, not one.**

**Why the factorisation is "by construction."** The worry — that the
reversible/irreversible cut is a gauge choice (the Heisenberg cut is mobile, the
Stinespring dilation non-unique) — is the right instinct for a continuum open
system and the *wrong* instinct for a syntactic one. There, interaction is a
partition drawn across a state; here, interaction is a **constructor**. $K$ is a
node in the term; there is no orbit to quotient because the grammar already printed
it. More strongly: the interaction-cut presentation has exactly **one cost-bearing
rewrite rule** (the cut), and it is irreversible (pops a cell, stamps which-path,
raises entropy); the only other dynamics is $\equiv$, whose loops are *trivial*
(return to the same class having computed and recorded nothing). A reversible loop
that *carries content* therefore has nowhere at top level — not $\equiv$
(content-free), not a $K$-firing (net-consuming) — and must live
$\mathrm{compute}$-internal. So **"reversible $\subseteq$ compute-internal" is a corollary of "one
irreversible rule plus one trivial-loop equivalence,"** and the boundary is
immovable not because $K$ is conveniently marked but because the rewrite system
leaves no other slot for coherence.

Consequence: §3's obstruction does not get *solved*, it fails to *arise*. The LTS
*is* the relation of $K$-firings; every transition is irreversible and
which-path-recording; all coherent recombination is compute-internal, strictly
below the transition relation. Bisimulation-soundness carries no proof obligation.

**The one residual question** relocates to *existence* of the local presentation:
which interactions can be put in interaction-cut shape with $\mathrm{compute}$
genuinely **local**? A reversible correlation that cannot be localised inside a
single $K$ because it spans vertices is a **non-Markovian** effect — the one thing
the by-construction factorisation cannot absorb (the presentation fails to exist,
not the boundary wobbles). Markovianity is the existence condition.

---

## 3. The typing of weights: a trichotomy, not a dichotomy

**Complex-weighted ciGSLTs describe interaction dynamics** — the quantum analogue
of the annihilation rule. **Real-weighted ciGSLTs describe the classical coarse
LTS.** Stated this way, the §3 "obstruction" of the first note was a *type error*:
demanding branching-time bisimulation of the layer where complex interference
lives is asking the wrong equivalence of the wrong layer. Typed correctly, each
equivalence applies where it is sound — amplitude / linear-time semantics at the
fine (complex) layer (interference is its *job*, not a defect); branching
bisimulation at the coarse (real) layer (decohered by construction, hence sound).
The loop/tree reading of §1 is then a *weight assignment*: reversible loops inside
interaction carry complex weights; the tree-level coarse LTS carries real ones.
**Loop content $=$ interaction-internal content $=$ complex content, one cut.**

**But mixed $\ne$ real.** Collapsing mixed states with classical probabilities
loses the object the RG programme needs. A *classical* distribution is a
**diagonal** density matrix — $\mathbb R^+$ over a fixed pointer basis, genuinely
real-weighted, the coarse layer. A general *mixed* state is non-diagonal: it still
carries off-diagonal coherences that are not $\mathbb R^+$ weights over any basis.
The clean picture is a **trichotomy**:

| Regime | Weights | Object |
|---|---|---|
| pure / unitary / interference | **complex** | interaction-internal ciGSLT |
| mixed / CPTP channel / residual coherence | **operator** | CPM (Selinger doubling) — *the missing middle* |
| classical / stochastic / diagonal $\rho$ | **real** | the coarse LTS |

The decoherence flow is $\textsf{complex} \xrightarrow{\text{double}}
\textsf{CPM} \xrightarrow{\text{diagonalise}} \textsf{real}$: pure $\to$ mixed by
tracing in the environment, mixed $\to$ classical by killing off-diagonals. The
two named buckets are the endpoints; the **partially-decohered** regime lives at
the **operator/CPM** level. The scale-invariant fixed point of the annihilation-
rule RG flow — the thing the energetics note wants to chase — *cannot* be
real-weighted (that is the trivial IR endpoint); a nontrivial self-similar regime
that neither fully decoheres nor stays fully coherent is by definition partially
coherent, i.e. **operator-weighted**. The most interesting object in the programme
is the one the dichotomy has no name for. (CPM is in the Abramsky–Coecke lineage
already cited; Coecke–Kissinger's doubling-then-decoherence hierarchy is the
diagrammatic form of exactly this three-level structure.)

**The dial becomes a functor.** The first note's "semiring is the dial" treated
$R$ as a static choice. The passage *between* levels is the content, and it is not
a re-setting of the dial but a **functor**: double-then-decohere (CPM doubling
followed by the diagonalising decoherence map). This is the quantum analogue of the
cost endofunctor.

> **Open target.** Is there a *quantum cost endofunctor* whose action on a
> complex-weighted ciGSLT is the CPM doubling and whose coarse-graining (the
> cell-pop / which-path forget) is the diagonalisation — making open-system
> evolution a composite of two functors in the cost paper's own idiom, with the RG
> flow as its iteration?

**The bisimulation spectrum falls out.** The complex/fine layer is sound only up to
trace (linear-time) — interference masks branch differences; the real/coarse layer
is sound up to bisimulation (branching-time) — decoherence has exposed them. So van
Glabbeek's two endpoints are the two ends of the decoherence flow: **trace at the
coherent UV end, bisimulation at the decohered IR end.** The RG step
*coarse-grains the dynamics* (fewer degrees of freedom) while *refining the sound
equivalence* (interference no longer masks distinctions); these are statements
about different objects (the dynamics and the equivalence on it), so they do not
conflict. What the flow tracks in the *middle* of the spectrum is open — and it is
the same open question as the operator-weighted middle.

---

## 4. MQ-calculus and Wigner's friend

The MQ-calculus (Stay & Meredith) is a concrete witness for the factorisation. Its
process weights are $\mathbb R^+$; the complex structure lives entirely in the
*state* sort (superposition, scalar, unitaries), and COMM is the map that consumes
it:
$$
x!(S)\ \mid\ \mathbf{for}(m\leftarrow x)\{P\}
\ \rightsquigarrow\ \sum_r |\langle r|S\rangle|^2\, P\{r/m\},
$$
the Born projection $\mathbb C\to\mathbb R^+$ applied *in the act of
communicating*. So MQ is **not** a complex-weighted ciGSLT: it is the
$\mathbb R^+$ instance of the graded ciGSLT, carrying a $\mathbb C$-linear state
datum, with Born-at-COMM as the coupling. This is exactly **option (b)** of the
first note's §5 ("unitary evolution *beside* the interaction relation, decoherence
identified with the interaction step"), now concrete, and it answers the note's own
diagnostic: the unitaries appear as a distinct *state-layer* reduction
($\llbracket U;P\rrbracket = \llbracket P\rrbracket(U\psi)$), not as transitions in
the relation bisimulation ranges over — so **decoherence is genuinely built in**,
the good case. MQ never exhibits $\mathbb C$-weighted *branching*: coherent
evolution (closed, no COMM) **xor** COMM (Born, $\mathbb R^+$, branching), never
both. That is the precise sense in which it *sidesteps* §3 rather than solving it —
and it sidesteps it via the factorisation.

**Wigner's friend.** MQ makes cut placement *syntactically explicit*, so the
**naive** paradox dissolves: MWI is one program (all unitaries inside one output
state, no COMM), Copenhagen another (nested $\mathbf{for}$, COMM at each), "either
may be executed." Good clarification — but the **sharp** paradox (Frauchiger–
Renner; the Local Friendliness no-go of Bong–Wiseman et al.; Brukner) turns on
precisely the freedom that dissolution uses. Its force is that the modeller may
*not* freely fix the program: Wigner treats the friend's measurement as a
reversible unitary on the whole lab while the friend treats the same event as a
collapse, both apply to the *same run*, and they disagree on a *jointly checkable*
statistic. MQ cannot host this, for an instructive reason: the property that makes
its LTS bisimulation-sound — Born at *every* COMM — is what forbids Wigner from
measuring the friend's lab in a coherence-revealing basis. Built-in decoherence and
modelling-the-sharp-paradox are incompatible **in one fixed ciGSLT**.

So the calculus-native statement, stronger than "two valid programs":

> The reversible/irreversible boundary is immovable *within* a single ciGSLT (§2);
> the sharp Wigner's friend is the assertion that **no single ciGSLT refines two
> observers' incompatible cuts over one run.** The friend's calculus has a COMM
> where Wigner's has a reversible unitary; the paradox is the non-existence of a
> common refinement, and the Local Friendliness inequalities are the experimentally
> checkable witness that it fails.

To *analyse* rather than dissolve, one needs either (a) a **reversible COMM**
(Wigner un-measuring the friend) — which breaks built-in decoherence and pushes
straight into the complex-weighted / linear-time regime; or (b) **reify the cut as
data** and test when two cuts agree on traces — which is the decoherence-functional
diagonality condition (consistent histories) *and* the Wells / Leifer–Milner–Sewell
"experiment $=$ context, bisimulation $=$ congruence" program read across two
observers. That these coincide is the payoff, and it ties to the qCCS
non-congruence residue (Ying). The continuum gap the draft flags is the obstacle:
the Wells construction must be made to yield a *congruence over the
$\mathbb C$-state continuum* before two cuts can be compared rather than chosen
between.

---

## 5. Field strength makes the two-level structure a phase

If the internal theory depends on a background occupation, $\tilde G(\phi)$, then
"has two-level structure" becomes "has it at field strength $\phi$," and the
characterisation turns from a *property* into a **phase**. The Higgs remark is the
mechanism, not an aside.

- The **order parameter / condensate is the located stack** $S(I,\,\cdot\,)$. Its
  presence is the broken phase.
- The **broken symmetry is the AC permutation symmetry of $K$** — the bag
  relabelling that, unbroken, makes every process interchangeably adjacent to every
  stack. *That symmetry is ambient authority.* Unbroken (symmetric, AC, no
  condensate): everything near everything, long-range, no capability discipline.
  Broken (located condensate present): only matching surfaces interact,
  short-range, capability discipline. **The object-capability discipline is the
  symmetry-breaking condensate; ambient authority is the restored (high-energy)
  phase.** "Local move, global reach" is the symmetric phase of a Higgs-like
  transition.

Direction (corrected to the physical sign): locality of the dilation $=$ a vertex's
environment is nearby $=$ short-range interactions $=$ *massive* mediators $=$
condensate **on**. So the clean two-level structure is the **broken, massive,
short-range, IR phase**; it *fails* in the **symmetric, massless, long-range, UV
phase**, where mediators are massless, a vertex's environment is everywhere, and
the dilation de-localises. This matches the RG arrow: UV coherent/non-local, IR
decohered/local. "Only working at certain energy ranges" $=$ "valid below the
symmetry-breaking scale."

- **"Different field strengths at different nearnesses" is the Yukawa mass
  spectrum:** different signatures couple to the *one* common condensate with
  signature-dependent strength — different effective nearness-cost, i.e. different
  mass and range. Not different vevs; different couplings to one vev.
- **The massless residue is the photon:** a signature that does *not* couple to the
  condensate stays massless, long-range, global, capability-free even in the broken
  phase — the residual ambient-authority channel that survives the capability
  discipline. If the analogy holds, **such a global capability must exist** (the
  unbroken $U(1)$).

**Rigour line.** All of this earns the word *phase* only if the breaking is
**spontaneous** — a phlo-potential over occupations with a degenerate minimum (a
Mexican hat), so the condensate forms because the symmetric configuration is
unstable, not because a knob was turned. A convex potential gives only *explicit*,
parametric $\phi$-dependence: a real but duller characterisation (indexed by
$\phi$, no transition, no critical point). The interesting Higgs version — with a
genuine phase boundary, the scale-invariant fixed point, the operator-weighted
middle — requires the non-convex potential. **Whether the phlo-potential has that
shape is the one thing the energetics framework could actually decide**, and the
thing to settle before writing any of this down as more than a correspondence.

---

## 6. The alternating-layer recursion (the central structure)

Instead of two levels, **alternating $n$-level ciGSLTs**, with the same enforcement
mechanism as rho.

In rho the alternation (process $\to$ name $\to$ process $\to\dots$, "never two
quotes in a row") is forced by typing: the only sort-changing maps are quote
$\widehat{(\cdot)}:\mathrm{Proc}\to\mathrm{Name}$ and its drop; there is no
$\mathrm{Name}\to\mathrm{Name}$ former, and $\widehat{\widehat P}$ is unformable.
The same two facts hold for the weight layers:

- **Same-type layers fuse.** Coherent-after-coherent is coherent
  ($\mathbb C\otimes\mathbb C$: unitaries compose to a unitary); classical-after-
  classical is classical ($\mathbb R^+\otimes\mathbb R^+ = \mathbb R^+$). Two
  adjacent complex layers *are* one; two adjacent real layers *are* one.
- **A genuine level boundary therefore requires a change of scalar** — the quote or
  the drop. **"No @@" is the coherent-composition law** ($\mathbb C$-in-$\mathbb C$
  fuses) in syntactic dress.

**Sort assignment: complex $=$ quote/name, real $=$ process.** This puts
decoherence in the right place. With complex $=$ quoted name, quote/drop is
**lossless reification**, not decoherence: $\widehat{C}$ contains all of the complex
ciGSLT $C$, and $\widehat{C}$ dropped is $C$, a structural law, no accounting. The
**lossy** step is the *real-layer COMM firing* — running a quoted complex interior
to a definite outcome — which is exactly where the cost endofunctor lives. Metering
the cut *is* recording which name was dropped. So the quantum cost endofunctor
(decohere $\circ$ CPM, §3) is the *dynamics* of the real layer (the analogue of
COMM), cleanly separate from the *structural* reflection (quote/drop), as COMM is
separate from $\equiv$. Complementarity is quantitative: the which-path log is the
distinguishability, the un-consumed interiors keep the visibility, Englert bounds
their sum.

**Anomalies = the two ways same-type layers fail to fuse:**

- $\mathbb R$-in-$\mathbb R$ that will not glue into one classical process $=$
  **contextuality** (no global joint distribution over the sub-statistics);
- $\mathbb C$-in-$\mathbb C$ that will not compose into one coherent process $=$
  **non-Markovian memory** (coherence reaching across a level it should have been
  sealed inside).

Hence **clean alternation $\iff$ no contextuality at the classical levels $+$
Markovianity at the quantum levels $\iff$ the dilation is local-and-self-similar at
*every* level** — §2/§5's two-level criterion, now stated for the whole tower. The
two-level case is $n=2$, one quote; the $n$-level tower is the RG depth (each quote
$=$ one rung of van Glabbeek's ladder $+$ one annihilation-rule coarse-graining);
and the reflexive fixed point $n\to\infty$ is the **all-scales self-similar
object** — the scale-invariant fixed point of the decoherence flow, existing only
in the anomaly-free regime.

**Why this must be option (c), not (b).** What makes rho higher-order is that a name
carries a *process* (reflection), not a bolted-on data sort; the same reflection is
what lets the tower recurse. MQ's complex stratum is a Hilbert-space datum — a name
with no process inside — so it truncates at $n=1$ and cannot climb. The alternating
construction is the completion: make the interior a genuine complex-weighted
ciGSLT, and rho's reflection gives the tower its recursion.

### 6a. The bootstrap signature

The minimal base move collapses the tower into one mutual recursion:
$$
I^{\mathrm{real}} = \mathrm{ciGSLT}\bigl(K^{\mathrm{real}}, K'^{\,\mathrm{real}},
  K_p^{\mathrm{real}}, K_e^{\mathrm{real}}, \mathrm{near}^{\mathrm{real}},
  I^{\mathrm{complex}}, \mathrm{Tokens}^{\mathrm{real}}\bigr),
$$
$$
I^{\mathrm{complex}} = \mathrm{ciGSLT}\bigl(K^{\mathrm{complex}},
  K'^{\,\mathrm{complex}}, K_p^{\mathrm{complex}}, K_e^{\mathrm{complex}},
  \mathrm{near}^{\mathrm{complex}}, I^{\mathrm{real}},
  \mathrm{Tokens}^{\mathrm{complex}}\bigr).
$$
Writing $\Phi_w(-)$ for "the $w$-weighted ciGSLT whose surfaces are drawn from
$(-)$," this is $I^{\mathrm{real}}=\Phi_{\mathbb R^+}(I^{\mathrm{complex}})$,
$I^{\mathrm{complex}}=\Phi_{\mathbb C}(I^{\mathrm{real}})$, each a fixed point of the
two-step composite $\Phi_{\mathbb R^+}\!\circ\Phi_{\mathbb C}$ (one quote/drop
cycle). It is rho's reflexive object $\mathrm{Name}=\widehat{\mathrm{Proc}}$ with
the two sorts coloured by two scalars. The **surface slot** is the *only* parameter
pointing at the other layer; everything else is intra-layer.

**The cross-drawing fixes each `compute`, and they come out dual.**

- $\mathrm{compute}_R$ takes complex surfaces, lands in real: $\mathbb C\to
  \mathbb R^+$, a complex thing read into a classical record — **measurement** (a
  dagger-effect $A\to I$).
- $\mathrm{compute}_C$ takes real surfaces, lands in complex: $\mathbb R^+\to
  \mathbb C$, classical data conditioning coherent evolution — **controlled
  preparation** (a dagger-state $I\to A$).

Measurement and preparation are dagger-adjoint, so $\mathrm{compute}_R
\dashv^\dagger \mathrm{compute}_C$ and **the dagger-compact structure the QFT/CQM
reading leaned on is *generated* by the recursion, not assumed** — the construction
supplies its own scalars and its own dagger. The pattern (measure, feed the outcome
forward to choose the next coherent operation, evolve, measure) is **adaptive
measurement-based computation** (Raussendorf–Briegel one-way style); the reflexive
limit is **self-referential MBQC** — a name carrying a measurement-based
computation that measures names — which is universal.

**The quantum subtlety enters `near` at the *real* layer.** Real-layer
$\mathrm{near}^{\mathrm{real}} : I^{\mathbb C}\times I^{\mathbb C}\rightharpoonup
\mathrm{Surf}$ matches two *complex* surfaces, so it is defined exactly when they
are **co-measurable** — when the corresponding observables commute. Complex-layer
$\mathrm{near}^{\mathrm{complex}}$ matches two *real* surfaces (classical control) —
total/tame. This *inverts* the naive expectation: unitary evolution is
deterministic and composes without obstruction; complementarity, Kochen–Specker,
and contextuality are statements about *measurement* statistics, and they land at
the only nontrivial `near`. **Contextuality is the failure-pattern of
$\mathrm{near}^{\mathrm{real}}$** — the complex surfaces that refuse to co-measure,
the commutativity graph with no consistent global section. The partiality of
`near` carried since the capability section *is* the carrier of the quantum
content. This demotes "no @@" from an enforced predicate to a **typing**:
$\mathbb C$-in-$\mathbb C$ at a surface position is simply unformable, as
$\widehat{\widehat P}$ is; the physics it forbade has moved to where it belongs —
the partiality of `near` (contextuality) and the locality of compute-memory
(Markovianity), both conditions on *operators*, not syntactic side-conditions.

**Tokens straddle the cut.** Each layer keeps its *own* token monoid (different
semirings — the dial as two grading objects). But a located stack lives at a
surface, and a surface of one layer is a term of the other, so the stack that funds
a crossing is keyed to the *opposite* layer:
$$
S^{\mathbb R^+}\bigl(I^{\mathbb C},\, s::S'\bigr)
\quad\text{(real tokens at a complex surface — measurement / which-path ledger),}
$$
$$
S^{\mathbb C}\bigl(I^{\mathbb R},\, \cdots\bigr)
\quad\text{(complex tokens at a classical surface — preparation).}
$$
The resource is real (the $\mathbb R^+$ which-path ledger) but spent *at* a complex
surface (the coherent interior being measured): **phlogiston funds the climb**, and
the located condensate **straddles the level boundary by construction** — label in
one layer, denomination in the other — which is why it can be the order parameter
for a transition *between* levels (§5).

### 6b. Three coherence conditions (definition vs. theorem)

The two equations are complete as a *signature*; degenerate instances satisfy them
(e.g. $\mathrm{near}^{\mathrm{real}}$ total and both computes reversible — the
cosmetic-colouring collapse to plain reflexive rho). What makes stage one a theorem
is the signature **plus**:

1. $\mathrm{compute}_R \dashv^\dagger \mathrm{compute}_C$ (the dual pair; the
   structural reflection $\widehat{(\cdot)}/{\sim}$ is the unit/counit of this
   adjunction);
2. $\mathrm{dom}(\mathrm{near}^{\mathrm{real}}) =$ the commutativity relation on
   $I^{\mathbb C}$ (contextuality has a home, and only that home);
3. located stacks cross as $S^{\mathbb R^+}(I^{\mathbb C},-)$ /
   $S^{\mathbb C}(I^{\mathbb R},-)$ (resource funds the climb; condensate straddles
   the cut).

**Pin (2) first** — it turns the partiality carried since the capability section
into the source of the quantum content, and once fixed, the dagger forces (1) and
(3). The remaining first-turn question recurs at the tower level: is "anomaly-free"
the right reading of "no @@," or does the reflection collapse some anomalies on its
own the way $\equiv$ does in rho (the above-or-below-the-congruence question)?

---

## 7. A bisimulation metric and analogs of the Einstein equations

Surfaces are processes, so they carry a behavioural pseudometric for free
(Desharnais–Gupta–Jagadeesan–Panangaden: $d(P,Q)\in[0,1]$, $d=0$ iff bisimilar, a
Kantorovich lift of the successor-metric to a metric on distributions, the fixed
point of a contractive functional). Three adjustments turn it into something that
can carry a field equation; one reorients the picture.

**Metrise the match *defect*, not similarity.** Bisimilar processes are
behaviourally identical and need never interact; $a$ and $\bar a$ interact
*because* they differ. Nearness for interaction is **duality**, not sameness. Using
the dagger of §6, the right quantity is $d(I^\dagger, J)$ — zero on perfect
matches, growing as the match degrades — defined inside a **cost-bounded
neighbourhood** of perfect match. The neighbourhood radius is an interaction range,
funded by the located stack (the tropical reading): **range and resource are the
same dial.**

**From pseudometric to curvature.** A field equation needs an infinitesimal line
element, i.e. a Hessian of $d^2$ near the diagonal. This needs (i) a *length*
metric — $d$ as an infimum of path-costs, the phlo/tropical reading, so finite
nearness is geodesic; (ii) the Kantorovich lift with **quadratic** cost $W_2$ (not
$W_1$): $W_1$ gives a metric but no Riemannian shadow, $W_2$ gives **Otto's** formal
Riemannian structure on distributions, which makes $d(x,x+dx)^2 \approx
g_{\mu\nu}\,dx^\mu dx^\nu$ meaningful — *rebuild the bisimulation metric with the
2-Kantorovich lift*; (iii) a reference measure — the **phlo distribution**
$\phi:\mathrm{Surf}\times\mathrm{Sig}\to R$, i.e. the stress-energy of the first
note. **Geometry from bisimulation-$W_2$; matter from phlo.**

**The signature is Lorentzian, and it is the two monoids.** Bisimulation distance is
symmetric/positive (Riemannian-flavoured); GR is Lorentzian. The cone comes from
the *other* monoid. Spatial metric (symmetric, $W_2$-bisimulation) $=$ the $K$
monoid; causal order $=$ reduction $\rightsquigarrow$, directed and irreversible
$=$ the $\mathord{::}$ monoid (the arrow of computation). A **Lorentzian length
space** (Kunzinger–Sämann) is a set with a causal relation plus a time-separation
$\tau$; here causal future $=$ reduction-reachable and $\tau(x,y) =$ the *maximal*
phlo along causal paths. Note the extremal flip: spatial `near` **minimises** cost
(tropical $\min$), proper time **maximises** (free fall is longest-aging, maximal
phlo). **Min over space, max over time** — the two monoids, with the Lorentzian
sign already in which extremum each takes. (Confluence $=$ microcausality, §1, is
the same statement: space-like configurations commute, time-like ones do not.)

**The field equation is a convexity, not a PDE.** The space of processes-mod-
bisimulation is infinite-dimensional and non-smooth — exactly the synthetic case.
Lott–Villani and Sturm define Ricci lower bounds on a metric-measure space by
**displacement convexity of entropy along $W_2$-geodesics** (curvature from a
metric and a measure, no charts). The Lorentzian, matter-coupled version is in
hand: Mondino–Suhr give an optimal-transport formulation of the Einstein equations,
and McCann's theorem says displacement convexity of Boltzmann entropy
*characterises* the strong energy condition of GR (matter curves spacetime).
Instantiated:

> **Einstein's equation $=$ the statement that the phlo measure controls the
> convexity of phlo-entropy along timelike $W_2$-geodesics of the bisimulation
> metric** — the timelike direction being the reduction order, the entropy the
> Boltzmann entropy of the token distribution. "Stress-energy $\leftrightarrow$
> metric" becomes a provable-or-refutable inequality.

The **local** counterpart, for a PDE-shaped object: **Bakry–Émery** — the
$\Gamma_2$ / Bochner identity of the **generator of cost-metered reduction** (the
infinitesimal cost endofunctor as d'Alembertian) gives a local Ricci object, and
$\Gamma_2 \ge K\,\Gamma$ with $K$ set by phlo is the local field equation. This is
the GoI / operator-algebra lineage cashing out: the generator is the operator,
Bochner is its curvature.

**The obstruction (same shape).** Bisimulation metrics are well-defined via a
*discount* $c\in(0,1)$ making the functional a contraction (Banach). But
discounting shrinks distance geometrically with depth, pushing the geometry toward
**ultrametric** (self-similar, totally disconnected) — and ultrametric spaces have
*degenerate curvature*. The contraction that gives the metric fights the geodesic
structure that gives curvature. **Resolution (the recurring move): discount the
*temporal* direction ($\mathord{::}$), not the spatial one (`near`).** This is where
irreversibility belongs, it leaves the $K$-metric undiscounted and able to be
geodesic, and the discount factor becomes a **redshift / proper-time attenuation**.
Whether the undiscounted spatial fixed point exists is the technical crux.

**Where the geometry lives.** Bisimulation is sound only at the *real* layer
(post-decoherence, branching-time). So the construction yields geometry on the
real-weighted ciGSLT: **classical GR, by construction** — correct, since spacetime
is a decohered, classical phenomenon. Geometry at the *complex* layer would need a
metric on interference / linear-time, the object we established has no good
branching pseudometric. **So "why is quantising gravity hard" gets a calculus-
native answer: the metric apparatus is intrinsically a real-layer, branching-time
construct, and the complex layer where it would have to live is the one without a
behavioural metric.** The clean, reachable target is classical Einstein at the real
layer via bisimulation-$W_2$ displacement convexity; the hard residue sits exactly
where the whole arc says it must — at the boundary the quote crosses.

---

## 8. What to settle first

In rough dependency order:

1. **The phlo-potential's shape** (§5). Convex $\Rightarrow$ explicit
   $\phi$-dependence (a duller characterisation); non-convex/degenerate
   $\Rightarrow$ a genuine Higgs phase, the scale-invariant fixed point, the
   operator-weighted middle. The energetics framework is what can decide it. This
   gates the "phase" language everywhere.
2. **The $\mathrm{near}^{\mathrm{real}}$ domain $=$ commutativity** (§6b, condition
   2). Pinning this turns carried partiality into the quantum content and forces
   the dagger pair and the token-crossing.
3. **The dual computes** $\mathrm{compute}_R \dashv^\dagger \mathrm{compute}_C$
   (§6a). Without genuinely different (lossy vs reversible) computes the scalar
   colouring is cosmetic and the tower collapses to plain rho.
4. **The quantum cost endofunctor** $=$ decohere $\circ$ CPM (§3). Whether
   open-system evolution composes from two functors in the cost idiom, with the RG
   flow as iteration and the fixed point in the operator-weighted middle.
5. **Wells over the continuum** (§4). A congruence on the $\mathbb C$-state
   continuum is the prerequisite to *analysing* (not dissolving) the sharp Wigner's
   friend — comparing two cuts rather than choosing between them.
6. **The undiscounted spatial fixed point** (§7). Discounting time not space is the
   move; its existence is the crux for the whole GR programme.

---

## 9. References introduced here (to verify)

Several are already in the companion note's bibliography; the following are the new
anchors of this line of thinking. Exact bibliographic details should be confirmed
before use.

**Diagrams, composition, and the dagger:**

- J. Baez and M. Stay. *Physics, Topology, Logic and Computation: A Rosetta Stone.*
  In B. Coecke, ed., *New Structures for Physics*, Lecture Notes in Physics 813,
  pp. 95–172. Springer, 2011. (arXiv:0903.0340.) — the canonical "diagram $=$
  composite of cuts; evaluation $=$ cut elimination."
- P. Selinger. *Dagger compact closed categories and completely positive maps.*
  Electronic Notes in Theoretical Computer Science 170:139–163, 2007. — the CPM
  (doubling) construction; the operator-weighted middle.
- B. Coecke and A. Kissinger. *Picturing Quantum Processes.* Cambridge University
  Press, 2017. — doubling-then-decoherence as a diagrammatic hierarchy.

**Wigner's friend / observer no-go:**

- D. Frauchiger and R. Renner. *Quantum theory cannot consistently describe the use
  of itself.* Nature Communications 9:3711, 2018.
- K.-W. Bong et al. *A strong no-go theorem on the Wigner's friend paradox.* Nature
  Physics 16:1199–1205, 2020. (Local Friendliness.)
- Č. Brukner. *A no-go theorem for observer-independent facts.* Entropy
  20(5):350, 2018.
- B. Englert. *Fringe visibility and which-way information: an inequality.* Physical
  Review Letters 77:2154–2157, 1996. — the distinguishability/visibility bound.

**Behavioural metrics and synthetic curvature:**

- J. Desharnais, V. Gupta, R. Jagadeesan, P. Panangaden. *Metrics for labelled
  Markov processes.* Theoretical Computer Science 318:323–354, 2004.
- F. Otto. *The geometry of dissipative evolution equations: the porous medium
  equation.* Communications in PDE 26:101–174, 2001. — $W_2$ Riemannian structure.
- J. Lott and C. Villani. *Ricci curvature for metric-measure spaces via optimal
  transport.* Annals of Mathematics 169:903–991, 2009.
- K.-T. Sturm. *On the geometry of metric measure spaces I, II.* Acta Mathematica
  196:65–177, 2006.
- R. McCann. *Displacement convexity of Boltzmann's entropy characterizes the
  strong energy condition from general relativity.* Cambridge Journal of
  Mathematics 8:609–681, 2020.
- A. Mondino and S. Suhr. *An optimal transport formulation of the Einstein
  equations of general relativity.* Journal of the EMS, 2023. (arXiv:1810.13309.)
- D. Bakry and M. Émery. *Diffusions hypercontractives.* Séminaire de Probabilités
  XIX, LNM 1123, pp. 177–206. Springer, 1985. — the $\Gamma_2$ criterion.
- M. Kunzinger and C. Sämann. *Lorentzian length spaces.* Annals of Global Analysis
  and Geometry 54:399–447, 2018. — causal/time-separation structure.
- R. Raussendorf and H. J. Briegel. *A one-way quantum computer.* Physical Review
  Letters 86:5188–5191, 2001. — adaptive measurement-based computation.

**Already in the companion note (recalled here for the spectrum/congruence
threads):** van Glabbeek (linear-time/branching-time spectrum); Larsen–Skou
(probabilistic bisimulation); Ying (qCCS, bisimulation-as-congruence);
Abramsky–Coecke (semiring of scalars, Born rule); Gell-Mann–Hartle / Griffiths /
Omnès (consistent histories); Litvinov–Maslov (tropical dequantisation).
