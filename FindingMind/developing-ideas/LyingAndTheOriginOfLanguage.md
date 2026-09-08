# The Gap

**Deception as a bootstrap mechanism for grounded symbol systems**

*Research note, dictated 7 September 2026. To be picked up later. Continues bootstrap-note ("Getting the Distribution Is Not Getting the Correspondence", draft 4) and the rho-life series.*

---

## 1. Where this picks up

The earlier note established the negative result and its two-sided evidence.

- **Linear A and Rongorongo.** A corpus large enough to train a transformer to emit well-formed utterances, and no community of speakers left. Decades of resistance to interpretation. The distribution survived; the correspondence did not.
- **Whale song.** No Rosetta stone, no cognates — and yet real progress, because the population is alive and its behaviour can be observed. Rongorongo minus the extinction.

Conclusion drawn there: the correspondence between a symbol system and the world is carried in the community of speakers, not in the symbol system. Decipherment is transport or direct observation of the using population; neither route is distributional.

That note also recorded that the bootstrap problem *has* to be soluble — humans and whales each solved it independently — and isolated three features common to both solutions: the solver was a population, the interface was not chosen, and there was an external criterion supplying the fitness signal.

**This note supplies a candidate mechanism for the bootstrap itself.**

---

## 2. Benedikt's argument, as recalled

Michael Benedikt, ACSA Distinguished Professor of Architecture at UT Austin, held the Hal Box Chair in Urbanism and wrote *Deconstructing the Kimbell* (1991) among much else. He died in August 2025. The argument below is recalled from conversation and should be cited as personal communication unless a published source turns up; no published version surfaced on a first search.

Benedikt argues that language originates in lying.

**The setting.** A band of early hominids on the plain, not far from the trees, dividing food — fruit, or the spoils of a kill.

**The prior state.** Many animals make reflexive noises, grounded in function: the sounds of breathing, howling in pain, the alarm call. These are triggered by conditions. They are not meaningful in the way that language is meaningful.

**The event.** Eve notices movement in the grass. The conditions land on her nervous system in just such a way as to trigger the snake warning call. She calls; the band scatters for the trees. In that split second Eve determines that it was not a snake — it was the wind — and at the same moment notices that she now stands in an advantageous position with respect to the division of the food.

What she sees in that moment is *the gap*: the distance between the reflexively triggered sound and the behaviour of the band. And she sees that the gap can be exploited.

**The follow-on.** If Eve does this regularly, others may notice the correlation between the snake call and the division of the food. The band learns Eve's trick from Eve's behaviour. Language is a virus (Burroughs, via Laurie Anderson).

**The reading.** It is the gap that gives the opening for meaning. A reflexive call and the condition it is associated with are welded together; nothing there needs interpreting. Prise them apart and semantics becomes possible — and necessary. This is the sense in which lying is the knowledge of good and evil offered at the centre of the garden.

*Note: Eve here occupies the same position she does in the Ouroboros vignette — the one who eats.*

---

## 3. The empirical situation is better than anecdotal

Worth recording, because it turns the parable into something with data behind it. All of this was checked and is real literature.

- **Tufted capuchins.** Wheeler (2009), *Monkeys crying wolf?*, Proc R Soc B 276: wild capuchins produce terrestrial-predator "hiccup" calls in non-predatory contexts where the caller stands to gain. False alarms are given by subordinates more than dominants, more when food is contestable, more when food is scarce, and from spatial positions where the caller would gain if others fled. Listeners run up into the canopy and out of the patch; the caller takes the food. **This is Eve's scenario, observed.**
- **Counterdeception.** Wheeler (2010); Wheeler & Hammerschmidt (2013): listeners are more likely to ignore the same call type when it is produced in competitive feeding contexts than elsewhere. The discount is context-specific — escape responses drop, vigilance responses do not.
- **Fork-tailed drongos.** Flower (2011), Proc R Soc B 278; Flower, Gribble & Ridley (2014), *Science* 344: drongos steal food using *mimicked* alarm calls of other species, and vary which call they mimic. The stated reason is that a deceptive signal loses power as its frequency rises relative to honest ones — so varying the signal is what sustains the deception.
- Earlier: Munn's kleptoparasitic antshrikes and shrike-tanagers; arctic foxes calling cubs off food.

The drongo result is the important one. **Deception does not merely exploit a signal repertoire; under frequency-dependent discounting it drives the repertoire to expand.** That is a mechanism for vocabulary growth, not just for the birth of meaning.

---

## 4. The framework translation

### 4.1 The three stages

**Stage 0 — the reflex.** The call is a rewrite rule, not a symbol: `for(@c <- sensor) call!(...)`. The correspondence to the world is a fact about the wiring. The receiver's response is another rewrite. Neither party needs a hypothesis. In the ladder's terms, both sides are *non-modal*: this is perception, reading structure the world has already computed.

**Stage 1 — the decoupling.** Eve's innovation is to make the call channel writable by her policy independently of the sensor. In the channel classification (scientist-note §12.6: internal / external / both-type), this is a **re-typing**: an emission that was driven only from the sensor side becomes driven from the policy side as well. Nothing new is added to the calculus. The gap is a change in which redexes can fire on a name.

**Stage 2 — semantics as the receiver's forced expense.** Once emission is decoupled, "call heard" no longer entails "snake present". The receiver's predicate cannot stay non-modal. It must be indexed by context — a hypothesis about the *sender*, not about the world. And that is exactly the machinery the mortal scientist already has: context-labelled modalities, and an environment whose most interesting contents are other scientists.

