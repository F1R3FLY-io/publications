#!/usr/bin/env python3
"""
herd.py -- the computations behind "Reading the Herd".

Standard library only.  Every number quoted in the research note is produced
by this file.  Run with:   python3 herd.py

Model
-----
A herd is N processes arranged on a ring.  Animal i carries a heading
s_i in {-1, 0, +1}  (-1 = break left, +1 = break right, 0 = undecided) and a
confidence g_i in [0,1] read off its own neighbourhood.

The semitopology (P, Open):  P = {0..N-1}; the basis is the set of contiguous
arcs of length >= k; Open is the closure of the basis under arbitrary unions,
together with the empty set.  This is a semitopology and not a topology: two
arcs of length k overlapping in fewer than k points meet in a non-open set.

Dynamics: asynchronous majority (a voter model with a window).  At each tick a
uniformly chosen animal adopts the sign of the weighted sum of headings in its
window; couplings may be weakened at designated seams to create clusters.
"""

import random
from math import inf

# ----------------------------------------------------------------- parameters

N      = 24      # herd size
W      = 2       # neighbourhood radius (window = 2W+1)
EPS    = 0.03    # per-update noise
TMAX   = 400     # tick horizon
TRIALS = 4000    # Monte Carlo trials per measurement
SEED   = 20260813

# pricing schedule (game-note kappa(d) = 2^d, charged per candidate examined)
C_HEAD  = 2      # observe one animal's heading           (spatial, depth 1)
C_CONF  = 4      # observe one animal's graded confidence (spatial, depth 2)
C_OPEN  = 4      # test one open set                      (the width engine)
C_STEP  = 8      # drive the herd forward one tick        (the depth engine)


# ------------------------------------------------------------------ mechanics

def couplings(seams):
    """Bond strength between i and i+1.  Seams are weakened bonds."""
    c = [1.0] * N
    for s in seams:
        c[s % N] = 0.0
    return c


def step(s, coup, rng):
    i = rng.randrange(N)
    tot = 0.0
    for d in range(-W, W + 1):
        if d == 0:
            continue
        j = (i + d) % N
        # bond strength along the path from i to j is the weakest link crossed
        lo, hi = (i, j) if d > 0 else (j, i)
        strength = 1.0
        for b in range(lo, lo + abs(d)):
            strength = min(strength, coup[b % N])
        tot += strength * s[j]
    if rng.random() < EPS:
        s[i] = rng.choice([-1, 1])
    elif tot > 0:
        s[i] = 1
    elif tot < 0:
        s[i] = -1


def confidence(s, coup, i):
    """What animal i deposits about itself: how firmly its neighbourhood agrees.

    This is structure the herd has already computed; the pride reads it rather
    than driving it (the read-vs-drive distinction of the game note)."""
    agree = 0.0
    tot = 0.0
    for d in range(-W, W + 1):
        j = (i + d) % N
        lo, hi = (i, j) if d >= 0 else (j, i)
        strength = 1.0
        for b in range(lo, lo + abs(d)):
            strength = min(strength, coup[b % N])
        tot += strength
        if s[j] == s[i] and s[i] != 0:
            agree += strength
    return agree / tot if tot else 0.0


def run_unfiltered(rng, seams=(), record_at=None):
    """As `run`, but returns the snapshots whether or not the herd concludes."""
    s = [0] * N
    coup = couplings(seams)
    s[rng.randrange(N)] = rng.choice([-1, 1])
    snaps = {}
    record_at = record_at or []
    for t in range(TMAX):
        if t in record_at:
            snaps[t] = list(s)
        step(s, coup, rng)
    return snaps


def run(rng, seams=(), record_at=None):
    """One stampede.  Returns (snapshots, ground_truth) or None if the herd
    fails to break (no majority at the horizon)."""
    s = [0] * N
    coup = couplings(seams)
    start = rng.randrange(N)
    s[start] = rng.choice([-1, 1])
    snaps = {}
    record_at = record_at or []
    for t in range(TMAX):
        if t in record_at:
            snaps[t] = list(s)
        step(s, coup, rng)
    tot = sum(s)
    if abs(tot) < 0.5 * N:
        return None                      # the herd fragmented: no conclusion
    return snaps, (1 if tot > 0 else -1)


