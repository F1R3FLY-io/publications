# INTEGRATION-NOTES-16 — draft18

**The nomenclature pass: engine → enzyme, color → flavor, and US spelling throughout.**

Applies on top of draft17. Terminology and orthography only — no argument changed,
no result stated, demoted or withdrawn.

Build: repo baseline + draft2..draft16 + draft17 + this delta, three `pdflatex` passes.
**680pp** (unchanged), 0 errors, 0 undefined references, 0 undefined citations,
8 overfull boxes — the same 8, unmoved. Verified in a clean room, not from the working
tree.

The page count holding at 680 across ~600 substitutions plus a new one-page remark is
luck, not design; nothing was tuned to preserve it.

**49 files.** 42 in The Turn, 3 in The Prestige, 3 Interludes, `finding_mind.tex`, and
one file in The Pledge — see the carve-out below.

---

## Decisions carried in

| # | decision | effect |
|---|---|---|
| 1 | nothing invisible to the reader changes | all labels, `\chEng`, `ch:eng`, the `eng:` namespace, and TikZ color names left alone |
| 2 | follow US practice, not a mechanical -our/-ise rule | `analogue` and `dialogue` **kept** (both standard in American academic prose; *analog* reads as electronics); `catalogue → catalog` |
| 3 | The Pledge is yours | `ThePledge/` excluded from the spelling sweep entirely |
| 4 | book scope only | `rho-life/engine-note` still says *engine* and *colour*; so do `engine.rho`, `engines.py`, and the deck |
| — | ahead of R2 | R2's four sites are untouched and unmoved |

---

## The rename

### engine → enzyme

**133 technical occurrences** across ch24 (102), ch29 (11), ch15 (8), ch45 (3), ch63 (3),
ch32, ch25, and `pledge_related`. Chapter title is now **"Enzymes and Individuals."**

**Seven left alone, deliberately** — every one an ordinary English use, not the technical
object: *engineer / engineering* (ch21 ×2, `pledge_ch03`, `prestige_ch02`), *economic
engine* (`pledge_ch02`), *reduction engine* (ch7), *the shared engine* (ch49), and the
two literary uses in "The Permitted Say" — the enormous engine of wanting, the galaxy as
one great cooling engine. Those last two are the nineteenth-century register used on
purpose, and renaming them would have been the joke landing on the wrong side.

The filenames `engine.rho` and `engines.py` stay as they are, since they name artifacts
in the repo.

### colour → flavor

**~45 sites.** Definition 24.1 is now **"The flavor of a learner"** rather than
"Flavored learner" — the adjectival form had the same awkward shape as the phrase it
replaced. Say if you want the adjective back; it is one line.

ch23 and ch22's board-game sense — *symmetrized over color*, *the color asymmetry*,
*from either color* — stays as **color**, US-spelled. That is the ordinary sense of the
word and the rename actually frees it up.

`\cite{CARho}`'s **monochromatic limit** survives unchanged and still attributed, since
it is their term for their result. The book's own word for that case would be
*single-flavor*, but nothing in the text needed one.

### The nomenclature remark — `eng:rem:nomenclature`, ch24 §3.1, p345

Written per your instruction, immediately after the enzyme is defined. Four paragraphs:

1. **The flirtation, and the payoff conceded in full.** Reservoirs, rate, dissipation,
   efficiency — and the specific win, that `eng:ecological-pyramid`'s one-in-ten trophic
   ratio is a toll of θ = ln 10, which is a thermodynamic statement and which *engine*
   makes audible.
2. **Why it was dropped.** The payoff is the nineteenth century's and the furniture comes
   with it whether or not it was sent for. The book's move is that the ledger is
   informational before it is thermal — erasure is what costs, θ is a rendezvous toll
   rather than a temperature — so naming the object after a steam engine imports a
   settled picture at exactly the point the reader should be unsettled. **Then the
   laudanum callback**: the interlude opening The Turn has its narrator distrust
   *inspiration* for smelling of the nineteenth century, of laudanum and séance; *engine*
   smells of the same decade and is worth distrusting for the same reason.
3. **Why enzyme is the better word on the merits, not just the safer one.** Substrate
   specificity; persistence while the binding is linear, which is
   `eng:persistence-load-bearing` verbatim; and rate as the single distinguishing degree
   of freedom, which is the only thing separating the fourth row of `eng:tab:profiles`
   from a source and from prey.
