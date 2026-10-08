"""
collide.py -- the computations for "Collision and Scattering for Process
Molecules".

Stdlib only, matching gradedsim.py, forager.py and fibre.py.

  E1  The collision kernel and its translation back to rho agree, outcome by
      outcome, once each translated candidate is weighted by the inverse of
      its fibre multiplicity mu = k! * prod_a T_a (a locally computable
      contingency-table count).  Negative controls: the naive translation and
      the translation with only the 1/k! symmetry factor.
  E2  The ideal gas.  With w_k = 0 for k >= 2 the kernel's outcome
      distribution is rho's; with w_2 > 0 the site rate acquires the virial
      term and the jump chain across sites is reweighted superlinearly.
  E3  The propensity tables used by the fast simulators of E4 and E5 agree
      with the generic enumerator of E1.
  E4  Lindemann-Hinshelwood collisional activation: the fall-off curve.
  E5  A lattice gas whose shear viscosity is produced by the collision
      kernel.  HPP (square, 4 directions) has a spurious invariant and its
      shear mode does not decay; FHP with pair collisions only keeps a
      conserved quantity on every horizontal line and stalls at 2/3; FHP with
      pair and triple collisions decays, with viscosity decreasing in the
      collision parameter.
  E6  Replicates of the E5 points used to test whether the viscosity is a
      function of the collision parameter alone, with standard errors.
"""

import itertools
import math
import random
import sys
from collections import Counter

TOL = 1e-12


# =============================================================== multisets

def ms(*atoms):
    return Counter(atoms)


def ms_key(c):
    return tuple(sorted(c.elements()))


def ms_add(*cs):
    out = Counter()
    for c in cs:
        out.update(c)
    return out


def ms_sub(a, b):
    """a - b, or None if b is not a sub-multiset of a."""
    out = Counter(a)
    for k, v in b.items():
        if out[k] < v:
            return None
        out[k] -= v
        if out[k] == 0:
            del out[k]
    return out


def compositions(n, k):
    """All k-tuples of non-negative integers summing to n."""
    if k == 0:
        if n == 0:
            yield ()
        return
    if k == 1:
        yield (n,)
        return
    for i in range(n + 1):
        for rest in compositions(n - i, k - 1):
            yield (i,) + rest


# =============================================================== matching
# A pattern is (consts, vars): a multiset of constant atoms in parallel with
# process variables.  Matching modulo AC: consts must be a sub-multiset of the
# target; the remainder is distributed over the variables in every way.
# Variables may receive the empty process 0.

def match(pattern, target):
    consts, vs = pattern
    rem = ms_sub(target, consts)
    if rem is None:
        return []
    if not vs:
        return [{}] if not rem else []
    types = sorted(rem)
    per_type = [list(compositions(rem[a], len(vs))) for a in types]
    out = []
    for choice in itertools.product(*per_type):
        sigma = {v: Counter() for v in vs}
        for a, comp in zip(types, choice):
            for v, c in zip(vs, comp):
                if c:
                    sigma[v][a] += c
        out.append(sigma)
    return out


def sigma_key(sigma):
    return tuple(sorted((v, ms_key(c)) for v, c in sigma.items()))


# =============================================================== kernel
# Outcome of a firing: (consumed message indices, substitution).  The kernel
# weights each distinct outcome once: w_k * g(sigma).

def kernel_outcomes(msgs, pattern, wk, g):
    out = Counter()
    for k, w in wk.items():
        if w == 0:
            continue
        for idx in itertools.combinations(range(len(msgs)), k):
            target = ms_add(*(msgs[i] for i in idx))
            for sigma in match(pattern, target):
                out[(frozenset(idx), sigma_key(sigma))] += w * g(sigma)
    return out


# =============================================================== translation
# The translated receipt is a family of k-fold self-joins, one branch per
# SPLIT of the pattern across the k join components: each constant occurrence
# goes to one component, and each variable v is split into v^1..v^k, to be
# recombined as @( *v^1 | ... | *v^k ), which equals @(S^1|...|S^k) by the
# congruence *@P == P.  A translated CANDIDATE is (ordering of k distinct
# messages, split, per-component match).

