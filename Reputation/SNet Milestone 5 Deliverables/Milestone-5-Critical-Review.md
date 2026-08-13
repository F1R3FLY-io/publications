# Critical Review — SNet Milestone 5: Type-Indexed Temporal Reputation System

**Reviewer perspective:** author of *Identity-Indexed Typing Judgments and the Adjudication of Capability* (May 2026), one of the package's cited inputs.
**Material reviewed:** scope proposal plus eight deliverables dated 26 July / updated 5 August 2026 (~29,000 words).

---

## 1. Summary judgment

This is a competent, unusually disciplined design package. It is well above the median for vendor deliverables of this kind: it is internally consistent, it distinguishes what is decided from what is not, and its threat model is sharper than most production reputation systems ever get. The authors clearly read the source material and did not simply reskin a score aggregator.

The problem is not sloppiness. The problem is that **the package adopts the vocabulary and the discipline of the typed-judgment framework while discarding its mathematical content**, and then reintroduces, as open governance questions, exactly the problems the mathematical content was there to solve.

Concretely: the framework's four load-bearing structures — spatial-behavioral types, the dependency context Γ, evidence levels as modalities over a labelled transition system, and the identification of witnessing radius with name distribution — are all absent. What survives is a set of English-language principles ("signatures prove attribution, not truth"; "direct beats witnessed beats hearsay"; "provenance must be traversable in reverse") and a records schema. Those principles are correct, and stating them clearly has real value. But they are now *asserted* rather than *derived*, which means they must be *enforced* rather than *guaranteed* — and the enforcement mechanism, throughout, is a governance manifest plus a benchmark suite.

The second-order consequence is visible in the package's own artifacts. Nineteen findings are marked "Adopted," yet thirty questions remain open, and the open set includes: what a reputation type actually is (Q1), what the evidence schema is (Q3), how coverage is measured (Q7), how conflicting evidence is adjudicated (Q10), and which projection is used (Q15). That is the entire semantic content of the system. The package is a well-built container awaiting its contents.

I would recommend accepting the design *direction* and rejecting the claim that the design is ready for a contract freeze in its current form — not because of what it says, but because of what it has deferred.

---

## 2. What the package gets right

Stated first, because the critique below should not be read as dismissal.

**Genuinely good work:**

- **The evidence/score separation is correctly stated and correctly enforced.** AD-4 and PRD-F16 require evidence queries and projection queries to use *distinct response contracts*, so a structured reputation object cannot be mistaken for a policy output. That is the single most important architectural decision in the package and they made it correctly and defended it in three separate documents.
- **The multiplication rule** (Evidence and Claims Model §5): three trusters relaying one monitoring service's report yields three articles and one source event. Faithful to the framework and crisply stated.
- **Rejection of Kolonin's global rater credibility** (E3) in favour of per-type, per-scope issuer credibility. This is a deliberate, correctly-argued deviation from a published algorithm, and they flagged it as such rather than quietly hoping nobody noticed.
- **Rejection of universal soulbound cross-network reputation** (P8) in favour of portable *evidence* re-evaluated under local policy. Right call, and consistent with locality.
- **Coverage as a mandatory response element**, with "unknown is not good" stated as an invariant rather than a nice-to-have.
- **The "manufactured credibility" threat model.** The observation that the adversary is rule-following, and that the danger is a perfectly-proved body of false, collusive, or selectively-complete evidence, is exactly the right frame. Most reputation-system threat models are about key compromise. This one isn't.
- **Intellectual honesty about maturity.** "Plausible for a proof of concept and unvalidated for production" (Architecture §14) is the correct claim and they refused to inflate it. Likewise A9, and the explicit note that the Kevin input is an AI-generated summary of an unavailable original.
- **P7's terminological separation** of Proof-of-Reputation from capability proof. Small, but it shows someone was reading carefully.

---

## 3. The central critique: what was taken, and what was dropped

The synthesis (§2.1) characterises the position paper as contributing "evidence-object semantics … direct/witnessed/hearsay evidence levels, full provenance, and the principle that 'provable' means derivable from authenticated claims, not true." That is an accurate description of what they took. It is also an accurate description of what they left.

### 3.1 "Type" has been reduced to an enum

Throughout the package, a reputation type is a string: `block-production reliability`, `equivocation/slashing`, `protocol-version compliance`. Q1's starting point is "narrow the PRD candidate set" — i.e., pick which strings.

