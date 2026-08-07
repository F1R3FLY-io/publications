# Integration Notes 14 — draft16

**Task.** Fold `rho-life/weighted-gslt-v2.{tex,pdf}` and the `rho-life/Gillespie`
implementation into *Finding Mind*, introduced in The Turn after the other
constructions on the category of GSLTs (cost-accounting and history), with the
downstream implications for the physics carried through.

**Build.** Base tree + `draft2`…`draft15` + this delta, three `pdflatex` passes.
**658 pages** (was 625). 0 errors, 0 undefined references, 0 undefined citations.
The overfull-box set is identical to the draft15 baseline apart from a single
line-number shift on a pre-existing box in `turn_gslt_ch06.tex` (213 → 225),
caused by text added upstream of it in the same file's chapter. Verified twice:
once in the working tree, once from a clean re-overlay of the repo baseline plus
this delta, which reproduced 658pp and the same overfull set.

**Decisions this pass implements** (D1–D7 as answered):

| | Decision |
|---|---|
| D1 | Placement **as asked** — new chapter immediately after ch12, *The History Monad* |
| D2 | Quantum construction **in** the machinery chapter; its interpretation stays in the physics part |
| D3 | The spiking network becomes **its own chapter in Part II (Ecology)**, with a forward reference from ch13 |
| D4 | The **partition** replaces refinement-completeness in the Decoration chapter; the latter is demoted to a named weaker alternative |
| D5 | **Full rewrite** of the complex-amplitudes chapter |
| D6 | This pass first; **R2 after** |
| D7 | A unit exists, so the monad question is worth asking; recorded as out of scope |

**Standing correction observed throughout:** the weight construction is presented
as an **endofunctor**, never as a monad. No monad structure is claimed or used.

---

## 1. New chapters

### ch13 — `turn_gslt_ch11.tex`, *Weighting: Rates from the Generated Logic*

Sits between ch12 (*The History Monad*) and ch14 (*Cost-Accounted Rho*). Label
`ch:weight`; label namespace `wt:`. ~16pp.

Framed in the register ch11 and ch12 use: each of the three constructions adjoins
one component to the state and makes the rules act on it — a stack that drains, a
word that accrues, a table that is rewritten.

Contents:

- `wt:def:refinement`, `wt:def:partition`, `wt:def:reqs` (R1 decidability,
  R2 locality, R3 structurality), `wt:prop:classify` (classification),
  `wt:rmk:default` (completing a partition is cheap).
- §`wt:sec:closure` with `wt:obs:closure` — the Boolean-demand dilemma and the
  cost of an external partition. Forward-pointed to `ob:rem:stratification`
  (ch18) and to `ch:notation` in the Prestige, since it is the same shape as the
  encoding seam.
- `wt:def:wmap`, `wt:def:config`, `wt:def:augmented`, `wt:con:functor`.
- `wt:rmk:notamonad` — **D7**. Names the candidate unit (every key to
  $1_\Semi$), says a multiplication would have to flatten a weighting of a
  weighting and that we do not ask whether it is canonical.
- `wt:rmk:geometric` — context rules carry a multiplicative factor, not a
  propensity, because addressing is not an event; and $\gfac$ as the home of a
  spatial bias.
- `wt:def:propensity` with the four factors separated, `wt:def:gate`,
  `wt:prop:gate`.
- `wt:def:coalgebra`, `wt:rmk:simulator` (simulator ≠ interpreter),
  `wt:thm:markov`, `wt:ex:nonmarkov`, `wt:rmk:selfloop`, `wt:thm:spim`.
- §`wt:sec:quantum`: `wt:def:hilbert`, `wt:def:jump` (with the $\sqrt\Sigma$
  normalisation and why conservativity forces it), `wt:def:hamiltonian`,
  `wt:prin:slogan`, `wt:prop:interference`, `wt:thm:degeneration`.

Two theorems are stated with proofs deferred to the note (`wt:thm:spim`,
`wt:thm:degeneration`). This is now disclosed in `pledge_map` — see §4.

### ch24 — `turn_rholife_ch05.tex`, *A Network That Learns by Rewriting*

Closes Part II, after ch23 (*Engines and Individuals*). Label `ch:spiking`;
namespace `spk:`. ~10pp.

The case for it being in Ecology rather than beside the construction is made in
the opening: the funding gate gives a neuron a metabolism, so a strong synapse on
a starving neuron is silent — mortal computation at a scale where the tokens are
countable.