# ------------------------------------------------------ the coalition modality

def arcs(k):
    return [[(a + d) % N for d in range(k)] for a in range(N)]


def crisp_coalition(s, k, direction):
    """[[ E-box (heading = direction) ]] = t  iff some open is uniformly so.
    A union of basis arcs is uniform iff each constituent arc is, so it is
    enough to test the N arcs of length exactly k.  Returns (holds, reads,
    opens_tested) with observations cached across arcs."""
    seen = set()
    tested = 0
    for arc in arcs(k):
        tested += 1
        ok = True
        for i in arc:
            seen.add(i)
            if s[i] != direction:
                ok = False
                break
        if ok:
            return True, len(seen), tested
    return False, len(seen), tested


def graded_coalition(s, coup, k):
    """<E>_V psi over the tropical/Viterbi algebra:
          value(direction) = max over opens O of  min over q in O of psi(q)
    with psi(q) = confidence(q) when q heads that way and 0 otherwise.
    Returns (best_direction, margin, witness_arc, reads, opens_tested)."""
    conf = {}
    tested = 0
    best = {-1: (0.0, None), 1: (0.0, None)}
    for arc in arcs(k):
        tested += 1
        for d in (-1, 1):
            m = inf
            for i in arc:
                if i not in conf:
                    conf[i] = confidence(s, coup, i)
                v = conf[i] if s[i] == d else 0.0
                m = min(m, v)
                if m == 0.0:
                    break
            if m > best[d][0]:
                best[d] = (m, arc)
    if best[1][0] >= best[-1][0]:
        d = 1
    else:
        d = -1
    margin = abs(best[1][0] - best[-1][0])
    return d, margin, best[d][1], len(conf), tested


# ---------------------------------------------------------------- the pride

def predict(rung, s, coup, k, rng, lookahead=6):
    """Returns (guess, cost).  guess in {-1, +1}; ties broken by a coin."""
    coin = lambda: rng.choice([-1, 1])

    if rung == 0:                                    # top: no theory at all
        return coin(), 0

    if rung == 1:                                    # one animal's heading
        i = rng.randrange(N)
        return (s[i] if s[i] != 0 else coin()), C_HEAD

    if rung == 2:                                    # sample of five
        idx = rng.sample(range(N), 5)
        tot = sum(s[i] for i in idx)
        return ((1 if tot > 0 else -1) if tot != 0 else coin()), 5 * C_HEAD

    if rung == 3:                                    # crisp coalition, E-box
        cost = 0
        holds = {}
        reads = 0
        for d in (1, -1):
            h, r, tested = crisp_coalition(s, k, d)
            holds[d] = h
            reads = max(reads, r)
            cost += C_OPEN * tested
        cost += C_HEAD * reads
        if holds[1] and not holds[-1]:
            return 1, cost
        if holds[-1] and not holds[1]:
            return -1, cost
        return 0, cost                               # bottom: no verdict

    if rung == 4:                                    # graded coalition
        d, margin, arc, reads, tested = graded_coalition(s, coup, k)
        cost = C_CONF * reads + C_OPEN * tested
        return (d if margin > 0.0 else 0), cost

    if rung == 5:                                    # topen: one member speaks
        i = rng.randrange(N)
        c = confidence(s, coup, i)
        if s[i] != 0 and c > 0.5:
            return s[i], C_CONF
        return 0, C_CONF

    if rung == 6:                                    # nu: drive it forward
        s2 = list(s)
        for _ in range(lookahead * N):
            step(s2, coup, rng)
        d, margin, arc, reads, tested = graded_coalition(s2, coup, k)
        drive = C_STEP * lookahead * N
        return d, C_CONF * N + C_OPEN * tested + drive

    raise ValueError(rung)


