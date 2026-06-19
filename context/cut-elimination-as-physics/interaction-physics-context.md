# Interaction, Cost, and Physical Correspondences — Context Note

*A working collection of the deeper research threads developed around the
"Continued Interactive GSLTs and the Cost Endofunctor" paper. Intended as
seed context for a separate note that takes the physics seriously. None of
what follows is claimed as theorem; the register throughout is "curious
correspondence worth investigating," in the spirit of Curry–Howard.*

---

## 0. The shared substrate

Everything below rests on one structural picture, already in the cost paper:

- The **interaction cut** `K(Kp(I,Cp), Ke(J,Ce)) → K'(C'p,C'e)` with a
  contraction `compute(I,J,Cp,Ce)` (Milner's pseudo-application in the
  process-calculus instances).
- Two monoids running orthogonally:
  - **`K` is spatial** — the interaction constructor records *what is in
    contact with what* (associative–commutative in rho, free/positional in
    λ). It is the geometry: nearness.
  - **`::` (cons) is temporal** — the token stack is a free monoid (a
    list), never commutative, recording *what is spent next*. It is the
    clock; a reduction step pops a cell (a tick).
- **Cut elimination is the dynamics** that couples them: each interaction
  consumes two introductions, migrates information through `compute`, and
  leaves a contractum whose redexes have a *new* adjacency. What is near
  what is therefore not fixed but dynamical.
- **`near : Surf × Surf ⇀ Surf`** is a (partial) **operator on surfaces**,
  not a predicate: when defined it returns the surface `L` at which two
  surfaces meet; interaction is gated on its being defined, and the
  returned `L` locates the resource purse `S(L, …)`.
- **Phlogiston / token stack** grades transitions by the signature monoid
  `(Sig, *, ())`. A graded transition system is a transition system valued
  in a semiring — this is the hinge for the QFT reading.

The two physical correspondences (GR, QFT) are this one substrate viewed
two ways. The decoherence and interaction-hosts-compute material is about
what happens *inside* a single interaction when it is not atomic.

---

## 1. The general-relativity correspondence

**Statement.** The dependence of dynamics on geometry has the shape of
Einstein's intuition:

- In GR: the distribution of matter–energy determines the metric (what is
  near what); the metric governs how matter moves.
- Here: the distribution of information in the `K`-tree determines the
  **nearness structure** (which cuts are enabled); the nearness structure
  governs how information flows at the next step, which redistributes the
  information.
- Dictionary: stress–energy source ↔ information distribution; metric ↔
  nearness structure; the Einstein coupling ↔ cut elimination (each step
  lets the distribution reshape the geometry and the geometry select the
  next redistribution).

**The ambient calculus makes it vivid.** There the contraction is spatial
restructuring rather than substitution, so a step *literally* relocates
information in the spatial hierarchy; the change in adjacency is the whole
content of the step, not a side effect.

**Object capabilities sharpen, rather than dissolve, the analogy.**
Without an authority discipline, space (`K`) and time (`::`) are separable
coordinates. But the **located stack `S(I,−)`** ties a temporal resource
(what may be spent next) to a spatial surface (who is near): a draw then
requires *both* spatial proximity (`near(I,J)` defined) and temporal
availability (a token at the head of the stack), so the two structures can
no longer be varied independently. Authority **entangles** space and time
into a single coupled structure gating interaction — a **spacetime**. The
same structural feature, the equational theory of `K`, governs both the
geometry and its entanglement with time.

**Open question.** Whether the nearness evolution obeys anything like a
field equation. The located-purse coupling is the candidate place to look
for a "field equation for nearness."

---

## 2. The quantum-field-theory correspondence

**The move.** A token distribution is a collection of **fields** over the
space of surfaces. A token resident at a surface is the *strength of its
field there*; its signature is an internal **flavour index** distinguishing
one field from another. Since a surface may carry many tokens of different
keys (many strengths at one point), a token distribution is a map

