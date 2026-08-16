I reviewed the **v5 quantum paper itself**, including the TeX source, and re-ran the main objections from v4 rather than merely checking whether the manuscript says they were fixed.

My assessment has changed materially.

**Recommendation: Major Revision / borderline Reject, rather than the clear Reject I gave v4.** There is now a serious paper here. Several of the v4 fatal flaws have been genuinely repaired. But I still see **two claims that I would expect a hostile PL/quantum reviewer to attack as incorrect or substantially overstated**, plus several major gaps that the paper appropriately labels as open but which constrain what the abstract and conclusion are entitled to claim.

The two most dangerous points are **“ownership from unforgeability”** and **“bosonic statistics are forced by structural congruence.”**

## Executive assessment

The strongest conceptual move in v5 is no longer “put complex numbers on races and call them quantum.” The paper now recognizes that a quantum interpretation requires considerably more structure: a register-indexed state space, global contention components rather than individual receipt sites, normalization after coherent fibre summation, column orthogonality, contractivity, explicit classical heralding, and restrictions addressing absorption and spectators. 

That is a substantial improvement.

The paper is also commendably explicit about what it **doesn't** establish: global norm preservation, a theorem characterizing the staged fragment, complexity containment, and recursion semantics remain open. 

That intellectual honesty changes the review considerably. I would not reject v5 merely because those are open problems.

I would, however, challenge some things currently presented as **established propositions**.

---

# 1. Major problem: unforgeability does not imply unique ownership

This is currently the cleanest attack.

Proposition “Ownership from unforgeability” says essentially:

1. a `new`-bound quantum name cannot be fabricated by a process that has not received it;
2. therefore quantum data has unique ownership.

The actual proposition is weaker than the surrounding prose. It establishes that an outsider cannot independently acquire the reference and that each datum has at most one consumer *per firing*. 

That is **capability unforgeability**, not linear ownership.

Suppose (q) is a fresh quantum name known to process (P). What prevents (P) from distributing (q) twice?

Conceptually:

```text
new q in {
    a!(q) | b!(q)
}
```

or whatever the well-formed rho encoding of that duplication is under the paper's name/payload conventions.

Freshness tells us nobody can *guess* (q). It does not tell us a holder of (q) cannot **copy the capability**.

This distinction is exactly why quantum process calculi such as CQP use linear/unique-ownership typing: after ownership is transferred, the sender cannot continue using the qubit. Contemporary descriptions of CQP emphasize precisely that property. ([ResearchGate][1])

The paper has a linear receipt arrow, but its global well-formedness definition only constrains receipts using that arrow and the placement/contractivity of graded clauses; it says “nothing else is refused.”  I therefore don't see a theorem preventing aliasing of a quantum name through ordinary process structure.

### Why this matters

If two processes can simultaneously possess the same quantum name, then

> “Unique ownership ... follows from unforgeability”

is false.

This doesn't necessarily destroy the whole model. It means you need another invariant.

I would change the claim from **ownership from unforgeability** to something like **authority from unforgeability**, unless you can prove a no-aliasing theorem.

Then introduce a linear capability discipline establishing something like:

[
\Gamma\vdash P
\quad\Longrightarrow\quad
\text{each quantum capability occurs in exactly one live ownership context}.
]

That would make the claim much stronger than its current informal argument.

**Severity: Major, potentially fatal to the claimed namespace result.**

---

# 2. “Bosonic statistics are forced by structural congruence” is overstated

This is the sentence I would most expect a quantum-information referee to circle.

The argument is:

[
P\mid Q \equiv Q\mid P
]

therefore messages on a channel are indistinguishable;

therefore the (n)-occupancy sector is the (n)th symmetric power;

therefore the statistics are bosonic and amplitudes are permanents. 

There is a good result hiding here, but the first implication does not automatically establish all the later ones.

