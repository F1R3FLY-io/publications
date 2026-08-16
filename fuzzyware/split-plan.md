# Splitting *Graded Where-Clauses* into three papers

Status: plan agreed 16 August 2026, in response to
`fuzzyware/review-of-graded-where-v4.md`. Paper I is drafted
(`graded-where-quantum-v5.tex`); Papers II and III are not yet written.

## Why split

Version 4 carried three theorem stacks of very different maturity in one
document: a resolution-algebra story that is settled, a quantum story that the
review showed to be under-supported, and a probabilistic-logic story that is
illustrative. The reviewer's own recommendation (his §14) was to separate the
formal-languages contribution from the quantum research programme. Splitting
also lets each paper carry the related work it actually needs — the quantum
paper belongs against the quantum-control literature, the PLN paper against
uncertain inference, and neither reader should have to read the other's
bibliography.

## The three papers

### I. The quantum case — `graded-where-quantum-v5`

The complex instance in full: the namespace split, the register-indexed
denotation, the contention-set completion, the quantum-safe fragment, dual-rail
gadgets, Shor, dilation and priced heralds.

Continues the version numbering from version 4 so the review can be tracked
against it. Drafted.

### II. The PLN case — `graded-where-pln`

The R≥0 instance in full. Carries over version 4 §§10–11 (PLN-valued clauses,
the mortal forager) and expands them:

- the resolution algebra PLN needs, and which of PLN's operations supply values
  rather than algebra;
- the forager: bootstrap trap, the two worlds, amortisation, the metabolic
  optimum for the personality parameter *k*;
- the trichotomy, and the dissolution of the complementary guard pair's
  dependence on negation in the hypothesis logic;
- honest framing per the review's §13: a constructed decision problem with an
  interior optimum, not an empirical result about PLN agents. Robustness
  analysis over the world parameters is the obvious addition;
- testimony and conservation of evidence, currently version 4's Obligation 6,
  as the forward-looking section.

Ships `forager.py`.

### III. The overview — `resolution-algebras`

The motivation and the algebra, with the two instances sketched.

- where the choice is: rho is silent on how a race is resolved, the resolver is
  a parameter, and the parameter has more than one value;
- why context-based full abstraction cannot expose the parameter, and what
  replaces it (separation between resolvers, one-sided support adequacy);
- the resolution algebra in detail — this is the part version 4 compressed and
  the part that carries the novelty claim: a semiring-valued expression placed
  exactly at candidate-match resolution;
- conservativity for **B**, the DNF theorem (weight maps are linear graded
  clauses), plasticity is free, the causal-history proposition;
- the join as recombiner and the scattering theorem, since it is instance-
  independent;
- sketches of the complex and R≥0 instances, each two or three pages, pointing
  at Papers I and II;
- the algebra table: **B**, [0,1], R≥0, Viterbi, tropical, **C**, quantale.

Ships `gradedsim.py`.

## What each paper owes the review

| Review item | Severity | Lands in | Status |
|---|---|---|---|
| Dilation undefined for non-contractive clauses | Fatal | I | closed — contractivity is a judgement |
| Local dilation ≠ global norm preservation | Fatal | I | closed at a configuration; conjectured in general |
| Herald lacks measurement semantics | Fatal | I | closed — the herald is classical by construction |
| Stage-one proposition counterexampled | Fatal | I | closed — restated, his example is now an instance |
| BQP-style containment conjectural | Major | I | claims aligned; still a conjecture |
| No ownership / compositional semantics | Major | I | closed — ownership from unforgeability |
| No bibliography / prior art | Major | all three | closed in I; owed by II and III |
| Contextual-insensitivity too strong | Major | I and III | closed — stated as a definition, with a citation |
| Gillespie identification imprecise | Moderate | III | jump chain, pointer to the weighted-GSLT note |
| Forager overinterpreted | Minor | II | to do |

## Sequencing

1. **Paper I** — drafted. Outstanding: a proof of the staging proposition
   (currently a measured survey), and the recursion problem raised by
   Bădescu–Panangaden, which is stated as an open problem and has no fix.
2. **Paper III** — next. It is mostly extraction and compression from version 4,
   and it is what a reader should meet first, so it should exist before Paper I
   is circulated.
3. **Paper II** — last. Needs the robustness analysis the review asks for, which
   is simulation work rather than writing.

## Files

| | source | scripts |
|---|---|---|
| I | `graded-where-quantum-v5.tex` | `dualrail.py`, `contention.py`, `collision.py`, `dilatesim.py`, `dualshor.py` |
| II | `graded-where-pln.tex` | `forager.py` |
| III | `resolution-algebras.tex` | `gradedsim.py` |

`graded-where-v4.tex` and `graded-where.tex` stay in the directory as the
record the review was written against.
