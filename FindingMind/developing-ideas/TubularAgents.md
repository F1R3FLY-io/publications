# The Tube

**Topological consequences of heterotrophy in the mortal scientist framework**

*Research note, dictated 7 September 2026. To be picked up later. Sibling to the rho-life series: scientist-note (§10 Trophic structure), compose-note, engine-note.*

---

## 1. The observation, as dictated

The reason for eating is to gain access to the token supply held inside the agent being consumed.

There is a distinct evolutionary advantage to minimising the time spent *exclusively* on energy extraction. If some portion of the extraction can be carried on internally to the heterotroph — so that the heterotroph can meanwhile mate, seek further food, form hypotheses about its environment, make predictions — then extraction runs concurrently with everything else the agent does, and that concurrency is worth paying for.

Hence eating has two phases:

1. the gross phase — getting the relevant material across the boundary and inside;
2. the internal phase — the remaining processes working on that material while the agent is engaged elsewhere.

The residue of the internal phase is the part that was not energy: refuse. It must leave.

Material in, refuse out: this makes the heterotroph, topologically, a **tube**.

The autotroph has no such need. Nothing about harvesting an external source calls for a tube structure.

And this is exactly the arrangement found in the physical world. Animals are tubes. Plants are not.

**Ask:** summarise the idea and investigate it within the mathematical apparatus of the mortal scientist.

---

## 2. Where this lands in the existing apparatus

The pieces this note needs are already built. Nothing new has to be assumed; the claim is that the tube is *derivable* from what is on the table.

- **Access profiles (scientist-note §10, Table 1).** A source is a persistent emitter: rate-limited, non-exclusive, non-adaptive. Prey is a linear send: exhaustible, exclusive, adaptive. Same sort, same rewrite rule, different multiplicity.
- **The asymmetry that generates the refuse.** A source emits *tokens*. Prey is a *computation* — a structured term that happens to hold a stack. Harvesting a source yields token and nothing else; breaking prey open yields token entangled with code. Refuse is not an extra postulate: it is the non-token part of a structured prey term, and it exists precisely because prey are computations and sources are not.
- **Media and interposed channels (engine-note §5).** `for(y <- x)P | x!(Q)` generalises to `for(y <- x1)P | C(x1,x2) | x2!(Q)`. The ladder of media by behaviour is relay / lossy / corrupting / reading / **converting**, with the engine at the top rung. `Chain` already builds a *d*-hop pathway.
- **Porosity and individuation (engine-note §5.6).** An individual is `Chn_in` closed, `Chn_boundary` open. Boundary channels require redex pathways to internal channels or they are stranding sites. Integration depth *d* costs reliability `p^d`; ownership is what buys depth.
- **The tower.** The outside of one level is the inside of the next.
- **Namespace separation as the guarantor of non-interference** (game-note draft 2): threat offers had to live on their own names, because a line signalling on a square's move channel would rendezvous with that square's receipt and play the move. Perception must not be able to act.

### The central identification

**The gut is an internalised converting medium — an engine — held inside the individual's boundary namespace.**

The mycorrhizal network was the external instance of the engine profile. The gut is the same profile drawn on the other side of the ownership line: a chain of interposed channels, each a converting medium, running from an intake channel to an expulsion channel, with the whole chain owned.

And then the observation that makes the topology non-metaphorical: *the lumen of the tube is not inside the individual*. It is boundary namespace — owned but open. The tube is the construction that surrounds a region of the outside with the inside. That is exactly the tower's generating step, applied to acquire boundary surface without acquiring exposure.

---

## 3. Statements to prove

Numbering provisional; all of these are candidates rather than results.

**Prop A (separation is not itself yielding).** In the harvest of a structured prey term, the rewrite steps that separate token from residue are metered but token-neutral. They cost and do not pay.

**Prop B (the serialisation penalty).** If separation is sequential with the scientist's control loop, the scientist pays its burn rate over the whole separation with no inquiry, no foraging and no mating during it. Write the foraging inequality of scientist-note §7 with a sequential fraction σ and derive the Amdahl-shaped bound on harvest rate. The advantage of concurrency should fall out as a strict inequality with an explicit threshold, not as a preference.

**Prop C (concurrency forces internalisation).** Concurrency here is parallel composition — trivially available in rho. The content is therefore not "extraction can be concurrent" but *where the extraction channels live*. If the separation runs on boundary channels, other agents can rendezvous with the partially-processed material; it is exclusive only if the channels are internal. So: exclusivity of a partially-extracted harvest requires the separation chain to be owned. This is the ownership-buys-depth corollary specialised to digestion.

