# INTEGRATION NOTES 20 — draft22

The nomenclature and spelling sweep, rebuilt and finished. Also completes draft21, which
was committed from my first cut and is two files short.

**708pp — unchanged.** 0 errors. 0 undefined references, 0 undefined citations, 0 multiply
defined labels. **The same 8 overfull boxes, at identical widths and identical line numbers**
as the draft21 build. The sweep is exactly page-neutral.

Clean-room verified: repo baseline + draft2..draft21 + this delta.

**40 files.** 30 in The Turn, 5 in The Prestige, 1 Interlude, plus 4 supersedes of draft21
chapters. No deletions, no new macros, no bibliography change.

## 0. First, the thing you asked me to check

You were right that I should look. I searched every `draft*` directory for copies of the
swept files. **They are not there.** Older copies of most of them exist — `turn_rholife_ch01`
in drafts 2, 6, 10, 11, 12, 16 and 17, `turn_causality_ch07` in drafts 4 and 7, and so on —
but every one of those predates draft18 and is pre-sweep. The only genuinely post-sweep files
anywhere in the repo are `draft18/turn_rholife_ch04.tex` and the draft20/draft21 deliveries.

So the sweep was rebuilt from scratch, for the second time, from the rules recorded in
INTEGRATION-NOTES-16.

## 1. What the rebuild found that the original sweep missed

This is the part worth your attention, because it means the sweep was never only a commit
problem.

Both earlier passes built their word list "from forms actually present" — which is the right
method, but only as good as the pattern used to enumerate. Checking with a broader pattern
turned up British forms **in files draft20 successfully committed**, which no lost delivery
can explain:

`prestige_ch01` (decentralised, idealisation, symbolised, vocalises) · `prestige_ch02`
(amortised, parameterises, unlabelled) · `prestige_agy_ch01` (canalised, homogenises,
relativised) · `prestige_apr_ch01` (fertilise, relativised, relativises) ·
`prestige_chs_ch01` (Mechanisation) · `turn_causality_ch01` (internalised) ·
`turn_suffering_ch01` (unravelling) · `turn_origins_ch07` (relativise)

Also missed everywhere, and now fixed: **internalising / internalisation** (9 occurrences) —
which is unfortunate, since it is a load-bearing word in the chapter this whole sequence of
passes was about.

The rebuilt list is larger than either predecessor: it includes `decentralisation`,
`factorisation`, `optimisation`, `quantised`, `symmetrised`, `amortised`, `memoised`,
`tropicalisation`, `parametrisation`, `uniformisation`, `mathematisation`, `synchronisations`,
`relativise`, `trivialises`, `localises`, `idealised`, `hybridise`, `metabolise`, `equalise`,
`renormalised`, `unsymmetrised`, `parenthesised`, `harbour`, `labour`, `unravelling`,
`relabelling`, `dialled`, `travelling`, `unlabelled`.

## 2. What was done

**Pass 1 — 29 files, 325 substitutions.** The never-committed backlog.
**Pass 2 — 15 files, 31 substitutions.** The forms both earlier lists missed, applied across
The Turn, The Prestige, and the Interludes including files that had already landed.

Masked throughout, so nothing inside them was touched: arguments of `\label`, `\ref`,
`\autoref`, `\cite`, `\input`, `\texttt`, `\verb`, `\lstinputlisting`, and every `lstlisting`
and `verbatim` block. `bibliography.tex` untouched — cited titles keep their authors'
spelling. The Pledge untouched, per your decision that it is yours.

**engine → enzyme, 25 technical sites in five files** — `turn_causality_ch07` (11, including
the section heading), `turn_gslt_ch09` (9, including the chapter title), `turn_origins_ch09`
(3), `turn_rholife_ch05` (1), `turn_causality_ch10` (1). **Chapter 15 now prints as
"Conversion, Enzymes, and the Virtual Token"** and §30.x as "Enzymes, conversion, and the
virtual token."

**colour, split by sense as before.** Token sense → *flavor* in `turn_gslt_ch09`,
`turn_causality_ch07`, `turn_origins_ch09` (9 sites). Board-game sense → *color* in
`turn_rholife_ch02` and `turn_rholife_ch03` (6 sites), where it means which side moves first.

