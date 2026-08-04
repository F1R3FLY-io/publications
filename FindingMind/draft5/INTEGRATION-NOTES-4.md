# Inserting the four vignettes, and switching the book to Palatino

Build: **435 pages** (was 391), three passes, zero errors, zero undefined
references, zero undefined citations, **two overfull boxes** — both pre-existing
(the TOC line and `turn_origins_ch02`'s display). The baseline had three; the
third, `turn_chs_ch01`'s chapter title, was fixed in passing. The `sec:gaps`
duplicate is still there; still out of scope.

Chapter numbering is untouched. The interludes are unnumbered, so nothing
downstream renumbers and every existing `\ref` resolves as before. Only
pagination moves.

## Files

Copy each into the tree at the indicated path.

| File | Destination | Status |
|---|---|---|
| `finding_mind.tex` | `FindingMind/` | revised — Palatino, interlude machinery, four insertion points, four frontispiece edits, attribution note |
| `Interludes/int_permitted_say.tex` | `FindingMind/Interludes/` | **new** |
| `Interludes/int_foam_born.tex` | `FindingMind/Interludes/` | **new** |
| `Interludes/int_begotten.tex` | `FindingMind/Interludes/` | **new** |
| `Interludes/int_right_sizing.tex` | `FindingMind/Interludes/` | **new** |
| `TheTurn/turn_hypercomp_ch05.tex` | `TheTurn/` | revised — one table set `\small` |
| `TheTurn/turn_ndo_ch02.tex` | `TheTurn/` | revised — one `\qquad` → `\quad` |
| `TheTurn/turn_sciencebot_ch09.tex` | `TheTurn/` | revised — one break opportunity |

`bibliography.tex` is **unchanged**. The Pledge and Prestige chapter files are
unchanged — the frontispiece edits live in the master.

The build tree is reconstructed by overlaying `draft2` → `draft3` → `draft4`
onto `ThePledge/`, `TheTurn/`, `ThePrestige/`. That reconstruction was compiled
first and came out at exactly 391 pages, so the 44-page delta is attributable.

## Where they sit

| | Page | Placement |
|---|---|---|
| *The Permitted Say* | 21 | after the Pledge frontispiece, before `pledge_ch01` |
| *The Foam-Born* | 57 | after the Turn frontispiece, before the Overarching Introduction |
| *begotten not made* | 411 | after the Prestige frontispiece, before the placeholder |
| *Right-Sizing* | 421 | after the placeholder, as **Coda** |
| A Note on the Interludes | 427 | after the Coda, before the bibliography |

Word counts: 3,147 / 2,524 / 3,078 / 2,031 — about 10,800 words, 34 pages.

## The body font

`\usepackage{mathpazo}` plus `\linespread{1.05}`, replacing nothing (the master
previously loaded no text font package at all and ran on Computer Modern).
Palatino carries the math as well as the text, so there is no seam between
prose and formulae. It costs 44 pages, which is the expected price of the wider
face on a 6-inch measure, and it puts the book in the same face as the
`agency` and `cuts-apertures` papers it draws on.

It also introduced four new overfull boxes. All four are now gone:

- **Chapter titles now set `\raggedright`** in the `titlesec` display format.
  This fixed `turn_sciencebot_ch04`'s 25.7pt overrun *and* the pre-existing
  `turn_chs_ch01` title overrun. Long chapter titles now wrap instead of
  running into the margin.
- `\emergencystretch` raised 2.5em → 3.5em.
- The sentience/consciousness comparison table in `turn_hypercomp_ch05` set
  `\small`. Content untouched.
- One `\qquad` → `\quad` in the Biosphere display in `turn_ndo_ch02` (0.55pt).
- A break opportunity in one `turn_sciencebot_ch09` bullet
  (`communication\hspace{0pt}-radius`).

## The interlude machinery

All of it lives in one block in the master preamble, after `microtype`.

- **Charter, via `\fontfamily{bch}`**, not `\usepackage{charter}` — the package
  would hijack `\rmdefault` for the whole book. Same for Courier (`pcr`) in the
  transcript block. Both are `psnfss`, so no new dependencies.
- **`\interludehead{kicker}{title}`** — `\cleardoublepage` so each opens on a
  recto, `\thispagestyle{empty}`, a grey small-caps kicker (Interlude / Coda),
  the title in large Charter italic, a 1.8in rule, and a `\phantomsection` +
  italic TOC entry.
- **`interludebody`** — measure narrowed 0.4in each side via `changepage`,
  11pt on 15.5pt leading, `\emergencystretch` 2.5em. Closes with
  `\clearpage\pagestyle{fancy}` to hand the book back.
- **`interlude` page style** — centred folio, no head, no rule. The running
  head does not remind you you are in a monograph.
- **No accent colour anywhere in the interludes.** Everything else in the book
  — chapters, sections, part numbers — is `accentblue`. Plain black is the
  cheapest available signal that the reader has stepped outside the argument.
- **`\asterism`** for scene breaks, built from three asterisks in a triangle
  (the `⁂` in the sources has no T1 glyph). All four vignettes use it,
  including *Right-Sizing*, whose source had bare gaps.
- **`\qline`** for the interviewer in *The Foam-Born* — indented both sides,
  italic.
- **`pidgin` / `\pline`** for the agentic creole in *begotten not made* —
  Courier 9/11.6, hanging indent with the speaker in a 5.4em box, thin rules
  above and below, **no frame and no tint**, because in this book a tinted
  `mdframed` box means *theorem* and a boxed transcript would read as one.

## Three things the log did not catch

Found by rendering pages, not by reading warnings.

1. **The asterism collapsed.** The first `\shortstack` had a −1.1ex correction
   that pulled the lower asterisk almost level with the upper two, giving
   `* * *` instead of a triangle. Retuned to `\\[0.3ex]` with a 1.1em gap.
2. **The transcript block broke badly** — first stranding its last exchange,
   then its closing rule, on the following page. `\nobreak` does not help; the
   breakpoint is the list's own trailing glue. Fixed by setting the block at
   9/11.6 rather than `\footnotesize`, which is tighter leading and reads more
   like a terminal anyway. It now sits whole on p. 414 with both rules and the
   following paragraph.
3. **In *The Foam-Born*, the editorial headnote and the first interviewer
   question were indistinguishable** — both indented italic, adjacent, reading
   as one block. The headnote is now `\small` at full measure, so indentation
   alone carries the interviewer's voice.

## Import repairs

- `\u003eroad exists` in the Drive export of *begotten not made* was a mangled
  escape; it is now `>road exists`.
- `\textperiodcentered` inside `\textsc` had no `TS1/bch/m/sc` shape. The
  channel name is now `\textsc{...}\,$\cdot$\,\textsc{tier-2}`.

## The frontispiece edits (the light option)

One clause each, picking up an image from the vignette that precedes it rather
than explaining the connection.

- **Pledge:** adds "It also asks what it costs to want the next step, and who
  is paying."
- **Turn:** adds "It is also, throughout, a structure too large to arrive whole
  in one small mind — which is itself one of the things it explains."
- **Prestige:** adds "It asks what a made thing owes its maker, and what a
  maker owes a made thing that has started walking on ahead."

## Flagged

- **The Prestige is still a placeholder**, now with a vignette on either side
  of it. The two interludes bracket an empty part; that was the decision, and
  the bracket should help shape what goes between them.
- **`T1/ppl/m/scit` undefined**, three occurrences — small-caps-italic Palatino
  in a running head. Pre-existing: the Computer Modern build had the identical
  warning at the identical three points. LaTeX substitutes small caps. Left
  alone.
- **The attribution note characterises the collaboration.** It says the
  sentences that survive are not all yours. Read it before you ship it — that
  is a judgement call about your own byline, and it should be your wording,
  not mine.
- **The capital-I convention.** The interludes use a capital first person
  against the book's lowercase *i*. The note explains this as deliberate. If
  you would rather not draw attention to it, that paragraph is the thing to
  cut.
- **The "five" motif.** Five founders, five personas and five makers, five
  committee members. Across 435 pages it reads as a signature rather than a
  tic, and it rhymes with the composed-learners material in the Turn — but it
  is now visible in a way it was not when these were separate documents.
- The vignettes are not in the bibliography. Their debts are listed in prose in
  the attribution note instead, which keeps fiction out of a technical
  reference apparatus. If you would rather have proper entries for Mahfouz,
  Barth, Carse, and the rest, that is a small addition to `bibliography.tex`.
