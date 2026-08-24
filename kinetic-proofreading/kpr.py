#!/usr/bin/env python3
"""
kpr.py --- Kinetic proofreading as a converter of tokens into accuracy.

Companion computations for the research note
  "Paying for Accuracy: kinetic proofreading, cost accounting, and what
   dissipation adds to a mortal learner's toolbox"

Standard library only.  Deterministic: every stochastic experiment is seeded.
Every number quoted in the note is printed by this script.

Conventions
-----------
Rates are dimensionless and scaled so that the RIGHT substrate dissociates at
rate 1.  The WRONG substrate dissociates at rate f > 1, so f is the
equilibrium (Michaelis-Menten) discrimination factor.  A single test stage is
a race between advance (rate a, metered: one token) and dissociation
(unmetered).  A ladder of N tests is the N-fold iterate.

  p_r = a/(a+1)      probability a right candidate survives one test
  p_w = a/(a+f)      probability a wrong candidate survives one test
  g   = p_r/p_w      discrimination gained per test = (a+f)/(a+1) <= f
"""

import math
import random

SEED = 20260817


# ----------------------------------------------------------------------
# Level 0.  The single stage, and the Michaelis-Menten ceiling.
# ----------------------------------------------------------------------

def survivals(a, f):
    return a / (a + 1.0), a / (a + f)


def gain(a, f):
    """Discrimination purchased by one funded test."""
    return (a + f) / (a + 1.0)


def ladder(a, f, N, c=0.0):
    """Exact figures for a ladder of N funded tests.

    Returns (error, yield, tokens_per_accepted_right).
    Token accounting: one token per advance, charged whether or not the
    candidate is eventually accepted, and charged for wrong candidates too
    (they enter at equal flux with the right ones).  The parameter c is the
    per-stage HOLDING charge: an intermediate occupying a stage is structure
    that must be kept, and in the cost-accounting framework keeping structure
    is metered.  c = 0 recovers the textbook rate-only account.
    """
    p_r, p_w = survivals(a, f)
    err = (p_w / p_r) ** N
    yld = p_r ** N

    def advances(p):
        # expected number of advance events per candidate entering
        return p * (1.0 - p ** N) / (1.0 - p) if p < 1.0 else float(N)

    def entries(p):
        # expected number of stages a candidate occupies
        return (1.0 - p ** N) / (1.0 - p) if p < 1.0 else float(N)

    num = (advances(p_r) + advances(p_w)
           + c * (entries(p_r) + entries(p_w)))
    tokens = num / yld if yld > 0.0 else float('inf')
    return err, yld, tokens


def efficiency(a, f):
    """Asymptotic factors of accuracy bought per factor of token spend."""
    p_r, _ = survivals(a, f)
    return math.log(gain(a, f)) / math.log(1.0 / p_r)


# ----------------------------------------------------------------------
# E1.  The unfunded presentation: catalysis cannot beat f.
#
# A ladder whose internal steps are RESTORED rather than consumed is a
# reversible chain at detailed balance.  We solve the steady state of both
# presentations on the same chain and compare the discrimination.
# ----------------------------------------------------------------------

def solve(A, b):
    """Dense Gaussian elimination with partial pivoting.  Small systems."""
    n = len(b)
    M = [row[:] + [b[i]] for i, row in enumerate(A)]
    for c in range(n):
        piv = max(range(c, n), key=lambda r: abs(M[r][c]))
        M[c], M[piv] = M[piv], M[c]
        pv = M[c][c]
        for r in range(n):
            if r == c:
                continue
            fac = M[r][c] / pv
            if fac == 0.0:
                continue
            for k in range(c, n + 1):
                M[r][k] -= fac * M[c][k]
    return [M[i][n] / M[i][i] for i in range(n)]


