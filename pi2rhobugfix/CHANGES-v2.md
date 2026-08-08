# Draft 2 (pass one) — response to the review

Draft 1: 19pp. Draft 2: 19pp (≈2.5pp of narrative cut, ≈2.5pp of mathematics added).
Pass two adds Appendix B, currently a structured stub, at an estimated 6–8pp.

## Review items

| # | Review item | Disposition |
|---|---|---|
| 1 | Expand Prop. 6.5 into the conceptual centrepiece | **Modified.** See below. |
| 2 | Introduce `Fresh2` much earlier | **Done + strengthened.** §1 "The idea, in four lines" displays it on page 2; Prop. 7.7 proves it is essentially forced. |
| 3 | Reduce historical rhetoric | **Done.** ~2.5pp cut. |
| 4 | Replace "the answer was available in 1993" | **Done.** Removed, with "the repair is not an invention" and "a correct encoding of a different thing". |
| 5 | Fully prove operational correspondence | **Pass two.** Appendix B stub sets out the induction and case analysis. |
| 6 | Fully prove address independence | **Done.** Lem. 11.6 (name usage, syntactic induction), Def. 11.7 + Lem. 11.9 (equivariance), Lem. 11.10 (address independence). |
| 7 | A theorem on why `Fresh2` is not a distributed name server | **Done.** §10: Thm. 10.2, Cor. 10.3. |
| 8 | Shorten historical narrative by ~2pp | **Done.** §§4–5 compressed from 5.5pp to 3pp. |
| 9 | Expand mathematics by ~3pp | **Done in part;** ~2.5pp now, ~6–8pp more in pass two. |

## Where the review was modified rather than followed

**Item 1.** The review paraphrases Prop. 6.5 as "finite syntax cannot generate
infinitely many fresh quoted names because substitution stops at quotation."
That is false as stated — Lybech's encoding does generate unboundedly many
fresh names from finite syntax, by communication. Elevating the sentence as
written would overclaim in exactly the way the review criticises elsewhere.

Replaced by a stronger *true* statement, moved into a new §3 that mentions no
encoding:

- **Thm. 3.3** (a reduction creates at most one name): `Nms(P') ⊆ Nms(P) ∪ {@P₂}`.
- **Cor. 3.4** (freshness costs communication): n distinct new names cost ≥ n rendezvous.
- **Thm. 3.5** (a reduction adds at most one quote level), **Cor. 3.6** (one quote level per rendezvous).
- **Lem. 3.7** (substitution transparency), moved here from §6.

The old Prop. 6.5 is now **Cor. 7.4**, a corollary of §3. This is what the
review asked for — a structural theorem about the calculus rather than an
implementation detail — but with a statement that survives scrutiny.

**Item 2.** "Fresh2 is the paper" is half right: the *discipline* (Def. 7.5) is
the paper and `Fresh2` is its minimal realisation. Moving `Fresh2` forward
without saying so makes it look like a gadget that happens to work. Added
**Prop. 7.7**: any admissible allocator costs ≥2 reductions and needs ≥2 lifts
mentioning its parameter; `Fresh2` attains both. Open problem 2 records that
this is a bound attained, not an optimality result.

**Item 6 and the `Fresh2` race are the same item.** Once address independence is
proved as equivariance under an injective relabelling, the race closes in a
corollary (**Cor. 11.11**). No redesign. The relabelling apparatus is cited to
Sangiorgi–Walker rather than reconstructed.

**The comparison table** (§12) is kept but re-scoped: it now compares
implementation assumptions and the name-traffic graph, not degrees of
correctness. The "correctness criterion" and "status" rows are gone, and the
paragraph beneath states that both encodings are correct under their own
assumptions, for the same source language, and that Lybech's is currently
proved in more detail.

## New material not requested by the review

- §3 in full (Thms. 3.3, 3.5 and corollaries).
- §10 in full (Thm. 10.2, Cor. 10.3, Cor. 10.5 on freshness confined to translation time).
- Remark 11.8: the address relabelling is *not* a rho substitution — it must
  descend under quotes, and by Lem. 3.7 no substitution does. It is available
  to the metatheory and to no rho context. This is the one-line statement of
  why the encoding is safe from encoded observers and unsafe from arbitrary
  ones, and it ties §11 back to §3.
- Remark 9.9 and Appendix C: the join form of `Fresh2` is deterministic. The
  theory assumes only the monadic form and pays for it in Cor. 11.11.

## Still open before submission

1. Appendix B (pass two).
2. Venue. Suggested: EXPRESS/SOS or CONCUR, with the appendix carried as a
   technical report if the page limit bites.
3. Whether §2.2's four-point notation argument stays at its present length; it
   is the one section a referee might call self-indulgent, though point (4)
   is load-bearing.
