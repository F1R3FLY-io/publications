"""Computational check of the geometric rendering of first-order terms.

Scheme A ("barycentric"): the literal definition, hub at the simplex barycentre.
Scheme B ("corner"): hub at the right-angled apex over the simplex.
"""
import itertools, math, random, sys
import numpy as np

SIG = {"z": 0, "u": 1, "b": 2, "t": 3}
GAMMA = 1.0  # sibling gap
DELTA = 1.0  # stem overshoot
TOL = 1e-7


def layout(scheme):
    """Assign coordinates: returns (dim, stem[c], child[c][i]) unit vectors."""
    off, stem, child = 0, {}, {}
    for c, n in SIG.items():
        if scheme == "A":
            k = max(1, n)
            E = np.zeros((k, 0))
            idx = list(range(off, off + k))
            off += k
            stem[c], child[c] = ("diag", idx), [("w", idx, i) for i in range(n)]
        else:
            idx = list(range(off, off + n + 1))
            off += n + 1
            stem[c], child[c] = ("e", idx[0]), [("e", j) for j in idx[1:]]
    D = off

    def vec(spec):
        v = np.zeros(D)
        if spec[0] == "e":
            v[spec[1]] = 1.0
        elif spec[0] == "diag":
            v[spec[1]] = 1.0 / math.sqrt(len(spec[1]))
        else:
            _, idx, i = spec
            n = len(idx)
            if n == 1:
                return v
            v[idx] = -1.0 / n
            v[idx[i]] += 1.0
            v /= np.linalg.norm(v)
        return v

    return D, {c: vec(s) for c, s in stem.items()}, {c: [vec(s) for s in ch] for c, ch in child.items()}


def shape(t, scheme, L):
    """Returns (segments, r): hub at origin, r = bounding radius about the hub."""
    D, stem, child = L
    c, args = t[0], t[1:]
    n = len(args)
    segs = []
    if n == 0:
        rho = 0.0
    else:
        subs = [shape(a, scheme, L) for a in args]
        rmax = max(r for _, r in subs)
        if scheme == "A":
            cn = math.sqrt((n - 1) / (2.0 * n))
            disp = cn * (2 * rmax + GAMMA)
        else:
            disp = math.sqrt(2) * rmax + GAMMA
        for i, (ss, _) in enumerate(subs):
            v = disp * child[c][i]
            if disp > 0:
                segs.append((np.zeros(D), v))
            segs += [(p + v, q + v) for p, q in ss]
        rho = disp + rmax
    segs.append((np.zeros(D), (rho + DELTA) * stem[c]))
    return segs, rho + DELTA


def seg_dist(p1, q1, p2, q2):
    d1, d2, r = q1 - p1, q2 - p2, p1 - p2
    a, e, f = d1 @ d1, d2 @ d2, d2 @ r
    c, b = d1 @ r, d1 @ d2
    den = a * e - b * b
    s = min(max((b * f - c * e) / den, 0.0), 1.0) if den > 1e-12 else 0.0
    t = (b * s + f) / e
    if t < 0:
        t, s = 0.0, min(max(-c / a, 0.0), 1.0)
    elif t > 1:
        t, s = 1.0, min(max((b - c) / a, 0.0), 1.0)
    return np.linalg.norm(p1 + s * d1 - p2 - t * d2)


def collisions(segs):
    """Pairs of segments meeting other than at a single shared endpoint."""
    bad = 0
    for (p1, q1), (p2, q2) in itertools.combinations(segs, 2):
        shared = [(x, y1, y2) for x, y1 in ((p1, q1), (q1, p1)) for x2, y2 in ((p2, q2), (q2, p2))
                  if np.linalg.norm(x - x2) < TOL]
        if shared:
            x, y1, y2 = shared[0]
            a, b = y1 - x, y2 - x
            if a @ b > 0 and abs(a @ b - np.linalg.norm(a) * np.linalg.norm(b)) < TOL:
                bad += 1
        elif seg_dist(p1, q1, p2, q2) < TOL:
            bad += 1
    return bad


def canon(segs):
    """Canonical form of the union of segments as a point set."""
    lines = {}
    for p, q in segs:
        d = (q - p) / np.linalg.norm(q - p)
        k = next(i for i, x in enumerate(d) if abs(x) > TOL)
        if d[k] < 0:
            d = -d
        foot = p - (p @ d) * d
        key = (tuple(np.round(d, 5) + 0.0), tuple(np.round(foot, 5) + 0.0))
        lines.setdefault(key, []).append(tuple(sorted((p @ d, q @ d))))
    out = []
    for key, ivs in lines.items():
        ivs.sort()
        cur = list(ivs[0])
        for lo, hi in ivs[1:]:
            if lo <= cur[1] + TOL:
                cur[1] = max(cur[1], hi)
            else:
                out.append((key, round(cur[0], 5), round(cur[1], 5)))
                cur = [lo, hi]
        out.append((key, round(cur[0], 5), round(cur[1], 5)))
    return tuple(sorted(out))


def terms(depth):
    if depth == 0:
        return [(c,) for c, n in SIG.items() if n == 0]
    prev = terms(depth - 1)
    out = []
    for c, n in SIG.items():
        out += [(c,) + a for a in itertools.product(prev, repeat=n)]
    return out


