# Reversibility as the history monad, referenced rather than rebuilt

Three files. Build: **625 pages** (draft14 was 623), three passes, zero errors,
zero undefined references, zero undefined citations. Eight overfull boxes,
byte-identical to the draft13 baseline — none added. Chapters 12 and 24 both
start on the same page they did before, so nothing reflowed around them.

`finding_mind.tex` and `bibliography.tex` untouched. No new macros, no new
environments, no new bibliography entries.

## Files

| File | Chapter | What changed |
|---|---|---|
| `TheTurn/turn_causality_ch03.tex` | 24 | rewritten — references `Hist` instead of rebuilding it |
| `TheTurn/turn_gslt_ch07.tex` | 12 | two labels added, one forward-pointer remark |
| `TheTurn/turn_origins_ch05.tex` | 58 | the stale "Part I" reference and the over-attribution |

## The identification

Ch. 24's `⟨P, τ⟩` under `r⁺` **is** ch. 12's `Hist` — pairs of a configuration
and a word in the free monoid of rewrite events, each rewrite appending. Ch. 24's
η is the monad unit, its π is the functor stripping the log, and `π∘η = id` is a
triangle identity of the adjunction. All of that was being reproved by hand
twelve chapters after it was established.

The residue is that ch. 24 also has the backward rules `r⁻`, which `Hist` does
not, so `𝒮^†` is not `Hist 𝒮` — the rule set is strictly larger. But the
addition costs nothing, and **ch. 12 already contained the reason without
drawing it**: the free history is the universal cover, hence a tree, so every
configuration with `τ ≠ ε` has exactly one parent and `r⁻` is *determined*, not
chosen.

That is now `prop:bwd-free` with a three-line proof, and `rmk:free-groupoid`
states what the object is: **`𝒮^†` is the free groupoid on the tree `Hist 𝒮`.**
Formally inverting the arrows of a tree imposes no relations because there are
no loops for the inverses to interact with. Which is the whole reason the
construction is cheap — reversibility is not bought by adding structure, it is
what a theory looks like once it stops discarding its past, and the history
monad is the operation of stopping.

## What ch. 24 looks like now

`con:revenv` states the envelope in four clauses, of which the first two are
`Hist 𝒮` verbatim and only the third is new. Then `prop:bwd-free`,
`rmk:free-groupoid`, and one paragraph noting that η, π and `π∘η = id` arrive
with the monad rather than needing separate proof.

**The `Physical Interpretation` remark is kept verbatim.** It is the chapter's
actual contribution and it is not in ch. 12. What follows it is new and is an
audit of it: the laws being symmetric is the construction; the asymmetry having
to go *somewhere* is the universal-cover proposition, since a tree has a root
and the root is the boundary condition; and nothing here supplies a reason the
world's boundary condition should be `τ = ε` rather than something else, which
the chapter now says rather than leaving a reader to wonder.

Two remarks are new beyond that.

**`rmk:coarse-cover`.** Ch. 12's erasure lattice, read as physics. `𝒮^†` sits at
the top; the rungs below are intermediate covers, and an intermediate cover is
a coarse-graining — a description keeping enough of the past to run backwards
through some transitions and not others. Which loops one may close is which
histories one can afford to forget, and by the ledger law `σ + κ = σ₀` with
Landauer, the affordable erasures are exactly the state-recoverable ones. An
observer at an intermediate cover has a time-asymmetric physics not because the
laws are asymmetric but because it cannot afford the receipts. This was
available for free and nobody was claiming it.

**`rmk:rev-undecorated`.** Says plainly that nothing in ch. 24 assigns a value
to a rewrite, that decoration arrives in ch. 26, and that ch. 30's `con:main`
builds `⟨P, τ, A⟩` — which is `Cost` applied to `Hist`. It then states that no
distributive law between the two monads appears anywhere in the book, so that
composite should be read as two constructions in a fixed order and not as a
canonical object.

That last paragraph is the point of the whole exercise. `pledge_related`
concedes the missing distributive law in the front matter; until now the body
gave a reader no way to see where the gap bites. It bites here, in the object
Part III's physics is built out of.

## Ch. 12

Two labels added, since ch. 24 now cites both propositions:
`ob:prop:hist-adjunction` and `ob:prop:univcover`. Nothing else in the chapter
changed.

One remark added, `ob:rem:hist-forward`, so the pointer runs both ways: ch. 24
is this construction under a physical reading, the backward rules cost nothing
by the universal-cover property, and the erasure lattice is what that chapter's
physics needs and has no name for.

Incidental effect worth noting — this is the cheapest available answer to the
standing complaint that `Hist` is the weak leg of the three monads. It has not
been made longer. It has been made load-bearing.

## Ch. 58

`origins_ch05` opened a passage with "Recall from Part~I that the reversibility
construction…". That has been wrong since the mind-first refactor; reversibility
is in `part:physics`. Now a `\ref`, so it cannot go stale again.

The same sentence claimed the reversibility construction **requires**
`bwd_r(φ) = conj(fwd_r(φ))`. Ch. 24 contains no amplitudes at all, so that was
an over-attribution as well as a stale reference. It now says the construction
makes the laws time-symmetric and that *this section* strengthens that to a
condition on the decoration.

**This is a minimal repair, not the real one.** See below.

## Open items

1. **Ch. 58 is a fourth site of the retracted complex apparatus.** The
   five-perspectives review found `sciencebot_ch05`, `hypercomp_ch05` and Gap 7.
   It did not find this one. The CPT condition, the gradient-coupled rules
   (`|fwd| > |bwd|`), and the entire broken-symmetry argument for the origin of
   life rest on complex amplitudes that `causality_ch09` retracts. The edit above
   corrects the attribution and leaves the dependency. It belongs with R2 in the
   next pass, which makes that pass three chapters and a gap rather than two.
2. **`prop:bwd-free` is stated for the free history.** Whether the backward
   rules are still determined at an *intermediate* cover — where a configuration
   may have several parents — is exactly the question `rmk:coarse-cover` gestures
   at, and it is not answered. If they are not, "partially reversible physics"
   has a precise meaning the book could use.
3. **The free-groupoid identification is mine** and is not stated anywhere else
   in the book or, as far as I can tell, in the source notes. It is a one-line
   argument, but it should have your eye before it ships as a remark.
4. `causality_ch11`'s open problem on the *universality of the reversibility
   envelope* — whether `𝒮^†` is minimal among reversible GSLTs receiving a
   morphism from `𝒮` — is now nearly answered by `rmk:free-groupoid`, since free
   constructions are minimal by universal property. I did not touch it, because
   turning "nearly" into "answered" wants the universal property stated and
   checked, and that is a result rather than an edit.
