# Behavior in higher-order languages — second draft

33pp, clean build (`pdflatex; bibtex; pdflatex; pdflatex`), no errors, no
overfull boxes, no undefined references or citations.

## The one structural change

v1 had an unfinished definition:

> We consider a context $c$ to be an **observation** if...

Everything downstream quantified over that word, so nothing downstream could
be stated. v2's answer is that there is no absolute notion — the class of
admissible observers is a **parameter**, written $\mathcal{D}$, and relative
pushouts are computed inside it rather than in the ambient theory
(§4, §5). Three things that looked separate become one theorem with three
choices of $\mathcal{D}$:

| Phenomenon | $\mathcal{D}$ |
|---|---|
| reflection in rho (§6) | operations preserving $\approx$, with $\approx_{Nm}$ = equality |
| encodings, incl. π→rho (§7) | images of source contexts |
| built-in data types (§8) | images of extended-theory contexts |

$\mathcal{D}$ is not legislated. §4.1 derives it from an *observation
relation* — a logical relation on types, freely chosen at the base types
other than $Pr$. For rho the single choice $\approx_{Nm}$ = equality
forces `@` out of $\mathcal{D}$ (Prop. 12), which is exactly v1's
reflection caveat, now a consequence rather than an apology.

The relation and $\mathcal{D}$ are mutually recursive; Prop. 11 shows the
functional is monotone (composite of two antitone maps) and takes the
greatest fixed point.

## New material

- **§4** Observation relations, admissible operations, the fixed point,
  the transported relation along an encoding.
- **§5.2** Two things Leifer–Milner did not need: the ground object (giving
  the open form, as in v1) and the ambient category (giving the relative
  form). Their proofs never leave $\mathcal{D}$, which is why Lemma 24 and
  Theorem 25 go through verbatim.
- **§5.3** Definition 20 — `observation`, finished.
- **§5.9** Working in $\mathcal{D}/\!\approx$. This replaces v1's
  "weakening" and Eq-relative pushouts, which v1 flagged as possibly
  intractable. The quotient LTS v1 already defined does the job: a functor
  into $\mathcal{D}/\!\approx$ *is* a map whose source equations become
  bisimulations. No new framework.
- **§8** Extension by data types, the practical target. Finitary algebraic
  $A$, lax models, purity and conservativity, canonicity from initiality,
  Scott-style encoding, `Bool` worked in full (§8.4).

## Cut, and why

- **Ambient calculus** — Remark 15. $n[p\mid q]\neq n[p]\mid q$, so the
  redex-IPO enumeration (Thm 30) does not apply; and Bonchi–Gadducci–Monreale
  show the IPO semantics for ambients is too fine anyway.
- **λ, name-passing λ, combinators, Turing machines, space calculus** —
  no theorem consumed them. λ is covered by Di Gianantonio–Honsell–Lenisa.
- **The strict `encoding` definition and the category λT** — demoted to the
  opening paragraph of §7.1 as the demand that turns out to be wrong.
- **`Float`** — §8.6. IEEE 754 is not an algebraic theory; there is no
  initial model for an encoding to be canonical with respect to. Carry it as
  an opaque ground type with a chosen relation, which Def. 8 already permits.

## Kept from v1 essentially intact

§2 (lambda theories), the rho/π theory boxes, the redex-IPO enumeration,
normality, closure of bisimilarity under ground substitution, the direct rho
congruence proof, the ρ!→ρ replication result and its proof, the π→ρ
translation table, and all the string diagrams (now Appendix C).

**Retracted from the earlier plan:** §2.2's metatheory / left-Kan material
was going to be cut as doing no work. It is what supplies canonicity in §8.2,
so it stays.

## Honesty apparatus

§1.4 is a table marking every result PROVED / SKETCH / OBLIGATION. The three
things marked OBLIGATION that actually block publication:

1. **Ob. 28** — is $[\![C_\pi]\!]$ decomposition-closed up to $\approx$ in
   rho? Cheapest to test, and everything depends on it. Expect failure on the
   nose; the question is whether the closure still excludes the discriminating
   contexts.
2. **Ob. 43, the ν lemma** — the largest piece of missing mathematics, and
   the reason the π→ρ proof was never written. §7.3 suggests proving it
   against ρ! and composing with Thm 36, so that the leg concerns only name
   creation.
3. **Thm 26** — weak congruence in general. Everything in §7 and §8 is about
   weak bisimilarity, so this is load-bearing; only the direct rho argument
   (Thm 33) is available.

Remark 31 raises a related question that should be settled rather than left
implicit: strict RPOs are fragile under AC equations on `|`, and the standard
repair is groupoidal RPOs (Sassone–Sobociński, now in the bib). The normality
assumption is doing the same work by another route.

## New §10: morphisms should carry contexts

The paper now ends on a definitional proposal rather than on a list of gaps.

The received definition of the semantic category takes morphisms to be
bisimulation-preserving maps of terms, where the bisimulation is over
context-labelled transitions. That does not quite parse: the labels **are**
contexts, so preserving the relation presupposes a map on labels, and a map
of terms does not determine one. Two encodings agreeing on every term can
disagree about how a source context is realised in the target, and will
then preserve different relations.

So §10 proposes that a morphism be a pair `(F, Φ)` — `F` on terms, `Φ` a
multifunctor on the context multicategory — with equivariance
`F(c[t⃗]) ≈ Φ(c)[F t⃗]` and preservation along `Φ`. Embeddings are the
morphisms whose `Φ` is faithful and whose preservation is reflected.

Three consequences, all immediate:

- **Prop 68**: `D = im(Φ)`. The observer subcategory stops being a parameter
  supplied alongside a translation and becomes a component of it. Everything
  D-relative in the paper is relative to a morphism's context map, and
  relative minimality is minimality in the image of Φ.