def translated_candidates(msgs, pattern, k):
    consts, vs = pattern
    ctypes = sorted(consts)
    csplits = [list(compositions(consts[a], k)) for a in ctypes]
    for order in itertools.permutations(range(len(msgs)), k):
        for choice in itertools.product(*csplits):
            comps = []
            for i in range(k):
                ci = Counter()
                for a, comp in zip(ctypes, choice):
                    if comp[i]:
                        ci[a] = comp[i]
                comps.append((ci, [f"{v}^{i}" for v in vs]))
            per = [match(comps[i], msgs[order[i]]) for i in range(k)]
            for parts in itertools.product(*per):
                sigma = {v: Counter() for v in vs}
                for i, s_i in enumerate(parts):
                    for v in vs:
                        sigma[v].update(s_i[f"{v}^{i}"])
                yield order, sigma


def contingency_count(rows, cols):
    """Number of non-negative integer matrices with the given row and column
    sums, by direct recursion over rows."""
    rows, cols = tuple(rows), tuple(cols)
    if sum(rows) != sum(cols):
        return 0
    memo = {}

    def go(i, cols):
        if i == len(rows):
            return 1 if not any(cols) else 0
        key = (i, cols)
        if key not in memo:
            tot = 0
            for comp in compositions(rows[i], len(cols)):
                if all(c <= m for c, m in zip(comp, cols)):
                    tot += go(i + 1, tuple(m - c for c, m in zip(comp, cols)))
            memo[key] = tot
        return memo[key]

    return go(0, cols)


def fibre_multiplicity(msgs_sel, pattern, sigma):
    """mu(outcome) = k! * prod over atom types a of the number of k x roles
    contingency tables with row sums (message i's count of a) and column sums
    (constant count of a, then sigma(v)'s count of a for each v).  Computable
    from the candidate's own data, hence admissible in a where-clause."""
    consts, vs = pattern
    k = len(msgs_sel)
    atoms = set()
    for m in msgs_sel:
        atoms |= set(m)
    mu = math.factorial(k)
    for a in sorted(atoms):
        rows = [m[a] for m in msgs_sel]
        cols = [consts[a]] + [sigma[v][a] for v in vs]
        mu *= contingency_count(rows, cols)
    return mu


def translation_outcomes(msgs, pattern, wk, g, mode):
    """mode: 'naive'   -- every candidate weighted w_k g
             'symm'    -- every candidate weighted w_k g / k!
             'fibre'   -- every candidate weighted w_k g / mu"""
    out = Counter()
    for k, w in wk.items():
        if w == 0:
            continue
        for order, sigma in translated_candidates(msgs, pattern, k):
            key = (frozenset(order), sigma_key(sigma))
            if mode == "naive":
                out[key] += w * g(sigma)
            elif mode == "symm":
                out[key] += w * g(sigma) / math.factorial(k)
            else:
                sel = [msgs[i] for i in order]
                out[key] += w * g(sigma) / fibre_multiplicity(sel, pattern, sigma)
    return out


def compare(a, b):
    """Worst relative per-outcome discrepancy, and whether the supports agree."""
    keys = set(a) | set(b)
    worst = 0.0
    for key in keys:
        x, y = a.get(key, 0.0), b.get(key, 0.0)
        worst = max(worst, abs(x - y) / max(1.0, abs(x)))
    sa = {k for k in keys if a.get(k, 0.0) > 0}
    sb = {k for k in keys if b.get(k, 0.0) > 0}
    return worst, sa == sb


def g_default(sigma):
    v = 1.0
    for name in sorted(sigma):
        v *= 1.0 + 0.25 * sum(sigma[name].values())
    return v


def random_instance(rng):
    atoms = "abc"
    consts = Counter(rng.choice(atoms) for _ in range(rng.randint(0, 2)))
    vs = [f"v{i}" for i in range(rng.randint(0, 2))]
    msgs = [Counter(rng.choice(atoms) for _ in range(rng.randint(1, 2)))
            for _ in range(rng.randint(2, 4))]
    wk = {1: rng.uniform(0.2, 2.0), 2: rng.uniform(0.2, 2.0),
          3: rng.uniform(0.0, 1.0)}
    return msgs, (consts, vs), wk