In the framework, `T` in `Γ ⊢ Sig(A) : T` is a spatial-behavioral formula with a satisfaction relation over an LTS. That is not decoration; it is what makes four things work that do not work here:

**(a) Type entailment is unavailable, so cross-context reuse must be banned outright.** PRD-F15, AD-3, and risk R6 all prohibit *any* automatic carry-over between types or scopes, and R6 is one of only two risks that block PRD sign-off. This is the right conservative call *given their model* — but it is a blunt instrument standing in for a subtyping order. With real types you get a derived answer to "when does standing in T₁ license a claim to T₂": exactly when `T₁ ≤ T₂`. The package instead defers this to "an explicit, bounded, versioned rule" (unwritten, unassigned) and meanwhile forbids everything. In a system whose whole purpose is composing judgments, a blanket carry-over prohibition is a significant functional loss, not just a conservative default.

**(b) Refutation has no semantics.** In §4.1 of the Evidence and Claims Model, an assertion carries a `relation` field with values `supports | refutes | neutral`. Nothing in the design checks that a refutation actually contradicts its target. Bob submits an event tagged `refutes` pointing at Carol's upgrade claim, and the system records a dispute — regardless of whether Bob's node-version trace bears on Carol's claim at all.

In the framework, a refutation of `Sig(A) : T` is a trace σ observed in the communal LTS with `σ ⊭ T`. That is *mechanically checkable*. The package has no such check, and it shows up as a named risk: **R8, negative-evidence weaponization** ("cheap accusations impose censorship, delay, or extortion"), mitigated by "rate limits, response rights, deadlines, abuse consequences." That is a governance patch over a missing well-formedness condition. Rate-limiting accusations is what you do when you cannot tell a relevant accusation from an irrelevant one.

**(c) Coverage has no principled denominator.** §6 of the Evidence and Claims Model requires coverage to state "the observation boundary … enumerable duties where they exist, or otherwise the sources and chain, time, or epoch ranges expected to be observed," and concedes that "a statistical performance expectation is not a completeness denominator." This is honest, and it is also an admission that they cannot compute the denominator in general. For slashing it works — the boundary is a block range. For *validation correctness* it does not: the set of validations that should have been checked is unbounded absent a duty schedule.

A formula gives you the denominator directly: checking `⟨a⟩⟨b⟩T` requires traces of modal depth 2 over the relevant channels, and coverage is the proportion of that observation set actually obtained. Q7 is marked Open with a starting point that restates the problem.

**(d) There is no type algebra, so there is no compositional aggregation.** P5 and Q17 correctly worry that a weighted mean over a shard "hides a failing minority," and ask how local projections combine "without a large shard or incumbent dominating." That question has a structural answer when scope is a namespace and types compose; it has only a statistical answer (quantiles, concentration measures) when they don't. The package takes the statistical route and marks it Open.

### 3.2 The dependency context Γ is absent entirely

This is the most consequential omission, and it is not mentioned anywhere in the package.

The framework's distinctive move is that Γ records **inter-agent capability dependencies**, not variable bindings: if Alice's database service depends on Bob's storage and Carol's authentication, then

```
Sig(Bob) : T_store,  Sig(Carol) : T_auth  ⊢  Sig(Alice) : T_db
```

is provable only by discharging those dependencies against Bob's and Carol's own standing judgments. Proof composition *is* dependency discharge.

The package's "provenance" is a different thing. It is a derivation graph over *evidence records*: `source event → article → structured reputation object → projection result` (ECM §6). Reverse lookup answers "if this record is invalidated, which records derived from it?" That is record lineage. It is not capability dependency.

The gap is concrete and testable. Suppose Bob's storage capability is refuted. In the framework, Alice's `T_db` judgment loses a premise and is no longer derivable — automatically, structurally. In the package, nothing happens: Alice's judgment never recorded a dependency on Bob's, because there is no field for it. The invalidation cascade traverses evidence provenance, and Alice's evidence does not descend from Bob's evidence.

This matters because the Introduction (§3) advertises precisely this: *"invalidating one data point can cascade to everything derived from it."* For evidence, true. For **capability**, false — and capability is what the system claims to adjudicate. For a validator-only V1 this may be tolerable, since validators' duties are largely independent. The moment agents, services, or composed products enter — which A12 and PRD §1 explicitly plan for — the omission becomes structural, and it is not the kind of thing that gets retrofitted into a frozen schema.