4. **What it surrenders** — Coase, mycorrhizal trade, the financial combinators — kept as
   analogies, on the grounds that a reader just told tokens have flavors is not helped by
   being told they are burned.

**The Wegscheider claim is in and flagged as ours.** In a reaction network the
no-arbitrage condition around a cycle is the Wegscheider condition for detailed balance,
and the potential whose existence it is equivalent to is a chemical potential — so the
virtual token has a second name and chemists have had it since 1901. The remark says in
its own text that this has not been checked against the chemical-kinetics literature and
should be read as a conjecture with a citation attached. If it survives your eye it is
worth more than the rename: it gives `\cite{vtok}`'s central theorem a second reading in
a literature that is not ours.

**Not done, offered:** the *engine graph* Γ is now the *enzyme graph*. In biochemistry
the natural name is a **reaction network**, and that would make the Wegscheider point
land without explanation. It is a further rename with its own cross-references, so I left
it. Worth a decision.

---

## The spelling sweep

Roughly 600 substitutions. Built from the words **actually present in the source**, not
from a rule, so the traps are safe by construction: *precise, premise, otherwise, revise,
exercise, promise, enterprise, disguise, pairwise, componentwise, surprising, arise,
noise* and the rest of the false-positive family are untouched (82 verified present
afterward).

Changed: `-our → -or` (behaviour 84, neighbour 16, labour, favour, harbour) ·
`-ise/-isation → -ize/-ization` (~90 forms: realise 36, characterise 13, factorisation,
normalisation, optimisation, generalise, internalise, decentralised, quantised,
memoised, tropicalisation, …) · `-lling → -ling` (modelling 25, labelling 13,
relabelling, unravelling, travelling, marvellous) · `-re → -er` (centre 15, fibre 17) ·
`-ce → -se` (defence 10, offence) · and *programme → program* (29), *grey → gray*,
*analyse → analyze*, *catalogue → catalog*, *judgement → judgment*,
*acknowledgement → acknowledgment*, *manoeuvre → maneuver*, *towards → toward*,
*whilst → while*, *amongst → among*, *sceptical → skeptical*.

**Protected during the sweep**, by masking before substitution: the argument of every
`\label`, `\ref`, `\cite`, `\input`, `\texttt` and `\verb`; every `lstlisting` and
`verbatim` block; every LaTeX color command including `\definecolor{grey}` and
`pattern color=gray`; and `bibliography.tex` in its entirety, so cited titles keep the
spelling their authors used.

Consequences of decision 1 worth knowing about, since the source now reads oddly in
three places:

- `\label{eng:coloured-learner}` names a definition titled *The flavor of a learner*.
- `\label{eng:def:engine}` names **Definition 24.2, Enzyme**.
- `\label{sci:prop:sexdefence}` keeps its British spelling.

All three are invisible in print. They are listed here so that a future grep for
"coloured" or "engine" in the source does not read as a missed substitution.

### The one Pledge file

`pledge_related.tex` line 77 said *"colours, engines, and the virtual token are a theory
of how…"*. That is not spelling, it is a list of objects that no longer exist under those
names, so it was changed. Nothing else in `ThePledge/` was touched — no `-our`, no
`-ise`, no *programme*. The confessional chapters and `pledge_map` are entirely as you
left them, so the book is now mixed-orthography until you run your own pass. **This is
the one thing in draft18 that a reader could notice**: The Pledge says *behaviour* and
The Turn says *behavior*.

---

## Open

- **The Wegscheider identification.** Flagged in its own text, but it is the one new
  claim in an otherwise mechanical pass.
- **Reaction network** for Γ, if you want the biochemical register carried all the way.
- **The rho-life note** and the deck still say engine and colour. Book and note now
  diverge on nomenclature as well as on content.
- **Mixed orthography** until The Pledge is swept.
- **R2 is next**, unchanged and unmoved: `sciencebot_ch05`, `hypercomp_ch05`,
  `origins_ch05`, and `rmk:reversal-status` in ch30.
- Notes 1–15 and `five_perspectives.tex` are now stale on both chapter numbers and
  nomenclature.