> **Thesis.** Meaning is not a property of the sign. Meaning is the theory that receivers are forced to build once senders are capable of lying. Semantics is the receiver's bill.

This is the claim to make precise and, if possible, prove. It would explain the negative result of the earlier note rather than merely restating it.

### 4.2 Propositions to attempt

**Prop A (no semantics before the lie).** In a population where every emission is sensor-driven, the receiver's optimal policy is a non-modal predicate on the signal. Formally: the modal rungs of the ladder buy nothing. Corollary: an alarm call in a wholly honest population is not a symbol, and the framework can say so with a purchase criterion rather than a definition.

**Prop B (the lie forces the modal rung).** With a nonzero deception rate, the non-modal predicate is strictly dominated, and the receiver's best available response requires context-indexing. The transition should have a **threshold** in the deception rate, computable in the same currency as the game note's ladder returns.

**Prop C (the deception equilibrium).** The liar's yield is proportional to the response rate, which declines in the population deception rate. Interior equilibrium. The empirical predictions are already made and already met: subordinates lie more, lying rises with contestability, listeners discount contextually.

**Prop D (elaboration under discounting).** The drongo result: at equilibrium there is pressure to *diversify* the signal, because a fresh signal starts at the honest response rate. Vocabulary size grows as a function of the deception pressure and the cost of manufacturing a new distinguishable name. Freshness-by-quotation says names are cheap to manufacture, which predicts fast growth — perhaps too fast, so what limits it is a real question.

**Prop E (the trick is horizontally transmitted code).** The band learns Eve's trick by watching Eve, not by descending from her. This is the reproduction material with the vertical arrow replaced by a horizontal one: a quoted process copied between contemporaries. Burroughs' virus is not a metaphor here — it is an R₀, and the thing it spreads degrades the resource it spreads on.

**Prop F (the headline: strategic use makes the corpus unidentifiable).** If emission is policy-driven, then the joint distribution over signals and world-states is a *mixture over sender policies*, and the mixture weights are facts about the population. A corpus gives the marginal. The marginal does not identify the mixture. Therefore no corpus, of any size, recovers the correspondence — and this is a theorem about strategic signalling, not a statement about data scarcity.

> If Prop F goes through, the earlier note's central observation is upgraded from an observation to a consequence: **the correspondence lives in the community because the senders are strategic.** Linear A resists because its semantics was never in the distribution to begin with. Whale song yields because the senders are still available to be modelled.

**Prop G (grounding is a drive, not a read).** The game note distinguishes two routes to any predicate: read it, if the environment has computed it into legible structure, or drive it, by rendezvous. A transformer over a corpus can only read. Under Prop F the readable object is the wrong one. The identifying information is available only to an agent that can *emit into the community and observe the response* — that is, one that can perform the intervention that separates the mixture components. **Grounding requires participation, not perception.** That is a sharp and unwelcome claim about what embodiment has to mean, and it is stated in the framework's own pricing vocabulary.

---

## 5. Caveats and honest gaps

1. **Deception is not the only bootstrap** — explicitly granted in the dictation. Lewis conventions and Skyrms' signalling games produce meaning by reinforcement under aligned interests, with no liar anywhere. The claim to defend is not that lying is necessary for signalling, but something narrower: **lying is what makes meaning require modelling** — what forces the receiver up from correlation to theory. Those two routes should end up producing different kinds of symbol system, and characterising the difference is a good problem.
2. **Deception is parasitic on an honest baseline.** The reflexive layer must exist first; there is nothing to exploit otherwise. Order matters, and it is a design constraint on any engineered bootstrap: build the reflex layer first, and make it functionally grounded, before there is anything for a policy to detach.
3. **The alignment with Dawkins & Krebs** (*Animal Signals: Information or Manipulation?*, 1978) should be checked. Their position — signals are manipulation rather than information transfer — is Benedikt's thesis arriving from ethology, and the costly-signalling literature (Maynard Smith & Harper, *Animal Signals*) is where the equilibrium of Prop C has presumably already been worked out in a different currency. Do not reinvent it; import it and pay it in tokens.
4. **Intentionality is not claimed and should not be.** The capuchin literature is careful about whether false alarms are deliberate; one paper tests whether adrenocortical stress explains the calling. The framework does not need deliberateness — it needs only that emission be policy-driven rather than sensor-driven, which is a structural condition on channels and is exactly what Prop A/B are stated in terms of.
5. **The gap-as-opening reading is doing philosophical work** that the formal apparatus may not reach. Keep the Genesis line as a closing remark, not as a premise.

---

## 6. What to do next

- Draft Props A and B properly. They are the ones that a ladder computation could actually decide, in the style of the noughts-and-crosses and Nim ladders — a signalling game with a deception rate as the environment parameter, and the modal rung's return computed against it. The pattern from compose-note ("whether truth is affordable is a property of the environment") suggests the answer will be: whether *meaning* is affordable is a property of the deception rate.
- Attempt Prop F as a genuine identifiability theorem. It is the one with reach outside the framework.
- Fold the result back into bootstrap-note §2.5, whose third common feature (an external criterion supplying the fitness signal) is here instantiated as *the food share* — and whose first feature (the solver was a population) is here explained rather than observed, since a lie requires someone to lie to.

---

## 7. Filing

Candidate fifth note in the rho-life series, or a substantial new section of bootstrap-note. Leaning towards its own note: Props C, D and F have measurable content and an empirical literature attached, which is more than a section can carry.