**Recommendation:** the evidence envelope needs a dependency field distinct from provenance, and the reverse-dependency index needs to traverse it. This is a schema-level change and therefore must land before the PoC contract freeze, not after.

### 3.3 Evidence levels demoted from modalities to labels

The framework indexes Hennessy–Milner modalities by evidence provenance — `⟨a⟩^D`, `⟨a⟩^W`, `⟨a⟩^H` — with refutation strength defined as the meet over a modal-strength order. Direct/witnessed/hearsay is a *lattice*, and that lattice yields monotonicity properties as theorems.

The package keeps the three names and drops the lattice. E5's disposition: "the numeric weight each level gets in a score is a projection-policy decision (Q10)." So the strength ordering becomes numbers chosen later by whoever writes the policy.

The cost appears in RG2, the metamorphic and adversarial gate, which must demonstrate that "identity splitting, relayed cycles, unrelated scope, wash rings, missing coverage, and policy replay do not create unearned influence." Several of those are lattice properties — closure under relay should be *provably* non-amplifying, not empirically tested against fixtures. The package plans to discover by simulation what the framework would give as a proof obligation, and simulation only covers the attacks you thought of.

There is also a definitional call buried here that deserves scrutiny. E5 and the running example classify Alice's cross-chain import as **witnessed** on the grounds that she "verified the finalized record herself," reserving **hearsay** for the case where she relays a monitoring service's report. But in the framework, *witnessed* means the issuer observed the communication event — which presupposes holding the relevant channel name. Alice does not observe chain-x's consensus; she reads its published output. That is relaying a claim with a cryptographic receipt attached, which looks structurally like hearsay-with-good-provenance rather than witnessing.

The classification is load-bearing: UJ-1 (external-chain import) is the flagship journey, hearsay "must not multiply the source's influence," and the whole V1 evidence supply is external imports. If cross-chain imports are hearsay, V1's evidence base is entirely hearsay-grade — which may well be the honest answer, and would change how much weight projections can carry. **This is worth pushing on explicitly**, because it is the one definitional decision that most affects what V1 can claim.

### 3.4 Witnessing radius / capability alignment is missing — and it would have answered R3

The framework's proposition that **witnessing radius = channel-name distribution** gives a structural account of who is entitled to attest to what: you can witness what you hold names for.

The package has no such account. Instead:

- **Q6** — "Who are the V1 trusters, and how are their capabilities, rotation, and revocation governed?" → starting point: "an explicit SNet-managed permissioned manifest."
- **Q2** — "who holds submit authority for each event class?" → Open.
- **R3, truster or validator cartel capture** — Critical, and one of only two risks blocking PRD sign-off. Mitigations: "per-action capabilities, explicit manifests, role separation, threshold rules, receipts, inclusion-delay monitoring, rotation/revocation, independently governed dispute path."

Every one of those mitigations is administrative. The framework's answer is that attestation authority is not granted by a manifest; it is *constituted* by name possession, and name possession is a fact about the computation rather than a row in a spreadsheet. That does not eliminate cartel risk — a cartel can still lie about what it saw — but it removes the failure mode where the manifest is the attack surface, which is exactly what R3 describes.

The package's own reasoning admits the shape of the problem: the threat model states that "permissioning makes submitters accountable, not honest." Correct. And then the mitigation for the resulting risk is more permissioning.

### 3.5 Agent-capability vs channel-capability (chip vs pin) is absent

The package's subject model is flat: stable identity × type × scope. Q1's candidate type list is a list of pins — block production, validation correctness, protocol compliance, shard responsiveness, bridge reliability, evidence publication.

The framework's correction is that an agent-capability is a **correlation structure across channels**, not a per-channel property, and that behavioral types describe cross-channel correlations rather than counts of communication events. The interesting validator pathologies live precisely there: timely except when also proposing; bridge reliability degrading exactly when stake concentration rises; correctness holding on sampled blocks and failing on the ones nobody checks. None of these is visible to a per-type scalar, however carefully scoped.

The package worries about this at the *shard* level (P5: a mean hides a failing minority) and not at all at the *subject* level. Equivocation is a correlation phenomenon; a design that models it as a per-type negative event will detect it only when someone else already caught it — which is, in fact, exactly what UJ-1 depicts.

### 3.6 Scope is a string where it should be a namespace