def steady_flux(a, f, N, species_off, reversible, back, kp=1e-4, on=1.0):
    """Steady-state product flux for one species on an (N+1)-state chain.

    states C_0..C_N.  bind: free -> C_0 at rate `on` (free held at 1).
    advance C_i -> C_{i+1} at rate a.  product from C_N at rate kp.

    reversible=True  : catalytic presentation.  C_{i+1} -> C_i at rate `back`,
                       and dissociation only from C_0.  Nothing is consumed;
                       the chain sits at detailed balance apart from the
                       vanishing product drain.
    reversible=False : proofread presentation.  Every C_i dissociates at rate
                       species_off, discarding the intermediate irreversibly.
    """
    n = N + 1
    A = [[0.0] * n for _ in range(n)]
    b = [0.0] * n
    for i in range(n):
        out = 0.0
        if i < N:
            out += a
        else:
            out += kp
        if reversible:
            if i > 0:
                out += back
            if i == 0:
                out += species_off
        else:
            out += species_off
        A[i][i] -= out
        if i < N:
            A[i + 1][i] += a
        if reversible and i > 0:
            A[i - 1][i] += back
    b[0] -= on          # influx into C_0
    occ = solve(A, b)
    return kp * occ[N]


def e1_catalysis_versus_dissipation(f=10.0, a=0.1, Ns=(0, 1, 2, 3, 4)):
    print("=" * 72)
    print("E1  Catalysis cannot beat f; funded advance can.")
    print("=" * 72)
    print(f"  f = {f}, advance rate a = {a}")
    print()
    print(f"  {'N':>2}  {'catalytic (restored)':>22}  {'proofread (consumed)':>22}")
    for N in Ns:
        # catalytic: back-rate equal to forward, dissociation only from C_0
        cr = steady_flux(a, f, N, 1.0, True, a)
        cw = steady_flux(a, f, N, f, True, a)
        pr = steady_flux(a, f, N, 1.0, False, 0.0)
        pw = steady_flux(a, f, N, f, False, 0.0)
        print(f"  {N:>2}  {cw / cr:>22.8f}  {pw / pr:>22.8f}")
    print()
    print("  The catalytic column is flat at 1/f = %.8f: adding steps to a"
          % (1.0 / f))
    print("  presentation that restores its intermediates buys nothing.")
    print("  The proofread column falls geometrically.")
    print()


# ----------------------------------------------------------------------
# E2.  The exchange rate.  Accuracy is a POWER LAW in tokens, not an
#      exponential, and the exponent does not depend on ladder depth.
# ----------------------------------------------------------------------

def e2_exchange_rate(f=10.0, a=0.1, Ns=range(1, 9)):
    print("=" * 72)
    print("E2  The exchange rate: error against tokens per accepted product.")
    print("=" * 72)
    print(f"  f = {f}, a = {a}, per-test gain g = {gain(a, f):.6f}, "
          f"yield/test p_r = {survivals(a, f)[0]:.6f}")
    print()
    print(f"  {'N':>2}  {'error':>12}  {'yield':>10}  {'tokens/acc':>12}"
          f"  {'ln(1/err)/ln(tok)':>18}")
    rows = []
    for N in Ns:
        e, y, t = ladder(a, f, N)
        rows.append((N, e, y, t))
        print(f"  {N:>2}  {e:>12.3e}  {y:>10.6f}  {t:>12.3e}"
              f"  {math.log(1/e)/math.log(t):>18.6f}")
    eta = efficiency(a, f)
    print()
    print(f"  predicted asymptotic exponent  ln g / ln(1/p_r) = {eta:.6f}")
    N1, e1, _, t1 = rows[-2]
    N2, e2, _, t2 = rows[-1]
    measured = (math.log(1 / e2) - math.log(1 / e1)) / (math.log(t2) - math.log(t1))
    print(f"  measured local slope between N={N1} and N={N2}      = {measured:.6f}")
    print()
    print("  So error falls as a POWER of the tokens burned, err ~ tok^-eta,")
    print("  and eta is a property of ONE stage.  Depth converts; it does not")
    print("  change the rate of conversion.")
    print()
    return eta


def e2b_rate_is_depth_free(f=10.0, avals=(0.02, 0.05, 0.1, 0.3, 1.0, 3.0)):
    print("-" * 72)
    print("E2b  The exchange rate is set by single-stage kinetics alone.")
    print("-" * 72)
    print(f"  {'a':>6}  {'g (gain/test)':>14}  {'p_r (yield/test)':>17}"
          f"  {'eta':>10}")
    for a in avals:
        p_r, _ = survivals(a, f)
        print(f"  {a:>6}  {gain(a, f):>14.6f}  {p_r:>17.6f}"
              f"  {efficiency(a, f):>10.6f}")
    print()
    print("  eta rises with a: per factor of accuracy, many weak stages are")
    print("  cheaper than few strong ones.  eta depends on a alone, not on N.")
    print()


