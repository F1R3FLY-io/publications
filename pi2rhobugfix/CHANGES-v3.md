# Behavior in higher-order languages — third draft

Base: the committed draft (`behavior-in-higher-order-languages/main.tex`,
39pp). This draft is 44pp. pdflatex + bibtex + pdflatex ×2: no errors, no
undefined references or citations, zero overfull boxes.

## 0. The repo copy did not build

Two things draft 4 uses were never committed. Both are now supplied:

- `bihol2.sty` lacked `\stProposal` (used three times in the status
  table) and the `conjecture` environment (used for Conj. `oslf`).
  Added, along with `\stDeleg` for future use.
- `bihol.bib` lacked the `probing` entry, cited twice in §10. Added as a
  working note in preparation.

Worth checking whether the sty and bib in the repo are simply older than
the tex.

## 1. Driven by the companion note (`pi2rhobugfix`)

- **§7.3 rewritten around two encodings.** Lybech's name-server encoding
  keeps Thm. `pirho` and its attribution; the note's local-allocator
  encoding is added as Thm. `nowns` with its clause table, and is the
  stated instance. New Prop. `forced` (the restriction cannot be
  dropped), Remark `usedonce` (the restriction is consumed at exactly one
  step of the proof), and a remark comparing the two repairs as a
  difference in ⟦D⟧ rather than in correctness.
- **New §7.4 "Relative minimality, and what transfers"** — the load-
  bearing new mathematics. Prop. `descend` (proved): relative-IPO labels
  are a subset of enabling labels, so ≈_e ⊆ ≈_D and the *soundness* half
  of Thm. `nowns` descends to Def. `encoding` for free. Ob. `collapse`
  (stated, with the expected route): the converse — that relative
  minimality loses no separating power in the image — which is what the
  *completeness* half needs. A remark spells out the direction, since it
  is easy to get backwards.
- **Ob. `compare` re-aimed.** Clause (1) is now identified as Ob.
  `collapse`; for Thm. `nowns` the whole comparison reduces to it, since
  that equivalence is already context-labelled.
- **Ob. `pifull` retired**, replaced by a remark: full abstraction for
  π→rho is no longer an obligation of this paper.
- **Ob. `decomp` gets a route.** ⟦c(p)⟧ = ⟦c⟧(⟦p⟧) on the nose for the
  note's encoding, so decomposition-closure becomes a syntactic question
  about six translation clauses.
- **Ob. `sep` gets a route**, via the fact that encoded contexts never
  drop a received name, so the asymmetry the separation theorem exploits
  is already recorded as the admissibility of ∗(@(−)) but not of @.
- **§6 gains Remark `metered`** — the three name-creation facts, which
  make the inadmissibility of @ quantitative and explain why §8's name
  discipline must be checked rather than assumed.
- **§8 cites rather than reproves.** Lem. `names` and Lem. `root` now
  point at the note's template-disjointness and address-relabelling
  results, including the observation that the relabelling is not a
  substitution of the theory and so is available to no context at all.
- **§10 fork 2 gains a witness.** The equivariance square commutes on the
  nose for Thm. `nowns`, so the strict case is not vacuous and laxness is
  demanded by the ν-equations rather than by the shape of the definition.
- **Bib**: new entry `nowns`.

## 2. Driven by the external review

- **Abstract** rebuilt around one thesis sentence (every such
  construction assumes all contexts may observe; that assumption fails
  where one wants the construction), with the three instances as
  consequence and the companion theorem named as the evidence.
- **New §1.1 "One instance, in full, before any machinery"** — the
  reviewer's central complaint was that the paper proves the framework
  before selling the problem. The vignette gives the encoding, the
  separating context (⟦P⟧ vs ⟦P|0⟧), and Lybech's separation result, on
  page 2, before any definition.
- **The diagram the reviewer asked for**: reflection / encoding /
  extension at three corners with D in the middle, with one line saying
  what D is in each.
- **§2 demoted.** "Metatheories and free extension" moved to Appendix
  C (`app:freeext`), replaced by a short pointer subsection that also
  gives a reading path for a reader who wants the argument and not the
  foundations.
- **Prop. `fixpoint` promoted** to its own subsection, §4.2 "The
  circularity, and why it is respectable", with the circle stated
  explicitly before it is closed, and three remarks on what the fixed
  point buys — including that the second antitone map is syntactic and
  so the fixed point is computable, §5 being that computation for rho.
- **Contributions** gain the worked-flagship item.
- **Status table** gains four rows: Thm. `nowns` and Prop. `forced`
  (proved, cited), Prop. `descend` (proved here), Ob. `collapse`.
- **Conclusion** now opens the argument for the parameterization on the
  companion theorem — a theorem written for other reasons that needs the
  restriction and uses it once — and the open-problem list is reordered
  so that Ob. `collapse` is item 1.

## 3. Not done, deliberately

The review asked for observation relations to arrive "far earlier". Def.
`obsrel` needs lambda theories to state, so it cannot precede §2; the
version of the advice implemented is to move §2's heavy half out and put
a real instance in §1. The review also asked for a decision between
"framework paper with obligations" and "shrink and prove everything";
the posture taken is the first, but the flagship is no longer an
obligation, which is what made that posture defensible.
