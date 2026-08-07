Yes. This note is worth writing, and it does address the conceptual issue that triggered it. But in its present form I would **not yet rely on it to repair *The Mortal Scientist***. The core construction is promising; the theorem is stated much more generally than the proof currently supports.

My referee recommendation would be **major revision**, concentrated around the central reconstruction theorem rather than the motivating idea.

The good news is that I think the essential insight survives:

[
\text{make structural destructuring observable}
\quad\Longrightarrow\quad
\text{bisimulation can be refined toward structural equivalence}.
]

What does **not** yet survive is the current claim that the particular ( (-)^+ ) construction establishes this mechanically for *any* Turing-complete interactive GSLT.

## 1. The central theorem currently fails at the definition of “interactive”

This is the first thing I would fix.

You define an interactive GSLT by requiring every rewrite left-hand side to have the form

[
\mathsf K(l_1,l_2)
]

up to (=_E). You explicitly list application as the interaction constructor for the lambda calculus. ([GitHub][1])

Then the canonical extension adds

[
\operatorname{open}*C:
C(x_1,\ldots,x_n)
\longrightarrow
\mathsf K(C*\bullet,[x_1,\ldots,x_n]).
]

But this new rule **does not have an interaction-headed left side**.

The proof that (\mathbb G^+) remains interactive says that, when (\mathsf K) has a unit (u),

[
C(\vec x)=_E\mathsf K(C(\vec x),u),
]

which rescues the rule. For cuts without a unit, it proposes instead

[
\mathsf K(\mathrm{Op}*\bullet,[C(\vec x)])
\to
\mathsf K(C*\bullet,[\vec x])
]

and says “nothing below changes.” ([GitHub][1])

But everything below **does** change.

Your Exposure lemma requires the bare term (P) to take an empty-context transition

[
P\xrightarrow{(\operatorname{open}*C,[-])}
\mathsf K(C*\bullet,[\vec t])
\quad\Longleftrightarrow\quad
P=_E C(\vec t).
]

([GitHub][1])

Under the proposed no-unit replacement, the bare term (C(\vec t)) has no such redex. You need the external wrapper

[
\mathsf K(\mathrm{Op}_\bullet,[C(\vec t)]).
]

So the replacement destroys precisely the lemma from which the reconstruction proof proceeds.

This means that the theorem, as currently proved, applies at best to a subclass of interactive GSLTs whose cut supplies some transparent unit/context allowing arbitrary terms to be interaction-headed.

That includes parallel-composition calculi rather naturally. It does **not** obviously include application-as-cut.

Given that the note advertises generality across interactive GSLTs, this is a foundational issue.

### Possible repair

I see two paths.

The conservative one is to restrict the theorem:

> Let (\mathbb G) be an interactive GSLT whose cut admits a transparent unit (u) satisfying (P=_E\mathsf K(P,u)).

Then prove the result there. This would already cover the spatial-concurrency cases motivating the paper.

The more general path is to change the observational construction itself. Instead of requiring every `open` to be a base rule satisfying the original interaction-headedness discipline, introduce an **observer extension** in which observation rules are a separate class of transition. Then prove adequacy for that enlarged labelled transition system. That may actually be conceptually cleaner: `open` is an observation instrument, not an ordinary interaction of the base calculus.

At present, the sentence “where (\mathsf K) has no unit … nothing below changes” should come out.

## 2. “Turing completeness supplies lists” is not justified

This is the second serious technical issue.

You need an encoding

[
[t_1,\dots,t_n]
]

which is:

1. injective modulo (=_E);
2. component-recoverable;
3. suitable for arbitrary argument sorts;
4. structurally exposable by the very `open` construction you're defining.

You say Turing completeness guarantees this and that “any standard encoding will do.” ([GitHub][1])

Turing completeness guarantees an ability to represent computable data in some sense. It does **not** automatically guarantee an internal coding function satisfying those algebraic properties relative to an arbitrary equational theory (E).

In particular, the claim

[
[\vec t]=_E[\vec s]\Rightarrow \vec t=_E\vec s
]

is an injectivity property of a particular representation. Computability alone doesn't supply it.

There is also a typing problem. A general lambda theory can be many-sorted. A constructor may have type

[
C:\tau_1\times\dots\times\tau_n\to\tau
]

with heterogeneous (\tau_i). What is the sort of

[
[t_1,\dots,t_n]?
]

