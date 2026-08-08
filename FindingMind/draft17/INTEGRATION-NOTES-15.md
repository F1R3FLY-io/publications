# INTEGRATION-NOTES-15 — draft17

**Pass A (the observer extension as a chapter) + Pass B (the repairs it licenses).**

Sources folded: `spatial-formulae-discrimination-power/extension-note-v2`,
`behavior-in-higher-order-languages/main`, and the items of
`rho-life/review-of-mortal-scientist.md` that those two notes answer.

Build: base tree + draft2..draft16 + this delta, three `pdflatex` passes.
**680pp** (baseline 658), 0 errors, 0 undefined references, 0 undefined citations,
0 multiply-defined labels. Eight overfull boxes — the same eight as the baseline,
with one line-number shift (`turn_gslt_ch05` 225 → 250, a pre-existing box below
text I inserted). Baseline verified by rebuilding a pristine overlay from the repo
separately, not by trusting the working tree.

---

## A finding to record before anything else

**There was no `scientist-note` delta to fold.** `rho-life/scientist-note.tex` has
not changed since commit `d52e548` (the K-instantiation fix), which the Aug 3
fold-in already carried. The note and `turn_rholife_ch01` have identical section
structure, 80 for 80, and the book is *ahead* — it carries `sci:rem:transducer` and
`sci:rem:wigner-scope`, which the note does not. Per decision (7) the note is not
being back-ported, so that divergence now widens by everything in Pass B below.

---

## Files

| file | what changed |
|---|---|
| `turn_gslt_ch12.tex` | **NEW** — ch20, ~16pp |
| `finding_mind.tex` | `\input` for the new chapter; six new macros; Part I framing box |
| `turn_gslt_ch05.tex` | ch10 — two remarks (BIHOL) |
| `turn_causality_ch05.tex` | ch16 — four remarks (BIHOL) |
| `turn_gslt_ch10.tex` | ch19 — `ob:rem:adequate-for-what` |
| `turn_rholife_ch01.tex` | ch21 — six sites, incl. two demotions/replacements |
| `turn_rholife_ch03.tex` | ch23 — one open problem re-pointed |
| `pledge_map.tex` | two additions to the load-bearing-unproved list |
| `prestige_ch02.tex` | `obs:hosting` sharpened |
| `bibliography.tex` | +4 entries (`ExtNote`, `BIHOL`, `Lozes`, `HirschkoffLozes`) |

**Renumbering.** New ch20 sits after ch19 (OSLF), last in Part I. Turn is now
ch6–65, Prestige ch66–68. The Mortal Scientist is **ch21** (was 20); everything
downstream shifts by one. Ref-safe: no hardcoded chapter numbers in the source.
`five_perspectives.tex` and notes 1–14 are now stale on chapter numbers.

---

## Pass A — the new chapter

**ch20, `turn_gslt_ch12.tex`, "Instruments: Making Structure Visible to
Bisimulation".** Label `ch:obsext`, namespace `ox:`, pp191–206.

Placed at the end of Part I because it is what Chapter 19's adequacy discussion
needs and what Chapter 21 immediately spends.

Spine:

- **§20.1** states (A) and (B) as claims *the book itself makes*, with the sites
  named, and credits the referee with the objection.
- **§20.2** the resolution — there is no such thing as *the* observational
  equivalence — plus `ox:rem:three` (operational reconstruction / logical
  expressiveness / observer capability kept apart) and `ox:rem:notclaim` (it is
  **not** claimed that structural inspection was interaction all along).
- **§20.3** `ox:cond:rpo`, `ox:cond:cong` inherited from `\cite{BIHOL}`; admissible
  `E` as (E1)–(E3) with `ox:rem:unit` on why the unit law breaks size-respect.
- **§20.4** `ox:def:ext` — probe on the **left** of the cut, so every administrative
  rule is interaction-headed and no unit for the cut is needed; indexed probe atoms;
  `ox:prop:still`; `ox:rem:pairs` on equations-vs-rewrite-pairs.
- **§20.5** `ox:lem:exposure` — **the load-bearing lemma**: the existential over
  splits in `φ | ψ` becomes an existential over transitions. Then `ox:lem:calib`,
  `ox:thm:main` with full proof, `ox:rem:domains`, `ox:rem:branching`,
  `ox:rem:binding`, `ox:prop:idem`.