Contents: `spk:def:neuron` (persistent for-comprehension), `spk:def:keys`,
`spk:prop:admissible`, `spk:prop:mcp` (McCulloch–Pitts as an *expressivity*
lemma, with the combinatorial-blowup caveat), `spk:rmk:dial`, `spk:prop:sum`,
`spk:cor:saturate` (with both provisos, including that the neuron's own firings
change the marking), `spk:rmk:notlogistic`, `spk:lem:bounded`,
`spk:def:hebb`, `spk:thm:one` (learning and inference are one relation),
`spk:rmk:hebbian`, `spk:rmk:stdp`, §`spk:sec:uniform` (namespace keys, tied to
`ch:scope` and to \chComp's learner-as-weighted-population), `spk:rmk:bosonic`.

`spk:rmk:bosonic` is deliberately the **negative** instance of
`wt:prin:slogan`: the neuron presentation quotients everything, so complexifying
it adds nothing at all. A principle whose negative instances are never displayed
is not doing any work.

The TikZ neuron figure from the note was **not** ported. It can be added later if
wanted; nothing refers to it.

---

## 2. The Machinery part

### `turn_gslt_ch07.tex` (ch12) — the duality table gains a third column

§"The duality" → §"The duality, and a third column". Table widened to
Cost / History / Weight, `\small` to fit the measure, with rows *adjoined
structure*, *indexed by*, *behaviour along →*, *records*, *free object*,
*physical reading*, and a new **monad?** row reading yes / yes / **not claimed**.
Closing paragraph points at `wt:rmk:notamonad` for what would have to be settled.

### `finding_mind.tex`

- New preamble block (Draft 16) defining `\Wgt \Wmaps \Cfg \Red \class \prop
  \propz \gfac \Rnn \Lop \Lind \Hilb`. **`\Hilb` is fraktur** because `\Hist`
  already owns `\mathcal{H}`; this collision is real and would have been silent.
- Two `\input` lines added.
- Part I frontispiece: "…a log that remembers it, **a rate at which it happens**,
  a logic that describes it…".
- Overarching Introduction, Part I paragraph: "two constructions" → "three
  constructions, each adjoining one component to the state".
- Part II frontispiece and the *Why the Unit Keeps Moving* essay: a fifth chapter
  acknowledged, running the architecture downward instead of upward.
- One stale count fixed: "the four ecology chapters are close to self-contained"
  → "the ecology chapters".

---

## 3. The physics — downstream

### `turn_causality_ch06.tex` (now ch28) — *Decoration* becomes downstream, not parallel

This chapter was re-deriving a weaker version of the imported construction, and
citing `\cite{StochSim}` while doing it. Three changes:

1. **New opening paragraph** locating the construction in Part I and saying what
   is new *here*: the parameter. "Chapter 13 developed two instances because
   those are the two an implementation runs; the question this chapter asks is
   what the other choices of semiring deliver."
2. **`def:refcomplete` demoted** (D4). The label is **kept** — it is referenced
   from ch30 — but now labels a *remark* titled "The weaker alternative, and why
   it is not taken". The remark states the strictness correctly (a partition is
   refinement-complete; the converse fails) and gives the three things that need
   disjointness rather than selection: incremental reclassification, summable
   propensity per class, and orthogonal projectors in the complex instance. It
   also concedes that overlapping keys with a most-specific rule is a natural
   thing to want, and that wanting it means specifying a different semantics.
   **Note for readers of earlier drafts: cross-references now read
   "Remark 28.x" where they read "Definition 28.x".**
3. **`def:dyndec` gains its consequence.** The chapter previously said only that
   the updates compose. It now says that the state is not the term, that the
   object is the configuration, that the process is Markov on configurations and
   in general not on terms, and — the framing worth keeping — that a physics
   whose constants respond to what has happened has not left the Markovian world
   but moved up a level inside it.

Also: the $\Real_{\ge 0}$ row of the four-instance table now says Gillespie is a
theorem about degeneration rather than an analogy; and **new
`rmk:geomatter`** records the geometric/matter split ($\gfac$ prices *where*,
$\dec$ prices *what*), flagged as an opportunity and not a result, with the
missing action named.

### `turn_causality_ch09.tex` (now ch31) — full rewrite (D5)

