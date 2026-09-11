# The Tube

**Topological consequences of heterotrophy in the mortal scientist framework**

*Research note, dictated 7 September 2026; revised 11 September 2026. Sibling to the rho-life series: scientist-note (§10 Trophic structure), compose-note, engine-note.*

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

## 4. Rate of exchange: why breathing is not eating

*Dictated 11 September 2026, as a refinement of §1.*

Respiration is also an exchange across a boundary — something is admitted, a fraction extracted, the remains expelled — and yet the mammalian lung is a bag, not a tube. The dictated intuition: the discriminating variable is the **rate** of exchange, digestion running orders of magnitude slower than ventilation.

The intuition points at something real, but rate as such is the wrong variable.

**Respiration goes tube where one would least expect it.** Fish ventilate unidirectionally, mouth to operculum. Birds move air one way through the parabronchi during both phases. Unidirectional pulmonary flow has also been shown in alligators (Farmer & Sanders 2010), the savannah monitor (Schachner et al. 2014) and iguanas, and Farmer (2015) argues from that distribution that it is ancestral for diapsids and therefore *not* an adaptation for high gas-exchange rates — the animals that have it include non-flying ectotherms — proposing instead reduced work of breathing, water loss and heat loss. High rate is neither necessary nor sufficient.

**Two parameters, not one.**

- **Occupancy ratio** Λ = τ_proc / τ_enc — residence time of an admitted load over the interval at which a further load could profitably be admitted.
- **Separability** μ ∈ [0,1] — the degree to which residue can leave past incoming material at the same aperture without bulk transport. μ = 1 for a miscible residue leaving by diffusion; μ = 0 for a bolus.

Λ is what sets the sequential fraction σ of Prop B: if the control loop is blocked for the whole residence, σ = Λ/(1+Λ), and the Amdahl bound bites only as Λ approaches unity. *That* is the sense in which slowness matters — relative to opportunity structure, not absolutely.

**Breathing fails on separability, not on rate.** The respiratory residue fraction is near 1: almost all admitted mass is expelled. Bag topology survives because μ ≈ 1. The lung in fact tolerates gross re-mixing — an inspired bolus of ~0.25 L against ~1.5 L of residual gas, six parts spent to one part fresh, every cycle — because residue and resource are the same phase and separation happens at the membrane, not at the aperture. Digestion has μ ≈ 0, and the cost of re-mixing is not a few percent of efficiency but contamination of the batch.

**Prop I (the tube criterion).** A tube is forced when the agent is **phagotrophic** — admitting bulk material with a non-absorbable fraction — and Λ ≳ 1 and μ ≈ 0. Failure of any one of the three is sufficient for a bag. This subsumes Prop D rather than replacing it: the mutual exclusion is what is paid, Λ is how much of the cycle it consumes.

**Amendment to Prop E.** It was stated for autotrophs and should have been stated for non-phagotrophs. Source access is one way to have nothing to expel; it is not the only way.

---

## 5. Counterexamples, and where they land

M. Stay (personal communication, 11 September 2026) proposed a list of bag-gutted animals — cnidarians, ctenophores, flatworms, xenacoelomorphs, ophiuroids — plus two cases outside the animals: fungi, which are heterotrophs, and plants such as *Monotropa uniflora* that parasitise fungi rather than photosynthesise. It is the right test set. It separates into one case that is factually mistaken, four the occupancy ratio predicts, and two that force a correction of scope.

| Case | Verdict | Why |
|---|---|---|
| Ctenophora | **fails** | has a through-gut |
| Cnidaria | predicted | low Λ; duty-cycles when Λ rises |
| Platyhelminthes | predicted | buys area; gutless where osmotrophic |
| Xenacoelomorpha | predicted | no lumen at all; diffusion-scale bodies |
| Ophiuroidea | predicted | secondary loss, microphagous diets |
| Fungi | scope | osmotroph, not phagotroph |
| Mycoheterotrophs | scope | osmotroph, not phagotroph |

**Ctenophora — the counterexample that fails.** Presnell et al. (2016) showed by time-lapse imaging that the ctenophore gut is unidirectional and functionally tripartite, waste leaving through terminal anal pores specialised to control outflow, resolving a long-standing misreading that had supported the blind-gut picture (see also Giribet 2016). More than a removed counterexample: ctenophores branch very deep, so a through-gut there is evidence that the construction is arrived at from an access profile rather than inherited with the bilaterian body plan — which is this note's thesis.

