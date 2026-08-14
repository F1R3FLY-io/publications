#!/usr/bin/env python3
"""
swarm.py -- the computations behind the honeybee half of the note.

Standard library only.  Every swarm number quoted in the research note is
produced by this file.  Run with:   python3 swarm.py

Model
-----
A swarm has S scouts and two candidate cavities A and B with qualities
q_A, q_B in (0,1].  A scout is uncommitted, or committed to A, or committed
to B; a committed scout counts toward the occupancy of her site.  Per tick:

  discovery     uncommitted -> i         with probability nu * q_i
  recruitment   uncommitted -> i         with probability rho * q_i * D_i / S
  abandonment   i -> uncommitted         with probability alpha
  cross-inhib.  a scout committed to i stops a scout committed to j != i
                with probability sigma

Recruitment is proportional to q_i * D_i because dance strength is
proportional to perceived quality and decays by a quality-independent
decrement per return trip (Seeley), so a site's total advocacy is
quality-weighted.  Abandonment is quality-independent for the same reason.

A site is DECIDED when its occupancy reaches the quorum threshold k.  The
swarm SPLITS when the second site also reaches k within the piping window W
(worker piping and warm-up take about an hour, during which a second quorum
would put two coalitions in the air).  The swarm DEADLOCKS when neither site
reaches k within the horizon.

The structural claim under test: two simultaneous quorums require 2k
committed scouts, so no swarm with S < 2k can split, whatever the qualities,
whatever the cross-inhibition, and however long one waits.
"""

import random

# ----------------------------------------------------------------- parameters

K       = 15      # quorum threshold (Seeley's reported figure is about 15)
NU      = 0.0060  # per-tick independent discovery
RHO     = 0.0500    # recruitment coefficient
ALPHA   = 0.0050   # abandonment (quality independent)
SIGMA   = 0.020   # cross-inhibition strength when enabled
HORIZON = 1500
WINDOW  = 60      # piping / warm-up window, in ticks
TRIALS  = 1000
SEED    = 20260813


def episode(rng, S, qA, qB, sigma, k=K, horizon=HORIZON, window=WINDOW,
            record=None):
    """One nest-site selection episode.

    Returns (outcome, t_decide, trace) where outcome is 'A', 'B', 'split'
    or 'deadlock'."""
    state = [0] * S                     # 0 uncommitted, 1 = A, 2 = B
    q = {1: qA, 2: qB}
    first = None
    first_t = None
    trace = {}
    record = record or []
    for t in range(horizon):
        D = {1: 0, 2: 0}
        for s in state:
            if s:
                D[s] += 1
        if t in record:
            trace[t] = (D[1], D[2])

        if first is None:
            for i in (1, 2):
                if D[i] >= k:
                    first, first_t = i, t
                    break
        if first is not None:
            other = 2 if first == 1 else 1
            if D[other] >= k:
                return 'split', first_t, trace
            if t - first_t >= window:
                return ('A' if first == 1 else 'B'), first_t, trace

        # dynamics.  Once a site has reached quorum its scouts are piping:
        # they neither abandon nor can be stopped, so a second quorum needs
        # k further scouts drawn from the remainder of the pool.
        for idx in range(S):
            s = state[idx]
            if s == 0:
                r = rng.random()
                pA = NU * qA + RHO * qA * D[1] / S
                pB = NU * qB + RHO * qB * D[2] / S
                if r < pA:
                    state[idx] = 1
                elif r < pA + pB:
                    state[idx] = 2
            elif s == first:
                continue                  # locked: piping
            else:
                if rng.random() < ALPHA:
                    state[idx] = 0
                elif sigma > 0 and rng.random() < sigma:
                    other = 2 if s == 1 else 1
                    if other == first:
                        continue          # cannot stop a piping scout
                    targets = [j for j in range(S) if state[j] == other]
                    if targets:
                        state[rng.choice(targets)] = 0
    return 'deadlock', None, trace


# ------------------------------------------------------------- measurement 1
def structural_threshold(qA=1.0, qB=1.0, trials=TRIALS):
    """Split and deadlock rates as the scout pool crosses 2k, with and
    without cross-inhibition.  Equal sites: the hardest case."""
    rows = []
    for S in (12, 16, 20, 24, 28, 29, 30, 31, 34, 40, 50):
        out = {}
        for sigma, tag in ((0.0, 'off'), (SIGMA, 'on')):
            rng = random.Random(SEED + S + int(sigma * 1000))
            c = {'A': 0, 'B': 0, 'split': 0, 'deadlock': 0}
            for _ in range(trials):
                o, _, _ = episode(rng, S, qA, qB, sigma)
                c[o] += 1
            out[tag] = (c['split'] / trials, c['deadlock'] / trials,
                        (c['A'] + c['B']) / trials)
        rows.append((S, out['off'], out['on']))
    return rows


# ------------------------------------------------------------- measurement 2
def unequal_sites(S=40, trials=TRIALS):
    """Cross-inhibition should matter most when the sites are near-equal."""
    rows = []
    for qB in (1.00, 0.95, 0.90, 0.80, 0.70):
        out = {}
        for sigma, tag in ((0.0, 'off'), (SIGMA, 'on')):
            rng = random.Random(SEED + int(qB * 100) + int(sigma * 1000))
            c = {'A': 0, 'B': 0, 'split': 0, 'deadlock': 0}
            for _ in range(trials):
                o, _, _ = episode(rng, S, 1.0, qB, sigma)
                c[o] += 1
            out[tag] = (c['split'] / trials, c['deadlock'] / trials,
                        c['A'] / max(c['A'] + c['B'], 1))
        rows.append((qB, out['off'], out['on']))
    return rows