def e2c_minimum_tokens(f=10.0, targets=(1e-2, 1e-3, 1e-4),
                       cs=(0.0, 1.0, 4.0), avals=None, Nmax=200):
    if avals is None:
        avals = [0.02 * (1.35 ** k) for k in range(30)]
    print("-" * 72)
    print("E2c  Cheapest (a, N) for a given accuracy target, tokens as currency,")
    print("     with and without a per-stage HOLDING charge c.")
    print("-" * 72)
    print(f"  {'c':>5}  {'target':>8}  {'best a':>8}  {'best N':>7}"
          f"  {'tokens/acc':>12}  {'yield':>10}")
    for c in cs:
        for tgt in targets:
            best = None
            for a in avals:
                for N in range(1, Nmax + 1):
                    e, y, t = ladder(a, f, N, c)
                    if e <= tgt and (best is None or t < best[3]):
                        best = (a, N, e, t, y)
            a, N, e, t, y = best
            edge = "  <-- at sweep edge" if a >= avals[-1] * 0.999 else ""
            print(f"  {c:>5}  {tgt:>8.0e}  {a:>8.4f}  {N:>7}  {t:>12.3f}"
                  f"  {y:>10.6f}{edge}")
    print()
    print("  NEGATIVE RESULT, recorded rather than smoothed over.  The optimum")
    print("  is interior in both a and N even at c = 0, so token accounting")
    print("  already supplies an optimum that a rate-only account does not.")
    print("  But the optimal ladder here is LONG (N of 5 to 10), and real")
    print("  proofreading ladders are one or two steps.  The holding charge c")
    print("  tested here does not close that gap: raising c raises the cost")
    print("  without shortening the ladder, because higher yield at larger a")
    print("  dominates.  Whatever makes biological ladders short is not this")
    print("  term, and the note should not pretend otherwise.")
    print()


# ----------------------------------------------------------------------
# E3.  Monte Carlo cross-check of the closed form.
# ----------------------------------------------------------------------

def e3_montecarlo(f=10.0, a=1.0, N=3, trials=400000):
    print("-" * 72)
    print("E3  Monte Carlo cross-check of the closed form.")
    print("-" * 72)
    rng = random.Random(SEED)
    p_r, p_w = survivals(a, f)
    acc_r = acc_w = 0
    tokens = 0
    for _ in range(trials):
        for p, tally in ((p_r, 'r'), (p_w, 'w')):
            for _ in range(N):
                if rng.random() >= p:
                    break
                tokens += 1
            else:
                if tally == 'r':
                    acc_r += 1
                else:
                    acc_w += 1
    e, y, t = ladder(a, f, N)
    print(f"  N = {N}, trials = {trials} per species")
    print(f"  simulated error   {acc_w / max(acc_r, 1):>12.6f}"
          f"     closed form {e:>12.6f}")
    print(f"  simulated yield   {acc_r / trials:>12.6f}"
          f"     closed form {y:>12.6f}")
    print(f"  simulated tok/acc {tokens / max(acc_r, 1):>12.4f}"
          f"     closed form {t:>12.4f}")
    print()


# ----------------------------------------------------------------------
# LEVEL 2.  One scientist.  The bottom rate is a bypass, and a bypass is a
# ceiling on what any amount of energy can buy.
# ----------------------------------------------------------------------

def leaky_ladder(a, f, N, b):
    """A fraction b of candidates bypass the ladder after the first test,
    because the assay returned bottom and the scientist had to act anyway."""
    p_r, p_w = survivals(a, f)
    r = p_w / p_r
    return b * r + (1.0 - b) * r ** N