Associative-commutative parallel composition gives you a **permutation-invariant syntax/configuration quotient**. It does not by itself establish the Hilbert-space inner product, normalization factors, creation/annihilation algebra, or canonical identification with bosonic Fock space.

For example, the normalized bosonic occupation state convention contains the familiar factorial normalization

[
|n\rangle
=========

\frac{(a^\dagger)^n}{\sqrt{n!}}|0\rangle.
]

The difference between “an unordered multiset containing (n) things” and “the normalized (n)-particle sector of bosonic Fock space” is not trivial.

Your **scattering theorem** is stronger and more defensible: once alternatives are indexed by permutations and symmetric continuations identify their outcomes, the coherent sum is

[
\sum_{\sigma\in S_m}
\prod_i A_{i,\sigma(i)}
=======================

\operatorname{Per}(A).
]

That follows directly from the construction as stated. 

But that gives you:

> rho structural congruence naturally induces the permutation symmetry responsible for permanent-valued recombination.

That is safer than:

> bosonic statistics are forced by structural congruence.

The latter sounds like a derivation of quantum statistics from AC syntax. You have derived a **combinatorial symmetry**. Additional semantic choices identify it with normalized bosonic Fock space.

**Severity: Major claim overreach, but readily repairable.**

---

# 3. Proposition 5.1 needs a much more explicit independence hypothesis

The new contention-component semantics is one of v5's strongest improvements.

You correctly moved from local receipt normalization to connected components of the conflict relation, use **maximal matchings** as alternatives, sum amplitudes within each outcome fibre, and only then impose unit norm. 

That addresses my principal v4 objection.

But the proposition then asserts

[
T=\bigotimes_iT_{C_i}.
]

That statement deserves considerably more proof than it currently receives.

Disjointness of the *current conflict graph* does not necessarily imply tensor independence of the induced continuations.

Two currently independent components may produce into common channels, introduce names whose scopes interact, enable a later joint receipt, or otherwise produce continuations whose register structure overlaps.

The manuscript itself immediately acknowledges that components may subsequently merge and split and labels global preservation conjectural. 

That's fine for **later** evolution. But Proposition 5.1 should make absolutely precise what its tensor claim means:

[
T_P:
H_{\operatorname{reg}(P)}
\rightarrow
H_{\operatorname{reg}(\mathrm{next}(P))}
]

and why current components act on genuinely tensor-factorizable subspaces.

If it only means:

> the *choice variables for this transition layer* factor as a Cartesian product of independent matching sets,

then say that.

That's combinatorially clear.

It is stronger to say the **Hilbert-space operator itself tensor-factorizes**, particularly given your earlier observation that overlapping registers destroy compositionality. 

I suspect the proposition can be repaired by distinguishing **event-factorization** from **state-space tensor factorization**.

**Severity: Major proof obligation.**

---

# 4. There is tension between the “quantum namespace” and the register definition

The register is defined as quantum names at which a process can produce or consume, while names occurring only as inert payloads are excluded. 

But quantum names themselves are mobile capabilities: receiving a quantum name makes it quantum, and quantum names travel on quantum channels. 

That creates a subtle semantic question.

Suppose a quantum channel (q) is currently present merely as payload but, after communication, becomes an active channel. Before reduction it isn't part of the register under the “inert payload” rule; afterward it is.

So the carrier Hilbert space itself can change according to reduction.

That isn't necessarily wrong—Fock-space/QFT formulations can handle variable sectors—but then the paper needs an explicit embedding

[
H_S\longrightarrow H_{S\cup{q}}
]

and needs to specify the vacuum state assigned to newly activated modes.

Otherwise statements such as (T^\dagger T=I) are underspecified because the domain and codomain are changing.

This becomes particularly important for the global-norm conjecture.

**Severity: Major semantic clarification.**

---

# 5. The herald story is much better, but “classical because quoted” doesn't itself define measurement

V4's herald problem has been **partially and meaningfully repaired**.

V5 explicitly puts the herald on the classical channel

[
h(r)=@r
]