- **§20.6** `ox:thm:sep` by characteristic formulae — proved directly rather than
  imported, which is what dissolves the image-finiteness objection — and
  `ox:cor:adequacy`.
- **§20.7** `ox:prop:nonvac`, `ox:ex:replication`, `ox:rem:minimality`.
- **§20.8** `ox:prop:mono` (the dial), `ox:conj:partial`, `ox:prop:downward`,
  `ox:rem:threerestrictions`.
- **§20.9** two cautions: derivations vs transitions (points at ch13), and cost
  bounded by depth but not equal to it.
- **§20.10** the construction is not capability-safe; `ox:rem:import`,
  `ox:rem:nodestructor`; the inherited no-hole-under-quote restriction.
- **§20.11** what this settles and where it is spent — four paragraphs pointing at
  ch19, ch21's crossover-loci identification, ch10's morphisms, and a standing
  warning.

**The standing warning is the part I would not want lost.** `ox:ex:replication`
shows the size of the gap between the spatial layer and bisimulation is a property
of the **presentation**, not of the calculus one has in mind. The book argues from
what a structural predicate can detect in several places and has never said this.
It is stated once, at the end of §20.11, rather than chased through every site.

---

## Pass A — BIHOL, in place (decision 2)

**ch16 `turn_causality_ch05`.** This chapter contained the sentence "The
bisimulation induced by LTS(S) is a congruence", unqualified, and the whole
machinery part leans on it. Three remarks replace it:

- `lts:rem:universal` — minimality is a universal property (IPO), not a subterm
  condition; and BIHOL's **open** form of Leifer–Milner drops the distinguished
  ground object, which a lambda theory needs because its terms carry variables. Also
  says plainly that existence of the pushouts is a hypothesis verified for no theory
  in this book.
- `lts:rem:higherorder` — labels carry processes, so they are matched **up to the
  bisimilarity being defined**, not syntactically; the functional stays monotone.
- `lts:rem:relative` — congruence is relative to a class `A` of admissible contexts.
  For rho: `out(n,−)`, `in(n,−)`, `−|−`, `∗(@(−))`, **excluding holes under the
  quote**, because communication turns on name equality rather than bisimilarity.
  Closes by noting this is the same shape as `ob:def:morphism`'s image restriction —
  "discovered three times as an obstacle before it could be recognised as a
  component".
- `lts:rem:modalonly` — the adequacy theorem in that chapter is for the modal
  fragment only, and points forward to ch20.

**ch10 `turn_gslt_ch05`.**

- `ob:rem:pair` — a morphism is the pair (F, Φ). In a context-labelled LTS the labels
  *are* contexts, so "preserves context-labelled bisimulation" presupposes a map on
  labels that a term map does not determine. The payoff: **the image clause is not a
  second clause**, it is `im Φ`. Two forks recorded and not chased — equivariance up
  to `∼` (bicategory), and faithfulness as a property rather than part of
  morphism-hood, or the adjunctions die.
- `ob:rem:faithdense` — hosting **is** faithfulness of Φ; exhausting **is** density
  of Φ. Flags the consequence for ch67.

**ch67 `prestige_ch02`.** `obs:hosting` gains a paragraph: the chapter already said
"those are faithfulness and density" as a gloss; ch10 makes it a statement, so
Goodman's two-way losslessness becomes a claim about one functor being faithful and
dense — and the world side of that functor is exactly the encoder the book does not
build.

---

## Pass B — ch21, the six sites

1. **§21.1.5** `sci:rem:adequacy-index` — supplies the index. "Exactly what
   interaction separates" becomes "exactly what interaction separates *for a learner
   holding these instruments*", and says that this is weaker than it looks and is the
   one that is true. **This closes review item 1**, the referee's first of five.
2. **§21.1.6** `sci:rem:three-limits` — two levels become three (ideal /
   capability-limited / budget-limited). Concedes explicitly that where the chapter
   reads as though budget alone fixes the accessible hypothesis class, the second
   restriction was silently held constant. **Bears on review item 7.**
