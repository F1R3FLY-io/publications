# Integration notes — draft30

Fourth and last delta of the graded-where-clause iteration. Closes the plan
agreed for draft27–30: **B3 *Inhabiting Choice*** in the Prestige before the
Kant chapter, the **front-matter pass** (T5), and the two bookkeeping items
T9/T10. Builds on the repo baseline + `draft2..draft29` (reading both
`draftN/*` and `draftN/src/*`).

Build: **863 pages**, 0 errors, 0 undefined references/citations, 0 multiply
defined labels, 9 overfull boxes (the baseline nine; none new). Clean-room
build from the committed deltas + this directory produced text-identical
output to the working overlay. Bibliography now 303 entries (+13).

## What this delta does

### 1. New chapter: *Inhabiting Choice* (`ThePrestige/prestige_inh_ch01.tex`, `ch:inhabit`)

Placed after *How Life-Like Is Life?* (`prestige_psy_ch02`) and before *Any
Future Metaphysics* (`prestige_ch03`), in Part XI. This is the constructive
residue of what the plan called "the critique of proof assistants and
transformers": it does not re-argue the critique, it states the one point the
critique turns on and reads it in three registers.

- **Abstractly** (`inh:sec:abstract`): *determinate systems collapse the
  spectrum* (`inh:def:determinate`, `inh:prop:collapse`); the λ-calculus
  witnesses are forced to βη (`inh:cor:lambda`); rho does not collapse
  (`inh:prop:noncollapse`, now an `align*` display); the distinction between
  *stating* a choice principle and *inhabiting* one (`inh:obs:inhabit`).
- **For the verification of software** (`inh:sec:verification`): the
  unauditability observation (`inh:obs:audit`); Iris and CertiKOS conceded as
  genuine achievements that live on the determinate side of the line
  (`inh:rmk:lean`).
- **For intelligence** (`inh:sec:ai`): observational invisibility of
  branching (`inh:prop:invisible`); the smallest behavior that separates the
  two sides — the vending machine — and the `do` program that settles it
  (`inh:sec:smallest`, pointing back to `ch:do`).
- **For the interlocutor** (`inh:sec:interlocutor`): experimenter vs
  spectator.
- **The saddle point** (`inh:sec:saddle`): desert annuals and bet-hedging as
  the biological picture; the three requirements on recursive
  self-improvement (`inh:obs:rsi`) — stated as an *observation*, listed in the
  Pledge's unproved-claims list.
- **One point, three registers** (`inh:sec:one`, table `inh:tab:three`) and
  **What is claimed** (`inh:sec:claims`), in the book's usual
  claims/non-claims form.

Hand-offs: `prestige_psy_ch02` gained a closing sentence pointing forward;
`prestige_ch03` (Kant) opener now reads "— and, from
Chapter~\ref{ch:inhabit}, …" and says "three preceding chapters".

### 2. Front matter (T5 closed) — `ThePledge/pledge_map.tex`, `ThePledge/pledge_related.tex`

`pledge_map.tex`:
- Map figure: a **Realization** node `(real)` with its arrow; caption and
  the "Four of the other five parts…" paragraph updated; part-number note
  now III–IX / XI–XII; seven parts named throughout.
- "Seven routes through"; page count "about eight hundred and fifty pages";
  sceptic's route (~70 pp) now includes `ch:inhabit`; **new engineer's
  route** (Grading → do → Realization → Instruments).
- Difficulty-gradient sentence on `ch:inhabit`.
- Unproved-claims list +5: `conj:global`/`conj:safe` (quantum, two views);
  `rs:rmk:bialgebra`; `cb:prin:width`; the price of a `do` (`do:sec:open`);
  `inh:obs:rsi`.
- `sec:quantum` paragraph rewritten to describe the v6 construction and the
  two views rather than the old weight-map story.

`pledge_related.tex`: two new paragraphs before Hyperon — *Verified
concurrency: Iris and CertiKOS* and *Causal inference: Pearl*.

Pledge files keep the book's British spelling, as before.

### 3. Bookkeeping: T9 and T10 closed

- **T9**: `turn_rholife_ch02.tex` l.1212 `Chapter~\ref{ch:weight}` →
  `ch:grading` (same chapter; cosmetic).