> **φ : Surf × Sig → R**

valued in a **semiring R**, with `φ_s(I)` the strength of mode `s` at
surface `I`.

**The semiring is the dial.** Cost, probability, and quantum amplitude are
the *same construction over three different semirings*. The cost calculus
as developed is the shadow of this field in a degenerate semiring:

| Semiring R | Reading |
|---|---|
| Boolean `({0,1}, ∨, ∧)` | bare nondeterministic LTS (un-weighted calculus) |
| Tropical `(ℝ∪{∞}, min, +)` | **cost** |
| Non-negative reals `(ℝ≥0, +, ×)` | probabilistic interaction (no interference) |
| Complex `(ℂ, +, ×)` | **amplitudes** → interference, QFT-style dynamics |

- **Tropical = cost, and ties to the join.** The **minimum-cost perfect
  matching** that surface-valued `near` produces for joins (when `near` is
  permissive enough that distant surfaces may interact if a located stack
  funds the meeting) is *precisely* the tropical reading of the field:
  `min` over `+`-accumulated path costs.
- **Complex = amplitudes.** Born rule `|·|²` is a **separate extraction
  laid on top**; genuine probabilities require the dynamics to be unitary —
  a *quadratic* conservation law, strictly stronger than the *linear*
  phlogiston conservation (token-count) the paper already has. Unitarity is
  an extra demand on the rewrite rules, not a freebie.

**Maslov / Litvinov dequantization** sends the ℂ (or log-) semiring to the
tropical one as ℏ → 0. So the min-cost matching is literally the
**classical limit** (stationary-phase / saddle point) of the
complex-weighted path sum: the cost reading and the quantum reading are the
*same object at two temperatures*.

**Diagrammatic dictionary** (discrete path integral):
- interaction cut = a vertex (incoming legs `Kp`, `Ke`; `compute` at the
  vertex; outgoing residual legs);
- `K`-tree = the diagram skeleton;
- weighted cut elimination, **summed over reduction sequences** = a sum
  over histories;
- forcing-by-unwrapping = a lowering/annihilation operator on an occupation
  number;
- linear token supply (n copies of an s-redex need n cells keyed to s) =
  **bosonic counting**; the stack is a Fock occupation, cons is the ladder.

---

## 3. The obstruction: bisimulation vs. interference

This is the honest hard part, and the reason the quantum reading is
deferred rather than asserted.

