# Mathematical review of `graded-where-quantum-v6`

## Summary

The revised paper is materially stronger than the earlier `graded-where` note. In particular, it now:

- normalizes over whole contention components rather than individual candidates or receipt sites;
- treats maximal matchings as the alternatives of a contention component;
- sums amplitudes over matchings with the same residual outcome before taking norms;
- distinguishes column normalization from column orthogonality;
- retracts the earlier false claim that reconvergence requires structurally congruent racers;
- introduces an explicit Julia–Halmos defect dilation for contractions;
- gives a concrete quantum instrument for the success/failure herald;
- states global norm preservation under unrestricted concurrency as a conjecture rather than a theorem.

Those are substantial improvements. The remaining problems are more focused. The most serious are: the linear operator is still not cleanly defined on a single well-specified Hilbert-space basis; register activation is not correctly tied to the operational semantics; the PostBQP discussion confuses contractivity with non-isometric filtering; and the per-component dilation only has a clear justification inside the staged factorizing fragment.

I would currently regard the paper as a promising candidate semantics for a restricted fragment, but not yet a complete quantum semantics for graded rho.

---

## 1. The carrier of the linear operator is still underspecified

The paper defines, for a register `S`, a Hilbert space

\[
H_S = \bigotimes_{x \in S} \mathcal F(x)
\]

with occupancy states as an orthonormal basis, and states that `[[P]]` lies in `H_reg(P)`. Later, however, the isometry and collision analysis treats distinct rho configurations as distinct columns of a transition operator.

These two viewpoints do not obviously coincide.

Two syntactically distinct rho configurations may have the same quantum-channel occupancies while differing in:

- installed receipts;
- continuations;
- classical messages;
- quoted processes;
- scopes and fresh-name structure;
- future enabled rewrites.

If the Hilbert basis remembers only occupancies, then such configurations are represented by the same basis vector and cannot simultaneously be distinct input columns of the operator. If the intended basis is instead indexed by classical control configurations paired with quantum occupancies, the current definition of the state space is incomplete.

### Suggested repair

Define a hybrid carrier explicitly. For example,

\[
\mathcal H = \bigoplus_C H_{\mathrm{reg}(C)},
\]

where `C` ranges over a precisely specified equivalence class of classical control configurations. Alternatively, define basis states directly as costed/configuration states decorated with quantum occupancy data.

Then define the one-step operator on that basis before discussing column orthogonality, absorption, spectators, or global isometry.

This is foundational: until the basis is fixed, statements about `T^†T = I` are not fully typed.

---

## 2. Register activation is not correctly tied to `new`

The paper says that the register grows by tensoring newly active modes with vacuum, and that `new` is the term former that invokes this embedding.

But by the paper's own definition, a name belongs to `reg(P)` when `P` can produce or consume on that name. In a term such as

```rholang
new q in q!(P)
```

`q` is already active in the body because the body can produce on it.

Conversely, genuinely dynamic activation may occur through substitution after communication. For example,

```rholang
for (x <- a) {
  x!(P)
}
```

may receive a previously inert name and then use it as a channel. The activation event is caused by communication/substitution, not by `new` alone.

### Suggested repair

Define register change operationally in terms of the source and target configurations of a rewrite:

- compute `reg(src(r))` and `reg(trgt(r))`;
- whenever the target register strictly extends the source register, apply the vacuum embedding;
- make this independent of which syntax constructor happened to expose the new active name.

This would also handle reflection and dynamically received channels uniformly.

---

## 3. The revised race semantics is coherent only because maximal progress changes the operational semantics

The new contention-component semantics is a real improvement. It avoids the earlier inconsistency in which independent redexes could be normalized differently depending on reduction order.

However, it does so by imposing maximal progress on quantum channels: enabled independent quantum events are bundled into one concurrent layer rather than admitted as distinct serial and simultaneous histories.

That is a legitimate design choice, but it should be stated more prominently as a semantic extension of rho, not merely as a consequence of complex-valued `where` clauses.

The quantum language is therefore not just:

> rho + complex `where`

but at least:

> rho + complex `where` + quantum namespace + linear-use discipline + contention components + maximal progress + concurrent bundling + halt tagging + staging + herald instrumentation.

### Why this matters

A proof-relevant GSLT semantics would naturally distinguish

```text
par2(r1,r2)
par1(r1, s(r2)) ; par1(t(r1), r2)
par1(s(r1), r2) ; par1(r1, t(r2))
```

as different rewrite histories, potentially with different amplitudes. The present paper instead removes the distinction by changing which one-step executions are admitted on quantum channels.

That is coherent, but it is a substantial semantic commitment and should be advertised as such.

---

## 4. Per-component completion is not a general global-isometry theorem

The paper correctly defines contention components and, under staging plus unit occupancy, proves a tensor factorization

\[
T_P = \bigotimes_i T_{C_i}.
\]

Within that fragment, it is natural to complete or dilate each component independently.

Outside that fragment, however:

- components may later merge or split;
- two components may act on overlapping state-space factors;
- generated names may change the active register;
- spectator configurations may create support collisions;
- distinct source configurations may have overlapping images.