`chain-x mainnet`, `shard X` — scope is an opaque label. Consequently PRD-F15's no-carry-over rule is a policy check on string equality, sharding topology and scope are two unrelated notions, and Q17 (how local projections combine) has no structural handle.

Where scope is a namespace, containment and disjointness are computational facts, "no carry-over across scopes" is a theorem about name disjointness rather than a validation rule, and shard topology and scope structure are the same object. This connects directly to the unguessability→unforgeability account: the registry converts structural unguessability into operational unforgeability, which is what makes scope boundaries enforceable rather than merely declared.

### 3.7 The rho substrate is treated as infrastructure, not as semantics

Architecture §1.2 reviews f1r3node-rust for signed deploys, CBC Casper, LMDB-backed rspace++, PathMaps, and HTTP/gRPC surfaces — i.e., as a generic ledger that happens to be the one on hand. §4.1 cites the paper's Rholang sketch (p. 9) for exactly one purpose: to support the argument that evidence submission should use separate channels.

That is the least interesting thing in the sketch. The reason to build this on rho and MeTTaIL is that under OSLF the language definition, the model checker, the proof system, and the attestation infrastructure are uniformly rho processes over uniformly defined terms — so the type checker and the "projection engine" are not two systems requiring separate correctness arguments.

The package instead specifies a bespoke deterministic projection engine (Architecture §4.1), a bespoke replay-equivalence audit harness (§7.2), and a bespoke evidence-query contract, and then owes RG1 and RG4 evidence that all three agree. §7.2 is candid about why: "Consensus establishes that validators execute the same state transition consistently. It cannot detect a dependency-traversal optimization that every validator executes incorrectly." True — and this is a correctness burden that a generated model checker would not incur, because there would not be two implementations to reconcile.

The synthesis also notes that "the later Meredith catamorphism draft referenced in meeting notes" was unavailable, while the terminology table (§6) lists "fold, catamorphism" under *projection policy*. They have adopted the word without the construction. If the projection-as-catamorphism story is what makes projections compositional and their equivalence provable, its absence is why §7.2's harness exists.

---

## 4. Problems independent of the framework

These stand on their own merits and are worth raising regardless of the above.

**4.1 The centre is empty.** Nineteen "Adopted" findings, thirty open questions — and the open set includes what a type is, what the schema is, how coverage is measured, how conflicts adjudicate, and which projection is used. The package's own closure rule permits conditional acceptance while all of these remain open, because only R3 and R6 block sign-off. It is possible to accept this design and still not know what the system computes.

**4.2 Every owner is a placeholder.** The prototype acceptance table assigns each metric to "SNet protocol/performance (TBD)," "SNet product (TBD)," "SNet security/protocol (TBD)." The Open Question Register states that "role placeholders are intentionally used until named delegates are assigned." So the accountability structure is itself TBD, and the closure rule ("a row that is blank, merely discussed, or has no owner is not closed") is currently unsatisfiable for every row. Ask for names before accepting the register as a decision instrument.

**4.3 The control apparatus may outgrow the thing it controls.** Decision Gates 1–3, validation gates RG1–RG6, six named contract freezes, eight decision points, six disposition categories, five status values, plus Q26.a–Q26.e. This is a lot of process for a system with one query implemented. There is a real risk that maintaining the registers becomes the deliverable. The roadmap's own sequencing principle — "do not promote work because a design document is complete" — should be applied to the design documents.

**4.4 PathMaps as a graph store is the schedule risk, and it is unquantified.** Architecture §6 concedes that "the current Rholang integration reconstructs or scans data for some operations and is not yet proven as the physical index for graph-shaped access," and defers everything to RG4 benchmarks whose thresholds are themselves TBD (Q26.e). The plausible failure mode is that PathMap-backed reverse-dependency traversal at realistic fanout is too slow, at which point AD-2 (no authoritative sidecar) fails. The roadmap's contingency is one sentence: "An RG4 failure revises the physical layout or the in-node hypothesis." There is no costed fallback, no alternative layout sketched, and no decision rule for when to abandon. Given that the entire milestone exists to test this hypothesis, that is thin.

**4.5 The cascade claim is stronger in prose than in design.** The Introduction says invalidation "can cascade to everything derived from it." The Evidence and Claims Model §7 offers three correction policies of which the **default (prospective) does not cascade at all** — published history stands, future projections change. Bounded lookback and retrospective are optional per type/scope, the horizon is Q9/Q14 (Open), and traversal cost is Q26 (Open). The headline feature is policy-optional and unbenchmarked. Fix the prose or fix the default.