- **Bisimulation is branching-time.** It is the equivalence the whole paper
  (and OSLF's adequacy) is built on. It *preserves* branch structure and
  *forbids* collapsing or cancelling sibling branches.
- **Interference is merge-with-cancellation.** Two coherent histories
  arriving at one state sum amplitudes that can cancel. That is a **DAG**,
  not a tree, and the cancellation is the whole content.
- A tree cannot represent merge-with-cancellation. The path-sum needed for
  QFT lives at the **linear-time** (trace / weighted-language) end of van
  Glabbeek's spectrum — the opposite end from bisimulation.
- Weighted/probabilistic bisimulation (Larsen–Skou) exists but is strictly
  *finer* than distribution/trace equivalence: it will **not** give
  interference.

So: the cost and probabilistic readings are compatible with the paper's
branching-time semantics; the genuinely *quantum* reading (cancellation,
Feynman sum) requires moving to linear-time, which is a real reformulation,
not a relabelling. **That tension is the research problem.**

### 3a. Many-worlds and decoherence

- **MWI is the natural partner of bisimulation for the decohered part.**
  MWI = no-collapse; bisimulation = no-quotient-of-the-tree. Both refuse to
  throw away branch structure. For the *measurement* content (superposition
  becoming a fan of outcomes you must not merge), the worlds *are* the
  bisimulation branches.
- **But MWI does not remove interference;** it relocates it into the
  unitary evolution of the universal wavefunction, where the amplitude sum
  over merged histories is still the linear-time layer.
- **Decoherence is the passage DAG → tree** (coherent/interfering/
  recombining → effectively classical branches that can no longer
  recombine). In the analogy, decoherence is the **emergence of the
  branching-bisimulation-respecting structure**: the moment siblings can no
  longer interfere is the moment the finer (branching) equivalence becomes
  *sound* where before only a trace-like equivalence was.
- So you do **not** get branching bisimulation for free from MWI; you get
  it *after decoherence*. Before that point you are stuck at linear-time.
- **Diagnostic.** Classical nondeterministic choice and quantum
  superposition have the *same branch skeleton* and differ *only* by
  interference. Branching bisimulation sees the skeleton; the
  semiring-weighting-plus-summation sees the difference.
- **Residue.** Even granting decoherence, entanglement makes a *local*
  action *globally* observable (a local measurement changes the reduced
  state of a remote entangled partner). So bisimulation may fail to be a
  **congruence** (the qCCS problem; Ying's quantum process algebra closes
  bisimulation under all contexts to fix exactly this). This "local move,
  global reach" shape **rhymes with the ambient-authority problem** in the
  capability section — likely the more productive thread than the
  field-equation one.

---

## 4. The annihilation rule, and where the DAG actually lives

This is the key refinement (Meredith): interaction is **not atomic**. The
rho paper proposes an alternative COMM rule, the **annihilation rule**,
which gives:

- a **scale-invariant** notion of interaction, with **quote level as a
  proxy for scale**; and
- the option of putting **more compute power inside interaction**.

**Where the DAG lives.** Not *beside* interaction (a separate coherent
layer) but **inside** it: interaction is a computation, and the DAG is the
internal reduction graph of that computation. The LTS transition that
bisimulation sees is the **coarse-graining** of that graph — net in-to-out,
with the intermediate compute summed over. One calculus seen at two
resolutions, not two layers.

**Phlogiston is which-path information.** The cost machinery is exactly the
instrument that "carefully accounts for all the computations in
interaction":

- Metering at sub-interaction granularity records the fine history (the
  sequence of internal compute steps realising one interaction). The
  consumed stack *is* that record (cf. Prop 4.1: stack drains in step with
  forced redexes).
- **Decoherence = forgetting the stack** (recording only that an
  interaction happened and its net effect).
- **Careful accounting = keeping the stack**, which reconstructs the DAG.
- So the cost endofunctor, applied at the scale of the annihilation rule's
  *internal* computation rather than the net COMM, **is the
  decoherence-vs-accounting dial.** The thing you meter is the thing
  decoherence erases. The cost machinery makes the QFT "sum over histories"
  literal.

**The exact framework: decoherent / consistent histories**
(Gell-Mann–Hartle, Griffiths, Omnès), offered as the right scaffold:

- A **coarse history** = a net interaction. The **fine histories** = the
  internal compute paths of the annihilation rule that produce it. Two fine
  paths landing on the same net interaction are the recombining branches of
  the DAG; their amplitudes add (and can cancel) in the accounting.
- The coarse-graining is **consistent (decoheres)** iff the **decoherence
  functional is diagonal** — cross-terms between *distinct* coarse
  histories vanish.
- **Consistency is not automatic.** It fails exactly when fine histories
  leading to *different* net interactions interfere — the entanglement /
  qCCS-congruence / ambient-authority situation again.
- Therefore the careful accounting is a **decoherence detector**:
  diagonalise the accounting → the coarse LTS is a sound,
  congruence-respecting abstraction (bisimulation legitimate). Off-diagonal
  mass between distinct coarse outcomes → the net-interaction abstraction
  *leaks* (a local annihilation is globally observable; the tree is a lie
  at that scale). Whether forgetting the phlo trace is safe is checkable
  *from the trace itself*.

**Scale-invariance ⇒ a renormalization-group structure (not a metaphor).**
If the annihilation rule looks the same at every quote level, coarse-
graining one level produces *the same rule one level up*: an interaction at
scale n is a DAG of interactions at scale n+1, self-similarly. The
effective vertex at coarse scale is the path-sum over the resolved sub-DAG
at finer scale — exactly an effective-field-theory vertex as a sum over
finer diagrams.

- **Decoherence becomes an RG flow**, not a one-shot collapse: fully-
  resolved coherent DAG = UV; classical decohered tree = IR; the flow
  between them is iterated coarse-graining / stack-forgetting.
- The **scale-invariant annihilation rule is a candidate fixed point** of
  that flow. Whether the flow has a nontrivial fixed point — a self-similar
  regime that neither fully decoheres nor stays fully coherent — would be a
  real theorem to chase.
- The **creation/annihilation reading closes the loop**: the annihilation
  rule as the Fock lowering operator is the same gesture as
  "forcing-by-unwrapping lowers an occupation" — the occupation/Fock
  dictionary and the which-path/accounting dictionary describe one operator.

**Open question that the actual rule decides.** Does the annihilation
rule's internal computation have a *canonical* fine-history (so the
coarse-graining's equivalence classes are well-defined), or are the fine
histories themselves only defined up to the structural congruence `≡`? If
`≡` already quotients some of the internal compute, then part of the DAG is
decohered *by the congruence* before any accounting — and the line between
"erased by decoherence" and "erased by `≡`" must be drawn. **First thing to
settle: does the annihilation rule's internal computation live above or
below the structural-congruence quotient?**

---

## 5. MQ-calculus (Stay & Meredith): interaction = measurement

*(Paper not located online during this discussion; reasoning is about the
principle, to be checked against the actual construction.)*

If interaction **is** measurement, then every LTS transition is by
definition a **post-measurement, decohered** event: the interaction
relation never carries a coherent superposition across which cancellation
could happen. So the §3 obstruction never arises *at the level of the
transition relation* — bisimulation over the MQ-calculus LTS is sound
automatically, because that LTS *is* the decohered tree by construction.
You don't have to *earn* the branching equivalence with a separate
decoherence argument.

**But this forces the question: where did the DAG go?** Where does
unitary/coherent (interfering) evolution live?

- **If unitary evolution is also folded into interaction** (every
  dynamical step is a measurement): not a quantum calculus that decoheres —
  a **classical stochastic** calculus. No coherence was ever present;
  permanently in the ℝ≥0 (post-Born) shadow; interference unrecoverable.
- **If unitary evolution lives *beside* the interaction relation** (as
  non-branching reductions on the quantum state, or structure on an
  entangled resource): the interesting case — a clean two-stratum calculus
  where the ℂ-amplitude/interference layer is the internal unitary dynamics
  (congruence-respecting, where the DAG lives) and the branching-
  bisimulation layer is the measurement-interaction LTS, with **decoherence
  identified with the interaction step itself**. The Born rule's status as
  "a separate extraction laid on top" = the LTS is the ℝ≥0 image of the
  ℂ-valued state layer.

**Note:** the annihilation-rule reading (§4) is a *third* answer, better
than both — coherent evolution is neither folded into nor beside
interaction but **inside** it, as the internal compute graph. MQ-calculus
and the annihilation rule may be two routes to the same two-resolution
picture.

**Residue (unchanged):** entanglement still makes the resulting
bisimulation potentially non-compositional (qCCS congruence problem).

**Diagnostic question for MQ-calculus:** does coherent/unitary evolution
appear as transitions in the *same relation* bisimulation is taken over, or
as a distinct non-interactive layer on the state? Latter ⇒ decoherence
genuinely built in; former ⇒ classical-stochastic shadow.

---

## 6. Interaction hosts a computational paradigm (the philosophical position)

The unifying stance: **`compute` is not a fixed combinator applied at
contact — it is a slot where a computation runs.** The interface is a
*locus of computation*, not merely a matching condition. Two independent
routes reach this:

- **The annihilation rule (Meredith):** put compute steps *inside* the
  firing of a cut (§4).
- **Cardelli's brane calculi (the precedent):** put the computation *on the
  membrane*. The boundary itself carries the operations (phago, exo, pino;
  bind/release in the PEP variants), so interaction is a process executing
  *at the interface*, not a structureless rendezvous between inert regions.

This is the same recognition reached by different mechanisms: the interface
computes. In the migration-spectrum terms of §8, brane calculi sit at the
far end — past "migration by spatial restructuring" (ambient calculus,
where `compute` relocates a subtree) to **"the surface is itself the
computational agent,"** where `compute` is a membrane interaction. The
interaction surface `I`/`J` is made *active*: where CCS's surface is an
inert label and rho's is a name, the brane surface *computes* during the
match.

**Two payoffs:**
- Brane operations are inherently **multi-party**, so brane calculi are the
  natural witness for the variadic / n-ary cut generalization.
- **Brane-in-membrane nesting is the spatial form of the self-similar / RG
  picture** of §4: an interaction at one level being a computation of
  interactions at the next.

The cost paper already gestures at this (conclusion future-work): brane
calculi as "the proving ground for how the construction behaves when the
active sites are themselves the boundaries that the restructuring moves."
The position above is the *principle* under that gesture — brane calculi
are the precedent for **interaction-as-host-of-computation**, and the
construction's `compute` is the slot where that hosting happens. (Heavier
development — positioning `compute` as a hostable interactive sub-calculus,
tied to the annihilation-rule / DAG-inside-interaction material — belongs
in the serious note, not the cost paper.)

