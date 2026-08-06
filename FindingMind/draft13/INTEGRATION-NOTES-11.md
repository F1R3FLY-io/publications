# Integration Notes 11 — Substack Part XII into the Pledge

Adds *The Pledge: Part XII — Inside and Outside* as a new numbered chapter at the
end of the confessional Pledge, immediately before the map.

Build verified: **623 pages** (from 619), **0 undefined references, 0 undefined
citations, 0 overfull boxes** — the same three counts as draft 12, which I rebuilt
from the base tree plus draft2–draft12 in order to have a baseline to compare
against.

| | |
|---|---|
| `ThePledge/pledge_ch05.tex` | **new** — *Inside and Outside, and What it Means to Assume the Virtue*, ch. 5, pp. 57–60 |
| `ThePledge/pledge_map.tex` | opening paragraph corrected (one sentence) |
| `finding_mind.tex` | one `\input` line; Pledge part-opener box gains one clause |
| `bibliography.tex` | **unchanged** |

---

## ONE THING NEEDS YOUR HAND BEFORE THIS SHIPS

The McCartney line is **not in the file.** In its place, on p. 58, is:

> Paul McCartney sang, "**[LYRIC TO PASTE: the closing couplet of *The End* — see
> the integration notes]**"

It compiles, it is impossible to miss on the page, and it is one paste away from
done — `pledge_ch05.tex`, the paragraph beginning "The Brits seem to have a handle
on this." I don't reproduce song lyrics, so I've left the sentence structured and
the words to you. You said you'd clear it; when you do, the permission you want is
for a reproduction of two lines in a book-length work of non-fiction, worldwide,
all editions.

Everything else in the chapter is verbatim from the post.

---

## Chapter numbering

This is the one consequence worth stating plainly. The chapter is numbered, so
everything after it moves up by one:

- The Turn now runs **ch. 6–62** (was 5–61)
- The Prestige now runs **ch. 63–65** (was 62–64) — *How the Trick Was Done* is
  ch. 63, *What a Symbol System Would Have to Be* is 64, *Any Future Metaphysics*
  is 65

Nothing broke, because nothing in the source hardcodes a chapter number — I checked
by grep across every `.tex` before making the change, and every cross-reference in
the book goes through `\ref`. The map and related-work chapters stay unnumbered
(`\chapter*` + `addcontentsline`), so they still sit outside the sequence.

What *is* now stale is external: `five_perspectives.tex` carries a chapter map in
its own prose, and integration notes 1–10 quote chapter numbers throughout. Neither
is part of the book.

## Placement, and why it reads better here than chronologically

The essay closes on a fear: that the narrative will only reflect your inner
experience, and that even a genuinely objective result would arrive framed as
personal vision. Four pages later the map ends with the gathered list of
load-bearing unproved claims and the line "Nothing else is hiding."

So the essay states the anxiety and the map answers it structurally — here is the
route through, here is what each route costs you, here is every place the weight
rests on something not yet proved. That sequence only works in this order. Put the
essay anywhere else in the Pledge and the answer arrives before the question.

## The edits inside the text

Kept: the lowercase *i*, the temporal fold ("you have simply turned the page"),
the sentence rhythm, every attribution.

Changed, and that is all that changed:

- Substack furniture removed (two subscribe blocks, the byline, the date).
- "the technical presentation for the next section of the book" → "for the Turn."
- The Ibn Arabi line is now the chapter **epigraph**, with the source attached.
  Where the post quotes it a second time inline, the sentence now reads "…i only
  just discovered Ibn Arabi and the wonderful formulation at the head of this
  chapter," so the discovery narrative survives without the quotation appearing
  twice on facing pages.
- "i believe i said as much at the very outset of this section" now carries
  "in Chapter~\ref{chap:prolegomena}."
- Quotes, dashes and apostrophes converted to LaTeX.

Labels: `chap:pledge-ch4` for the chapter, following the existing off-by-one naming
(ch. 4 is `chap:pledge-ch3`), and `sec:pledge-xii` for the section, as you asked.
That label does make the gap visible — the Pledge now runs `sec:pledge-i` through
`vii`, then `xii`. Parts VIII, IX and XI have never been brought in; X is
`prestige_ch01`.

No `\cite`, no `\footnote`. The other four Pledge chapters contain zero of each,
and the post attributes inline anyway, so the bibliography is untouched.

The `epigraph` package was already loaded in the preamble and had never been used
anywhere in the book. This is its first use. I set the measure to 0.62\textwidth
and suppressed the rule locally, inside the chapter file, so nothing in the
preamble changes and no other chapter is affected if you use it again later.

## The two in-place edits

**The map's opening.** It said "It is the last thing in the Pledge, and it is the
only chapter in the book that is about the book." That was already false when
related-work was added, and would now be false twice. It reads:

> The confessional part of the Pledge has just ended; this chapter and the one
> after it are the only chapters in the book that are about the book.

Which also does a small piece of work the old sentence didn't: it tells the reader
the register has changed.

**The Pledge part-opener box.** Its list of what the act asks — compositionality,
agency, information, causation, what it costs to want the next step — now ends
"…and what it would take to bridge the two cultures that have to meet for any of
this to be checkable." One clause, so the box covers the chapter that is now in it.

## One thing I'd flag for a later pass

The paragraph beginning "For example, i'm not interested in any formulation…"
states three demands together — mathematically precise, physically testable,
narratively available to someone with neither — and that is the clearest statement
of the book's own success condition anywhere in the Pledge. Nothing else states it.

The Prestige then audits the book against exactly those three. Right now that audit
is measured against a standard the reader met once, in passing, in a paragraph that
doesn't announce itself as the standard. Setting it off — a displayed list, or a
named remark the Prestige can point back to — would cost three lines and would give
ch. 63 something to be answerable to. I didn't do it, because you said minimum.
