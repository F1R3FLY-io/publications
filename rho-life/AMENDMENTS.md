# Book-side amendments implied by `scope-note`

Kept per the plan: the note stands alone and disagrees quietly; this is the
list to work from when the material is folded into *Finding Mind* after the
review cycle. Chapter references are to the draft-8 build.

## Must change

**`TheTurn/turn_origins_ch02.tex`, `def:copy-number`.**
Currently extensional (a namespace is a finite set of channels) and
ungraded (`CN(P,N)` is an integer, defined by full bisimilarity). The note
replaces this on three counts: scope becomes a name predicate
(Def. 3.2), the count acquires a resolution argument (Def. 4.2), and in a
stratified scope the output is a series (Def. 4.8). Suggested handling in
the book: keep the existing definition as the flat, full-resolution case
and add the general one after it, so that the Gap 1 / Gap 2 discussion in
the same chapter still reads correctly.

**`TheTurn/turn_origins_ch02.tex`, the biosignature remark.**
`AI(P) >> 1 and CN(P,N) >> 1` needs the resolution argument or it inherits
the overcounting problem the note identifies. Minimum change: write
`CN_n` and add a sentence that `n` is the observer's affordable depth.

**`TheTurn/turn_origins_ch04.tex`, "The Joint Rise of Assembly Index and
Copy Number".** Currently qualitative — selection raises both. The note
gives the functional form (`CN >= m p^{-h}`, Prop. 7.3) with slope set by
medium reliability. This is a strict strengthening and the remark should
carry it, plus the falsifiable version in Rem. 7.4 (the ratio
`log CN / h` clusters near `log(1/p)` for systems using their
architecture).

**`TheTurn/turn_origins_ch08.tex`, §"Copy number is the other axis".**
Currently says copy number "does not extend the depth bound; it does
something orthogonal". That is the claim the note contradicts: the axes
are coupled, and `rmk:depth-width` in the same section already has the
ingredients (`p^d` fragility versus linear width) without drawing the
conclusion. Rewrite the section around the frontier.

**`TheTurn/turn_origins_ch08.tex`, third open gap.**
"`AI(E)` presumes a solution to the individuation fixed point." The note
argues this and the affordable-scope question are the same circularity
(open problem 4). Either merge the two statements or cross-reference.

## Should change

**`TheTurn/turn_origins_ch08.tex`, first open gap** (distinct levels,
distinct edges). Prop. 5.6 of the note discharges this whenever the strata
are disjointly rooted. The gap should be narrowed to the non-disjoint
case rather than left whole.

**`turn_rholife_ch04`, `eng:rem:fixpoint`.** If the mu-versus-nu point in
Rem. 3.13 of the note holds up, individuation-as-a-fixed-point should say
which fixed point, and why a budgeted learner can only inhabit the least
one.

**`turn_rholife_ch03` / `compose-note` §5.** The unique-decomposition
proposition (Prop. 3.5) is the syntactic face of the factorisation
theorem. Worth a forward reference so the two are visibly one result.

## Watch

**`prop:nesting-bound` and the characteristic-formula route.** Rem. 6.5 of
the note flags that a lower bound routed through `chi` rather than through
the tower needs to know whether one joining step can more than increment
modal depth. The book's existing bounds go through the tower and are not
affected, but anything built on the other route would be.

**Terminology collision.** The book uses "namespace" for the extensional
object throughout; the note uses "scope" for the intensional one and
reserves "namespace" for the companion notes' usage. If the material is
folded in, pick one word and sweep.