---

## 7. Lineage: Geometry of Interaction

Noticing physical structure in the dynamics of cut elimination is **not
new**; this work adds an entry to an old register rather than opening it.
This is the strongest framing for the whole physics coda (alongside
Curry–Howard).

- **Girard's Geometry of Interaction** took cut elimination as a *dynamics*
  — proof normalisation as the resolution of a **feedback equation**, the
  **execution formula**, information flowing along proof paths — and went
  through **operator algebras and C\*-algebras** *precisely in pursuit of
  correspondences to physical theories*. Nilpotency of the execution
  operator = termination; trace/feedback = composition. This is the
  physical-mathematical register being joined.
- **Categorical GoI** (Abramsky–Jagadeesan; Abramsky–Haghverdi–Scott;
  Haghverdi–Scott) abstracted that dynamics into **traced monoidal /
  partially-traced categories** — the categorical axiomatisation of the
  feedback dynamics. Haghverdi–Scott show the operator-algebraic original
  is *exactly captured* in the categorical framework.
- **Categorical Quantum Mechanics** (Abramsky–Coecke) is the sharpest
  precedent: the *same* compositional/interaction structure serves both
  logic and quantum theory, and — crucially — from compact closure +
  biproducts there **emerge a semiring of scalars `C(I,I)` and a Born
  rule**. That is the *actual apparatus* the QFT reading (§2) leans on
  (semiring-valued weighting, Born as separate extraction) — so the QFT
  side is not merely analogically related to CQM but shares its machinery.