- **T10**: every hardcoded Roman part number in body prose replaced by a
  `\ref{part:…}`. Mapping used (checked against the concepts cited at each
  site, not the numeral):
  - GSLT chapters (`turn_gslt_ch02/03/04/05/10`): "Part III" →
    `part:machinery` (still III — but no longer load-bearing); ch02's "host
    language of Part IV" → `part:ecology`.
  - Science-bot chapters (`turn_sciencebot_ch01/03/05`): "Part I" (path
    amplitudes, vectorial account, decoration) → `part:physics`.
  - Origins chapters (`turn_origins_ch01/02/04/05/06`): "Part I" →
    `part:physics`; "Part II" (logical topology, coherence clusters,
    behavioral primes, nerve) → `part:sciencebot`. Two stale "(Section 6)" /
    "(Section 4)" paper-section pointers became `Section~\ref{sec:clusters}`
    / dropped.
  Note: `turn_hypercomp_ch0*.tex` also carry "Part I/II" but are **not
  input** by `finding_mind.tex`; left untouched.

### 4. Spelling sweep
`parcelled` → `parceled` (psy_ch02); `labelled`/`organises` → US
(gslt_ch03). Nothing else found in the shipped non-Pledge files.

## Manifest (`src/`, 21 files)

| File | Status | Change |
|---|---|---|
| `prestige_inh_ch01.tex` | **new** | *Inhabiting Choice* (`ch:inhabit`, `inh:*`) |
| `finding_mind.tex` | mod | `\input` of new chapter after psy_ch02; Part XI box sentence |
| `bibliography.tex` | mod | +13 (Inhabiting, Iris, CertiKOS, Herd, LarsenSkou91, GivanDeanGreig03, HumphriesDriver70, MaynardSmith82, SegerBrockmann87, Cohen66, Burnet59, Nakagaki00, Kirk24) → 303 |
| `prestige_ch03.tex` | mod | Kant opener hand-off; "three preceding chapters" |
| `prestige_psy_ch02.tex` | mod | closing hand-off sentence; spelling |
| `pledge_map.tex` | mod | T5: figure, routes, counts, unproved list, quantum §, engineer's route |
| `pledge_related.tex` | mod | T5: Iris/CertiKOS and Pearl paragraphs |
| `turn_rholife_ch02.tex` | mod | T9 |
| `turn_gslt_ch02/03/04/05/10.tex` | mod | T10 (+ 2 spellings in ch03) |
| `turn_origins_ch01/02/04/05/06.tex` | mod | T10 |
| `turn_sciencebot_ch01/03/05.tex` | mod | T10 |
| `finding_mind.pdf` (top level) | — | preview, 863 pp |

Suggested commit layout, as for draft27–29: `FindingMind/draft30/src/*.tex`,
`FindingMind/draft30/INTEGRATION-NOTES-28.md`,
`FindingMind/draft30/finding_mind.pdf`.

## Tracked inconsistencies

| # | Item | Status |
|---|---|---|
| T1 | Grading chapter forward-refs to do chapter | closed (draft28) |
| T2 | Mortal Scientist hypotheses as clauses | closed (draft28) |
| T3 | Composing Learners stale "canonical section" | closed (draft28) |
| T4 | Grading → Realization forward-ref | closed (draft29) |
| T5 | Front matter (map, routes, counts, unproved list, related work) | **closed (draft30)** |
| T6 | Ecology box / "the ecology part" wording | closed (draft29) |
| T7 | do chapter → Realization forward-ref | closed (draft29) |
| T8 | Inhabiting Choice hand-off from psy_ch02 / Kant | **closed (draft30)** |
| T9 | `ch:weight` vs `ch:grading` cosmetic ref | **closed (draft30)** |
| T10 | Hardcoded Roman part numbers in body prose | **closed (draft30)** — hypercomp files (not input) untouched |

Nothing open. Two things worth keeping an eye on, not inconsistencies:

- The two-views stance on the quantum chapter (reductions carry coherence vs
  unitary-in-equations) is deliberately **not** reconciled; `conj:global` and
  `conj:safe` sit in the unproved list.
- The price of a `do` (`do:sec:open`) and the GPU width thesis
  (`cb:prin:width`) are stated as owed, with Saph1r3 described as nascent.

## Verification performed
- 3× pdflatex on the overlay; same on a clean room rebuilt from
  `FindingMind/draft*/{,src/}` + this directory; `pdftotext` outputs identical.
- Label existence checked for every label newly cited from the Pledge.
- Relative-position prose grep ("preceding/next chapter/part") on changed
  files; all accurate against the current `\input` order.
- US-spelling scan with the false-friend list on all shipped non-Pledge files.

## Next
Site regeneration (findingmind repo) from this build: parameterize
`_build/convert.py` / `gen.py` paths, add a Realization landing section, fix
the FormSubmit placeholder (needs your activation hash), the Figure 23.2
`[diagram]` placeholder, the hard-coded "seven hundred numbered" count, stale
`HOSTING.md`, move v1–v5 out of the published tree, and the contents anchor
links.