**Prop D (two ends, not one).** Intake and expulsion must sit on distinct names. On a single channel, an offer of refuse and a receipt for intake rendezvous — the organism eats its own residue, or worse, the refuse blocks the intake. This is the game-note's perception-must-not-act argument one level up. The single-opening organism is therefore not impossible but *rate-limited by a mutual exclusion*: intake and expulsion must alternate on the shared name.

> **Empirical check, and it is favourable.** Cnidarians and flatworms have a gastrovascular cavity with one opening and feed in discrete alternating bouts. Bilaterians with a through-gut feed continuously. The framework should predict the throughput gap quantitatively, from the exclusion alone.

**Prop E (autotrophs are not tubes).** For source access: non-exclusive, so nothing is gained by fast acquisition; unstructured, so there is no residue to expel. Both of the tube's two ends lose their reason. The prediction is not merely that plants need no tube but that they should instead buy *boundary area* directly — which is what a leaf and a root system are.

**Cor F (the second heterotroph's expense).** Scientist-note §10 already carries "intelligence is a heterotroph's expense", derived from the limit-of-inquiry proposition and structural recursion. The tube should be a *second corollary of the same proposition*: exclusivity plus structure forces both purchases. Intelligence and the tube are siblings. If that derivation goes through it is the strongest result available here, because it makes a gross anatomical fact a consequence of an access profile.

**Prop G (transient versus persistent tubes).** A food vacuole is a tube that is built and dissolved per meal. A gut is one maintained. Persistence carries a maintenance cost against the metabolic stack; there should be a threshold harvest rate above which the permanent tube beats the transient one. Amoeba below the threshold, annelid above it.

**Prop H (gut depth tracks incommensurability).** Engine-note's theorem — commerce across an incommensurable boundary is epistemic only — says that tokens in a foreign colour are not nutritive as such; they must be converted. Digestion *is* that conversion, and the number of interposed converting stages should scale with the colour distance between prey namespace and consumer namespace. Carnivores eat close relatives and have short guts. Herbivores eat far ones and have long guts, often renting whole foreign namespaces (symbionts) inside the boundary to do conversions they cannot do themselves. A rented engine inside the tube is a *market inside an individual* that survives Coase because the lumen is not the inside.

---

## 4. What to write down formally

- The **two-phase harvest** as a term: an exclusive acquisition rendezvous on a boundary channel, in parallel with a persistent `Digest` chain on internal channels, terminating in a send on the expulsion channel. The existing `Forage` (scientist-note App A) and `Chain` (engine.rho §7) are most of it.
- The **tube as a namespace shape**: `Chn_boundary` factors as `Chn_in ⊔ Chn_out`, with a directed redex pathway from in to out and no return path. Then: *tube* is a formula in the logic, not a designation. That is the same move as "being a scientist is a depth-≥2 dialogue loop through a both-type channel".
- Whether the **absence of a return path** is forced or merely typical. Coprophagy, caecotrophy and rumination are return paths, and they are exactly the cases where one pass leaves too much unextracted. Cheap to state as a condition on residual yield.
- The **physical topology as a shadow**. A through-gut makes the body a genus-1 surface. That is real but is not the content; the content is the channel topology, and the surface is what an embodiment of that channel topology in three-space looks like. Say so explicitly so that the note does not appear to be arguing from anatomy.

---

## 5. Open questions

1. Is the tube **forced** or **favoured**? Prop "composite resolution" says the overlap region between trophic strategies is populated by composites rather than atoms. Mixotrophs should therefore be predicted, and their tubes should be predicted to be facultative.
2. Does the **surface-area argument** connect to the r\* result of *Compression versus Autonomy*? A tube buys boundary area at fixed volume; r\* is a critical radius. There is plausibly one calculation here rather than two.
3. Where does **circulation** enter? Beyond a certain body size the tube's yield must be distributed, which is a second internal medium with a different profile (relay rather than converting) and different topology (branching, and returning). If the tube follows from the access profile, does the vascular tree follow from the tube plus the allometric constraint?
4. What is the **assay analogue**? Tasting is an assay run at the intake channel that decides whether to admit; vomiting is a refutation acted on after admission. The trichotomy and budget-relative refutation should both apply, and the lossy-medium correction owed upstream to scientist-note §4 may be the same correction.
5. Does the tube give a **new grading dimension**, or is it already covered by the vector over typed namespaces (Met, Src, Prb, Mat, and the candidate fifth Chn)? Suspicion: the tube is a fact about `k_chn`, which would be an independent argument for the fifth namespace.

---

## 6. Filing

Suggested home: a section of a rho-life note rather than a standalone. It is short, and it is a corollary of the trophic-structure material, so §10 of the scientist-note is the natural site — unless Props B, D and H develop enough measurable content to stand as the fifth note in the series.

*Second dictated note pending.*
