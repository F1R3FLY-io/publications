#!/usr/bin/env python3
"""
choicesim.py -- computations backing "Choice Principles as Graded Where-Clauses".

Everything the note reports as a number is produced here. Five experiments:

  E1  attainment: a graded clause is a quantifier; a resolver is a selection
      function; the two are related by phi-bar(p) = p(eps(p)).  Verified for
      Bool, R>=0, Viterbi (max,x) and tropical (min,+).

  E2  the product: Escardo-Oliva's binary product of selection functions,
      computed against the sequential chaining of two graded receipt sites
      through the graded modality.  They must agree.

  E3  LLPO: a two-candidate race whose crisp guards are complementary
      Sigma^0_1 approximations.  The clause's support is never empty, but
      every stage-uniform selector is defeated by a diagonal instance.

  E4  fairness: the bootstrap trap and its repair by psi (+) eps.

  E5  the interference deficit: supp is a proper subset of Reach for a
      C-valued clause, and no R>=0 clause reproduces it.

No dependencies beyond the standard library.
"""

from itertools import product as cartesian
from fractions import Fraction
import cmath

SEP = "=" * 72


# ---------------------------------------------------------------------------
# Resolution algebras.  (V, oplus, otimes, zero, one) plus, where it exists,
# a definable selection function "sel" picking a candidate the quantifier
# attains.  sel is None when no definable section exists.
# ---------------------------------------------------------------------------

class Algebra:
    def __init__(self, name, oplus, otimes, zero, one, sel=None):
        self.name, self.oplus, self.otimes = name, oplus, otimes
        self.zero, self.one, self.sel = zero, one, sel

    def total(self, values):
        acc = self.zero
        for v in values:
            acc = self.oplus(acc, v)
        return acc


BOOL = Algebra("Bool", lambda a, b: a or b, lambda a, b: a and b, False, True,
               sel=lambda vs: next((i for i, v in enumerate(vs) if v), None))

RNN = Algebra("R>=0", lambda a, b: a + b, lambda a, b: a * b, 0.0, 1.0,
              sel=None)  # a section needs randomness; see E1 commentary

VITERBI = Algebra("Viterbi", max, lambda a, b: a * b, 0.0, 1.0,
                  sel=lambda vs: max(range(len(vs)), key=lambda i: vs[i]))

INF = float("inf")
TROPICAL = Algebra("tropical", min, lambda a, b: a + b, INF, 0.0,
                   sel=lambda vs: min(range(len(vs)), key=lambda i: vs[i]))

CPLX = Algebra("C", lambda a, b: a + b, lambda a, b: a * b, 0j, 1 + 0j,
               sel=None)


# ---------------------------------------------------------------------------
# E1.  Attainment.
#
# A clause at a receipt site denotes psi : Cand -> V.  Read as a quantifier in
# the sense of Escardo-Oliva it is
#     phi-bar(p) = SUM_{c in Cand} psi(c) (x) p(c)          (K-monad)
# A resolver is a selection function
#     eps : (Cand -> V) -> Cand                             (J-monad)
# and the monad morphism J -> K sends eps to  p |-> p(eps(p)).  The resolver
# REALISES the clause when the two agree on the clause's own value, i.e.
#     phi-bar(psi) = psi(eps(psi)).
# ---------------------------------------------------------------------------

def quantifier(alg, psi, p):
    """phi_psi(p) = SUM_c psi(c) (x) p(c)."""
    return alg.total(alg.otimes(psi[c], p[c]) for c in range(len(psi)))


def attainable_at(alg, psi, p, tol=1e-9):
    """Is phi_psi(p) the value of p at SOME candidate?  If so a selection
       function realises the clause at this payoff; if not, no section into
       Cand can, and the resolver must be lifted to a larger codomain."""
    q = quantifier(alg, psi, p)
    for c in range(len(psi)):
        if isinstance(q, bool) or isinstance(p[c], bool):
            if q == p[c]:
                return True
        elif abs(q - p[c]) < tol:
            return True
    return False