A homogeneous list does not contain arbitrary differently sorted arguments unless you introduce a universal term/code sort, dependent sum, tagged coproduct, etc.

That is another actual construction choice.

### I would simplify this drastically

Instead of deriving lists from Turing completeness, **freely adjoin an argument-vector constructor for every signature profile**.

For example,

[
\operatorname{args}_C(x_1,\dots,x_n)
]

with exactly the appropriate sorts.

Then:

[
\operatorname{open}*C(C(\vec x))
\to
\mathsf K(C*\bullet,\operatorname{args}_C(\vec x)).
]

You lose the slogan “Turing completeness is enough,” but you gain:

* a genuinely mechanical signature construction;
* no arbitrary encoding choice;
* correct sorting;
* immediate injectivity;
* no hidden computational assumption.

And I don't think you need Turing completeness anywhere in the central result afterward.

That would make the theorem **more general**, not less.

## 3. The theorem silently needs a well-foundedness condition on (E)

The Reconstruction proof says it inducts on term size:

> “the number of constructor occurrences in some (hence any, since (E) is size-respecting on the presentations we consider) (=_E)-representative of least size.”

([GitHub][1])

But the theorem is stated for an arbitrary interactive GSLT.

And your motivating structural theory includes a unit:

[
P\mid 0 = P.
]

That equation is **not size-respecting**.

Indeed, the next paragraph emphasizes that exposure enumerates decompositions including

[
P=P\mid 0.
]

([GitHub][1])

So the proof's stated induction measure is inconsistent with one of its central examples.

I think this is repairable. Use

[
\mu(P)=\min{|Q|:Q=_E P}
]

and choose a **minimal representative** before decomposing. Then prove that if

[
C(t_1,\dots,t_n)
]

is minimal in its equivalence class, each (t_i) is itself minimal and has strictly smaller (\mu) for non-nullary (C). Otherwise replacing a nonminimal child would give a smaller representative of (P).

But you have to prove that:

* minimum representatives exist;
* every nontrivial equivalence class admits the needed minimal-root decomposition;
* binding doesn't invalidate the measure.

As written, the theorem contains an unstated presentation restriction.

## 4. The binding case is treated too casually

The paper says that when (C) binds, opening exposes an abstraction, and because the substrate is a lambda theory “the induction goes through on abstractions as it does on closed terms.” ([GitHub][1])

That needs considerably more work.

For a binder, the components aren't ordinary closed terms. You have to specify:

* what exactly the constructor arguments are in the underlying lambda theory;
* whether they are abstractions, contexts, nominal abstractions, de Bruijn functions, etc.;
* the equality used on those abstractions;
* how `open` respects (\alpha)-equivalence/substitution;
* what the context labels mean when descending underneath binding.

“Lambda theory handles binding” may ultimately be the right answer, but it is not presently a proof.

Since the theorem's generality over GSLTs with binding is one of the things differentiating it from a first-order syntax observation, this deserves either a precise lemma or a scope restriction.

## 5. Your notion of bisimulation needs to be separated carefully from the literature's “strong bisimulation”

There is a potential equivocation here.

Your transition labels are

[
(\rho,D)
]

where (D) is the **exact redex position/context**, and bisimulation must match the same labelled step. ([GitHub][1])

That is a very informative transition system.

But the motivating CCS example appeals to the standard expansion law to assert that

[
a.0\mid b.0
\sim
a.b.0+b.a.0.
]

([GitHub][1])

Standard CCS strong bisimulation labels transitions by actions, not by exact syntactic redex contexts.

If your context-labelled relation records the position at which a rule fires, it may already be strictly finer than ordinary strong bisimilarity.

So before using the expansion-law pair as the witness for

[
\sim_{\mathbb G^+}\subsetneq\sim_{\mathbb G},
]

you need to show that the particular GSLT presentation of CCS has

[
\sim_\mathbb G
]

coinciding with the standard strong bisimulation being invoked.

Otherwise the paper begins by contrasting spatial logic with one bisimulation from the literature and resolves the tension using a different, potentially much finer, context-labelled bisimulation.

That would be a serious category error.

At minimum I want a proposition:

> For the standard CCS presentation as an interactive GSLT, context-labelled bisimulation after forgetting/quotienting the administrative positional component coincides with ordinary strong bisimulation.

Or, if it does not, use a witness appropriate to your actual (\sim_\mathbb G).