**Verified surviving, as intended:** the ordinary-English "reduction engine" in
`turn_gslt_ch02`; "the shared engine" in `prestige_chs_ch01`; the two literary engines in
*The Permitted Say*; the label `eng:def:engine`; `eng:rem:nomenclature`, which discusses the
word on purpose; `sci:prop:sexdefence`, which keeps its British spelling in the label;
`engine.rho` and `engines.py` as filenames; `analogue` and `dialogue`, kept per the draft18
decision.

**Residual after both passes, outside The Pledge: zero.**

## 3. draft21 was committed from my first cut

The committed `draft21/` holds 8 files. The corrected delivery was 10. Missing:

- `turn_conclusion_ch01.tex` — contains "The part closes by observing that such a learner
  holds information of a shape that makes a choice meaningful", which the split made false.
- `turn_origins_ch01.tex` — the Part VI opener, still naming the hypercomputational tower
  among the preceding parts.

Both are in this delta. Four committed draft21 chapters are also superseded here, and two of
them for structural reasons rather than spelling:

- `turn_causality_ch01.tex` — the committed copy lacks the §`sec:physics-ladder` caveat that
  the part's last chapter is not a rung.
- `turn_entropy_ch01.tex` — the committed copy still says "the previous chapter" in
  `ent:rmk:thermostat`, where the previous chapter is now *Synthesis*.
- `turn_origins_ch07.tex`, `turn_suffering_ch01.tex` — spelling only.

`finding_mind.tex`, `bibliography.tex`, `pledge_map.tex` and `pledge_related.tex` as committed
are current and are **not** resent.

## 4. The deferred repair, now unblocked

`turn_rholife_ch01`'s opening sentence read **"The preceding parts asked what a physics is and
why a scientist inside one would be misled."** Ecology is Part II; both of those parts come
after it. I reported this last round and did not ship it, because shipping a pre-sweep copy
would have collided with the sweep. That reason is gone — the file is in this delta — so the
repair is made. It now credits Part I with the apparatus and says the parts that follow take
the scientist for granted, which preserves the original's point and reverses its direction.

Page-neutral: the paragraph regrew to the same depth.

## 5. What this leaves

- The Pledge is still British-spelled. `pledge_related` alone carries 26 forms. The book is
  mixed-orthography by your decision, and that is the one remaining thing a reader could
  notice.
- Two labels are now permanently misleading and are left alone deliberately, as before:
  `eng:def:engine` names Definition 24.2 *Enzyme*, and `eng:coloured-learner` names *The
  flavor of a learner*. Also `part:choice`, which since draft21 names the Suffering part.
- The `rho-life` note, `engine.rho`, `engines.py` and the deck still say engine and colour.
  Book and note continue to diverge on nomenclature.

## 6. One process note

Both the draft18 delivery and the draft20 reconstruction were lost between my handing them
over and their landing in the repo — 44 files and then 30. draft21 landed short by two. I
don't know what the mechanism is, and I can't see it from here. But the pattern is that
**large deltas lose files and small ones arrive intact**, which is worth knowing before the
next big pass. If it helps, I can keep future deliveries under some size, or ship a manifest
file whose only job is to be diffable against the directory after you commit.

## Files

```
SWEEP — The Turn (30)
  turn_gslt_ch01 ch02 ch03 ch05 ch06 ch07 ch08 ch09 ch10 ch11 ch12
  turn_scope_ch01
  turn_rholife_ch01 ch02 ch03 ch05
  turn_causality_ch05 ch06 ch07 ch08 ch09 ch10 ch11
  turn_sciencebot_ch03 ch06
  turn_origins_ch02 ch06 ch09
  turn_conclusion_ch01*   turn_origins_ch01*      (*missing from committed draft21)

SWEEP — The Prestige (5)
  prestige_ch01 ch02 prestige_agy_ch01 prestige_apr_ch01 prestige_chs_ch01

SWEEP — Interludes (1)
  int_foam_born

SUPERSEDES draft21 (4)
  turn_causality_ch01   ladder caveat + spelling
  turn_entropy_ch01     ent:rmk:thermostat fix + handoff fix
  turn_origins_ch07     spelling
  turn_suffering_ch01   spelling
```