def e1():
    print(SEP)
    print("E1  attainability:  is phi_psi(p) a value of p?")
    print(SEP)
    import random
    rng = random.Random(7)
    trials = 20000
    cases = [
        ("Bool,      crisp psi", BOOL, [True, False, True], "bool"),
        ("Viterbi,   crisp psi", VITERBI, [1.0, 0.0, 1.0], "real"),
        ("Viterbi,   graded psi", VITERBI, [0.5, 0.2, 1.0], "real"),
        ("tropical,  crisp psi", TROPICAL, [0.0, INF, 0.0], "real"),
        ("tropical,  graded psi", TROPICAL, [1.0, 4.0, 2.0], "real"),
        ("R>=0,      crisp psi", RNN, [1.0, 0.0, 1.0], "real"),
        ("R>=0,      graded psi", RNN, [0.5, 0.2, 0.3], "real"),
    ]
    for label, alg, psi, kind in cases:
        hits = 0
        for _ in range(trials):
            if kind == "bool":
                p = [rng.random() < 0.5 for _ in psi]
            else:
                p = [round(rng.random(), 6) for _ in psi]
            if attainable_at(alg, psi, p):
                hits += 1
        print("  %-24s attained on %6.2f%% of %d random payoffs"
              % (label, 100.0 * hits / trials, trials))
    print()
    # refinement: over an idempotent (+), attainment holds exactly when the
    # winning candidate carries the multiplicative unit -- i.e. when the
    # clause RESTRICTS rather than DISCOUNTS.
    psi = [0.5, 0.2, 1.0]
    agree = 0
    for _ in range(trials):
        p = [round(rng.random(), 6) for _ in psi]
        att = attainable_at(VITERBI, psi, p)
        win = max(range(len(psi)), key=lambda c: psi[c] * p[c])
        if att == (abs(psi[win] - VITERBI.one) < 1e-12):
            agree += 1
    print()
    print("  refinement (Viterbi, graded psi): 'attained' agrees with 'the")
    print("  winning candidate carries the unit' on %6.2f%% of payoffs"
          % (100.0 * agree / trials))
    print()
    print("  Attainment is total exactly when (+) is idempotent AND psi is")
    print("  crisp.  A genuinely graded clause over (+,x) induces an")
    print("  EXPECTATION quantifier, and expectations are the standard")
    print("  non-attainable quantifiers: no section into Cand realises them.")
    print()
    # the lift: randomised selection attains the normalised expectation
    psi = [0.5, 0.2, 0.3]
    tot = sum(psi)
    probs = [v / tot for v in psi]
    rng2 = random.Random(19)
    N = 400000
    for p in ([0.9, 0.1, 0.4], [0.2, 0.7, 0.5]):
        target = quantifier(RNN, psi, p) / tot
        acc = 0.0
        for _ in range(N):
            u, s, i = rng2.random(), 0.0, 0
            for i, pr in enumerate(probs):
                s += pr
                if u <= s:
                    break
            acc += p[i]
        print("  lift to distributions: E[p(eps(p))] = %.6f   target = %.6f"
              % (acc / N, target))
    print()
    print("  So the R>=0 row is not a failure but a LIFT: the resolver selects")
    print("  into distributions over Cand rather than into Cand.  That lift is")
    print("  randomness, and randomness has a Weihrauch home (WWKL).")
    print()


def close(a, b, tol=1e-12):
    if isinstance(a, bool) or isinstance(b, bool):
        return a == b
    return abs(a - b) < tol


def fmt(v):
    if isinstance(v, bool):
        return str(v)
    if isinstance(v, complex):
        return "%.4f%+.4fi" % (v.real, v.imag)
    if v == INF:
        return "inf"
    return "%.6f" % v


# ---------------------------------------------------------------------------
# E2.  The product of selection functions vs sequential graded sites.
#
# Escardo-Oliva's binary product of selection functions eps0, eps1 is
#     (eps0 (x) eps1)(p) = (a0, a1),  a0 = eps0(\x. p(x, eps1(\y. p(x,y)))),
#                                     a1 = eps1(\y. p(a0, y)).
# The graded reading of the same object is: site 0 fires, and the clause at
# site 1 is evaluated against the term site 0 left behind -- i.e. chaining
# through the graded modality <K>_V.  If the identification
#     (x) of selection functions  =  sequential composition of graded sites
# is right, the two computations agree for every payoff p.
# ---------------------------------------------------------------------------

def eo_product(alg, eps0, eps1, p, dom0, dom1):
    def inner(x):
        q = [p(x, y) for y in dom1]
        return q[eps1(q)]
    outer = [inner(x) for x in dom0]
    a0 = dom0[eps0(outer)]
    q1 = [p(a0, y) for y in dom1]
    a1 = dom1[eps1(q1)]
    return (a0, a1)


def graded_chain(alg, psi0, psi1_of, dom0, dom1):
    """<K>_V applied twice: sum over first-site candidates of
       psi0(c0) (x) [ sum over second-site candidates of psi1(c0)(c1) ].
       Returns the total value and the argmax-style section where defined."""
    totals = {}
    for i, c0 in enumerate(dom0):
        psi1 = psi1_of(c0)
        inner = alg.total(psi1)
        totals[c0] = alg.otimes(psi0[i], inner)
    total = alg.total(totals[c0] for c0 in dom0)
    return total, totals