**Cnidaria — the predicted duty cycle.** Mean digestion time in *Aurelia aurita* is about an hour (Ishii & Tanaka 2001); against a drifting tentacle feeder's encounter interval on dilute plankton that is low Λ, and Prop I predicts a bag. The prediction with teeth is what happens when Λ rises, and it has been observed: *Pelagia noctiluca* ephyrae on a dense, pulsed prey field saturate the gut in ~15 min and digest for ~18 h, which Gordoa et al. (2013) read as implying diel feeding periodicity. Fast fill, slow clear, forced alternation — Prop D observed in one organism, with the ratio measured.

**Platyhelminthes and Xenacoelomorpha — area instead of length.** Acoels have no gut lumen: the mouth opens into a syncytial digestive parenchyma without epithelial lining, particles handled by phagocytosis. At a few cell diameters there is no bulk transport to arrange and μ has no work to do. A triclad's branched blind gut is the same move at larger scale — boundary area rather than a second opening, which is the purchase Prop E assigns to autotrophs. Cestodes are sharper still: no digestive system at all, monomers absorbed across the tegument. An animal, a bilaterian, a heterotroph, descended from through-gut ancestors — and not a tube, because something else did the digestion. No formulation in terms of *heterotrophy* survives that; Prop I survives it via the phagotrophy clause.

**Ophiuroidea — secondary loss under a diet shift.** Brittle stars lack an anus, as do paxillosid asteroids, and Hejnol & Martín-Durán (2015) catalogue repeated independent losses of the anal opening across Bilateria. The direction matters: loss from a through-gut ancestor, not a lineage that never built one, and concentrated in microphagous habits — suspension and deposit feeding on small particles, small indigestible fraction per load, low Λ. Within-clade test: macrophagous ophiuroids should duty-cycle markedly harder than microphagous congeners, and if they do not, Prop I is in trouble on ground of its own choosing.

**Fungi and mycoheterotrophs — the scope was stated too widely.** Not counterexamples; a correction, and a useful one, since they name the clause the note left implicit. Heterotrophy is a claim about carbon source. The tube argument is about **phagotrophy** — admitting bulk material containing something that will not be absorbed. A fungus secretes enzymes outward and takes up monomers; the residue never crosses the boundary. Mycoheterotrophs do the same one remove further out, drawing fixed carbon through a mycorrhizal interface (Leake 1994; Bidartondo 2005), with *Monotropa uniflora* the resolutive case.

> **Obs (osmotrophs evert the lumen).** The lumen is boundary namespace — owned but open. An osmotroph runs the same converting chain *without* the ownership: the substrate **is** the lumen, and the engine sits at the boundary rather than folded inside it. The fungus is the tube construction turned inside out — which is why the mycorrhizal network appeared in engine-note as the external instance of the very profile the gut draws internally.

The cost structure is then legible rather than anomalous: internalising buys exclusivity over a partially extracted harvest (Prop C); externalising surrenders exclusivity and never has to transport residue. Prediction: osmotrophs should be vulnerable exactly where that trade is worst — to theft of extracellular product by neighbours.

**A prior statement of Prop B.** Honesty about priority: the functional claim is already in the literature. The same review that catalogues the losses states in passing that a one-way gut processes food more efficiently and permits uptake while the animal is still digesting (Hejnol & Martín-Durán 2015). The contribution claimed here is not the observation but the derivation — two openings following from concurrency plus ownership, with the threshold a computable function of Λ and μ rather than a qualitative preference.

---

## 6. What to write down formally

- The **two-phase harvest** as a term: an exclusive acquisition rendezvous on a boundary channel, in parallel with a persistent `Digest` chain on internal channels, terminating in a send on the expulsion channel. The existing `Forage` (scientist-note App A) and `Chain` (engine.rho §7) are most of it.
- The **tube as a namespace shape**: `Chn_boundary` factors as `Chn_in ⊔ Chn_out`, with a directed redex pathway from in to out and no return path. Then: *tube* is a formula in the logic, not a designation. That is the same move as "being a scientist is a depth-≥2 dialogue loop through a both-type channel".
- Whether the **absence of a return path** is forced or merely typical. Coprophagy, caecotrophy and rumination are return paths, and they are exactly the cases where one pass leaves too much unextracted. Cheap to state as a condition on residual yield.
- **Λ as a term-level quantity.** τ_proc as the metered cost of the digestion chain, τ_enc as the expected wait on the intake channel under the ambient offer rate. Then Prop I is an inequality between two things the framework already meters, and the tube threshold is *derived* rather than posited.
- **μ as a namespace condition.** μ is not a physical parameter but a statement about whether residue and intake can share a name without rendezvous — whether the sorts make the crossed redex impossible. Gas exchange has μ = 1 because spent medium and resource are the same sort and extraction happens at the wall, not the aperture. State it as a condition on the sort discipline and Prop I becomes entirely internal.
- The **physical topology as a shadow**. A through-gut makes the body a genus-1 surface. That is real but is not the content; the content is the channel topology, and the surface is what an embodiment of that channel topology in three-space looks like. Say so explicitly so that the note does not appear to be arguing from anatomy.

