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