RUNGS = {
    0: r"$\top$ (no theory)",
    1: "one heading",
    2: "sample of five",
    3: r"crisp $\exists\Box$",
    4: r"graded $\langle E\rangle$",
    5: "topen member",
    6: r"$\nu$ (drive forward)",
}


# ------------------------------------------------------------- measurement 1
def ladder(k=13, obs_t=90, seams=(), trials=TRIALS):
    """Coverage, precision and cost per rung.

    A rung may return 0 -- the third truth value.  Coverage is the fraction of
    trials on which it issues a verdict at all; precision is its accuracy given
    that it does.  Accuracy folds the two together by tossing a coin on bottom,
    which is what an animal that must commit actually does."""
    rng = random.Random(SEED)
    verdicts = {r: 0 for r in RUNGS}
    hits = {r: 0 for r in RUNGS}
    acc = {r: 0.0 for r in RUNGS}
    cost = {r: 0.0 for r in RUNGS}
    n = 0
    while n < trials:
        out = run(rng, seams, record_at=[obs_t])
        if out is None:
            continue
        snaps, truth = out
        s = snaps[obs_t]
        coup = couplings(seams)
        n += 1
        for r in RUNGS:
            g, c = predict(r, s, coup, k, rng)
            cost[r] += c
            if g != 0:
                verdicts[r] += 1
                hits[r] += (g == truth)
                acc[r] += (g == truth)
            else:
                acc[r] += 0.5
    return {r: (verdicts[r] / n,
                (hits[r] / verdicts[r]) if verdicts[r] else 0.0,
                acc[r] / n,
                cost[r] / n) for r in RUNGS}, n


# ------------------------------------------------------------- measurement 1b
def reversal(k=13, times=(30, 50, 70, 90, 120), trials=3000):
    """The question the coalition modality actually answers.

    A sample majority reports which way the herd is LEANING; the coalition
    modality reports that it has COMMITTED.  Measure, at each time, the
    precision of each verdict -- equivalently one minus the reversal rate."""
    rng = random.Random(SEED + 7)
    samp = {t: [0, 0] for t in times}     # [verdicts, correct]
    coal = {t: [0, 0] for t in times}
    n = 0
    while n < trials:
        out = run(rng, record_at=list(times))
        if out is None:
            continue
        snaps, truth = out
        coup = couplings(())
        n += 1
        for t in times:
            s = snaps[t]
            g2, _ = predict(2, s, coup, k, rng)
            if g2 != 0:
                samp[t][0] += 1
                samp[t][1] += (g2 == truth)
            g3, _ = predict(3, s, coup, k, rng)
            if g3 != 0:
                coal[t][0] += 1
                coal[t][1] += (g3 == truth)
    return {t: (samp[t][0] / n, samp[t][1] / max(samp[t][0], 1),
                coal[t][0] / n, coal[t][1] / max(coal[t][0], 1))
            for t in times}, n