def e2():
    print(SEP)
    print("E2  (x) of selection functions  vs  chained graded sites")
    print(SEP)
    dom0 = ["a", "b"]
    dom1 = ["u", "v", "w"]
    # a payoff that genuinely couples the two sites
    table = {("a", "u"): 0.10, ("a", "v"): 0.60, ("a", "w"): 0.20,
             ("b", "u"): 0.50, ("b", "v"): 0.05, ("b", "w"): 0.40}
    p = lambda x, y: table[(x, y)]

    alg = VITERBI
    sel = alg.sel
    a0, a1 = eo_product(alg, sel, sel, p, dom0, dom1)
    print("  Escardo-Oliva product selects  (%s, %s)  with payoff %.4f"
          % (a0, a1, p(a0, a1)))

    # the same thing as two graded sites: clause at site 0 is the marginal
    # best-continuation, clause at site 1 is the payoff itself
    psi1_of = lambda c0: [table[(c0, y)] for y in dom1]
    psi0 = [alg.total(psi1_of(c0)) for c0 in dom0]     # <K>_V of the payoff
    total, totals = graded_chain(alg, [1.0, 1.0], psi1_of, dom0, dom1)
    i0 = sel([totals[c] for c in dom0])
    c0 = dom0[i0]
    i1 = sel(psi1_of(c0))
    c1 = dom1[i1]
    print("  chained graded sites select    (%s, %s)  with payoff %.4f"
          % (c0, c1, p(c0, c1)))
    print("  total value of the chain <K>_V<K>_V = %.4f" % total)
    agree = (a0, a1) == (c0, c1)
    print("  agree: %s" % agree)

    # exhaustive check over random-ish payoff tables
    import random
    random.seed(11)
    bad = 0
    for _ in range(20000):
        t = {(x, y): round(random.random(), 4) for x in dom0 for y in dom1}
        pp = lambda x, y: t[(x, y)]
        A = eo_product(alg, sel, sel, pp, dom0, dom1)
        f1 = lambda c0: [t[(c0, y)] for y in dom1]
        _, tot = graded_chain(alg, [1.0, 1.0], f1, dom0, dom1)
        j0 = dom0[sel([tot[c] for c in dom0])]
        j1 = dom1[sel(f1(j0))]
        if A != (j0, j1):
            bad += 1
    print("  disagreements over 20000 random payoff tables: %d" % bad)

    # NEGATIVE CONTROL.  The identification is not vacuous: it requires the
    # chaining to aggregate in the SAME algebra the selection function
    # optimises in.  Chain with (+) = sum (expectation) while still selecting
    # by argmax and the two computations come apart.
    bad2 = 0
    random.seed(11)
    for _ in range(20000):
        t = {(x, y): round(random.random(), 4) for x in dom0 for y in dom1}
        pp = lambda x, y: t[(x, y)]
        A = eo_product(alg, sel, sel, pp, dom0, dom1)
        j0 = dom0[sel([sum(t[(x, y)] for y in dom1) for x in dom0])]
        j1 = dom1[sel([t[(j0, y)] for y in dom1])]
        if A != (j0, j1):
            bad2 += 1
    print("  negative control (chained in the wrong algebra): %d disagreements"
          % bad2)
    print()


# ---------------------------------------------------------------------------
# E3.  LLPO as a two-candidate race.
#
# Instance: two channels, each fed by a semi-decision procedure.  At most one
# of the two ever produces.  The clause's crisp guards are the complementary
# approximations "channel 0 has not yet produced" / "channel 1 has not yet
# produced", so the support has exactly the LLPO shape: at least one of the
# two disjuncts holds, and no finite stage decides which.
#
# A "stage-k selector" is a resolver that inspects k stages and commits.  We
# construct, for each k, an instance defeating it -- the diagonal that puts
# the degree strictly above the computable ones.
# ---------------------------------------------------------------------------

def instance(fires, at):
    """returns produce(side, stage) -> bool for the instance where side
       `fires` produces at stage `at`, and the other never produces."""
    def produce(side, stage):
        return fires is not None and side == fires and stage >= at
    return produce


def support(produce, stage):
    """the clause's support at a finite stage: candidate i is admissible iff
       side 1-i has not been seen to produce.  LLPO: never empty."""
    return [i for i in (0, 1) if not produce(1 - i, stage)]