def e4_leak_ceiling(f=10.0, a=0.1, bs=(0.0, 0.01, 0.05, 0.2, 0.6104), Nmax=10):
    print("=" * 72)
    print("E4  The bottom rate is a bypass, and a bypass is a ceiling.")
    print("=" * 72)
    print("  b = fraction of assays that return bottom and are acted on anyway.")
    print("  The two anchors measured elsewhere in the corpus: b = 0.6104 at")
    print("  purse 60, b = 0 at purse >= 200.  Intermediate values are swept,")
    print("  not measured.")
    print()
    hdr = "  " + f"{'N':>2}" + "".join(f"{('b=' + str(b)):>13}" for b in bs)
    print(hdr)
    for N in range(1, Nmax + 1):
        row = "  " + f"{N:>2}"
        for b in bs:
            row += f"{leaky_ladder(a, f, N, b):>13.3e}"
        print(row)
    print()
    print(f"  {'b':>8}  {'floor':>12}  {'N* (within 10% of floor)':>26}"
          f"  {'tokens at N*':>13}")
    p_r, p_w = survivals(a, f)
    for b in bs:
        floor = b * (p_w / p_r) if b > 0 else 0.0
        Nstar = None
        for N in range(1, 200):
            e = leaky_ladder(a, f, N, b)
            if b > 0 and e <= 1.1 * floor:
                Nstar = N
                break
            if b == 0 and e <= 1e-9:
                Nstar = N
                break
        _, _, t = ladder(a, f, Nstar) if Nstar else (0, 0, float('nan'))
        fl = f"{floor:.3e}" if b > 0 else "0 (none)"
        print(f"  {b:>8}  {fl:>12}  {str(Nstar):>26}  {t:>13.3e}")
    print()
    print("  Past N*, tokens keep being spent and accuracy does not move.")
    print("  Accuracy per token falls to zero at FINITE depth: the exchange")
    print("  rate saturates in energy the way Michaelis-Menten saturates in")
    print("  substrate, and the bottom rate is playing the role of K_M.")
    print()


# ----------------------------------------------------------------------
# LEVEL 2b.  Energy substitutes for memory.  The ratchet versus the register.
# ----------------------------------------------------------------------

def sprt_samples(g, target):
    """Wald: samples for a sequential likelihood-ratio test whose per-sample
    log-likelihood ratio is ln g --- the SAME evidence one ladder stage
    extracts, so the comparison is between bookkeeping schemes and not
    between tests."""
    return math.log(1.0 / target) / math.log(g)


def e5_ratchet_versus_register(f=10.0, a=0.1, targets=(1e-2, 1e-3, 1e-4, 1e-6),
                               mem_prices=(0.0, 0.5, 1.0, 2.0, 4.0, 8.0)):
    print("=" * 72)
    print("E5  The ratchet and the register: energy bought instead of memory.")
    print("=" * 72)
    print("  A proofreading ladder is a sequential likelihood-ratio test whose")
    print("  accumulator has been replaced by position on the ladder.  It")
    print("  carries no state and cannot recover from one unlucky sample.")
    print("  The sequential test carries an accumulator, recovers, and wastes")
    print("  nothing -- but the accumulator must be HELD, and holding is")
    print("  priced.  m below is the price per step of holding it.")
    print()
    print(f"  {'target':>8}  {'ladder N':>9}  {'ladder tok':>11}"
          f"  {'SPRT samples':>13}  " +
          "  ".join(f"{('SPRT m=' + str(m)):>12}" for m in mem_prices))
    cross = {}
    for tgt in targets:
        N = 1
        while ladder(a, f, N)[0] > tgt and N < 400:
            N += 1
        _, y, tok = ladder(a, f, N)
        s = sprt_samples(gain(a, f), tgt)
        costs = [s * (1.0 + m) for m in mem_prices]
        print(f"  {tgt:>8.0e}  {N:>9}  {tok:>11.3e}  {s:>13.3f}  " +
              "  ".join(f"{c:>12.3f}" for c in costs))
        lo, hi = 0.0, 1e9
        for _ in range(200):
            mid = (lo + hi) / 2
            if s * (1.0 + mid) < tok:
                lo = mid
            else:
                hi = mid
        cross[tgt] = lo
        _, y, _ = ladder(a, f, N)
    print()
    print(f"  {'target':>8}  {'memory price at which the ratchet wins':>42}"
          f"  {'right candidates discarded':>27}")
    for tgt in targets:
        N = 1
        while ladder(a, f, N)[0] > tgt and N < 400:
            N += 1
        _, y, _ = ladder(a, f, N)
        print(f"  {tgt:>8.0e}  {cross[tgt]:>42.3f}  {1.0 - y:>27.6f}")
    print()
    print("  Read the last column as the price actually paid: the correct")
    print("  candidates thrown away ARE the register the learner did not buy.")
    print()


# ----------------------------------------------------------------------
# LEVEL 3.  A population.  Correlated stages are a lossy converter.
# ----------------------------------------------------------------------

def _phi(x):
    return 0.5 * (1.0 + math.erf(x / math.sqrt(2.0)))