Retitled **"Complex Amplitudes, and Where the Coherence Is"**. `\label{sec:quantum}`
retained; all eight existing call sites unaffected.

Kept essentially verbatim: `rmk:ch09status` (updated to say what has changed),
`rmk:obstruction`, the "easy to disguise" paragraph, the density-matrix
retraction, and routes 1–2.

New structure:

- §`sec:obstruction` now has **two subsections**. The semantic obstruction is the
  old one. The **constructive obstruction** is new: conservativity
  (`def:conservativity`) forces $\sqrt{\Sigma\text{rate}}$ over
  $\Sigma\sqrt{\text{rate}}$, and that choice removes cross-derivation coherence
  by construction, independent of the choice of equivalence.
- **`rmk:two-obstructions`** — they agree in verdict and disagree in everything
  else; one is about observation, the other about arithmetic; and the second
  *survives a move to linear time*, which shrinks route 1 considerably.
- **New §`sec:coherence`, "Where the coherence is"** — the rewrites are directed
  and dissipative, the equations symmetric and unitary, and a quantum reading has
  exactly two slots. Three consequences: complexification is not free and one can
  now see what it costs; the obstruction is relocated, not removed; and —
  **the largest change in this pass** — the ontology of Part IV **survives**.
  Coherence sited in the equational theory never asks bisimulation to pool runs,
  so the concession that route 1 would destroy the bisimulation-classes-are-real
  ontology is now explicitly retracted rather than left standing.
- **`rmk:coherence-gaps`** — what is still not answered: which equations a
  physical theory would withhold and where $h_e$ comes from; and the equivalence
  appropriate to $H \neq 0$, which is the precise form in which the
  branching-time question returns.
- §`sec:whatwouldchange` — routes 1 and 2 as before; **route 3 replaced**
  ("find the right equivalence" → "attach amplitudes to derivations", which is
  the actual shape of what would be needed); **route 4 added**, explicitly marked
  as not being a route to cancellation between rewrites.
- §"Simulation, in the meantime" rewritten. The old verdict — "a computation in
  search of a semantics" — is withdrawn and replaced by an account of what the
  refusal was actually waiting for: **a fixed generator**. `wt:def:hilbert`
  supplies it by indexing the basis with configurations. What remains withheld is
  the interpretation and not the computation.

### `turn_causality_ch08.tex` (now ch30) — *Extended Modal Operators*

- `def:ext-modal` clause (ii) re-pointed from `def:refcomplete` to
  `wt:prop:classify`, and reworded from "most refined witness" to "unique
  witness supplied by the partition discipline".
- **New `rmk:reversal-status`** on clause (iii). Says plainly that
  $\dec^\dagger = \overline{\dec}$ is a stipulation carried forward, that the
  complex instance as built has the reverse of a jump being $L^\dagger$ with the
  asymmetry in the dissipator, that the two readings agree on the tropical
  coordinates and have not been reconciled on the complex one, and that
  `wt:def:jump` is what is actually built.
  **This is the handoff to R2** — it is the same retracted complex apparatus as
  `sciencebot_ch05`, `hypercomp_ch05`, `origins_ch05` and Gap 7, and it is now
  flagged in the body rather than only in the front matter.
- The state/rule factorisation paragraph gains a sentence tying the moving
  boundary to `wt:thm:markov`.

### Smaller physics edits

- **`turn_causality_ch01.tex`** (ch25): ladder-table row for the semiring axiom
  now says the decoration is the weighting of ch13 *with its codomain chosen*;
  the Organization paragraph rewritten for both ch28 and ch31.
- **`turn_causality_ch10.tex`** (ch32): the *Simulate* step now records that the
  object being stepped is a chain over configurations, and that the same loop at
  $H \neq 0$ is a quantum-jump unravelling — "one procedure and not two".
  `\cite{StochSim}` kept here (it is a claim about the implementation), but the
  word "prototype" removed, since the prototype it named no longer exists.
- **`turn_causality_ch11.tex`** (ch33): the "no interference" negative result
  restated for *two* structural reasons, and **two new negative results added** —
  no equivalence appropriate to $H \neq 0$, and no action relating the geometric
  and matter parts of a weighting.

---

## 4. Beyond the physics part

- **`turn_origins_ch06.tex`**: the gradient-coupled-rules gap **half closes**.
  Time-varying weight maps are not an extension of the framework but its defining
  feature. What remains open is the *externally driven* case, and the note now
  says what the repair is (bring $\Phi$ inside, as a configuration component or
  as traffic with its own keys) and that only the first keeps the process Markov.
  Also: "the quantum Gillespie algorithm of Part~I" → a real reference.