**4.6 "Disputed" will be the modal answer.** Query answer states are yes / no / disputed / insufficient evidence, and all adjudication is pushed into projection policy (Q10). Since making a claim is cheap and tagging an event `refutes` is cheap (see 3.1(b)), the equilibrium is that every interesting query returns "disputed" with two inspectable piles. The evidence layer — the layer the package calls canonical — is then deliberately uninformative on exactly the questions people ask it. The framework's graded refutation strength exists to avoid this; the package's binary flag reintroduces it.

**4.7 One source is an AI-generated summary of an unavailable original.** The Kevin update (approx. 5 June 2026) is described exactly that way, and it nonetheless drives A4 and Q5 — the definition of what "stored in F1R3FLY" means, which governs the entire state-authority model. The package flags the weakness honestly. It should not have been load-bearing anyway; get the original.

---

## 5. On attribution

Worth noting the framing, whatever you decide to do about it.

The package presents itself as a synthesis of equals: Meredith supplies evidence semantics, Kolonin supplies scoring, Goertzel bridges them, tokenomics supplies a consumer. In practice, the architecture that emerged is the typed-judgment framework's: evidence-first canonicity, evidence levels, provable ≠ true, simultaneous proof and refutation, append-only with correction-by-new-event, per-type/per-scope credibility, no implicit cross-context transfer. Kolonin's actual contribution — temporal weighted liquid rank — is demoted to "a candidate projection, not the default" (P3, Provisional), and his global rater credibility is explicitly rejected (E3). Goertzel's paper is described as "schematic."

Meanwhile the position paper is characterised as "a theoretical framework with a Rholang implementation sketch; nothing is built" — factually true, and the only source given a "nothing is built" tag despite Kolonin's being prototyped only on historical data and the tokenomics simulations being declared "illustrative rather than implementable."

Not an accusation; the citations are present and correct. But the design is substantially one framework with a second framework's algorithm listed as an option, described as a balanced synthesis.

---

## 6. What to ask for

**Before conditional acceptance:**

1. **Name the owners.** Every "TBD" role placeholder. The register is not a decision instrument until it has people in it.
2. **Decide Q1 and Q3 in substance, not in principle.** What is a type, concretely, for the first two evidence classes — and is it a label or a checkable predicate? Everything downstream depends on the answer and it is currently deferred past the point where it can be changed cheaply.
3. **Resolve the witnessed/hearsay classification of cross-chain imports** (§3.3 above). This determines what V1's evidence base is actually worth.
4. **Add capability dependency to the envelope** as a field distinct from record provenance (§3.2). Schema change; must precede the PoC contract freeze.
5. **Get a costed fallback for RG4 failure** on PathMap graph access, with a decision rule and a named alternative layout.
6. **Fix the cascade language** in the Introduction to match the prospective default, or change the default.

**Worth pressing on, less urgently:**

7. Whether refutation should carry a well-formedness condition rather than a relation tag — this converts R8 from a governance problem into a validation rule.
8. Whether the RG2 metamorphic properties (relay non-amplification, partition invariance) should be discharged as proof obligations rather than fixtures.
9. Whether scope should be a namespace rather than a string, before Q26 fixes the physical layout — because if it should, the layout follows from it rather than being benchmarked into existence.
10. Supply the catamorphism draft. If projections are folds with a compositional equivalence theory, the §7.2 audit harness has a much smaller job.

---

## 7. Bottom line

Accept the direction; do not accept it as a design ready for freeze. The package is honest about being a hypothesis, and it should be held to that: its own strongest defensible claim is "plausible for a proof of concept and unvalidated for production," and its central architectural bet (in-node graph access on PathMaps) is untested with unspecified thresholds.

The deeper issue is that the framework was mined for principles and left behind as mathematics. The result is a design that is *correct in what it says* and *unenforced in how it says it* — every structural guarantee reappears downstream as a governance control, a benchmark, or an open question. That trade may be defensible for a validator-only V1 shipped quickly. It is not defensible for the agent-and-artifact system A12 and PRD §1 promise to grow into, because the missing structures (dependency contexts, type entailment, namespace scope) are schema-level and do not retrofit.
