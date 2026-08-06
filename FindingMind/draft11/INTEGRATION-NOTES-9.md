# Integration Notes 9 — reordering the Prestige

Supersedes the ordering described in `INTEGRATION-NOTES-8.md`. Everything else in
those notes still holds.

New order:

| Ch. | Title | Label | File |
|---|---|---|---|
| 62 | How the Trick Was Done | `ch:trick` | `prestige_ch01.tex` |
| 63 | What a Symbol System Would Have to Be | `ch:notation` | `prestige_ch02.tex` |
| 64 | Any Future Metaphysics | `ch:kant` | `prestige_ch03.tex` |

**The files were renumbered to match the reading order.** `prestige_ch01.tex` in this
delta is the Trick chapter, not the Kant chapter. Since delta directories overwrite by
filename, dropping draft10 over draft9 replaces the old `prestige_ch01.tex` (Kant) with
the new one (Trick) and the labels do the rest — nothing in the master or in any other
chapter refers to these by filename.

Build verified: **606 pages, 0 undefined references, 0 undefined citations.**

---

## Why this ordering, structurally

The reorder does more than move the reveal forward. It inverts the direction of the
book's central pivot, and the inversion is an improvement.

Under the old order, ch. 62 (Kant) established "the noumenal is a budget constraint" and
ch. 64 (notation) cited it, calling budget-relative notation "the third instance of
§62.5.1's pivot." Kant was the source and notation was the debtor.

Under the new order that reverses. §63.3 now states its own case — the move is named
from the two instances the Turn already supplied (modal depth, counting) and the third
is added on its own terms — and **§64.5.1 becomes the place where all three are
gathered**, with a new closing argument that did not exist before:

> Three instances is enough to stop calling it a coincidence. What is going on is that
> every apparent limit on what a computational agent can know, once you look at it
> closely enough to say where the limit is, resolves into a price. Not one of them
> resolves into a prohibition.

Kant stops being the chapter the others cite and becomes the synthesis, which is a
better close for the book than ending on machinery.

---

## Rewrites the reorder required

### `prestige_ch01.tex` (Trick, now ch. 62)

- **New chapter opening** (~2 paragraphs before §62.1). It now does the part-establishing
  work Kant used to do — reclaims the *Prolegomena* thread, says why the reclaiming is
  deferred to ch. 64, and states the Prestige's job before subverting it: *"The Prestige
  is supposed to bring it back. What i am going to do instead — first, and at length —
  is show you the method."*
- §62.1 trimmed of the Pledge/Turn recap now carried by the new opening.
- §62.4's two references to Kant were backward and are now forward. Rewritten as
  **anticipation** rather than citation: the chapter now glosses the first-personal
  transcendental argument and the noumenal-as-budget pivot in enough detail to stand
  alone, then adds three lines making the forward-naming deliberate rather than awkward:

  > i am naming these before Chapter 64 makes them, which is not the usual courtesy. It
  > is the point. Anything that chapter recovers of Kant, it recovers having already told
  > you what it is standing on, and a reader who wants to discount it accordingly now has
  > the means.

### `prestige_ch02.tex` (Symbol System, now ch. 63)

- Roadmap entry for §63.3 reworded — it no longer claims to be "the third appearance of
  §62.5.1's pivot" but flags that ch. 64 will gather it.
- **§63.3 opening rewritten** to name the move from scratch rather than inherit it. It
  now states the pattern (a barrier that resolves into money, not logic), gives the two
  prior instances in its own words, and adds the third. One forward pointer to ch. 64 as
  the gathering place, marked as such.
- §63.3's closing sentence de-referenced.

### `prestige_ch03.tex` (Kant, now ch. 64)

- **Chapter opening rewritten.** It no longer announces itself as the first thing in the
  Prestige. It now opens by acknowledging that the method has already been shown, and
  argues that coming last is the harder and more credible position: *"the borrowed word
  comes back last, and it comes back in full view of the trick."*
- **New qualification after the chapter's central claim.** The word *computational* in
  "any computational agent" is flagged as the scope restriction doing its work one last
  time, with the premise named and the gap between thinking and showing conceded.
- **§64.5.1 rewritten** as described above — the gathering of three instances, with the
  new "resolves into a price, not a prohibition" argument. A now-redundant paragraph
  restating the pivot was deleted.
- **Coda handoff rewritten** from future to past: *"It is also a condition we arranged,
  and Chapter 62 said how. i have put the reading of Kant last rather than first so that
  it would have to stand up in that light, and i think it does."*

### `prestige_placeholder.tex` and `finding_mind.tex`

Chapter summary reordered; `\input` order and comments updated.

---

## Still open from Notes 8

Items 1, 3 and 4 from the previous notes stand unchanged: the alternative title *The
Clear Plastic Cups*; the two dropped TikZ figures (`fig:locus` especially); and whether
Part V should re-bill Composing Learners as a precondition rather than an extension.

Item 2 — whether `obs:sidestep` should also appear in `ch:sci` — is now **more** decided
against by the reorder than before. With the Trick chapter opening the Prestige, the
reveal lands early enough that pre-announcing it in Part V would cost the part its
opening move for no gain.