- **`turn_hypercomp_ch07.tex`**: the approximate-oracle gap gets **easier in one
  half and harder in the other**. The unravelling is well posed over a finite
  configuration space; but by `wt:prop:interference` an amplitude attached to a
  rewrite carries no coherence, so the quantity a graded definition wants is a
  rate and not an amplitude unless the presentation supplies a Hamiltonian. The
  definition needs restating on that basis and has not been.
- **`turn_rholife_ch01/ch02/ch03.tex`**: five `\cite{StochSim}` sites converted
  to internal `\ref{ch:weight}`. In ch19 the Grothendieck/weighted-coalgebra
  sentence is rewritten to match what the chapter now says, and
  `sci:prop`'s "this is StochSim's theorem with a different label on the target"
  becomes "this is Chapter 13's construction with a different label".
- **`pledge_map.tex`**: the fifth-case paragraph now discloses that ch31 "reaches
  an obstruction and then goes sideways", and that two of its results are stated
  without proofs (SPiM conservativity, the degeneration theorem) which live in the
  source note — "a deliberate compression and not a gap, but a reader who wants
  to check them will have to go and get them". *What this book does not do*
  updated: two obstructions, named, plus the one slot for coherence neither
  touches.
- **`prestige_ch03.tex`**: the paragraph on varying the semiring across the
  population updated — bisimulation refuses to pool the runs, *and* the only
  conservative normalisation has already added them where nothing can cancel;
  followed by the location of the one slot a presentation does have.
- **`bibliography.tex`**: `StochSim` retitled to the v2 note. Five new bibitems:
  `priami`, `spim` (Phillips–Cardelli), `mcp` (McCulloch–Pitts),
  `hebb`, `stdp` (Bi–Poo).

---

## 5. Notation reconciliation

The book's spellings win throughout, per standing convention.

| Note | Book | Note |
|---|---|---|
| `w` (weight map) | `\dec` | already the book's decoration — same object |
| `\mathcal{K}` (codomain) | `\Semi` | the note's `\mathcal{K}` collides with `\ctx` |
| `\mathrm{class}_r` | `\class` | new |
| `\mathcal{H}` (Hilbert) | `\Hilb` = fraktur | **`\Hist` already owns `\mathcal{H}`** |
| `\mathcal{L}` (Lindbladian) | `\Lind` | shares `\mathcal{L}` with `\Lrn`, in a different part |

The note's modality $\langle K_j\rangle$ (one per rule *and* choice of redex
position) and the book's $\langle K\rangle$ over minimal contexts
(`def:cdHML`, ch15) are treated as the same object, with the position carried by
the context. **This was checked and is right, but it is an identification and not
a definition** — if the position quantification ever needs to be explicit, ch15
is where it would go.

---

## 6. Open after this pass

1. **R2 is next** (D6). It is `sciencebot_ch05`, `hypercomp_ch05`, `origins_ch05`
   and Gap 7 — three chapters and a gap. It now has something to point at:
   $\lambda(z) = |z|^2$ as an interpretation map, and coherence-only-from-$H$.
   `rmk:reversal-status` in ch30 is a fourth site and belongs with it.
2. **The `def:refcomplete` label now names a remark**, not a definition. If that
   reads badly in ch30's clause (ii) — it currently doesn't, since that clause was
   re-pointed away — consider renaming to `rmk:refcomplete` in a later pass, which
   would need the one remaining call site updated.
3. **`rmk:geomatter` is a claim I have not seen you make elsewhere.** The split of
   a reduction-graph weighting into a positional factor and a payload factor is
   forced by the propensity formula, but calling it geometry-and-matter is an
   interpretation. It wants your eye.
4. **The Part IV ontology retraction** in §`sec:coherence` is the biggest single
   assertion in this pass. It says a concession the book has been carrying since
   draft4 can be withdrawn. If it is wrong, it is wrong loudly and in the place a
   reviewer will look.
5. **The TikZ neuron figure** was not ported. The Ecology part has figures, so it
   would not be out of place.
6. Untouched and still owed: `prop:leastaction` (Bellman not Lagrange),
   Landauer without a temperature, and `thm:compact-consensus`'s demotion, which
   draft14 left as a conjecture.
