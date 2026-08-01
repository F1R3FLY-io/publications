"""Cost-instrumented ladder. Charges each rule by the modal depth of the formula
it installs, kappa(d) = 2^d per candidate examined, and computes the EXACT
expected metering per game alongside the exact outcome distribution."""
from functools import lru_cache
from ttt import (LINES, CENTRE, CORNERS, SIDES, OPP, E, X, O,
                 winner, moves, play, winning_moves, threats, fork_moves,
                 value, optimal_moves)

KAPPA = {1: 2, 2: 4, 3: 8, 4: 16, 5: 1, 6: 32}   # 2^depth; positional is depth 0-ish
# rule depths: win=1, block=2, fork=3, blockfork=4, positional=0(+1), chi=5

def decide(b, p, r):
    """returns (candidate move list, metering cost charged for this decision)"""
    q = O if p == X else X
    ms = moves(b)
    n = len(ms)
    cost = 0
    if r >= 6:
        return optimal_moves(b, p), KAPPA[6] * n
    if r >= 1:
        cost += KAPPA[1] * n
        w = winning_moves(b, p)
        if w: return w, cost
    if r >= 2:
        cost += KAPPA[2] * n
        w = winning_moves(b, q)
        if w: return w, cost
    if r >= 3:
        cost += KAPPA[3] * n
        f = fork_moves(b, p)
        if f: return f, cost
    if r >= 4:
        cost += KAPPA[4] * n
        of = fork_moves(b, q)
        if of:
            if len(of) == 1: return of, cost
            forcing = []
            for m in ms:
                nb = play(b, m, p)
                if winner(nb) is not None: continue
                t = threats(nb, p)
                if len(t) == 1:
                    rep = next(iter(t)); nb2 = play(nb, rep, q)
                    if not fork_moves(nb2, q) and rep not in of: forcing.append(m)
            if forcing: return forcing, cost
            # fall through to positional (the corrected rung)
    if r >= 5:
        cost += KAPPA[5] * n
        if CENTRE in ms: return [CENTRE], cost
        opp = [c for c in CORNERS if c in ms and b[OPP[c]] == q]
        if opp: return opp, cost
        cor = [c for c in CORNERS if c in ms]
        if cor: return cor, cost
        sid = [c for c in SIDES if c in ms]
        if sid: return sid, cost
    return ms, cost

def analyse(rx, ro):
    """exact (P(Xwin), P(draw), P(Owin), E[X's metering], E[O's metering])"""
    @lru_cache(maxsize=None)
    def rec(b, p):
        w = winner(b)
        if w == X: return (1.0, 0.0, 0.0, 0.0, 0.0)
        if w == O: return (0.0, 0.0, 1.0, 0.0, 0.0)
        if not moves(b): return (0.0, 1.0, 0.0, 0.0, 0.0)
        cand, c = decide(b, p, rx if p == X else ro)
        acc = [0.0] * 5
        for m in cand:
            r = rec(play(b, m, p), O if p == X else X)
            for i in range(5): acc[i] += r[i] / len(cand)
        if p == X: acc[3] += c
        else:      acc[4] += c
        return tuple(acc)
    return rec(tuple([E] * 9), X)

RUNGS = ["T (random)", "win", "+block", "+fork", "+block-fork", "+positional", "chi (exact)"]

if __name__ == "__main__":
    print("Exact expected metering per game, and outcome, as X against each opponent rung\n")
    print(f"{'':14s}" + "".join(f"  vs r={s}      " for s in range(7)))
    for r in range(7):
        row = f"{RUNGS[r]:14s}"
        for s in range(7):
            w, d, l, cx, co = analyse(r, s)
            row += f" {w:.2f}/{d:.2f}/{l:.2f}"
        print(row)

    print("\nExpected metering (own cost per game) as X:")
    for r in range(7):
        cs = [analyse(r, s)[3] for s in range(7)]
        print(f"{RUNGS[r]:14s} " + " ".join(f"{c:7.1f}" for c in cs))

    print("\nExpected metering (own cost per game) as O:")
    for r in range(7):
        cs = [analyse(s, r)[4] for s in range(7)]
        print(f"{RUNGS[r]:14s} " + " ".join(f"{c:7.1f}" for c in cs))

    # ---- the foraging inequality, made numeric ----
    print("\n=== Which rung maximises net yield?  net = sigma_prey * P(win) - price * E[cost] ===")
    print("rows: prey stack sigma; cols: opponent rung; entry: argmax rung (net at that rung)")
    for price in (0.05, 0.2, 1.0):
        print(f"\n  price per metered unit = {price}")
        print("   sigma |" + "".join(f"  opp r={s} " for s in range(7)))
        for sigma in (20, 50, 100, 200, 400, 800, 1600):
            row = f"  {sigma:5d} |"
            for s in range(7):
                best, bestv = None, -1e18
                for r in range(7):
                    w, d, l, cx, co = analyse(r, s)
                    net = sigma * w - price * cx
                    if net > bestv: bestv, best = net, r
                row += f"   {best} ({bestv:6.0f})"
            print(row)
