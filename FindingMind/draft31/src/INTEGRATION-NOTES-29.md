# Integration notes — draft31

**The four origin-of-intelligence stories, told twice: a confessional chapter closing the Pledge, and a formal chapter in the Prestige reading the same material as three pressures on one ledger. 863 pp → 891 pp. 0 errors, 0 undefined references, 0 undefined citations, 0 multiply-defined labels. Same 9 overfull boxes as draft30 at identical widths.**

Built against the repo as committed through draft30 (reading `draftN/`, `draftN/src/` and draft30's `src/src/`). Baseline verified first: repo + draft2–draft30 reproduces draft30's 863 pages, 9 overfull boxes, 0 undefined. Clean-room verified after: repo + draft2–draft30 + this directory → 891 pp, `pdftotext` output byte-identical to the working overlay.

Source for both chapters: Max Fisher, *Humans' origin story is being radically revised* (The Bigger Picture, Nov 2026, 49:50), its transcript, its published sources list, and the primary literature behind each story (reference compilation of 8 Nov).

---

## Manifest (6 files + this note + PDF)

Diff this table against the directory after committing. Word counts are `wc -w` on the delivered file.

| File | Destination | Status | Words | md5 (first 8) |
|---|---|---|---|---|
| `pledge_ch06.tex` | `ThePledge/` | **new** — ch. 6, *Four Stories, and the One i Was Already Telling* | 3228 | ad86bf7c |
| `prestige_stories_ch01.tex` | `ThePrestige/` | **new** — ch. 72, *Four Pressures, One Ledger* | 5577 | 6ed43906 |
| `pledge_map.tex` | `ThePledge/` | 3 sites — page count, difficulty gradient, unproved-claims list | 3766 | e1ebb162 |
| `turn_sciencebot_ch02.tex` | `TheTurn/` | 1 line — `\label{prop:isolation-closure}` added to *Isolation Licenses Closure* | 400 | f8f1b815 |
| `finding_mind.tex` | root | 2 `\input`s; one clause added to the Part XI opener box | 7754 | 049f00a6 |
| `bibliography.tex` | root | +57 entries (303 → 360) | 6775 | 06510daa |
| `finding_mind.pdf` | — | rebuilt, 891 pp | | |

No files deleted. No chapter moved. No macros or environments added.

**Numbering.** One chapter inserted in the Pledge at position 6, so every numbered chapter from the Turn on shifts +1 (Mortal Scientist 26, Suffering 32, Complex Amplitudes 39). One chapter inserted in Part XI at position 72, between *On Lying as the Origin of Meaning* (71) and *If This Is a Simulation, Which Kind* (73); Kant is 78. No hardcoded chapter numbers exist in the source, so this is `\ref`-safe. INTEGRATION-NOTES-28's chapter numbers are now off by one from ch. 6 and by two from ch. 72.

Suggested commit layout, as for draft27–30: `FindingMind/draft31/src/*.tex`, `FindingMind/draft31/INTEGRATION-NOTES-29.md`, `FindingMind/draft31/finding_mind.pdf`.

---

## Decisions applied

You asked for the draft without answering the six placement questions, so the recommended option was taken in each case. Every one is a one-line change if you want the other.

1. **Pledge placement:** after ch. 5 *Inside and Outside*, before the map. Reason: ch. 5 ends on the fear of a narrative that merely reflects your interior; ch. 6 is the exterior checking in. The map's opening sentence ("The confessional part of the Pledge has just ended") stays true.
2. **Prestige placement:** between `ch:lie` and `ch:sim`. The chapter uses Goodman and the lie (narrative 3's deception strand) and discharges `sec:soluble`'s "not whether but how"; it precedes simulation. Alternative (directly after `ch:trick`) would put it ahead of both things it leans on.
3. **The Pledge chapter is a full draft in your voice**, for you to cut. Lowercase i, "Dear Reader", British spelling, zero `\cite`, zero `\footnote`, attributions inline — matching ch. 1–5. It is yours; everything in it is revertible, and three places are flagged below as the ones most worth your eye.
4. **Fisher is named inline in the Pledge** (as Goertzel and West are) **and cited in the Prestige** (`fisher2026`, the video), alongside the primary literature.
5. **Sequencing:** delivered now as draft31, on top of draft30, rather than folded into an earlier delta.
6. **Titles** as proposed.

---

## ch. 6 — *Four Stories, and the One i Was Already Telling* (`chap:pledge-ch5`, `sec:pledge-stories`)

Label follows the file's existing off-by-one convention (`pledge_ch05` = `chap:pledge-ch4`). The section label does not continue the Substack `sec:pledge-N` numbering since there is no corresponding post.

Six sections, ~3,200 words (ch. 5 is ~1,150; ch. 2 is ~3,300):

1. **A Sunday Evening** — the confession: the schoolbook story was the one in your head; the video; the week of reading.
2. **The Four Stories** — one paragraph each, in Fisher's register (the monolith and its two objections; the nine-month-old and the "we"; the fox farm, the execution hypothesis, the tyranny of cousins; Jebel Irhoud, cassava, density, the collective brain).
3. **What i Found When i Looked Back** — the recognition scene, with `\ref`s to §I (the whale = story 2; the tribal ceiling = story 4), §II (coalitions in the wetware = story 3), §III (models of models = the social brain), §VII (the Goertzel density argument = Henrich's bottleneck, "two ends of one rope"). Closes: "Convergence is not confirmation. But it is the thing i was afraid i would not have: an outside."
4. **The Monolith Was in the Maths Too** — the confessional version of the Prestige reveal: the one place the formalism assumes something at no cost is the same move the schoolbook makes. Promissory; names no label.
5. **Byproduct** — González-Forero's spandrel claim handed to the reader now, as the empirical twin of the budget-constraint pivot; "If they are wrong about it, i am probably wrong about it too."
6. **Which Story i Want to Be True, and Why That Is a Bad Reason** — the preference for story 3 conceded; the three stories as three line items on one ledger; the reading instructions ("when you meet the mortal scientist, think of the ape on the savanna…"); Fisher's AI question deferred to the Prestige.

**Three things flagged for your eye, in your own chapter:**

- §1 says "i asked a bot to gather the literature behind all four." True, and in the spirit of the interludes and `ch:psy`, but it is a sentence about your process that you did not write. Delete at will; nothing depends on it.
- §4 and §5 make two claims about your own history — that the monolith was the story in your head while building the framework, and that you wish you had had the byproduct sentence when you started. Both follow from what you told me ("i'm familiar with the first") but they are stated more strongly than you stated them.
- §6's line "i was raised on it" closes the chapter's confession. It is the kind of sentence that is either exactly right or too much, and only you can tell which.

Biographical content is otherwise limited to what is already in the book (the Goertzel conversation, the sections cited). No new names, dates or places about you.

---

## ch. 72 — *Four Pressures, One Ledger* (`ch:stories`, namespace `st:`)

Lowercase-i register (matching the native Part XI chapters). US spelling. ~5,600 words, 8 sections.

**Named objects:** `st:prin:schema` (pressure/trigger/class) · `st:obs:byproduct` · `st:obs:monolith` · `st:obs:join` · `st:obs:coalition` · `st:obs:resolver` · `st:rem:machiavelli` · `st:obs:nonrival` · `st:obs:ratchet` · `st:obs:density` · `st:obs:variability` · `st:conj:flicker` · sections `st:sec:schema/monolith/we/schemers/hivemind/ledger/ai/claims`.

**The spine, and what each piece rests on in the book:**

- §72.1 The schema. `st:prin:schema` states the reading rule. `st:obs:byproduct` identifies González-Forero's spandrel with the ledger premise, citing `ch:sci`, `suf:prop:derived`, `sec:kant-affordable`. Their caveat (no comparison against a direct-selection model) and their 2018 model's different answer are both stated.
- §72.2 **The monolith is the angle brackets** (`st:obs:monolith`). Aiello–Wheeler = `sci:lem:exposure` (closed term, positive burn rate, dies); "who does the first talking ape talk to" = the `comm` rule fires on a pair or not at all; the monolith and `prop:isolation-closure` make the same move, and the objections to one are `obs:social`'s objections to the other. The dispersal evidence is kept and handed to story 4.
- §72.3 **We: the join is the trigger** (`st:obs:join`). Shared intentionality = a join pattern whose body neither party fires alone; Herrmann 2007's gap = interaction rules, not term language; the pressure = the funding gate closing on single-party rewrites; `com:obs:amort` (cooperation is amortized perception) and the dyad→population levels of `ch:com`. **The disagreement is used:** `com:prop:nomet` (no metabolic mutualism) vs. Tomasello's richer foragers, reconciled by `com:obs:two` — the surplus of a shared hunt is epistemic, not metabolic. The calculus has no opinion on dates, one on order: the join precedes anything a population does to its members.
- §72.4 **Schemers: the coalition can starve you.** Two strands kept apart. *Self-domestication:* `st:obs:coalition` (execution = forcing a debit the target cannot cover; needs no new primitive); `st:obs:resolver` (**selection against reactive aggression revises the resolution algebra, not the term language** — `gw:prin:aperture`, `gw:def:algebra`; support adequacy `gw:cor:adequacy` keeps it one species of computation; and the reading *predicts gracilization as relaxation*, which is what the byproduct premise wants). DeSilva/Villmoare and Lord et al. named as live objections. *Machiavellian:* `st:rem:machiavelli` (modeling a modeler is quoting a quoter; Pledge §III's explosion; `sec:kant-affordable`'s modal depth as price); DeCasien 2017 named against the social brain; then the hand-off to `ch:lie` — a coalition is an audience that can impose the receiver's expense, so the tyranny of cousins is the condition under which Benedikt's mechanism has teeth.
- §72.5 **Hivemind: culture is quoted code, and density is compactness.** `st:obs:nonrival` (= `com:obs:two`; a practice is `@P`, grade-three access, non-rival, *editable* because reflection); `st:obs:ratchet` (= `sec:assembly-depth`'s two axes, `thm:ai-bound`, `rmk:depth-width`, `scm:prop:frontier` — Tasmania is the remark run as history); `st:obs:density` (= `conj:degradation` + `sec:topology`'s compactness + `rmk:consensus-missing`'s budget; the Pledge §VII clusters and Powell–Shennan–Thomas's thresholds as two ends of one rope); `st:obs:variability` (**Potts's window is the hypothesis of `gw:prop:plastic`**: a gene is a rewrite rule, a practice is a datum the clause reads, `gw:rmk:state-is-term`). Vaesen 2016 and the Diamond axis as caveats.
- §72.6 **Three pressures, one ledger.** The three stories as three coordinates of `com:sec:typed`'s overlap vector: funding gate / resolver / transmission. The genuine conflict located (self-domestication lowers variance, the ratchet needs it) and left as a trade-off. `st:conj:flicker` stated with its falsifiers.
- §72.7 **What this changes about the thing we are building.** Fisher's three AI verdicts sharpened: no population-controlled resolver → no selecting pressure for social cognition (extends `sec:blind-spot-returned`); the join is cheap and is not the brain; and the one i would most want you to read — *a system that is a collective brain by inheritance and not by membership is a Tasmania*: it inherits the depth of the population that wrote the corpus without the width that sustains it (`st:obs:ratchet`, `psy:prin:lapse`).
- §72.8 Claims / not claimed, in the book's form; convergence not confirmation, with the `suf:rem:solms` discipline cited.

**Things in this chapter that are mine and load-bearing, for your eye:**

1. `st:obs:resolver` — placing self-domestication in the resolution algebra rather than the term language. i think this is the best thing in the chapter and it is not in any source note.
2. `st:obs:variability` — Potts's rate window as the hypothesis of `gw:prop:plastic`. Clean if right; it makes the graded clause the formal content of "culture is faster than genes".
3. The Tasmania reading of corpus-trained systems in §72.7. It follows from `rmk:depth-width` but nobody has said it.
4. `st:conj:flicker` — a simulation spec, not a result. Every object in it exists in the book; the composite has not been built.
5. `st:obs:coalition` leans on `ch:chs` for "a coalition that can force an outcome"; `ch:chs` mentions Coalition Logic in its connections section rather than developing it. The observation is stated so that it needs only the forcing idea, but the citation is thinner than the others.

**Scope note honoured:** most content is stated as `observation` (a reading) with one `conjecture`; nothing is stated as a theorem. The empirical disputes in each literature are named in the section that uses the literature.

---

## Lighter sites

- **`turn_sciencebot_ch02.tex`** — the proposition *Isolation Licenses Closure* had no label (Chapter~\ref{ch:trick} refers to it by chapter). Added `\label{prop:isolation-closure}` so `st:obs:monolith` can name it. One line; nothing else in the file touched (it is a draft30 T10 file; the committed copy is the base).
- **`finding_mind.tex`** — two `\input`s; Part XI opener box gains one clause ("what the four stories anthropology tells about the origin of human intelligence look like when each is read as a pressure on one ledger"). The Pledge box is unchanged; its existing text already covers the new chapter.
- **`pledge_map.tex`** — "About eight hundred and fifty pages" → "Nearly nine hundred pages"; difficulty gradient gains a sentence on `ch:stories` (assumes nothing a reader of the Pledge's last chapter lacks; states each cited Ecology result in a sentence before using it); unproved-claims list +1 (`st:conj:flicker`, with the convergence-not-confirmation caveat). British spelling kept, as before.
- **`bibliography.tex`** — +57, none colliding (checked by key and by author: no Tomasello, Wrangham, Henrich, Potts, Boyd, Diamond, Hublin, Aiello, Harari, McBrearty, Boehm or Dunbar entries existed). Two verified live this session (`gonzalezforero2025`, DOI 10.1098/rstb.2024.0114; `tomasello2025`, oecs.mit.edu/pub/sep9e3c2); the rest are standard references entered from knowledge and worth a pass before print. `thieme1997` keeps the author's "Palaeolithic".

---

## Verification performed

- 3× pdflatex on the overlay; clean room from repo + committed deltas + this directory; `pdftotext` identical (891 pp both).
- Overfull set: the baseline nine at identical widths (diffed).
- Every `\ref` and `\cite` in the two new chapters resolves; `grep` for "previous/next/last chapter", "chapter before/after" in the new files and their neighbours (`pledge_ch05`, `pledge_map`, `prestige_lie_ch01`, `prestige_sim_ch01`) — all accurate against the new `\input` order. The lie chapter's closing "There is a reason this chapter sits where it does" and the sim chapter's opening reference to `ch:notation` both remain true.
- US-spelling scan with the false-friend list on `prestige_stories_ch01.tex` (three fixes applied: paleontologist, Paleolithic, program). Pledge file is British by design.
- Rendered and inspected the opening page of each new chapter.

## Open

- Whether the Pledge chapter's §2 story summaries are too long for your taste now that the Prestige retells them; the two were written to be read three hundred pages apart.
- `sec:kant-affordable` still gathers three instances; `st:rem:machiavelli` cites it as pricing political depth but does not add a fourth. Same deliberate non-edit as draft23's `ch:psy2`.
- Site regeneration (findingmind repo) from this build — the draft30 list stands, plus two new chapters to route.
