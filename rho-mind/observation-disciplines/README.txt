Observation Disciplines --- an omnibus
L. G. Meredith, F1R3FLY.io

BUILD
  pdflatex omnibus.tex   (run three times for the table of contents,
                          cross-references and page numbers to settle)
  No bibtex; the bibliography is a manual thebibliography in omnibus.tex.
  Requires: geometry, amsmath/amssymb/amsthm/mathtools/stmaryrd, mathpartir,
  tikz (arrows.meta, positioning, fit, backgrounds), enumitem, booktabs,
  array, listings, microtype, hyperref, fontenc[T1].
  Built and verified with pdfTeX 3.141592653-2.6-1.40.25 (TeX Live 2023).
  Clean: 0 errors, 0 undefined references, 0 overfull boxes, 0 underfull boxes.

FILES
  omnibus.tex          master: preamble, front matter, \input list, bibliography
  part0.tex            Part 0  --- The argument in ten pages
  part1.tex            Part I  --- Generation
  part2.tex            Part II --- Four containers, four logics
  part3.tex            Part III --- Placement
  part4.tex            Part IV --- Grading
  part5.tex            Part V  --- Price
  part6.tex            Part VI --- The range, worked
  part7.tex            Part VII --- Learning, and the fractal
  part8.tex            Part VIII --- Shown, not shown, open
  tablecollected.tex   the collected-measurements tabular, \input twice
                       (Part 0 sec 0.8 and Part VI sec 35) so the two copies
                       cannot drift apart
  omnibus.pdf          the built document, 68pp

NOTATION RULES OBSERVED
  The reflective higher-order calculus is written "rho calculus", never with
  the Greek letter. Quotation is @P and dereference is *x; corner quotes are
  not used.

FIGURES (all TikZ, no external assets)
  1  the primitivity axis: semiframe / frame / quantale     (Part II)
  2  the four dials                                          (Part 0)
  3  the hypothesis fibration                                (Part VII)
  4  the medium tower                                        (Part VII)
  5  two containers, one learner                             (Part VII)