# ------------------------------------------------------------- measurement 3
def one_sided_errors(k=K, trials=40000):
    """A scout senses the quorum by encounter rate, not by counting.

    With n scouts present at a cavity of effective area a, a sensing scout
    resident for a fixed spell registers each present scout independently
    with probability p = c / a.  Her verdict is 'quorum' iff her registered
    count reaches k.  Reports, per true occupancy n, the rate of false
    negatives and false positives."""
    rng = random.Random(SEED + 11)
    out = {}
    for a in (1.0, 1.25, 1.5, 2.0):
        p = min(1.0, 0.90 / a)
        rows = []
        for n in (k - 3, k, k + 3, k + 6, k + 10, k + 15):
            fn = fp = 0
            for _ in range(trials // 6):
                seen = sum(1 for _ in range(n) if rng.random() < p)
                if n >= k and seen < k:
                    fn += 1
                if n < k and seen >= k:
                    fp += 1
            rows.append((n, fn / (trials // 6), fp / (trials // 6)))
        # the apparent threshold: smallest n at which the verdict is more
        # often 'quorum' than not
        app = None
        for n in range(k, 4 * k):
            hits = sum(1 for _ in range(2000)
                       if sum(1 for _ in range(n) if rng.random() < p) >= k)
            if hits > 1000:
                app = n
                break
        out[a] = (p, rows, app)
    return out


# ------------------------------------------------------------- measurement 4
def margin_clock(S=22, qA=1.0, qB=0.9, obs_t=250, trials=800):
    """Does the advocacy margin at an undecided moment predict the wait
    until a quorum?"""
    rng = random.Random(SEED + 12)
    rows = []
    n = 0
    while n < trials:
        state_rng = random.Random(rng.random())
        o, td, trace = episode(state_rng, S, qA, qB, SIGMA, record=[obs_t])
        if obs_t not in trace:
            continue
        dA, dB = trace[obs_t]
        if max(dA, dB) >= K:
            continue                      # already decided
        if o == 'deadlock':
            wait = HORIZON - obs_t
        elif td is None or td < obs_t:
            continue
        else:
            wait = td - obs_t
        n += 1
        margin = abs(dA - dB) / max(S, 1)
        rows.append((margin, wait, o))
    rows.sort(key=lambda r: r[0])
    q = len(rows) // 4
    bands = [rows[:q], rows[q:2 * q], rows[2 * q:3 * q], rows[3 * q:]]
    out = []
    for b in bands:
        if not b:
            continue
        out.append((sum(x[0] for x in b) / len(b),
                    sum(x[1] for x in b) / len(b),
                    sum(1 for x in b if x[2] == 'deadlock') / len(b),
                    len(b)))
    return out, n


# ------------------------------------------------------------------- reporting

def main():
    print("=" * 78)
    print("NEST-SITE SELECTION -- computations")
    print(f"k={K} nu={NU} rho={RHO} alpha={ALPHA} sigma={SIGMA} "
          f"window={WINDOW} horizon={HORIZON} seed={SEED}")
    print("=" * 78)

    print(f"\n[S1] THE STRUCTURAL THRESHOLD  (equal sites, 2k = {2*K})")
    print(f"     {'S':<5}{'split off':>11}{'dead off':>10}{'clean off':>11}"
          f"{'split on':>11}{'dead on':>10}{'clean on':>11}")
    for S, off, on in structural_threshold():
        mark = "" if S >= 2 * K else "   S < 2k"
        print(f"     {S:<5}{off[0]:>11.4f}{off[1]:>10.4f}{off[2]:>11.4f}"
              f"{on[0]:>11.4f}{on[1]:>10.4f}{on[2]:>11.4f}{mark}")

    print("\n[S2] CROSS-INHIBITION AGAINST SITE DISPARITY  (S=40)")
    print(f"     {'q_B':<7}{'split off':>11}{'dead off':>10}{'best off':>10}"
          f"{'split on':>11}{'dead on':>10}{'best on':>10}")
    for qB, off, on in unequal_sites():
        print(f"     {qB:<7.2f}{off[0]:>11.4f}{off[1]:>10.4f}{off[2]:>10.4f}"
              f"{on[0]:>11.4f}{on[1]:>10.4f}{on[2]:>10.4f}")

    print("\n[S3] QUORUM ERRORS ARE ONE-SIDED  (encounter sensing)")
    res = one_sided_errors()
    for a, (p, rows, app) in res.items():
        print(f"     cavity area x{a}  (detection p={p:.3f})   "
              f"apparent threshold in true bees: {app}")
        print(f"       {'true n':<9}{'false neg':>12}{'false pos':>12}")
        for n, fn, fp in rows:
            print(f"       {n:<9}{fn:>12.4f}{fp:>12.4f}")

    print("\n[S4] THE ADVOCACY MARGIN AS A CLOCK  (S=22, q=1.0/0.9, t=250)")
    mc, n = margin_clock()
    print(f"     trials: {n}")
    print(f"     {'margin quartile':<18}{'mean margin':>13}"
          f"{'ticks to quorum':>18}{'deadlocked':>13}")
    for nm, (m, w, d, c) in zip(["lowest", "second", "third", "highest"], mc):
        print(f"     {nm:<18}{m:>13.4f}{w:>18.1f}{d:>13.4f}")

    print("\n" + "=" * 78)


if __name__ == "__main__":
    main()