def e1(trials=3000, seed=1):
    rng = random.Random(seed)
    worst = {"naive": 0.0, "symm": 0.0, "fibre": 0.0}
    supp_ok = True
    nonempty = 0
    max_mu = 0
    over = {"naive": [], "symm": []}
    for _ in range(trials):
        msgs, pat, wk = random_instance(rng)
        ker = kernel_outcomes(msgs, pat, wk, g_default)
        if ker:
            nonempty += 1
        for mode in worst:
            tr = translation_outcomes(msgs, pat, wk, g_default, mode)
            w, same = compare(ker, tr)
            worst[mode] = max(worst[mode], w)
            if mode == "fibre":
                supp_ok = supp_ok and same
            elif ker:
                tk, tt = sum(ker.values()), sum(tr.values())
                over[mode].append(tt / tk)
        for key in ker:
            idx = sorted(key[0])
            sigma = {v: Counter(c) for v, c in key[1]}
            mu = fibre_multiplicity([msgs[i] for i in idx], pat, sigma)
            max_mu = max(max_mu, mu)
    return {
        "trials": trials,
        "nonempty": nonempty,
        "worst_fibre": worst["fibre"],
        "worst_symm": worst["symm"],
        "worst_naive": worst["naive"],
        "support_equal": supp_ok,
        "max_mu": max_mu,
        "overcount_naive_max": max(over["naive"]),
        "overcount_symm_max": max(over["symm"]),
        "overcount_naive_mean": sum(over["naive"]) / len(over["naive"]),
        "overcount_symm_mean": sum(over["symm"]) / len(over["symm"]),
    }


# =============================================================== E2

def e2():
    # (a) w_k = 0 for k >= 2 reproduces rho outcome by outcome.
    rng = random.Random(2)
    worst = 0.0
    for _ in range(2000):
        msgs, pat, wk = random_instance(rng)
        rho = kernel_outcomes(msgs, pat, {1: wk[1]}, g_default)
        ker = kernel_outcomes(msgs, pat, {1: wk[1], 2: 0.0, 3: 0.0}, g_default)
        worst = max(worst, compare(rho, ker)[0])
    # (b) bare-variable receipt with n messages: total rate and crisp support
    def site(n, w1, w2):
        msgs = [ms("m") for _ in range(n)]
        out = kernel_outcomes(msgs, (Counter(), ["y"]), {1: w1, 2: w2},
                              lambda s: 1.0)
        return sum(out.values()), out
    rates = []
    for n in (1, 2, 5, 10, 20):
        r0, _ = site(n, 1.0, 0.0)
        r1, _ = site(n, 1.0, 0.1)
        rates.append((n, r0, r1, n * 1.0 + math.comb(n, 2) * 0.1))
    # (c) jump chain across two sites with 2 and 20 messages
    rows = []
    for w2 in (0.0, 0.01, 0.1, 1.0):
        a = 2 * 1.0 + math.comb(2, 2) * w2
        b = 20 * 1.0 + math.comb(20, 2) * w2
        rows.append((w2, a / (a + b), b / (a + b)))
    # (d) within one site, the distribution over WHICH message is bound by
    # the COMM branch: identity kernel on k=2 resolves symmetric.
    return {"worst_rho_vs_kernel": worst, "rates": rates, "jump": rows}


# =============================================================== E3