def rand_term(depth, rng):
    c = rng.choice(list(SIG)) if depth > 0 else "z"
    return (c,) + tuple(rand_term(depth - 1, rng) for _ in range(SIG[c]))


def show(t):
    return t[0] if len(t) == 1 else f"{t[0]}({','.join(show(a) for a in t[1:])})"


def main():
    rng = random.Random(7)
    pool = {t for t in terms(2)}
    while len(pool) < 85 + 400:
        pool.add(rand_term(rng.choice([3, 4]), rng))
    pool = sorted(pool, key=lambda t: (len(show(t)), show(t)))
    for scheme in "AB":
        L = layout(scheme)
        ncoll, first, seen, clashes, rmax = 0, None, {}, [], 0
        for t in pool:
            segs, r = shape(t, scheme, L)
            rmax = max(rmax, r)
            if collisions(segs):
                ncoll += 1
                first = first or show(t)
            k = canon(segs)
            if k in seen:
                clashes.append((show(seen[k]), show(t)))
            seen[k] = t
        print(f"scheme {scheme}: dim={L[0]} terms={len(pool)} with-self-contact={ncoll} "
              f"(smallest: {first}) non-injective pairs={len(clashes)} max radius={rmax:.1f}")
        for c in clashes[:5]:
            print("   clash:", c)


if __name__ == "__main__":
    main()


# ---------------------------------------------------------------------------
# Realisation D ("chromatic"): argument position in space, constructor in colour.
# All constructors share the axes e_1..e_N (N = max arity); there are no stems.
# A figure is a set of coloured hubs plus the spokes joining them.

def shape_D(t, sig, N):
    """Returns (spokes, hubs, r); spokes = [(p, q, colour)], hubs = [(p, colour)]."""
    c, args = t[0], t[1:]
    hubs, spokes = [(np.zeros(N), c)], []
    if not args:
        return spokes, hubs, 0.0
    subs = [shape_D(a, sig, N) for a in args]
    rmax = max(r for _, _, r in subs)
    lam = math.sqrt(2) * rmax + GAMMA
    for i, (ss, hh, _) in enumerate(subs):
        v = np.zeros(N)
        v[i] = lam
        spokes.append((np.zeros(N), v, c))
        spokes += [(p + v, q + v, col) for p, q, col in ss]
        hubs += [(p + v, col) for p, col in hh]
    return spokes, hubs, lam + rmax


def check_D():
    rng = random.Random(7)
    pool = {t for t in terms(2)}
    while len(pool) < 85 + 400:
        pool.add(rand_term(rng.choice([3, 4]), rng))
    N = max(SIG.values())
    ncoll, seen, clashes, rmax = 0, {}, 0, 0
    for t in pool:
        spokes, hubs, r = shape_D(t, SIG, N)
        rmax = max(rmax, r)
        ncoll += bool(collisions([(p, q) for p, q, _ in spokes]))
        # hubs must be pairwise distinct and never interior to a spoke
        for (h, _), (p, q, _) in itertools.product(hubs, spokes):
            d = q - p
            foot = p + min(max((h - p) @ d / (d @ d), 0.0), 1.0) * d
            if min(np.linalg.norm(h - p), np.linalg.norm(h - q)) > TOL and np.linalg.norm(h - foot) < TOL:
                ncoll += 1
        key = tuple(sorted((tuple(np.round(p, 5) + 0.0), col) for p, col in hubs))
        clashes += key in seen
        seen[key] = t
    print(f"scheme D: dim={N} terms={len(pool)} with-self-contact={ncoll} "
          f"non-injective pairs={clashes} max radius={rmax:.2f}")


RHO = {"0": 0, "par": 2, "send": 2, "recv": 3, "quote": 1, "drop": 1, "y": 0}


def rho_figure(path):
    """TikZ for  for(y <- @0){*y} | @0!(0)  under Realisation D."""
    q0 = ("quote", ("0",))
    t = ("par", ("recv", ("y",), q0, ("drop", ("y",))), ("send", q0, ("0",)))
    spokes, hubs, r = shape_D(t, RHO, 3)
    P = np.array([[0.88, -0.30], [-0.62, -0.42], [0.0, 1.0]])  # axonometric projection
    lab = {"0": "$0$", "par": "$\\mid$", "send": "$!$", "recv": "for", "quote": "$@$", "drop": "$*$", "y": "$y$"}
    out = []
    for p, q, col in spokes:
        a, b = p @ P, q @ P
        out.append(f"\\draw[line width=1.3pt,c{col}] ({a[0]:.3f},{a[1]:.3f})--({b[0]:.3f},{b[1]:.3f});")
    for p, col in hubs:
        a = p @ P
        out.append(f"\\node[hub,fill=c{col}] at ({a[0]:.3f},{a[1]:.3f}) {{}};")
        out.append(f"\\node[lab] at ({a[0]:.3f},{a[1]:.3f}) {{\\scriptsize {lab[col]}}};")
    open(path, "w").write("\n".join(out) + "\n")
    print(f"rho example: radius={r:.3f} hubs={len(hubs)} spokes={len(spokes)}")
    for p, col in hubs:
        print("  ", col, np.round(p, 3))


if __name__ == "__main__":
    check_D()
    if len(sys.argv) > 1:
        rho_figure(sys.argv[1])