The paper itself acknowledges this by leaving global norm preservation conjectural.

### Suggested repair

Make the scope of per-component dilation explicit:

- **inside the staged, factorizing fragment:** local component dilation is justified;
- **outside it:** either refrain from claiming a physical quantum interpretation, or assemble the full finite transition operator and apply a global contraction/dilation.

A global fallback is mathematically simple: for a finite raw operator `A`, choose `λ >= ||A||`, set `T = A/λ`, and dilate `T` by the Julia–Halmos construction. This gives a unitary embedding for any finite weighted rewrite operator, at the cost of explicit postselection/heralding.

---

## 5. The PostBQP discussion confuses contractivity with non-isometric filtering

The paper uses

\[
\operatorname{diag}(1,\epsilon), \qquad 0 < \epsilon < 1,
\]

as the witness for postselection-like overshoot, but this operator is a contraction:

\[
\|\operatorname{diag}(1,\epsilon)\| = 1.
\]

So the problem is not that the filter is noncontractive.

The real distinction is between:

- an isometry, where `A^†A = I`, and
- a contractive but non-isometric filter, where `A^†A < I` on some subspace.

A non-isometric contraction is physically valid as the successful Kraus operator of an instrument. The source of PostBQP power is **free conditioning on the success branch**, not failure of contractivity.

### Suggested repair

Reorganize the section around the following hierarchy:

1. `||A|| <= 1` is the condition required for defect dilation;
2. `A^†A = I` is the condition under which no defect/herald branch is needed;
3. a contractive non-isometry is a filter and must be represented as an instrument with success/failure outcomes;
4. free renormalization of only the success branch is postselection;
5. charging retries or retaining the herald prevents that conditioning from being free.

This would align the conceptual story with the paper's own later and better machinery.

---

## 6. Halt tagging preserves unitarity by retaining timing information

The paper identifies an important problem: an absorbing normal form can be reached from another branch, making the two columns non-orthogonal. Its repair is to tag a halted state with the step at which it halted.

Mathematically, that is equivalent to adjoining a clock/history register:

\[
|Q,t\rangle \perp |Q,t'\rangle \quad (t \neq t').
\]

This is a valid dilation technique, but it has an interpretive consequence: branches that eventually become the same rho term at different times no longer interfere because the halt-time record preserves which-time information.

### Suggested clarification

State explicitly whether the halt tag is:

- part of the observable state;
- hidden but retained forever;
- later uncomputed/erased before observation.

If it is never erased, some delayed reconvergences that would occur in a pure path-sum semantics are intentionally suppressed.

A different design is to pad shorter computations with stationary identity steps to a common final depth. The two approaches are not obviously equivalent.

---

## 7. Conflict via “same receipt” needs more care for persistent receipts

The paper defines two candidates to conflict when they share a datum or share a receipt.

For a linear receipt, sharing the receipt naturally creates mutual exclusion because the receipt is consumed.

For a persistent receipt, however, the receipt survives firing. Therefore “same receipt” does not by itself imply the same operational exclusivity unless the semantics additionally imposes that a persistent receipt may fire at most once in a maximal-progress layer.

### Suggested repair

Either:

- define conflict differently for linear and persistent receipts; or
- state as part of the layer semantics that each receipt occurrence, persistent or not, contributes at most one firing to a concurrent step.

Because contention components are connected components of this conflict relation, this point affects the whole normalization construction.

---

## 8. Contractivity cannot generally be an elaboration-time property of a dynamically formed contention component

The paper says that a clause is contractive at a component when its matrix has norm at most one, and that this is checked at elaboration.

But the paper also correctly emphasizes that completion is nonlocal and depends on the actual contention component present in a configuration.

In a mobile higher-order calculus, future contention components can depend on:

- dynamically received names;
- installed receipts;
- changing message populations;
- aliasing of quantum capabilities;
- generated names;
- reflection.

So there is a tension between “contractive at a component” and “checked at elaboration.”

### Suggested repair

Distinguish two notions:

- **static clause contractivity:** a syntactic sufficient condition on the finite table or local matrix generated by a clause;
- **runtime component admissibility:** the actual completion/isometry condition for the dynamically assembled contention component.

For circuit-shaped closed terms these may coincide, but they should not be presented as the same judgement for the full calculus.

---

## 9. The quantum/classical namespace still leaves an aliasing obligation

The paper correctly retracts the earlier claim that unforgeability implies unique ownership. A `new`-bound quantum capability can be copied.

The replacement argument is that copying the name does not clone the datum because a message is consumed only once.

That addresses one immediate cloning concern, but it does not yet prove that future aliased uses preserve a globally linear quantum operation. Aliases can affect:

- which receipts are enabled;
- how the conflict graph is formed;
- which contention components merge;
- whether staging assumptions remain valid;
- which matrix was actually checked for contractivity;
- whether apparently independent terms act on the same quantum mode.

The paper acknowledges part of this as an open obligation. That is appropriate.

### Suggested repair