and says the classical receive is where the quantum instrument acts. 

Good.

But there remains a semantic step hidden inside the phrase “written as a classical message.”

The dilation produces

[
A|\gamma\rangle|\mathrm{ok}\rangle
+
D_A|\gamma\rangle|\mathrm{fail}\rangle .
]

That is still a coherent joint state. 

For `ok` or `fail` to become an ordinary classical rho message, something must implement the quantum instrument

[
\rho\mapsto
A\rho A^\dagger\otimes|\mathrm{ok}\rangle\langle\mathrm{ok}|
+
D_A\rho D_A^\dagger\otimes|\mathrm{fail}\rangle\langle\mathrm{fail}|.
]

Merely declaring the destination name classical does not mathematically induce this map.

You effectively acknowledge this by saying “that is where the instrument acts.” 

So **define the instrument**.

That would resolve most of my objection. At present the syntax tells us *where* measurement happens, while the semantics still needs to tell us *what measurement is*.

**Severity: Major but straightforwardly repairable.**

---

# 6. “Leaving the herald unread is a decoherence event” appears backwards without an environment semantics

The manuscript says an unread herald is a record and leaving it unread causes decoherence. 

Not automatically.

If the global state remains

[
A|\psi\rangle|ok\rangle+D_A|\psi\rangle|fail\rangle,
]

then the system-plus-herald state is still pure and coherent.

Decoherence of the system appears if you **trace out** the herald:

[
\rho_S
======

\operatorname{Tr}_H
|\Psi\rangle\langle\Psi|.
]

But the manuscript simultaneously emphasizes that the herald is “neither traced nor projected.”

You cannot have all three claims without distinguishing perspectives:

* globally, the herald remains quantum/coherent;
* relative to a subsystem that ignores it, tracing gives decoherence;
* if it is actually classicalized, an instrument has already destroyed the coherence.

This needs tightening because it touches the exact quantum/classical boundary the paper claims as a contribution.

**Severity: Major conceptual inconsistency.**

---

# 7. The quantum-safe “Proposition” should not be called a proposition

This is partly presentation, but important presentation.

You state:

> **Proposition [The quantum-safe fragment]**

and then:

> “in every configuration searched...”

Immediately afterward you correctly explain that it is a proposition **about the search**, not a theorem, and that no proof excludes a third failure mode. 

Calling this a Proposition still gives it theorem-like epistemic status.

I'd rename it:

**Empirical Finding 7.x (Finite-state search of the candidate safe fragment).**

Then state the actual conjecture separately:

[
\boxed{
\text{halt-tagged + staged + dual-rail + token-conserving}
\Rightarrow
\text{pairwise orthogonal columns}
}
]

This would make the paper look *more* rigorous, not less.

The empirical search is valuable precisely because you are not pretending it proves the invariant.

**Severity: Moderate.**

---

# 8. The Shor result is now appropriately scoped

I attacked this heavily in v4.

V5 fixes most of the rhetorical problem.

The paper now explicitly says the Shor construction establishes expressiveness of the fragment, **not** a native quantum idiom and not an efficient classical simulation. It even observes that the circuit-shaped example cannot test the contentious semantic machinery. 

That's exactly right.

I would retain the example.

But I'd change:

> “expressive enough for the standard algorithms”

to something like

> “expressive enough to encode this standard circuit model and, concretely, this Shor instance.”

One (N=15) construction is evidence of a compilation scheme; the general universality result should rest on the gate lemmas, not the example.

**Severity: Minor.**

---

# 9. The postselection section is substantially improved, but the PP proposition is stronger than what is shown locally

The paper correctly recognizes why unrestricted nonunitary filtering is dangerous: postselection raises quantum computation to PP; Aaronson's PostBQP = PP result is the appropriate reference. ([Scott Aaronson][2])

The Julia–Halmos construction is also now correctly restricted to contractions, matching the mathematical condition under which the standard defect-operator construction applies. ([PubMed Central (PMC)][3])