# ------------------------------------------------------------- measurement 1c
def twinedness_witness(k_below=12, k_above=13):
    """A structural check, not a statistical one.  Exhibit a state on which two
    disjoint coalitions are decided at k = N/2, and verify that no state can do
    so at k > N/2."""
    s = [-1] * (N // 2) + [1] * (N // 2)
    L = crisp_coalition(s, k_below, -1)[0]
    R = crisp_coalition(s, k_below, 1)[0]
    L2 = crisp_coalition(s, k_above, -1)[0]
    R2 = crisp_coalition(s, k_above, 1)[0]
    # exhaustive: at k > N/2 any two arcs of length k on the ring intersect
    worst = None
    for a in range(N):
        for b in range(N):
            A = set((a + d) % N for d in range(k_above))
            B = set((b + d) % N for d in range(k_above))
            if not (A & B):
                worst = (a, b)
    return (L, R), (L2, R2), worst


# ------------------------------------------------------------- measurement 2
def earliness(k=13, target=0.80, seams=(), trials=1200):
    """Earliest observation time at which each rung reaches `target` accuracy."""
    times = list(range(10, 200, 10))
    rng = random.Random(SEED + 1)
    acc = {r: {t: 0.0 for t in times} for r in RUNGS}
    n = 0
    while n < trials:
        out = run(rng, seams, record_at=times)
        if out is None:
            continue
        snaps, truth = out
        coup = couplings(seams)
        n += 1
        for t in times:
            for r in RUNGS:
                g, _ = predict(r, snaps[t], coup, k, rng)
                acc[r][t] += 0.5 if g == 0 else (1.0 if g == truth else 0.0)
    first = {}
    for r in RUNGS:
        first[r] = None
        for t in times:
            if acc[r][t] / n >= target:
                first[r] = t
                break
    return first, {r: {t: acc[r][t] / n for t in times} for r in RUNGS}, n


# ------------------------------------------------------------- measurement 3
def twinedness(obs_t=90, trials=2500):
    """P(both directions coalition-decided) as k varies.  2-twinedness on the
    ring is exactly k > N/2, and above that threshold the probability must be
    zero for structural reasons, not statistical ones."""
    out = {}
    for k in range(4, N + 1, 1):
        rng = random.Random(SEED + 2 + k)
        both = 0
        one = 0
        none = 0
        n = 0
        while n < trials:
            r = run(rng, record_at=[obs_t])
            if r is None:
                continue
            s = r[0][obs_t]
            n += 1
            L = crisp_coalition(s, k, -1)[0]
            R = crisp_coalition(s, k, 1)[0]
            if L and R:
                both += 1
            elif L or R:
                one += 1
            else:
                none += 1
        out[k] = (both / n, one / n, none / n)
    return out


# ------------------------------------------------------------- measurement 4
def cohesion_vs_legibility(obs_t=90, trials=2000):
    """The prey's counter-move.  Weakening cohesion (more seams) buys
    illegibility and costs consensus.  Reports, per seam count:
      P(herd reaches a conclusion at all), and pride accuracy at rung 4."""
    out = {}
    for nseams in range(0, 7):
        seams = tuple(round(i * N / nseams) % N for i in range(nseams)) if nseams else ()
        rng = random.Random(SEED + 3 + nseams)
        coup = couplings(seams)
        decided = 0
        attempts = 0
        verdicts = 0
        wins = 0
        n = 0
        while n < trials and attempts < trials * 12:
            attempts += 1
            r = run(rng, seams, record_at=[obs_t])
            if r is None:
                continue
            decided += 1
            snaps, truth = r
            n += 1
            g, _ = predict(4, snaps[obs_t], coup, 13, rng)
            if g != 0:
                verdicts += 1
                wins += (g == truth)
        out[nseams] = (decided / attempts,
                       verdicts / max(n, 1),
                       wins / max(verdicts, 1), n)
    return out


# ------------------------------------------------------------- measurement 5
def budget_confusion(k=13, obs_t=90, budgets=(60, 120, 200, 320, 480), trials=2500):
    """How often is a bottom verdict the herd's indeterminacy, and how often is
    it the pride's poverty?  From outside the cut the two are one value."""
    rng = random.Random(SEED + 4)
    out = {b: [0, 0, 0] for b in budgets}    # [bottom-under-budget, of which
    n = 0                                    #  decided-with-full-purse, total]
    while n < trials:
        r = run(rng, record_at=[obs_t])
        if r is None:
            continue
        s = r[0][obs_t]
        n += 1
        # full-purse verdict
        full = {}
        spend = {}
        for d in (1, -1):
            h, reads, tested = crisp_coalition(s, k, d)
            full[d] = h
            spend[d] = C_HEAD * reads + C_OPEN * tested
        decided_full = full[1] != full[-1]
        for b in budgets:
            # a budgeted scan stops when the purse runs out
            affordable = {}
            for d in (1, -1):
                affordable[d] = full[d] and spend[d] <= b
            decided_b = affordable[1] != affordable[-1]
            out[b][2] += 1
            if not decided_b:
                out[b][0] += 1
                if decided_full:
                    out[b][1] += 1
    return {b: (v[0] / v[2], (v[1] / v[0]) if v[0] else 0.0) for b, v in out.items()}, n


# ------------------------------------------------------------- measurement 6
def topen_multiplier(trials=3000, obs_t=90):
    """One observation buys how many predictions?  In a two-cluster herd the
    clusters are the topens; sample one member and predict the rest."""
    seams = (0, N // 2)
    rng = random.Random(SEED + 5)
    inside_hits = inside_tot = 0
    outside_hits = outside_tot = 0
    n = 0
    while n < trials:
        r = run_unfiltered(rng, seams, record_at=[obs_t])
        s = r[obs_t]
        n += 1
        i = rng.randrange(N)
        cluster_i = 0 if i < N // 2 else 1
        for j in range(N):
            if j == i or s[j] == 0 or s[i] == 0:
                continue
            cluster_j = 0 if j < N // 2 else 1
            if cluster_i == cluster_j:
                inside_tot += 1
                inside_hits += (s[j] == s[i])
            else:
                outside_tot += 1
                outside_hits += (s[j] == s[i])
    return (inside_hits / inside_tot, inside_tot / n,
            outside_hits / outside_tot, outside_tot / n, n)


# ------------------------------------------------------------- measurement 7
def margin_predicts_commitment(k=13, k_warn=7, obs_t=60, trials=1500):
    """What the grading buys that the crisp modality does not -- and the price.

    At k > N/2 the crisp verdict is already functional, so grading adds nothing
    to WHICH WAY.  Worse, min over a coalition of 13 is zeroed by a single
    dissenter, so the graded reading at the same k is crisp in disguise: this is
    why [1] reports identical numbers for the two rungs.  The grading pays only
    once the coalition size is RELAXED as well.  Here the pride carries a
    weaker hypothesis -- coalitions of size k_warn -- purely as an early-warning
    signal, and we ask what its margin predicts about the wait until a genuine
    coalition at k is decided."""
    rng = random.Random(SEED + 8)
    rows = []
    n = 0
    while n < trials:
        out = run(rng, record_at=[obs_t])
        if out is None:
            continue
        snaps, truth = out
        s = snaps[obs_t]
        coup = couplings(())
        if crisp_coalition(s, k, 1)[0] or crisp_coalition(s, k, -1)[0]:
            continue                       # already committed; nothing to warn
        n += 1
        d, margin, arc, reads, tested = graded_coalition(s, coup, k_warn)
        s2 = list(s)
        wait = None
        for tick in range(1, 12 * N + 1):
            step(s2, coup, rng)
            if tick % (N // 4) == 0:
                if crisp_coalition(s2, k, 1)[0] or crisp_coalition(s2, k, -1)[0]:
                    wait = tick
                    break
        if wait is None:
            wait = 12 * N
        rows.append((margin, wait, d == truth))
    rows.sort(key=lambda r: r[0])
    q = len(rows) // 4
    bands = [rows[:q], rows[q:2 * q], rows[2 * q:3 * q], rows[3 * q:]]
    out = []
    for b in bands:
        if not b:
            continue
        out.append((sum(x[0] for x in b) / len(b),
                    sum(x[1] for x in b) / len(b),
                    sum(1 for x in b if x[2]) / len(b),
                    len(b)))
    return out, n


# ------------------------------------------------------------------- reporting

def main():
    print("=" * 74)
    print("READING THE HERD -- computations")
    print(f"N={N} W={W} eps={EPS} TMAX={TMAX} seed={SEED}")
    print("=" * 74)

    print("\n[1] THE LADDER  (k=13, observation at t=90)")
    tab, n = ladder()
    print(f"    trials with a conclusion: {n}")
    print(f"    {'rung':<26}{'coverage':>10}{'precision':>11}{'accuracy':>10}{'cost':>9}{'return':>12}")
    prev_a = prev_c = None
    order = sorted(RUNGS, key=lambda r: tab[r][3])
    for r in order:
        cv, pr, a, c = tab[r]
        ret = "" if prev_a is None or c == prev_c else f"{(a-prev_a)/(c-prev_c):+.5f}"
        print(f"    {RUNGS[r]:<26}{cv:>10.4f}{pr:>11.4f}{a:>10.4f}{c:>9.1f}{ret:>12}")
        prev_a, prev_c = a, c

    print("\n[1b] LEAN VERSUS COMMITMENT  (precision of each verdict over time)")
    rv, nr = reversal()
    print(f"    trials: {nr}")
    print(f"    {'t':<6}{'sample cover':>14}{'sample prec':>13}{'coal cover':>13}{'coal prec':>12}")
    for t, (sc, sp, cc, cp) in rv.items():
        print(f"    {t:<6}{sc:>14.4f}{sp:>13.4f}{cc:>13.4f}{cp:>12.4f}")

    print("\n[1c] TWINEDNESS, STRUCTURALLY")
    below, above, worst = twinedness_witness()
    print(f"    split state, k=12: (L,R) decided = {below}")
    print(f"    split state, k=13: (L,R) decided = {above}")
    print(f"    disjoint pair of arcs of length 13 on the ring: {worst}")

    print("\n[2] EARLINESS  (first t reaching 80% accuracy)")
    first, curves, n2 = earliness()
    print(f"    trials: {n2}")
    for r in RUNGS:
        print(f"    {RUNGS[r]:<26}{str(first[r]):>8}")
    print("    accuracy curves (t = 20,40,60,80,100,140):")
    for r in RUNGS:
        row = "  ".join(f"{curves[r][t]:.3f}" for t in (20, 40, 60, 80, 100, 140))
        print(f"    {RUNGS[r]:<26}{row}")

    print("\n[3] TWINEDNESS  (P both decided / one / neither), N/2 = %d" % (N // 2))
    tw = twinedness()
    for k, (b, o, z) in tw.items():
        mark = "  <- 2-twined" if k > N / 2 else ""
        print(f"    k={k:<3} both={b:.4f}  one={o:.4f}  neither={z:.4f}{mark}")

    print("\n[4] COHESION VERSUS LEGIBILITY  (the prey's counter-move)")
    cv = cohesion_vs_legibility()
    print(f"    {'seams':<8}{'P(conclusion)':>16}{'pride coverage':>17}{'pride precision':>18}")
    for ns, (p, cov, pr, n3) in cv.items():
        print(f"    {ns:<8}{p:>16.4f}{cov:>17.4f}{pr:>18.4f}")

    print("\n[5] BOTTOM: INDETERMINACY OR POVERTY?  (k=13, t=90)")
    bc, n4 = budget_confusion()
    print(f"    trials: {n4}")
    print(f"    {'budget':<10}{'P(bottom)':>12}{'of which affordable-if-rich':>30}")
    for b, (pb, frac) in bc.items():
        print(f"    {b:<10}{pb:>12.4f}{frac:>30.4f}")

    print("\n[6] TOPEN MULTIPLIER  (two clusters, seams at 0 and N/2)")
    ih, isz, oh, osz, n5 = topen_multiplier()
    print(f"    trials: {n5}")
    print(f"    same cluster : agreement {ih:.4f} over {isz:.1f} predictions/observation")
    print(f"    across seam  : agreement {oh:.4f} over {osz:.1f} predictions/observation")
    print(f"    multiplier   : {isz * (2*ih-1):.2f} correct-net predictions per observation inside")

    print("\n[7] WHAT THE GRADING BUYS  (relaxed k=7 margin at t=60, herd uncommitted at k=13)")
    mp, n6 = margin_predicts_commitment()
    print(f"    trials: {n6}")
    print(f"    {'margin quartile':<18}{'mean margin':>13}{'ticks to commit':>18}{'direction right':>18}")
    names = ["lowest", "second", "third", "highest"]
    for nm, (m, w, a, c) in zip(names, mp):
        print(f"    {nm:<18}{m:>13.4f}{w:>18.1f}{a:>18.4f}")

    print("\n" + "=" * 74)


if __name__ == "__main__":
    main()