Downgrade claims such as “ownership is not needed” to a conjectural or argumentative status until an aliasing theorem is proved for the quantum-safe fragment.

---

## 10. The permanent theorem is sounder than the stronger BosonSampling rhetoric

The combinatorial theorem is attractive: if all maximal matchings of an `m`-fold contention land in the same symmetric continuation, the resulting amplitude is the permanent of the matching matrix.

That is a meaningful result.

However, a permanent-shaped amplitude is not by itself a full BosonSampling instance. BosonSampling also fixes:

- the physical one-particle unitary;
- the normalized bosonic Fock-space convention;
- occupation-number factorial factors;
- the induced sampling distribution over output occupations.

The paper itself now explicitly says that the canonical identification with normalized bosonic Fock space is not settled.

### Suggested repair

Replace claims of the form

> “an `m`-fold contention is a BosonSampling instance”

with something like

> “an `m`-fold contention realizes the permanent-valued combinatorics underlying bosonic scattering.”

Likewise, the two-path example reproduces the algebraic interference pattern associated with Hong–Ou–Mandel interference, but calling it literally HOM should be reserved for a complete optical representation theorem.

---

## 11. The “completion” proposition is close to an admissibility equation rather than a substantive theorem

The completion condition says, in effect, that after summing amplitudes over matchings with the same output, the resulting column has norm one, together with the additional requirement that distinct columns be orthogonal.

That is mathematically correct, but close to restating the definition of an isometry in the chosen basis.

The genuinely interesting theorem would be a syntactic criterion implying those equations automatically.

### Suggested reframing

Treat the current completion equation as a **semantic admissibility condition**, then make the main theorem target something like:

> staged, token-conserving, halt-tagged, dual-rail terms induce mutually orthogonal columns, so physical admissibility reduces to local contention-fibre normalization.

That would be a strong theorem if proved. At present the corresponding claim remains conjectural or empirically surveyed.

---

## 12. The dilation and instrument sections are genuine improvements

The following parts should be retained and emphasized:

- the defect operators are correctly placed;
- the Julia–Halmos block is the right unitary completion for a contraction;
- the successful branch leaves the intended contraction unchanged;
- the success/failure herald is represented explicitly;
- the herald instrument is defined as a trace-preserving completely positive map;
- the paper now distinguishes the globally pure joint state, the reduced decohered state, and the measured classical outcome.

These points resolve one of the major defects of the earlier manuscript, where the herald was treated inconsistently as both coherent and classical.

---

## 13. A cleaner global alternative remains available

There is a conceptually simpler construction that may be useful as a reference semantics, even if the paper prefers the more local contention-component interpretation.

1. Assign a complex weight to each base communication proof using the graded `where` clause.
2. Assign weight-combination functions to all contextual rewrite constructors of the rho GSLT, including `par_1`, `par_2`, and `new`.
3. Treat rewrite proofs as proof-relevant: distinct derivation trees are distinct alternatives even when they have the same source and target.
4. Assemble the raw one-step operator

\[
\langle Q|A|P\rangle
=
\sum_{r:P\Rightarrow Q} w(r).
\]

5. Add a distinguished stationary self-loop to stuck or out-of-gas states.
6. Use cost accounting to bound execution depth by `n`.
7. Choose `λ >= ||A||` and define `T = A/λ`.
8. Apply the Julia–Halmos dilation of `T` with fresh ancillas.
9. Postselect on the all-success branch for `n` steps, yielding a state proportional to `A^n|P>`.

This construction does not require any causal-diamond coherence law because serial, simultaneous, and differently nested proof trees are allowed to have different amplitudes. It also preserves the ordinary GSLT contextual proof structure instead of imposing maximal-progress bundling.

It is less local and potentially more expensive, but mathematically it provides a useful baseline against which the paper's restricted local semantics can be compared.

---

## Overall recommendation

The paper has improved considerably and now contains several genuinely solid repairs. In particular, contention-component completion, explicit column-orthogonality analysis, and the defect-dilation/instrument construction are worthwhile advances over the earlier note.

I would nevertheless recommend **major revision** before treating the paper as a complete quantum semantics.

The highest-priority fixes are:

1. define one unambiguous Hilbert-space carrier and the one-step operator on its basis;
2. repair the register-activation story so that it follows actual rewrites/substitution rather than `new` alone;
3. rewrite the PostBQP section around non-isometric filtering and free conditioning rather than noncontractivity;
4. sharply delimit where per-component dilation is proved valid;
5. state maximal progress as an explicit semantic extension of rho;
6. clarify the status of halt tags, persistent receipts, and dynamic component contractivity;
7. weaken the BosonSampling/HOM rhetoric to the combinatorial results actually established.

With those repairs, the strongest defensible claim would be something like:

> A finite, staged, contractive fragment of graded rho admits a coherent contention-based interpretation whose admissible components can be embedded into unitary dynamics by defect dilation; permanent-valued recombination arises from symmetric maximal matchings, while extension to unrestricted reflective concurrency remains open.

That is already an interesting result and does not need the stronger claims that currently create most of the mathematical vulnerability.