So my v4 contraction objection is **resolved**.

However, Proposition “Overshoot” says the unrestricted graded fragment *decides PostBQP, hence PP*. 

That requires more than observing that arbitrary nonunitary filters resemble postselection. You need a compilation of PostBQP circuits into the language with polynomial overhead—or invoke your universal gate construction plus a precise postselection gadget and prove the simulation.

Likewise, the ledger conjecture correctly admits it isn't yet a complexity containment theorem. 

I would phrase the established result as:

[
\text{PostBQP}\subseteq
\text{unrestricted graded-rho}
]

if you can prove the encoding.

Avoid language suggesting equality unless the converse is established.

**Severity: Moderate/Major.**

---

# 10. The permanent theorem is one of the strongest results in the paper

This deserves emphasis because the aggressive review should distinguish weaknesses from genuine strengths.

The scattering theorem is crisp:

if a contention component contains (m) patterns and (m) data, matchings correspond to permutations, and if the continuation identifies all assignment outcomes, the amplitude becomes

[
\sum_{\sigma\in S_m}\prod_iA_{i,\sigma(i)}
==========================================

\operatorname{Per}(A).
]

If the permutation sign is inserted, it becomes the determinant. 

This is mathematically transparent and directly tied to process structure.

I would actually make **this** result more central and tone down the grander bosonic claim.

“Rho joins naturally implement permanent-valued coherent recombination” is interesting even if the eventual physical interpretation changes.

---

# 11. The paper now has a real related-work section

This fixes another major v4 deficiency.

You now engage CQP, QML, QGCL, Bădescu–Panangaden, weighted computation, stochastic calculi, linear optics, KLM, Aaronson–Arkhipov, and related material, and you narrow the novelty claim to the placement of the semiring-valued expression at **candidate-match resolution** with rho joins providing recombination. 

That is a much more defensible novelty claim.

I would nevertheless resist the sentence:

> “The natural comparison class is not quantum process calculi but quantum control.”

It's **both**.

Your object is still a mobile process calculus carrying quantum information. CQP-style ownership, operational equivalence, measurement, and compositionality questions remain directly relevant even if quantum alternation is the closer comparison for the central novelty.

Say:

> “The closest comparison for coherent race resolution is quantum control/alternation, although quantum process calculi supply important contrasting treatments of ownership, communication, and measurement.”

That is harder for a referee to object to.

---

# 12. The deepest unresolved problem is exactly the one the paper says it is

The recursion issue is not cosmetic.

The manuscript acknowledges that quantum alternation is incompatible with the usual Löwner-order fixed-point construction and that rho recursion through reflection does not magically remove that semantic problem. 

Good.

But this means the paper presently has two levels of language:

[
\text{finite/staged quantum fragment}
]

for which a quantum interpretation is becoming credible, and

[
\text{full reflective rho calculus}
]

for which that interpretation is not established.

I would make that boundary explicit **much earlier**.

At present “rho calculus + quantum resolution” sounds more general than what has actually been justified.

The strongest defensible thesis is currently:

> **A finite, contractive, staged fragment of graded rho admits a candidate quantum interpretation; the extension to unrestricted reflective concurrency remains open.**

That is still a substantial result.

---


[1]: https://www.researchgate.net/publication/231537366_Analysis_of_a_Quantum_Error_Correcting_Code_using_Quantum_Process_Calculus?utm_source=chatgpt.com "(PDF) Analysis of a Quantum Error Correcting Code using Quantum Process Calculus"
[2]: https://www.scottaaronson.com/talks/anthropic.html?utm_source=chatgpt.com "Computational Complexity and the Anthropic Principle"
[3]: https://pmc.ncbi.nlm.nih.gov/articles/PMC7513250/?utm_source=chatgpt.com "Representation and Characterization of Nonstationary Processes by Dilation Operators and Induced Shape Space Manifolds - PMC"