def _phi_inv(p):
    lo, hi = -12.0, 12.0
    for _ in range(200):
        mid = (lo + hi) / 2
        if _phi(mid) < p:
            lo = mid
        else:
            hi = mid
    return (lo + hi) / 2


def e6_population(f=10.0, a=17.0, ks=(0.0, 0.25, 0.5, 0.75, 1.0),
                  ms=(1, 2, 4, 8), trials=200000):
    print("=" * 72)
    print("E6  A population is a converter too, and overlap is a tax on it.")
    print("=" * 72)
    p_r, p_w = survivals(a, f)
    g = gain(a, f)
    # calibrate a gaussian one-stage test to the same (p_r, p_w)
    thr = -_phi_inv(p_r)
    delta = -_phi_inv(p_w) - thr         # mean shift for the wrong species
    print(f"  each reviewer calibrated to p_r = {p_r:.6f}, p_w = {p_w:.6f},"
          f" g = {g:.6f}")
    print("  (reviewers are deliberately WEAK stages, so that eight of them")
    print("   still leave measurable statistics)")
    print(f"  k = grading of namespace overlap between reviewers"
          f" (k=0 disjoint, k=1 identical)")
    print()
    print("  n_eff / m, the fraction of burned tokens that became discrimination")
    print("  " + f"{'k':>6}" + "".join(f"{('m=' + str(m)):>12}" for m in ms))
    rng = random.Random(SEED + 1)
    table = {}
    for k in ks:
        row = "  " + f"{k:>6}"
        for m in ms:
            ar = aw = 0
            s = math.sqrt(k)
            t = math.sqrt(1.0 - k)
            for _ in range(trials):
                u = rng.gauss(0.0, 1.0)
                if all(s * u + t * rng.gauss(0.0, 1.0) > thr for _ in range(m)):
                    ar += 1
                u = rng.gauss(0.0, 1.0)
                if all(s * u + t * rng.gauss(0.0, 1.0) - delta > thr
                       for _ in range(m)):
                    aw += 1
            err = aw / max(ar, 1)
            neff = math.log(1.0 / err) / math.log(g) if err > 0 else float('nan')
            table[(k, m)] = (err, neff)
            row += f"{neff / m:>12.4f}"
        print(row)
    print()
    print("  At k = 0 the reviewers multiply: n_eff = m, every token converted.")
    print("  At k = 1 they are one reviewer wearing m hats: n_eff -> 1, and the")
    print("  other m-1 reviewers' tokens bought nothing.  Adding reviewers")
    print("  keeps costing and stops discriminating.")
    print()
    print("  detail (k, m) -> error, n_eff")
    for k in ks:
        for m in ms:
            e, ne = table[(k, m)]
            print(f"    k={k:<5} m={m:<3} error={e:.6f}  n_eff={ne:.4f}")
    print()


def e7_population_versus_individual(f=10.0, a=0.1, budget_ms=(1, 2, 3, 4, 6, 8)):
    print("-" * 72)
    print("E7  m cheap stages against one expensive one, at matched tokens.")
    print("-" * 72)
    print(f"  {'m':>3}  {'m disjoint stages':>19}"
          f"  {'one stage, m-fold slower':>26}")
    p_r, p_w = survivals(a, f)
    for m in budget_ms:
        many = (p_w / p_r) ** m
        # one stage given m times the advance budget: slow the advance down by
        # m, which is what buying a longer look with the same tokens means
        one = (a / m) / ((a / m) + f) / ((a / m) / ((a / m) + 1.0))
        print(f"  {m:>3}  {many:>19.3e}  {one:>26.3e}")
    print()
    print("  A single stage cannot be made to multiply by spending more on it:")
    print("  its discrimination is capped by f no matter how slow the advance.")
    print("  Only composition multiplies.  That is the whole reason ladders")
    print("  exist, at every scale.")
    print()


# ----------------------------------------------------------------------

def main():
    print()
    print("kpr.py --- computations for 'Paying for Accuracy'")
    print(f"seed = {SEED}")
    print()
    e1_catalysis_versus_dissipation()
    e2_exchange_rate()
    e2b_rate_is_depth_free()
    e2c_minimum_tokens()
    e3_montecarlo()
    e4_leak_ceiling()
    e5_ratchet_versus_register()
    e6_population()
    e7_population_versus_individual()
    print("done.")


if __name__ == "__main__":
    main()