def e3():
    print(SEP)
    print("E3  LLPO: support never empty, no stage-uniform selector correct")
    print(SEP)
    # support is never empty, over every instance and every stage
    empties = 0
    for fires in (None, 0, 1):
        for at in range(0, 40):
            pr = instance(fires, at)
            for s in range(0, 60):
                if not support(pr, s):
                    empties += 1
    print("  empty supports over 3x40x60 instance/stage pairs: %d" % empties)

    # diagonal: defeat every stage-k selector
    def selector(k, tie):
        """inspect stages 0..k; if one side has produced, take the other;
           otherwise fall back on the fixed preference `tie`."""
        def choose(pr):
            for s in range(k + 1):
                if pr(0, s):
                    return 1
                if pr(1, s):
                    return 0
            return tie
        return choose

    defeated = []
    for k in range(0, 8):
        for tie in (0, 1):
            # the adversary lets the side the selector will pick produce,
            # just after the selector has stopped looking
            adversary = instance(fires=tie, at=k + 1)
            picked = selector(k, tie)(adversary)
            wrong = adversary(picked, k + 1)   # picked a side that produces
            defeated.append((k, tie, picked, wrong))
    allwrong = all(w for *_, w in defeated)
    print("  stage-k selectors defeated by the diagonal, k = 0..7, both ties: %s"
          % allwrong)
    print("  (the selector commits at stage k; the adversary produces at k+1")
    print("   on exactly the side committed to, so the commitment is wrong)")
    print("  hence the clause is inhabited but its section is not computable")
    print("  uniformly in the instance: degree C_2 = LLPO, not the identity.")
    print()


# ---------------------------------------------------------------------------
# E4.  Fairness: the bootstrap trap and psi (+) eps.
#
# A receipt whose clause evaluates to 0 is not merely improbable: it is
# outside the support, hence never fires, hence never acquires the evidence
# that would raise its value.  Zero is absorbing.  The repair is a disjunct.
# ---------------------------------------------------------------------------

def forager(n0, eps, horizon=400, k=4, s0=0.5, payoff=1.0, seed=3):
    """A stripped-down version of the forager of the graded-where note: the
       clause value is the PLN power s*c with c = n/(n+k); firing updates the
       evidence count.  With n0 = 0 the value is identically 0."""
    import random
    rng = random.Random(seed)
    n, s = n0, s0
    fires = 0
    stack = 0.0
    for _ in range(horizon):
        c = n / (n + k) if (n + k) > 0 else 0.0
        val = s * c + eps                     # psi (+) eps
        if val <= 0.0:
            continue                          # outside the support
        if rng.random() < min(val, 1.0):
            fires += 1
            good = rng.random() < 0.64        # world A posterior
            stack += payoff if good else -0.4
            n += 1
            s = ((s * (n - 1)) + (1.0 if good else 0.0)) / n
    return fires, stack


def e4():
    print(SEP)
    print("E4  fairness: zero is absorbing; psi (+) eps restores it")
    print(SEP)
    for n0, eps, label in [(0, 0.0, "n0=0, no disjunct"),
                           (0, 0.02, "n0=0, psi (+) 0.02"),
                           (1, 0.0, "n0=1, no disjunct")]:
        f, st = forager(n0, eps)
        print("  %-22s  firings = %4d   terminal = %8.2f" % (label, f, st))
    print()
    print("  A continuously enabled receipt with clause value 0 never fires:")
    print("  a fairness violation manufactured by the value algebra, not by")
    print("  the scheduler.  Weak fairness is the clause combinator psi (+) eps.")
    print()


# ---------------------------------------------------------------------------
# E5.  The interference deficit.
#
# Minimal reconvergent race: two candidates whose outcomes coincide.  With
# C-valued clauses of equal magnitude and relative phase pi the coefficient
# on that outcome is 0 although the outcome is operationally reachable.  No
# R>=0 clause can do this.
# ---------------------------------------------------------------------------

def deficit(w0, w1, alg):
    """two candidates, same outcome: coefficient is w0 (+) w1."""
    return alg.oplus(w0, w1)


def e5():
    print(SEP)
    print("E5  interference deficit: supp is a proper subset of Reach")
    print(SEP)
    a = 1 / (2 ** 0.5)
    for phase, name in [(0.0, "0"), (cmath.pi, "pi"), (cmath.pi / 2, "pi/2")]:
        w0 = complex(a, 0)
        w1 = a * cmath.exp(1j * phase)
        coef = deficit(w0, w1, CPLX)
        print("  relative phase %-4s  coefficient = %-18s  |coef|^2 = %.6f"
              % (name, fmt(coef), abs(coef) ** 2))
    print()
    # no non-negative clause reaches 0 on a reachable outcome with both
    # candidates in the support
    print("  over R>=0, both candidates in the support forces coefficient > 0:")
    worst = min(deficit(x / 100, y / 100, RNN)
                for x in range(1, 101) for y in range(1, 101))
    print("    min over 100x100 strictly positive pairs = %.6f" % worst)
    print()
    print("  A section always lands in its fibre, so a resolver with a")
    print("  non-empty interference deficit is not a choice function at all.")
    print("  C is transverse to the lattice, not high in it.")
    print()


if __name__ == "__main__":
    e1()
    e2()
    e3()
    e4()
    e5()