- **Prop 69**: Definition 41 (encoding) and Definition 67 (morphism) are the
  same notion. The clause demanding that source equations become
  bisimulations is not extra — it is equivariance up to ≈.
- **Prop 70**: hosting and exhausting are faithfulness and density of Φ, not
  properties of the term map. Stated of `F` they look like bespoke criteria
  in Gorla's style; stated of `Φ` they are what one always asks of a functor.

**Conjecture 71**: generated logics are functorial on this morphism class,
though not on term maps alone — structural connectives are indexed by term
constructors, constructors are contexts, so Φ transports the structural
layer where a term map cannot. Companion work concluded the assignment is
not functorial; the conjecture says that was an artifact of the morphisms.

§10.3 names four open choices rather than presenting the definition as
finished: multifunctor vs profunctor; strict vs up-to-≈ (which forces a
bicategory — and this is *the same decision* as Remark 31's strict-vs-GRPO
question, so it should be made once); faithfulness kept out of morphism-hood
(build full abstraction in and the category has too few morphisms for the
cost and history adjunctions); and whether the decorations lift.

§10.4 settles the overlap with the reference account of GSLTs: machinery and
justification here, category and monads there, citing this paper for the
morphisms. Duplication becomes dependency.

## Attribution: the Lybech correction

The π→rho encoding in the draft is **Lybech's corrected encoding**, not
Meredith–Radestock's. That was already true of v1; it was just invisible,
because the bib key for Lybech 2022 was `pi-rho`, which reads as the
Meredith–Radestock paper, and because the π→rho theorem itself carried no
citation at all.

Fixed in v3:

- The duplicate/misnamed `pi-rho` entry is gone; the paper is cited as
  `lybech` (EPTCS 368, arXiv 2209.02356), with the technical report as
  `lybech-tr`. Citations that meant the *original* encoding now point at
  `rho` (Meredith–Radestock 2005), which is where that encoding lives.
- §7.3 now opens with the history: the two errors Lybech found, their
  common cause (the parameter invariant "n,p are the most recently
  replicated names", destroyed by the parallel split because substitution
  does not recur under quotes), and the fix (a single name server on a
  fixed `v`). Theorem 43 is attributed to him in its statement.
- New bib entries: van Glabbeek, Gorla–Nestmann, Bendixen–Bojesen–Hüttel–
  Lybech on rho as a higher-order Ψ-calculus, Sangiorgi on HOπ.

### This changes the status of the main result

v2 listed the ν lemma as "the largest piece of new mathematics" and the
reason π→rho was never proved. That was wrong: it is proved, against
Gorla-style criteria, in Lybech 2022. What is open is narrower and better
posed — whether those criteria imply Definition 41. That is now
**Obligation 45, the comparison theorem**, and it is broken into four
numbered parts.

His `≈^N` (barbs restricted to a name set) is D-relative bisimilarity with
D stipulated rather than derived. Saying so is the paper's clearest
statement of what the observer machinery buys, and Remark 47 says plainly
where the two presentations come apart: a barbed presentation assumes the
observations worth making are the names a term is ready to communicate on,
which is true of rho and π and false in general.

### And it supplies a second, already-proved flagship

New **§7.4 Separation**. Lybech proves rho cannot be encoded in π (or HOπ)
under those criteria, and the mechanism is that reflection manufactures
observable *free* names at runtime. In our vocabulary that is the same fact
as Proposition 12, `@` is not admissible — seen from outside rather than
inside. **Obligation 50** asks for the D-relative re-derivation, and §7.4
says why it is the sharpest available test of the apparatus.

Two other things Lybech already proved, now cited instead of reproved:
his parameter-independence proposition (which was Lemma 57 here) and his
quote-depth stratification lemma (the rigorous form of Lemma 56).

## Two remarks the encodings needed anyway

- **Remark 38**: rho!→rho is D-relative and false without the restriction —
  an arbitrary rho context can `out(n, junk)` and corrupt the duplicator's
  store, while no encoded context mentions `n`. The proved theorem was
  already an instance of the thesis and never said so.
- **Remark 39**: the input-guarding that Note 2's comprehension enforces is
  exactly Lybech's divergence fix.
- **Remark 58**: the failure mode §8's name discipline does *not* exclude —
  static increments live under quotes, so a root that is bound and
  re-substituted will not reach them. Bool is safe; a recursive type whose
  fold replicates its continuation would not be safe for free.

## Typography

The rho calculus is never written with the Greek letter. The name is an
acronym — the **r**eflective **h**igher-**o**rder calculus — and dropping the
periods from r.h.o. gives the transliteration of the letter after π, which is
the pun: the rho calculus comes after the π-calculus. Writing ρ throws the
acronym away and keeps only the joke.

So: *rho calculus*, *rho*, `\rhoc^{!}`. The macro `\rhoc` (= `\mrm{rho}`) is
in `bihol2.sty` for math-mode use; π keeps its Greek letter throughout, since
it is genuinely named for one. A footnote at the first occurrence (§3) states
the etymology.

## Build notes

- `bihol2.sty` replaces `bihol.sty`. Only two real changes: biblatex →
  natbib+bibtex (biblatex is not present in every texlive install), and
  `esvect`/`bbding` made optional with a `\vv` fallback. All v1 macros are
  preserved, plus `\D`, `\Dbis`, `\Dbisn`, `\ext` for the new material.
- `bihol.bib` gains seven entries: the two Sassone–Sobociński GRPO papers,
  Fiore–Plotkin–Turi, Gorla on encodability criteria, Mogensen on Scott
  encodings, Milner's *Functions as processes*, and Plotkin's SOS.

## Open question for the authors

The title page lists three authors. That was an assumption, not a decision —
say the word and it goes back to two.