def e3():
    rng = random.Random(3)
    worst = 0
    checked = 0
    # Lindemann activation: pattern A|M, constants only
    for _ in range(200):
        nA, nM, nS = rng.randint(0, 6), rng.randint(0, 6), rng.randint(0, 3)
        msgs = ([ms("A")] * nA + [ms("M")] * nM + [ms("S")] * nS)
        out = kernel_outcomes(msgs, (ms("A", "M"), []), {1: 1.0, 2: 1.0},
                              lambda s: 1.0)
        worst = max(worst, abs(sum(out.values()) - nA * nM))
        checked += 1
    # FHP: head-on pair (d, d+3) and triple (0,2,4)
    for _ in range(200):
        n = [rng.randint(0, 3) for _ in range(6)]
        msgs = []
        for d in range(6):
            msgs += [ms(f"p{d}")] * n[d]
        for d in range(3):
            out = kernel_outcomes(msgs, (ms(f"p{d}", f"p{d+3}"), []),
                                  {2: 1.0}, lambda s: 1.0)
            worst = max(worst, abs(sum(out.values()) - n[d] * n[d + 3]))
        out = kernel_outcomes(msgs, (ms("p0", "p2", "p4"), []), {3: 1.0},
                              lambda s: 1.0)
        worst = max(worst, abs(sum(out.values()) - n[0] * n[2] * n[4]))
        checked += 1
    return {"checked": checked, "worst_abs": worst}


# =============================================================== E4
# One location x.  Laws (persistent receipts on x):
#   activation    for( @(A | M) <= x ) { x!(A*) | x!(M) }     weight k1  (k=2)
#   deactivation  for( @(A* | M) <= x ) { x!(A) | x!(M) }     weight km1 (k=2)
#   decomposition for( @A* <= x ) { x!(B) }                   weight k2  (k=1)
#   recycling     for( @B <= x ) { x!(A) }                    weight r   (k=1)

def lindemann_sim(N, nM, k1, km1, k2, r, products, rng):
    nA, nS, nB = N, 0, 0
    t = 0.0
    intA = 0.0
    made = 0
    while made < products:
        a1 = k1 * nA * nM
        a2 = km1 * nS * nM
        a3 = k2 * nS
        a4 = r * nB
        a0 = a1 + a2 + a3 + a4
        dt = rng.expovariate(a0)
        intA += nA * dt
        t += dt
        u = rng.random() * a0
        if u < a1:
            nA -= 1; nS += 1
        elif u < a1 + a2:
            nS -= 1; nA += 1
        elif u < a1 + a2 + a3:
            nS -= 1; nB += 1; made += 1
        else:
            nB -= 1; nA += 1
    return made / intA


def e4(seed=4):
    rng = random.Random(seed)
    k1, km1, k2, r = 1.0, 1.0, 10.0, 50.0
    N = 50
    rows = []
    for nM in (1, 3, 10, 30, 100, 300, 1000):
        est = [lindemann_sim(N, nM, k1, km1, k2, r, 10000, rng) for _ in range(3)]
        mean = sum(est) / len(est)
        sd = math.sqrt(sum((e - mean) ** 2 for e in est) / (len(est) - 1))
        exact = k1 * k2 * nM / (km1 * nM + k2)
        rows.append((nM, mean, sd, exact, k1 * nM, k1 * k2 / km1))
    return {"params": (k1, km1, k2, r, N), "rows": rows}


# =============================================================== E5
# Lattice gas.  Sites are channels; a particle moving in direction d is a
# message site!(p_d).  Laws, at every site s:
#   stream     for( @p_d <= s ) { (s + e_d)!(p_d) }          weight h   (k=1)
#   pair       for( @(p_d | p_{d+3}) <= s ) { s!(p_{d+1}) | s!(p_{d+4}) }
#              and the mirror image to (d-1, d+2), each     weight w2/2 (k=2)
#   triple     for( @(p_0|p_2|p_4) <= s ) { s!(p_1)|s!(p_3)|s!(p_5) } and back
#                                                            weight w3  (k=3)
# HPP: 4 directions, pair (E,W) <-> (N,S).

class Fenwick:
    def __init__(self, n):
        self.n = n
        self.t = [0.0] * (n + 1)
        self.v = [0.0] * n

    def set(self, i, val):
        d = val - self.v[i]
        if d == 0:
            return
        self.v[i] = val
        i += 1
        while i <= self.n:
            self.t[i] += d
            i += i & -i

    def total(self):
        s = 0.0
        i = self.n
        while i > 0:
            s += self.t[i]
            i -= i & -i
        return s

    def find(self, u):
        """smallest i with prefix sum > u"""
        pos = 0
        step = 1 << (self.n.bit_length())
        while step:
            nxt = pos + step
            if nxt <= self.n and self.t[nxt] <= u:
                pos = nxt
                u -= self.t[nxt]
            step >>= 1
        return min(pos, self.n - 1)