---

## 7. Open questions

1. Is the tube **forced** or **favoured**? Prop "composite resolution" says the overlap region between trophic strategies is populated by composites rather than atoms. Mixotrophs should therefore be predicted, and their tubes should be predicted to be facultative.
2. Does the **surface-area argument** connect to the r\* result of *Compression versus Autonomy*? A tube buys boundary area at fixed volume; r\* is a critical radius. There is plausibly one calculation here rather than two.
3. Where does **circulation** enter? Beyond a certain body size the tube's yield must be distributed, which is a second internal medium with a different profile (relay rather than converting) and different topology (branching, and returning). If the tube follows from the access profile, does the vascular tree follow from the tube plus the allometric constraint?
4. What is the **assay analogue**? Tasting is an assay run at the intake channel that decides whether to admit; vomiting is a refutation acted on after admission. The trichotomy and budget-relative refutation should both apply, and the lossy-medium correction owed upstream to scientist-note §4 may be the same correction.
5. Does the tube give a **new grading dimension**, or is it already covered by the vector over typed namespaces (Met, Src, Prb, Mat, and the candidate fifth Chn)? Suspicion: the tube is a fact about `k_chn`, which would be an independent argument for the fifth namespace.

---

6. Does the **osmotroph/phagotroph split** have a framework-native characterisation? If the osmotroph runs the converting chain outside the ownership boundary, the split should fall out of where the chain sits relative to `Chn_boundary`, and osmotrophy should carry a predictable exposure to theft that phagotrophy does not.
7. Is there a **second critical point at high Λ**? Ruminants and hindgut fermenters run τ_proc far above τ_enc and answer with buffering — a sequence of chambers, and in the ruminant a deliberate return path. Does the framework predict the chamber count?

---

## 8. Filing

Suggested home: a section of a rho-life note rather than a standalone. It is short, and it is a corollary of the trophic-structure material, so §10 of the scientist-note is the natural site — unless Props B, D and H develop enough measurable content to stand as the fifth note in the series.

*Second dictated note: written up as `LyingAndTheOriginOfLanguage`.*

---

## References

Bidartondo, M. I. (2005). The evolutionary ecology of myco-heterotrophy. *New Phytologist* 167: 335–352.
Farmer, C. G. (2015). The evolution of unidirectional pulmonary airflow. *Physiology* 30. doi:10.1152/physiol.00056.2014
Farmer, C. G. & Sanders, K. (2010). Unidirectional airflow in the lungs of alligators. *Science* 327: 338–340.
Giribet, G. (2016). Zoology: at last an exit for ctenophores. *Current Biology* 26: R918–R920.
Gordoa, A., Acuña, J. L., Farrés, R. & Bacher, K. (2013). Burst feeding of *Pelagia noctiluca* ephyrae on Atlantic bluefin tuna eggs. *PLoS ONE* 8(9): e74721.
Hejnol, A. & Martín-Durán, J. M. (2015). Getting to the bottom of anal evolution. *Zoologischer Anzeiger* 256: 61–74.
Ishii, H. & Tanaka, F. (2001). Food and feeding of *Aurelia aurita* in Tokyo Bay. *Hydrobiologia* 451: 311–320.
Jangoux, M. (1982). Digestive systems: Ophiuroidea. In *Echinoderm Nutrition*, Balkema.
Leake, J. R. (1994). The biology of myco-heterotrophic ('saprophytic') plants. *New Phytologist* 127: 171–216.
Presnell, J. S. et al. (2016). The presence of a functionally tripartite through-gut in Ctenophora. *Current Biology* 26: 2814–2820.
Schachner, E. R., Cieri, R. L., Butler, J. P. & Farmer, C. G. (2014). Unidirectional pulmonary airflow patterns in the savannah monitor lizard. *Nature* 506: 367–370.