3. **§21.3.2** `sci:rem:senses-instruments` — the objection stated in the chapter's
   own vocabulary and answered: perception is not a fourth grade of access smuggled
   in beside the three; it is interaction in a theory whose observer holds
   instruments — and the instruments are *held*. Two consequences: Ob joins the free
   parameters, and `κ_sense` cannot be derived (§20.9.2), so the schedule stays a
   modelling choice with a reason attached rather than an oversight.
4. **§21.15.2** — decision (4). `sci:prop:section` **demoted from Proposition to
   Remark**, label kept, retitled "Cost is not already a weighting", and the
   subsection retitled "There is no canonical section". The refutation is cited to
   **ch13, not to the referee**: a weight is a propensity, a cost enters through the
   funding gate `χ(r,k,σ)` (`wt:def:gate`), and those are different data with
   different types. What survives: each monotone cost-to-rate map defines a section,
   none canonical. **Review item 18, which had become an internal contradiction
   between ch13 and ch21 in draft16.**
   - Cross-refs: `turn_rholife_ch03`'s coherence open problem re-pointed from
     Proposition to Remark, and given a second half — whether the answer depends on
     which cost-to-rate map was chosen, since independence *would be* the canonicity
     the withdrawn claim wanted.
5. **§21.15.3** — decision (5). `sci:prop:morphisms` **replaced** with the positive
   result and a proof: logic-preserving maps of S are exactly bisimulation-preserving
   maps of S⁺, so the assignment is functorial on the image of `Obs(−,Σ)`. The
   narrowing was a change of the theory the morphisms live over, not a narrowing.
   Subsection retitled "…and that costs nothing". The paragraph this replaces is
   named in the text as the weakest one the section contained.