## 6. The “known intensionality” proposition is imported at vastly greater generality than the cited results support

This is probably the second biggest mathematical overreach after the no-unit problem.

You write:

[
=_{\mathcal L(\mathbb G)} = =_E
]

whenever the target logic has enough Boolean structure, citing Sangiorgi, Hirschkoff–Lozes–Sangiorgi, Caires–Lozes, etc. ([GitHub][1])

But those are results about particular spatial logics and particular calculi. They do not establish:

> for every arbitrary GSLT, the automatically generated structural logic separates exactly the quotient of its term algebra by arbitrary (E).

Caires/Vieira's extensionality result is likewise explicitly developed for a particular distributed process calculus with failures, not as a theorem about arbitrary signatures. ([researchgate.net][2])

You are actually trying to **generalize beyond exactly that calculus-by-calculus literature**, so importing the key logical-separation step from that literature as though it were already generic defeats the purpose.

I think you should prove the required logical result directly for the class of presentations under consideration.

Something like:

> For finite closed terms over a freely generated many-sorted signature modulo an admissible structural congruence (E), the full generated structural language separates distinct (E)-classes.

Then spell out “admissible.”

If that theorem cannot be proved for arbitrary (E), that tells you exactly how to restrict the advertised GSLT class.

Right now the central corollary

[
=_{\mathcal L(\mathbb G)}
=========================

# =_E

\sim_{\mathbb G^+}
]

has one new theorem and one assumed equality whose genericity is not established. ([GitHub][1])

## 7. Image-infinite branching creates an adequacy issue you dismiss too quickly

You correctly observe that bisimulation itself does not require image finiteness. ([GitHub][1])

But **logical characterization of bisimulation** often does.

With only finitary Boolean conjunctions and ordinary diamonds, Hennessy–Milner-style logical equivalence need not characterize bisimulation on arbitrary infinitely branching systems absent additional saturation/compactness hypotheses.

Your extension deliberately creates infinite branching when (E) gives infinitely many decompositions—for example through replication. ([GitHub][1])

So “the reconstruction theorem works without image finiteness” and “the generated logic is adequate for that bisimulation” are separate claims.

The former may be true.

The latter needs either:

* infinitary conjunction;
* characteristic fixed points;
* image-finiteness;
* modal saturation;
* or your independent proof that structural formulae already characterize (=_E).

Again, a direct structural-separation theorem would clean this up.

## 8. Idempotence is not established by the proof given

The intuition is good: newly introduced constructor tags are nullary, so exposing them again should not buy additional discrimination.

But the proof says:

> “by Theorem 4 both sides equal (=_E).”

([GitHub][1])

The theorem originally establishes, for (P,Q\in\Terms(\Sigma)),

[
P\sim_{\mathbb G^+}Q\iff P=_E Q.
]

Applying the theorem to (\mathbb G^+) gives a statement about **(\Sigma^+)-terms under (\mathbb G^{++})**.

It does not automatically give a statement about all (\Sigma^+)-terms under (\mathbb G^+).

That's exactly what idempotence is trying to establish.

So there is a small circularity/domain mismatch in:

[
\sim_{\mathbb G^{++}} = \sim_{\mathbb G^+}.
]

I believe the proposition is probably true under the corrected construction, but prove it with an explicit bisimulation relating the extra administrative atom observations, rather than invoking Reconstruction on both sides.

Also be precise about the domain: equality of relations on (\Terms(\Sigma)), on (\Terms(\Sigma^+)), or after embedding into (\Terms(\Sigma^{++}))?

Those are different statements.

## 9. The partial-extension “dial” has a good monotonicity theorem, but the adequacy claim at every point is not established

The monotonicity intuition is fine, especially because newly added administrative rules have fresh labels:

