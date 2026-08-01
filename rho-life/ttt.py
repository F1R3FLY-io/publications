"""Brute-force noughts-and-crosses, to ground every numeric claim in the note."""
from functools import lru_cache
from itertools import product

LINES = [(0,1,2),(3,4,5),(6,7,8),(0,3,6),(1,4,7),(2,5,8),(0,4,8),(2,4,6)]
CENTRE = 4
CORNERS = (0,2,6,8)
SIDES = (1,3,5,7)
OPP = {0:8, 8:0, 2:6, 6:2}

E, X, O = 0, 1, 2

def winner(b):
    for (i,j,k) in LINES:
        if b[i] != E and b[i]==b[j]==b[k]:
            return b[i]
    return None

def moves(b):
    return [i for i in range(9) if b[i]==E]

def play(b, i, p):
    l = list(b); l[i] = p; return tuple(l)

# ---------- reachability census ----------
def census():
    seen = set()
    terminal = 0
    games = 0
    def rec(b, p):
        nonlocal terminal, games
        seen.add(b)
        w = winner(b)
        if w is not None or not moves(b):
            terminal += 1
            return 1
        n = 0
        for m in moves(b):
            n += rec(play(b,m,p), O if p==X else X)
        return n
    games = rec(tuple([E]*9), X)
    return len(seen), terminal, games

# symmetry group of the square acting on cell indices
def sym_maps():
    idx = lambda r,c: r*3+c
    maps = []
    for k in range(4):
        for flip in (False, True):
            m = [0]*9
            for r in range(3):
                for c in range(3):
                    rr, cc = r, c
                    for _ in range(k):
                        rr, cc = cc, 2-rr
                    if flip:
                        cc = 2-cc
                    m[idx(r,c)] = idx(rr,cc)
            if m not in maps: maps.append(m)
    return maps
SYMS = sym_maps()

def canon(b):
    return min(tuple(b[m.index(i)] for i in range(9)) for m in SYMS)

def canon_census():
    seen = set()
    def rec(b,p):
        seen.add(canon(b))
        if winner(b) is not None or not moves(b): return
        for m in moves(b): rec(play(b,m,p), O if p==X else X)
    rec(tuple([E]*9), X)
    return len(seen)

# ---------- tactical predicates ----------
def winning_moves(b, p):
    return [m for m in moves(b) if winner(play(b,m,p))==p]

def threats(b, p):
    """cells that would complete a line for p right now"""
    return set(winning_moves(b,p))

def fork_moves(b, p):
    """moves after which p has >= 2 distinct winning cells (a double threat)"""
    out = []
    for m in moves(b):
        nb = play(b,m,p)
        if winner(nb) is not None: continue
        if len(threats(nb,p)) >= 2: out.append(m)
    return out

# ---------- exact minimax ----------
@lru_cache(maxsize=None)
def value(b, p):
    """value to X: +1 X wins, 0 draw, -1 O wins, with optimal play"""
    w = winner(b)
    if w == X: return 1
    if w == O: return -1
    if not moves(b): return 0
    vals = [value(play(b,m,p), O if p==X else X) for m in moves(b)]
    return max(vals) if p==X else min(vals)

def optimal_moves(b, p):
    best = value(b,p)
    return [m for m in moves(b) if value(play(b,m,p), O if p==X else X)==best]

# ---------- the ladder ----------
def rung_moves(b, p, rung):
    """Return the set of moves the rung-`rung` policy considers. Cumulative rules,
       highest priority first; ties broken uniformly at random by the evaluator."""
    q = O if p==X else X
    ms = moves(b)
    if rung >= 1:                      # WIN
        w = winning_moves(b,p)
        if w: return w
    if rung >= 2:                      # BLOCK
        w = winning_moves(b,q)
        if w: return w
    if rung >= 3:                      # FORK
        f = fork_moves(b,p)
        if f: return f
    if rung >= 4:                      # BLOCK FORK
        of = fork_moves(b,q)
        if of:
            if len(of) == 1:
                return of
            # force the opponent with a threat that does not create a fork for them
            forcing = []
            for m in ms:
                nb = play(b,m,p)
                if winner(nb) is not None: continue
                t = threats(nb,p)
                if len(t)==1:
                    reply = next(iter(t))
                    nb2 = play(nb,reply,q)
                    if not fork_moves(nb2,q) and reply not in of:
                        forcing.append(m)
            if forcing: return forcing
            return of
    if rung >= 5:                      # POSITIONAL (Newell & Simon tail)
        if CENTRE in ms: return [CENTRE]
        opp = [c for c in CORNERS if c in ms and b[OPP[c]]==q]
        if opp: return opp
        cor = [c for c in CORNERS if c in ms]
        if cor: return cor
        sid = [c for c in SIDES if c in ms]
        if sid: return sid
    return ms

# exact outcome distribution between two rung policies
def outcome(rx, ro):
    from functools import lru_cache as lc
    @lc(maxsize=None)
    def rec(b, p):
        w = winner(b)
        if w == X: return (1.0, 0.0, 0.0)
        if w == O: return (0.0, 0.0, 1.0)
        if not moves(b): return (0.0, 1.0, 0.0)
        cand = rung_moves(b, p, rx if p==X else ro)
        acc = [0.0,0.0,0.0]
        for m in cand:
            r = rec(play(b,m,p), O if p==X else X)
            for i in range(3): acc[i] += r[i]/len(cand)
        return tuple(acc)
    return rec(tuple([E]*9), X)

if __name__ == "__main__":
    n_pos, n_term, n_games = census()
    print("reachable positions (incl. start):", n_pos)
    print("terminal positions:", n_term)
    print("distinct games:", n_games)
    print("positions up to symmetry:", canon_census())

    # fork census
    tot = 0; forky = 0
    seen = set()
    def walk(b,p):
        global tot, forky
        if b in seen: return
        seen.add(b)
        if winner(b) is not None or not moves(b): return
        tot += 1
        if fork_moves(b,p): forky += 1
        for m in moves(b): walk(play(b,m,p), O if p==X else X)
    walk(tuple([E]*9), X)
    print(f"non-terminal positions: {tot}; of which a fork is available to the mover: {forky} ({100*forky/tot:.1f}%)")

    print("\nvalue of the empty board (optimal both sides):", value(tuple([E]*9), X))

    print("\n--- ladder vs random (rung 0) ---")
    print("rung |  as X: win/draw/loss     |  as O: win/draw/loss")
    for r in range(6):
        wx, dx, lx = outcome(r, 0)
        wo, do, lo = outcome(0, r)
        print(f"  {r}  |  {wx:.4f} {dx:.4f} {lx:.4f}  |  {lo:.4f} {do:.4f} {wo:.4f}")

    print("\n--- full round robin (row = X policy, col = O policy), entry = P(X win), P(draw), P(O win) ---")
    for rx in range(6):
        row = []
        for ro in range(6):
            w,d,l = outcome(rx,ro)
            row.append(f"{w:.2f}/{d:.2f}/{l:.2f}")
        print(f"X={rx}: " + "  ".join(row))