**Calibration / honest scope.** The GoI tradition's physics affinity is
strongest as **operator-algebraic and quantum-information** structure
(Hilbert spaces, C\*-algebras, traced/compact-closed categories, CQM) and
as Hamiltonian/dynamical-systems language for cut elimination. It is **not**
a general-relativity or field-equation correspondence. So the GR and QFT-
field readings here remain distinct contributions; the GoI program is the
prior establishment that *this kind* of correspondence is worth taking
seriously.

---

## 8. Lineage: Cardelli (ambients and branes)

- **Mobile ambient calculus** (Cardelli–Gordon, 2000): already cited in the
  cost paper as the instance where `compute` is **spatial restructuring**
  (dissolving/relocating an ambient boundary) rather than substitution —
  the cleanest case where a step literally moves information through space.
  Anchors the GR reading's "what is near what is dynamical."
- **Brane calculi** (Cardelli, 2004): generalize ambients by placing
  computation **on the membranes** rather than inside them, with
  interaction by membrane fusion and fission. The precedent for
  **interaction-hosts-computation** (§6); the proving ground for the
  construction when the active sites are themselves the boundaries that the
  restructuring moves; and the spatial form of the self-similar / RG picture
  via membrane nesting.

---

## 9. Verified bibliography (for the serious note)