[
\mathcal C\subseteq\mathcal C'
\Rightarrow
\sim_{\mathbb G^{+\mathcal C'}}
\subseteq
\sim_{\mathbb G^{+\mathcal C}}.
]

([GitHub][1])

But then you assert that the logic retaining structural connectives only for (\mathcal C) is adequate for exactly the corresponding partial bisimulation. ([GitHub][1])

That doesn't automatically follow from the endpoint theorem.

Partial observational equivalences can have interactions between opened and unopened constructors. A formula can recursively inspect opened constructors underneath contexts involving unopened ones; equations can mix constructors; binding can move things across apparent structural boundaries.

You need a **partial reconstruction theorem**, not merely “Proposition 2 relativized to (\mathcal C).”

This might be the most interesting theorem in the note, incidentally. The endpoint

[
\sim_\mathbb G\leadsto =_E
]

is intuitive. The exact characterization of every intermediate observational signature is potentially much richer.

I would not give it away as an immediate corollary.

## 10. “Canonical” is currently too strong

The paper repeatedly says the extension is mechanical and “chooses nothing.” ([GitHub][1])

But it presently chooses or assumes:

* a list encoding;
* an injection of heterogeneous constructor arguments into that encoding;
* overloading behavior for (\mathsf K);
* a cut unit when available;
* a different `Op` wrapper when it isn't;
* treatment of binders;
* the visibility of administrative labels.

And your own Open Problems section asks whether canonicity can actually be upgraded to an adjunction. ([GitHub][1])

I would therefore call it **uniform** or **presentation-directed** now, and reserve *canonical/free* for when you have a universal property.

If you replace the list hack with freely adjoined typed argument constructors, you get much closer to genuine canonicity.

## 11. The paper's most useful philosophical conclusion overreaches the theorem

This matters directly for *The Mortal Scientist*.

You conclude:

> “Structural observation is not a second channel. … Structural inspection *is* interaction.”

([GitHub][1])

The construction proves something subtler:

> structural observation **can be represented as interaction after conservatively extending the observer's transition theory with administrative destructuring actions**.

That is not the same statement.

You've added transitions whose whole purpose is to expose structure. In the base theory, those transitions did not exist. So saying “nothing is being added to the observational apparatus” is rhetorically slippery: the *form* of observation remains transition observation, but the set of observable transitions has absolutely been enlarged. ([GitHub][1])

This is analogous to saying X-ray inspection isn't a second perceptual channel because I can model the X-ray machine as ordinary interaction. True at one level of abstraction, but the machine still supplies additional observational capability.

For *The Mortal Scientist*, this distinction is crucial.

My previous criticism was:

> if the scientist can structurally inspect things, then behavioural bisimulation of the original world isn't its epistemic ceiling.

This note gives you a very good response:

> correct—the relevant observational relation is bisimulation in an **observer-extended theory** whose administrative interactions encode exactly the structural inspections available to the scientist.

That resolves the contradiction cleanly.

You do **not** need the stronger metaphysical claim that structural observation was interaction all along.

In fact I think naming this explicitly as an **observer extension** would improve both papers enormously.

## 12. This suggests a better formulation of the central result

I think your actual result wants to be parameterized by observational capability.

Instead of:

[
\mathbb G\mapsto\mathbb G^+
]

with “everything opened,” define an observation signature (O), containing destructors the observer is allowed to exercise:

[
\mathbb G\mapsto\mathbb G^O.
]

Then define

[
P\approx_O Q
\quad\Longleftrightarrow\quad
P\sim_{\mathbb G^O}Q.
]

Now you get a family

[
O\subseteq O'
\Rightarrow
\approx_{O'}\subseteq\approx_O.
]

At one endpoint:

[
O=\varnothing
\Rightarrow
\approx_O=\sim_\mathbb G.
]

At the fully structural endpoint, under the reconstruction hypotheses:

[
O=O_{\rm all}
\Rightarrow
\approx_O = =_E.
]

That is the clean mathematical resolution of the alleged contradiction:

> **there is no single observational bisimulation against which “the” spatial logic must be measured; observational equivalence is indexed by the destructuring capabilities of the observer.**

This is stronger and more precise than “bisimilarity is indexed by the theory.”

And it meshes beautifully with your capability section.

## 13. In fact, the capability-safe version may be the real theorem

The security section initially looks like a caveat. I increasingly think it contains the deeper result.

You correctly observe that unrestricted `open` destroys encapsulation. ([GitHub][1])

If instead observers possess particular opening capabilities (A), then the equivalence becomes

[
\sim_{\mathbb G,A}.
]

Two processes can be equivalent for an observer lacking a destructor and distinguishable to one possessing it.

That gives you an **epistemically indexed bisimulation** with the observation rights represented *inside the calculus* rather than as a meta-level subset of the signature.

That is much more compelling for *The Mortal Scientist* than global (\mathbb G^+).

It also answers my original objection almost exactly:

> What can the scientist discriminate?

Not “everything structurally expressible” and not “everything base bisimulation discriminates,” but:

[
\boxed{\text{everything distinguishable by interactions available under its current observational capabilities}.}
]

The generated structural logic can then correspond to the maximally endowed ideal observer, while mortal scientists occupy restricted fragments/capability sets.

That strikes me as the conceptually correct endpoint.

## 14. But “the factory is the unique constructor with no destructor” should not be stated as a general fact

This section becomes speculative too quickly.

You say a capability is the output of the **unique** destructor-free constructor. ([GitHub][1])

Why unique?

A system can have:

* several fresh-name generators;
* multiple opaque abstract types;
* sealed constructors;
* hardware roots;
* distinct authorities with independent minting operations.

And in nominal calculi, freshness is often a binder/quantifier rather than the output of an ordinary constructor at all.

The important property is not uniqueness. It is:

> the mechanism generating unforgeable authority must itself not be generically destructible/reconstructible by the observer-extension machinery.

I would remove “unique” completely unless a categorical argument later proves it.

## 15. There is one subtle but important labeling mistake in the Exposure discussion

You say the top-level transitions occur “one transition per decomposition, with the atom in the label recording which constructor.” ([GitHub][1])

But your transition label is defined as

[
(\rho,D),
]

not the target term. ([GitHub][1])

The atom (C_\bullet) is in the **target**. The constructor identity is indirectly present in the label because the rule name is (\operatorname{open}_C).

Later the proof says:

> “The atom (C_\bullet) appearing in the label…”

([GitHub][1])

Strictly, it doesn't.

This doesn't kill the proof because (\operatorname{open}_C) itself distinguishes (C). But clean this up. A hostile formal-methods referee will notice it immediately.

## 16. “Every decomposition becomes a transition” may introduce multiplicity questions

Your transition relation appears extensional:

[
P\xrightarrow{(\rho,D)}P'
]

if a decomposition exists. If two different (E)-matches yield the same label and same target modulo (E), are they one transition or two derivations?

Yet the text says “one transition per decomposition.” ([GitHub][1])

That distinction did real work in the weighted-GSLT paper.

For ordinary bisimulation, duplicate parallel edges typically don't matter. But if this construction is later composed with weights/costs, derivational multiplicity absolutely matters.

So distinguish:

* the **transition relation**;
* the **set of derivations witnessing a transition**.

This note can say reconstruction only needs existence, while quantitative lifts must preserve multiplicity explicitly.

That will save you trouble when these papers recombine.

## 17. The cost interpretation is suggestive, but depth is not generally step count

You wisely call the cost section a direction rather than a theorem. Keep it that way.

A structural formula of syntactic depth (d) does not necessarily require exactly (d) administrative steps:

* conjunction may require two branches;
* negation can require exhaustive failure/search depending on checking semantics;
* separating conjunction may enumerate many (E)-decompositions;
* sharing/caching alters cost;
* a failed observation can explore more than a successful one.

You acknowledge the decomposition-search issue. ([GitHub][1])

I'd therefore say:

> nesting depth lower-bounds/maximally bounds the sequential destructuring depth, while actual checking cost depends additionally on branching and Boolean evaluation strategy.

That is more useful to the scientist paper than pretending depth itself is the derived price.

## 18. I would be cautious about the novelty statement until you search harder for “reification/destructor” constructions

Your literature discussion is already much stronger than average, and Caires–Vieira indeed explicitly establish that spatial observations can characterize extensional equivalences in appropriate distributed models. ([researchgate.net][2])

Your claimed novelty is narrower:

> a uniform syntactic transformation of presentations making constructor structure transition-observable.

([GitHub][1])

That may well be novel in this formulation.

But the underlying construction—reifying constructors as tags and providing observation/destructor actions—is so elementary and close to standard definability/tester constructions that I would not yet say “we have not seen written down” without a more targeted literature pass over:

* definability of constructors by experiments;
* applicative/contextual bisimulation with destructors;
* labelled transition systems derived from destructor contexts;
* environmental bisimulation;
* pattern-matching observations;
* abstract data-type observational equivalence;
* reactive systems / Leifer–Milner contexts;
* bigraphical contexts;
* coalgebraic “predicate lifting” versus observation extension.

This isn't because I think the note lacks value if precedent exists. Quite the opposite: the value may be precisely that it packages a dispersed idea into a generic GSLT construction.

Just make the priority claim maximally conservative until that search is exhausted.

# What I think the note gets exactly right

The fundamental conceptual move is good:

> The statement “spatial logic distinguishes bisimilar processes” is incomplete because it suppresses **which observational transition system supplies the bisimulation**.

That is the insight the scientist paper needed. Your abstract articulates it very clearly. ([GitHub][1])

And the Caires/Vieira lineage supports the broad philosophical point: structural observations do not have to be inherently “intensional”; an extensional observational model can expose them. ([researchgate.net][2])

Your contribution can therefore be framed very cleanly as:

[
\boxed{
\text{spatial discriminability can be operationalized by a presentation-directed observer extension}
}
]

rather than as a contradiction between spatial and behavioural reasoning.

That survives my review.

# What I would make the main theorem say

Right now I think you're trying to prove too much in one theorem.

I would aim for something like:

> **Reconstruction theorem.** Let (\mathbb G=(\Sigma,E,R)) be an interactive GSLT satisfying conditions A–D: the observation extension is well-sorted; constructor arguments have a freely adjoined injective representation; administrative opening is directly observable; and (E) admits a well-founded constructor decomposition modulo congruence. Then for closed base terms,
> [
> P\sim_{\operatorname{Obs}(\mathbb G)}Q
> \iff
> P=_E Q.
> ]

Then a separate theorem:

> **Logical correspondence.** If the generated structural language separates (E)-classes, then
> [
> =_{\mathcal L(\mathbb G)}
> =========================
>
> \sim_{\operatorname{Obs}(\mathbb G)}.
> ]

And then:

> **Observer monotonicity.** If (O\subseteq O'), then
> [
> \sim_{\mathbb G^{O'}}\subseteq\sim_{\mathbb G^O}.
> ]

That decomposition makes the dependencies transparent:

[
\text{operational reconstruction}
\neq
\text{logical expressiveness}
\neq
\text{observer capability}.
]

At present those three are braided together enough that a weakness in one looks like a weakness in all three.

# Implication for *The Mortal Scientist*

The note **does resolve the apparent contradiction I identified**, provided the scientist paper is revised to say the right thing.

I would *not* repair it by writing:

> “structural inspection is actually just ordinary interaction, therefore base bisimulation remains the epistemic ceiling.”

I would repair it as:

> “The scientist's observational equivalence is indexed by its admissible experiments. Structural connectives correspond to administrative destructuring experiments in an observer extension of the base GSLT. For the ideal observer with the full structural observation signature, the induced bisimulation reaches structural equivalence; restricted or mortal observers occupy coarser points of the same family.”

That is substantially better than the original claim.

It also links naturally to your resource story:

[
\text{ideal full observer}
\quad\supseteq\quad
\text{capability-limited observer}
\quad\supseteq\quad
\text{budget-limited actually performed observations}.
]

Those are three different restrictions, and keeping them distinct will fix several issues in *The Mortal Scientist*.

## Verdict

I would absolutely continue this standalone note.

But my current verdict is:

**Strong central idea, central theorem not yet proved at advertised generality.**

The highest-priority fixes are, in order:

1. fix the no-unit-cut failure;
2. eliminate the Turing-completeness/list claim in favor of a genuinely typed free argument representation;
3. state the needed conditions on (E) and repair the induction;
4. prove rather than generically import structural-logic separation of (E);
5. verify that your context-labelled (\sim_\mathbb G) really has the relation to ordinary process-calculus bisimilarity that the motivating examples assume;
6. recast the result as an **observer/capability-indexed family of bisimulations**.

If those work out, I think this note could become more than a patch for the scientist paper. The observer-indexed formulation is a useful standalone conceptual result: it says that the familiar intensional/extensional distinction is partly a statement about **which destructuring capabilities have been admitted to the observer**, not an immutable property of a structural connective itself. That is a much sharper formulation of the folklore.

[1]: https://github.com/F1R3FLY-io/publications/blob/main/spatial-formulae-discrimination-power/extension-note.tex "publications/spatial-formulae-discrimination-power/extension-note.tex at main · F1R3FLY-io/publications · GitHub"
[2]: https://www.researchgate.net/publication/222297224_Extensionality_of_Spatial_Observations_in_Distributed_Systems?utm_source=chatgpt.com "(PDF) Extensionality of Spatial Observations in Distributed Systems"