6. **§21.17.3** — the Wigner argument now says which word was load-bearing and why
   the chapter was not entitled to it until now. The contradiction is removed; the
   *other two* prongs of review item 22 (the encoder, and Wigner's actual scope) are
   untouched and go to Pass C.
7. **§21.14.2** — audit table gains two rows: the observation signature Ob, and the
   cost-to-rate map. Closing paragraph rewritten: the list has now grown twice, and
   the reader is told to assume it is still short of complete. **Review item 19,
   partially.**

---

## Front matter

`pledge_map`'s "Where the weight rests on something unproved" gains two items:

- `ox:cond:rpo` + `ox:cond:cong` — described as **the most widely relied-upon
  unproved thing in the book**, assumed by every chapter of the machinery part and
  verified for no theory in it, invisible until ch20 forced it into the open.
- `ox:thm:main` holds for the first-order fragment only, and every calculus the book
  cares about has binders.

The list's closing line "Nothing else is hiding" is doing more work than it was.

---

## Pass C — the tiered plan (decision 3), for draft18+

Nineteen of the twenty-three review items are untouched. Sorted by what the fix
costs, not by where they appear.

**T1 — demotions and hedges. Cheap, mechanical, high credibility return.**
The reviewer's diagnosis is one move repeated: *A permits B ⟹ A forces B*. Each of
these is a sentence, not a section.

| item | site | fix |
|---|---|---|
| 5 | `sci:cor:band` | rich-and-shallow is profitable; needs δ = f(σ) or an equilibrium. Keep the inequality, drop "we did not import optimal foraging theory" |
| 10 | `sci:rem:nogradient` | "gradient descent is not native here" survives; "therefore tree search" does not |
| 11 | `sci:cor:intelligence` | label "intelligence is a heterotroph's expense" as interpretation. Also: a finite Θ cannot support an indefinitely emitting source without recycling — a real internal inconsistency |
| 12 | `sci:prop:depthgrade` | syntactic depth ≠ semantic influence. Demote to a tendency |
| 14 | `sci:prop:selection` | 2ⁿ bounds the space, not the cost per sample. Weaken to "exhaustive pre-screening is infeasible" |
| 16 | §21.13.1 | "microcanonical in the strict sense" → a distribution supported on a conserved-Θ shell. The strict claim needs equal a-priori probability |
| 20 | MDL | metabolic pricing gives an Occam-*like* pressure; MDL needs cost proportional to code length, and the schedule is free |
| 21 | motivation | drop "the same quantity"; §21.16's pragmatist's objection already concedes it |
| 23 | §21.17.4 | "close to forced" → "under the premises adopted here, mutually reinforcing" |

**T2 — repairs that need an argument.**

- **Item 13** (crossover loci vs modal labels). Partly answered: §20.11 makes the
  identification *well typed*. It explicitly does **not** supply an enabling theorem.
  So the fix is to state `sci:prop:locivlabels` / `sci:prop:alignment` as an
  approximation with the type now correct, and name the enabling theorem as open.
- **Item 8** (population necessity) and **item 9** (evidence dependency). The
  confinement proposition assumes every move is bounded by r; a single learner can
  make a larger move. Population is *one* nonlocal proposal mechanism. This one
  touches the book's largest conceptual claim and reaches into ch22–24.
- **Item 4** (destructive assay forces namespace logic). Replace with an
  **amortisation theorem**: reusable class hypotheses amortise acquisition cost
  across specimens, giving induction a selective advantage. This is the better result
  and it is provable. Knock-on: item 15 ("species make science possible") should
  become **renewable observational type**, which generalises the claim rather than
  weakening it.
- **Item 6** (to eat is to be edible). The lemma gives "metabolism requires an
  outward-facing surface", not "metabolism exposes the capability by which it is
  consumed".

**T3 — new writing.**

- **Item 3, capability discovery.** The largest gap, and the one that underwrites
  predation, foraging, exposure, and half the evolutionary argument. Predicate
  recognition is not witness extraction; a Boolean `where` guard does not synthesise
  an unforgeable name. Something operational must turn a satisfied namespace
  description into a channel a receipt can use. Note that ch20 §20.10 is directly
  relevant and cuts *against* the easy answer: the observer extension is not
  capability-safe precisely because opening yields arguments as data, so any
  discovery semantics built on it inherits the confinement failure.
- **Item 2, experiment synthesis.** An assay is a pair (K, φ), so φ did not determine
  K. Rewrite as "given a probe context K, a hypothesis can be installed as a
  complementary guarded verdict mechanism", and leave experimental design open.
- **Item 17, the phase diagram.** Needs simulation. The Gillespie prototype in
  `rho-life/Gillespie` is the obvious instrument, and this is probably the highest
  value next step for the research programme rather than for the book.
- **Item 22, the remaining two prongs.** The encoder decides which physical
  regularities become terms, names, nesting, channels, rewrites, resources — so the
  correspondence problem may simply have relocated. `sci:rem:wigner-scope` concedes
  part of this; `ch:notation` says what re-derivation would cost. What is missing is
  the admission that OSLF adequacy explains why *an observational logic of a
  computational substrate* fits that substrate, which is a long way from complex
  analysis fitting physics.

**Structural suggestion, worth a decision.** The reviewer proposes sorting ch21's
results into three tiers — formal consequences / model-dependent consequences /
conjectures — and observes that too many items in the latter two are written as
though they belong in the first. The book already has the machinery for this: the
status column of the physics part. Applying it to ch21 would do more for the
chapter's credibility than any individual repair in T1.

---

## Open after this pass

- **R2 is next** (decision 6) — the semiring rewrite of message fidelity, four sites:
  `sciencebot_ch05`, `hypercomp_ch05`, `origins_ch05`, and `rmk:reversal-status` in
  ch30. Unchanged by this pass.
- `ox:conj:partial` is, I think, the most interesting statement in the vicinity, and
  §20.8 says so in the book.
- `ox:rem:binding` — the first-order restriction. Now advertised in the front matter,
  which makes it more urgent than when it sat inside a research note.
- `sci:prop:morphisms`'s proof is three lines and rests entirely on
  `ox:cor:adequacy`. If the corollary's hypotheses fail for a theory of interest, the
  positive result fails with it, and §21.15.3 is then worse off than the paragraph it
  replaced. Worth your eye.
- `ob:rem:pair`'s two forks (bicategory; faithfulness as property) are recorded and
  not chased. The second one matters: get it wrong and the C/H adjunctions of ch11–12
  do not survive.
- The Part IV/Part III prose-versus-LaTeX part-numbering mismatch is still
  documented rather than fixed, and I avoided `\ref{part:machinery}` in new prose
  because of it.