**Geometry of Interaction / CQM:**

1. J.-Y. Girard. *Towards a geometry of interaction.* In J. W. Gray and
   A. Scedrov, eds., *Categories in Computer Science and Logic*,
   Contemporary Mathematics, vol. 92, pp. 69–108. AMS, 1989.
2. J.-Y. Girard. *Geometry of interaction I: interpretation of system F.*
   In R. Ferro et al., eds., *Logic Colloquium '88*, Studies in Logic and
   the Foundations of Mathematics, vol. 127, pp. 221–260. North-Holland,
   1989. *(The C\*-algebra / Hilbert-space execution-formula paper.)*
   *(GoI II — "Deadlock-free algorithms," COLOG-88, LNCS 417, 1989/1990 —
   is a separate optional cite for the development, not the C\* content.)*
3. S. Abramsky and R. Jagadeesan. *New foundations for the geometry of
   interaction.* Information and Computation, 111(1):53–119, 1994.
4. S. Abramsky, E. Haghverdi, and P. Scott. *Geometry of interaction and
   linear combinatory algebras.* Mathematical Structures in Computer
   Science, 12(5):625–665, 2002.
5. E. Haghverdi and P. Scott. *A categorical model for the geometry of
   interaction.* Theoretical Computer Science, 350(2–3):252–274, 2006.
6. S. Abramsky and B. Coecke. *A categorical semantics of quantum
   protocols.* In *Proc. 19th IEEE Symposium on Logic in Computer Science
   (LICS '04)*, pp. 415–425. IEEE Computer Society, 2004.

**Cardelli:**

7. L. Cardelli and A. D. Gordon. *Mobile ambients.* Theoretical Computer
   Science, 240(1):177–213, 2000.
8. L. Cardelli. *Brane calculi — interactions of biological membranes.* In
   V. Danos and V. Schächter, eds., *Computational Methods in Systems
   Biology (CMSB 2004)*, LNCS, vol. 3082, pp. 257–278. Springer, 2005.

**Physics frameworks to pull in (not yet verified citations):**

- Consistent / decoherent histories: Gell-Mann & Hartle; Griffiths;
  Omnès.
- Maslov dequantization / idempotent (tropical) analysis: Litvinov,
  Maslov.
- Quantum process algebra & bisimulation-as-congruence: Ying (qCCS);
  Gay & Nagarajan (CQP) as related.
- van Glabbeek, linear-time / branching-time spectrum.
- Larsen & Skou, probabilistic bisimulation.

---

## 10. The throughline for the serious note

The cost construction is not merely *analogous* to physics; it appears to
supply the **apparatus** that makes the physical readings literal:

- The **semiring grading** (CQM's scalars) is the dial between cost,
  probability, and amplitude.
- The **phlogiston stack** is which-path information; **decoherence is
  forgetting it**, **accounting is keeping it**, and the **decoherence
  functional's diagonality is checkable from the trace**.
- The **annihilation rule's scale-invariance** turns decoherence into an
  **RG flow** with the scale-invariant rule as a candidate fixed point.
- **`compute`-as-host** (Cardelli's brane move, and the annihilation rule's
  internal DAG) is where the coherent layer lives.
- The whole thing sits in the **GoI lineage** of taking cut-elimination
  dynamics as physical-mathematical structure, with **CQM** as the sharpest
  precedent and the actual source of the semiring/Born apparatus.

The one persistent hard residue across every thread — interference
cancellation vs. branching bisimulation, entanglement vs. congruence,
ambient authority vs. located capability — is **the same shape: local move,
global reach.** That is the knot the serious note has to cut.
