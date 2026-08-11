# INTEGRATION-NOTES-17 — draft19

**Title page: new subtitle, and the three-act line reworded.**

1. Subtitle *The Mortal Scientist and the Ecology of Thought* added.
2. *The Pledge, the Turn, and the Prestige* becomes *A Magic Trick in Three Acts:
   The Pledge, the Turn, and the Prestige*.

Applies on top of draft18. One file, four lines. No prose, no argument, no result
touched.

Build: repo baseline + draft2..draft18 + this delta, three `pdflatex` passes.
**680pp** (unchanged), 0 errors, 0 undefined references, 0 undefined citations,
**8 overfull boxes — the same 8, at the same line numbers, to the same
sub-point widths.** Byte-identical overfull set against the pre-edit build of the
same tree.

**1 file:** `finding_mind.tex`.

---

## Decisions carried in

| # | decision | effect |
|---|---|---|
| 1 | keep both, new subtitle above | the three-act line stays, now the third line |
| 2 | update `pdftitle` and the running header | both carry the full subtitle |
| 3 | reword the three-act line | now names the trick before naming its parts |

---

## The three edits

### 1. Title page (l.508–514)

```latex
\title{%
  \vspace{-0.8in}
  {\large\scshape Working Draft}\\[0.6em]
  {\Huge\bfseries Finding Mind}\\[0.5em]
  {\Large\itshape The Mortal Scientist and the Ecology of Thought}\\[0.4em]
  {\large\itshape A Magic Trick in Three Acts: The Pledge, the Turn, and the Prestige}
}
```

One inserted line. **The size is the only judgement call in this delivery:** the new
subtitle is set `\Large\itshape` against the three-act line's existing
`\large\itshape`, one step larger, so that the two italic lines are not twins. You
declined the "demote the three-act line" option, so the three-act line is untouched —
the separation comes entirely from the new line being bigger, not from the old one
being smaller. If you would rather they sit at the same size, change `\Large` to
`\large` and nothing else moves.

Leading between them is `0.4em`, against `0.5em` above, so the two italic lines read
as a pair under the title rather than as two independent lines.

The reworded third line sets to roughly 5.6in on the 6in measure — one line, no
overfull, comfortable margins. Two observations, neither of them blocking:

- **It is now wider than the subtitle above it**, so the block no longer tapers; it
  goes narrow, wide, wider. It reads cleanly as rendered, but if you want the taper
  back the fix is to break after the colon and set the three names on their own line.
- The line reads *Acts*, matching the magician's formula the three parts are named
  after. An intermediate revision in this session said *Actions*; that was an
  autocorrect artefact and is not in the shipped file.

### 2. PDF metadata (l.135)

```latex
  pdftitle={Finding Mind: The Mortal Scientist and the Ecology of Thought},
```

Verified in the built file: `pdfinfo` now reports the full title. `pdfauthor`
unchanged.

### 3. Running header, verso (l.497)

```latex
\fancyhead[RE]{\small\itshape Meredith --- Finding Mind: The Mortal Scientist and the Ecology of Thought}
```

Measured, not assumed: at `\small\itshape` Palatino the line sets to roughly 4.6in
against a 6in measure, so it clears the verso page number on the left with about an
inch of air. No overfull head, and the header rule is unmoved. Rendered and checked
visually as well as in the log.

Two things worth knowing about this one:

- **It is a long header for a 680pp book.** Standard practice would be title verso,
  chapter or section recto — which is what the recto side already does via
  `\rightmark`. Seventy-four characters of italic on every verso is a deliberate
  choice, not an oversight; if it reads as noise once you see it in the run,
  `Meredith --- Finding Mind` back on the verso costs one line.
- The fancy pagestyle is active over the front matter and the header therefore
  appears on ten pages of the current build; `\chapter` openings take `plain`, and the
  body's verso pages are not picking it up. That behaviour is **pre-existing** and was
  not introduced here — it is worth a look at some point, but it is not this pass's
  business and I have not touched `\pagestyle`.

---

## One repo finding, unrelated to the edit

The committed `draft18` holds **five entries** — `INTEGRATION-NOTES-16.md`,
`finding_mind.pdf`, `finding_mind.tex`, `pledge_related.tex`,
`turn_rholife_ch04.tex`.

INTEGRATION-NOTES-16 describes **49 files**, and the base tree contains zero
occurrences of `enzyme` and zero of `flavor`. So the 46-file nomenclature sweep of
`TheTurn/`, `ThePrestige/` and `Interludes/` — the engine → enzyme rename, the
colour → flavor rename, `eng:rem:nomenclature`, and the ~600-substitution US spelling
sweep — is **not in the repository**. Only the three files above survived the commit.

Consequences:

1. The tree I built and verified against is draft18-as-committed. It comes to 680pp
   with the same 8 overfull boxes, which is why the numbers above match the draft18
   notes exactly — the nomenclature pass changed no pagination, so its absence is
   invisible to the build. **The line counts and the overfull set are therefore valid;
   the words in the chapters are not draft18's.**
2. I have not shipped a PDF with this delta. Building one now would produce a book
   whose title page is right and whose Part II still says *engine* and *colour*, which
   is worse than shipping no PDF.
3. `pledge_related.tex` in draft18 already carries "flavors, enzymes" while the
   chapters it points at do not — so the committed state is internally inconsistent
   until the missing files land.

If you still have the draft18 delivery, dropping the remaining 46 files into
`FindingMind/draft18/` restores it; nothing in this delta conflicts with any of them.
`finding_mind.tex` is the only file both passes touch, and the draft18 copy is the one
I edited, so this delta supersedes it cleanly.

---

## Reference safety

No labels, refs, cites, macros, environments or bibliography entries added, removed or
renamed. Nothing outside the preamble and the title block was opened.