def lattice_run(geom, L, f, a, w2, w3, h, T, dt_obs, rng):
    if geom == "fhp":
        D = 6
        dirs = [(1, 0), (0, 1), (-1, 1), (-1, 0), (0, -1), (1, -1)]
        cx = [math.cos(math.pi * d / 3) for d in range(6)]
        ky_scale = math.sqrt(3) / 2          # y = (sqrt3/2) r
    else:
        D = 4
        dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)]   # E, N, W, S
        cx = [1.0, 0.0, -1.0, 0.0]
        ky_scale = 1.0
    S = L * L
    n = [[0] * D for _ in range(S)]
    phase = [math.sin(2 * math.pi * (s // L) / L) for s in range(S)]

    def poisson(lam):
        # Knuth; lam is small
        Lm = math.exp(-lam)
        k, p = 0, 1.0
        while True:
            p *= rng.random()
            if p <= Lm:
                return k
            k += 1

    for s in range(S):
        for d in range(D):
            lam = f * (1 + a * cx[d] * phase[s] * (2 if geom == "hpp" else 1))
            n[s][d] = poisson(max(lam, 0.0))

    cells = Fenwick(S * D)
    coll = Fenwick(S)
    half = D // 2

    def cweight(s):
        ns = n[s]
        c = w2 * sum(ns[d] * ns[d + half] for d in range(half))
        if geom == "fhp" and w3:
            c += w3 * (ns[0] * ns[2] * ns[4] + ns[1] * ns[3] * ns[5])
        return c

    A = 0.0
    for s in range(S):
        for d in range(D):
            cells.set(s * D + d, float(n[s][d]))
            A += n[s][d] * cx[d] * phase[s]
        coll.set(s, cweight(s))

    def neighbour(s, d):
        q, r_ = s % L, s // L
        dq, dr = dirs[d]
        return ((r_ + dr) % L) * L + (q + dq) % L

    t = 0.0
    obs = [(0.0, A)]
    next_obs = dt_obs
    events = 0
    while t < T:
        rs = h * cells.total()
        rc = coll.total()
        a0 = rs + rc
        t += rng.expovariate(a0)
        while next_obs <= t and next_obs <= T + 1e-9:
            obs.append((next_obs, A))
            next_obs += dt_obs
        if t >= T:
            break
        events += 1
        u = rng.random() * a0
        if u < rs:
            c = cells.find(u / h)
            s, d = divmod(c, D)
            if n[s][d] == 0:      # numerical edge
                continue
            s2 = neighbour(s, d)
            n[s][d] -= 1
            n[s2][d] += 1
            A += cx[d] * (phase[s2] - phase[s])
            cells.set(s * D + d, float(n[s][d]))
            cells.set(s2 * D + d, float(n[s2][d]))
            coll.set(s, cweight(s))
            coll.set(s2, cweight(s2))
        else:
            s = coll.find(u - rs)
            ns = n[s]
            opts = []
            for d in range(half):
                w = w2 * ns[d] * ns[d + half]
                if w > 0:
                    if geom == "fhp":
                        opts.append((w / 2, (d, d + 3), ((d + 1) % 6, (d + 4) % 6)))
                        opts.append((w / 2, (d, d + 3), ((d - 1) % 6, (d + 2) % 6)))
                    else:
                        opts.append((w, (d, d + 2), ((d + 1) % 4, (d + 3) % 4)))
            if geom == "fhp" and w3:
                w = w3 * ns[0] * ns[2] * ns[4]
                if w > 0:
                    opts.append((w, (0, 2, 4), (1, 3, 5)))
                w = w3 * ns[1] * ns[3] * ns[5]
                if w > 0:
                    opts.append((w, (1, 3, 5), (0, 2, 4)))
            tot = sum(o[0] for o in opts)
            if tot <= 0:
                continue
            v = rng.random() * tot
            for wgt, ins, outs in opts:
                if v < wgt:
                    break
                v -= wgt
            for d in ins:
                ns[d] -= 1
                A -= cx[d] * phase[s]
            for d in outs:
                ns[d] += 1
                A += cx[d] * phase[s]
            for d in set(ins) | set(outs):
                cells.set(s * D + d, float(ns[d]))
            coll.set(s, cweight(s))
    while len(obs) < int(round(T / dt_obs)) + 1:
        obs.append((next_obs, A))
        next_obs += dt_obs
    k = 2 * math.pi / (L * ky_scale)
    return obs, k, events


def fit_decay(series, floor=0.2):
    """Least-squares fit of log A against t over the window where the
    seed-averaged A/A0 stays above `floor`."""
    A0 = series[0][1]
    pts = [(t, math.log(v / A0)) for t, v in series if v / A0 > floor]
    if len(pts) < 3:
        return float("nan")
    n_ = len(pts)
    mt = sum(p[0] for p in pts) / n_
    ml = sum(p[1] for p in pts) / n_
    num = sum((p[0] - mt) * (p[1] - ml) for p in pts)
    den = sum((p[0] - mt) ** 2 for p in pts)
    return -num / den


def lattice_study(geom, L, f, a, w2, w3, h, T, dt_obs, seeds, base_seed):
    acc = None
    events = 0
    k = None
    for i in range(seeds):
        rng = random.Random(base_seed + 1000 * i)
        obs, k, ev = lattice_run(geom, L, f, a, w2, w3, h, T, dt_obs, rng)
        events += ev
        if acc is None:
            acc = [[t, v] for t, v in obs]
        else:
            for j, (t, v) in enumerate(obs):
                acc[j][1] += v
    series = [(t, v / seeds) for t, v in acc]
    gamma = fit_decay(series)
    return {"gamma": gamma, "nu": gamma / k ** 2, "k": k, "events": events,
            "series": series}


def plateau(series):
    """Mean of A/A0 over the second half of the run."""
    A0 = series[0][1]
    tail = series[len(series) // 2:]
    return sum(v for _, v in tail) / len(tail) / A0


def fit_nu(rows):
    """Least-squares fit nu = alpha / lam + beta over rows (lam, nu)."""
    xs = [1.0 / lam for lam, nu in rows]
    ys = [nu for lam, nu in rows]
    n_ = len(xs)
    mx, my = sum(xs) / n_, sum(ys) / n_
    alpha = sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / \
        sum((x - mx) ** 2 for x in xs)
    beta = my - alpha * mx
    res = max(abs(y - (alpha * x + beta)) for x, y in zip(xs, ys))
    return alpha, beta, res


def e5(quick=False):
    """Collision parameter lam = w2 * f; triple weight tied as w3 = w2 / f so
    that per-particle pair and triple collision rates both scale as lam."""
    out = {}
    L, a, h, T = 16, 0.5, 1.0, 60.0
    seeds = 2 if quick else 6
    k = 2 * math.pi / (L * math.sqrt(3) / 2)
    out["k"] = k
    # HPP: x-momentum on every row is a conserved quantity.
    r = lattice_study("hpp", L, 2.0, a, 1.0, 0.0, h, T, 1.0, seeds, 50)
    out["hpp"] = (plateau(r["series"]), r["events"])
    # FHP ideal gas: no collisions at all.
    r = lattice_study("fhp", L, 2.0, a, 0.0, 0.0, h, T, 1.0, seeds, 60)
    out["fhp_ideal"] = (plateau(r["series"]), r["events"])
    # FHP pair collisions only: J0 = n0 - n3 is conserved on every row.
    r = lattice_study("fhp", L, 2.0, a, 1.0, 0.0, h, T, 1.0, seeds, 65)
    out["fhp_pair"] = (plateau(r["series"]), r["events"])
    # FHP pair + triple collisions: the sweep.
    rows = []
    for f, w2 in ((2.0, 0.25), (2.0, 0.5), (1.0, 1.0), (2.0, 1.0),
                  (2.0, 2.0), (4.0, 1.0), (2.0, 4.0)):
        rr = lattice_study("fhp", L, f, a, w2, w2 / f, h, T, 1.0, seeds, 70)
        rows.append((f, w2, w2 * f, rr["gamma"], rr["nu"],
                     plateau(rr["series"]), rr["events"]))
    out["fhp_full"] = rows
    out["fit"] = fit_nu([(row[2], row[4]) for row in rows])
    # a decay curve for the figure
    out["curve"] = [(t, v / rr["series"][0][1]) for t, v in rr["series"]]
    return out


def e6(seeds=6, base=500):
    """Replicate the collapse points with fresh seeds and report the spread of
    per-run decay rates (standard error of the mean)."""
    L, a, h, T = 16, 0.5, 1.0, 60.0
    k = 2 * math.pi / (L * math.sqrt(3) / 2)
    rows = []
    for f, w2 in ((1.0, 1.0), (2.0, 0.5), (2.0, 2.0), (4.0, 1.0)):
        gs = []
        acc = None
        for i in range(seeds):
            rng = random.Random(base + 1000 * i)
            obs, k, ev = lattice_run("fhp", L, f, a, w2, w2 / f, h, T, 1.0, rng)
            gs.append(fit_decay(obs))
            if acc is None:
                acc = [[t, v] for t, v in obs]
            else:
                for j, (t, v) in enumerate(obs):
                    acc[j][1] += v
        g_avg = fit_decay([(t, v / seeds) for t, v in acc])
        m = sum(gs) / len(gs)
        se = math.sqrt(sum((g - m) ** 2 for g in gs) / (len(gs) - 1) / len(gs))
        rows.append((f, w2, w2 * f, g_avg / k ** 2, m / k ** 2, se / k ** 2))
    return rows


# =============================================================== main

def main():
    which = sys.argv[1:] or ["e1", "e2", "e3", "e4", "e5", "e6"]
    if "e1" in which:
        r = e1()
        print("E1 kernel = translation")
        for k, v in r.items():
            print(f"   {k:24s} {v}")
    if "e2" in which:
        r = e2()
        print("E2 ideal gas")
        print(f"   worst |rho - kernel(w_k>=2 = 0)| = {r['worst_rho_vs_kernel']:.3e}")
        for row in r["rates"]:
            print("   n=%2d  rate(w2=0)=%7.3f  rate(w2=0.1)=%7.3f  formula=%7.3f" % row)
        for row in r["jump"]:
            print("   w2=%5.2f  P(site 2 msgs)=%.6f  P(site 20 msgs)=%.6f" % row)
    if "e3" in which:
        r = e3()
        print("E3 propensity tables vs generic enumerator", r)
    if "e4" in which:
        r = e4()
        print("E4 Lindemann", r["params"])
        for row in r["rows"]:
            print("   M=%5d  k_uni=%.4f +- %.4f  exact=%.4f  low=%.1f  high=%.1f" % row)
    if "e5" in which:
        r = e5(quick="quick" in which)
        print("E5 lattice gas, k =", r["k"], " k^2 =", r["k"] ** 2)
        print("   HPP        plateau A/A0 = %.4f  events=%d" % r["hpp"])
        print("   FHP ideal  plateau A/A0 = %.4f  events=%d" % r["fhp_ideal"])
        print("   FHP pairs  plateau A/A0 = %.4f  events=%d" % r["fhp_pair"])
        for row in r["fhp_full"]:
            print("   f=%.1f w2=%.2f lam=%.2f  Gamma=%.5f  nu=%.4f  plateau=%.4f  events=%d" % row)
        print("   fit nu = alpha/lam + beta: alpha=%.4f beta=%.4f max residual=%.4f" % r["fit"])
        print("   curve", [(t, round(v, 4)) for t, v in r["curve"]])
    if "e6" in which:
        print("E6 collapse replicate: f, w2, lam, nu(avg series), nu(mean of runs), s.e.")
        for row in e6():
            print("   f=%.1f w2=%.2f lam=%.2f  nu=%.4f  mean=%.4f  se=%.4f" % row)


if __name__ == "__main__":
    main()
