#!/usr/bin/env python3
"""
engines.py -- computations for the fourth rho-life note,
"Engines and Individuals".

Four blocks:
  (1) the price code: effective resistance = detection strength
  (2) individuation: isoperimetry, r*, and the merged-population bound
  (3) value decay along a pathway and the profitable radius
  (4) inventory engines: state-dependent rates make arbitrage a
      property of the marking

Exact where exact is possible; float where the model is a model.
"""

import itertools
import numpy as np
import networkx as nx
from fractions import Fraction

np.set_printoptions(precision=4, suppress=True)
rng = np.random.default_rng(20260801)


# --------------------------------------------------------------------
# shared graph machinery
# --------------------------------------------------------------------

def incidence(G, edges):
    """Signed incidence B: |E| x |V|, row for u->v is e_u - e_v."""
    idx = {v: i for i, v in enumerate(G.nodes())}
    B = np.zeros((len(edges), G.number_of_nodes()))
    for k, (u, v) in enumerate(edges):
        B[k, idx[u]] = 1.0
        B[k, idx[v]] = -1.0
    return B


def cut_projector(B):
    """Orthogonal projection onto im(B) = the exact (gradient) 1-cochains."""
    L = B.T @ B
    Lp = np.linalg.pinv(L)
    return B @ Lp @ B.T


def edge_resistances(G):
    """R_eff for every edge, unit conductances."""
    edges = list(G.edges())
    B = incidence(G, edges)
    P = cut_projector(B)
    return edges, np.diag(P).copy(), P


def betti(G):
    return G.number_of_edges() - G.number_of_nodes() + nx.number_connected_components(G)


# --------------------------------------------------------------------
# (1) the price code
# --------------------------------------------------------------------

def barbell(k=5, cluster=nx.complete_graph):
    """Two dense clusters joined by a single bridge."""
    A = cluster(k)
    Gh = nx.Graph()
    for (u, v) in A.edges():
        Gh.add_edge(("a", u), ("a", v))
        Gh.add_edge(("b", u), ("b", v))
    Gh.add_edge(("a", 0), ("b", 0))          # the bridge
    return Gh


def price_code_demo():
    print("=" * 68)
    print("(1) THE PRICE CODE")
    print("=" * 68)

    G = barbell(5)
    edges, Reff, P = edge_resistances(G)
    b1 = betti(G)
    print(f"barbell(K5,K5,bridge): |V|={G.number_of_nodes()} "
          f"|E|={G.number_of_edges()}  beta_1={b1}")

    bridge = (("a", 0), ("b", 0))
    # true log-rates come from a potential: exact, hence arbitrage-free
    p = {v: rng.normal() for v in G.nodes()}
    ell = np.array([p[u] - p[v] for (u, v) in edges])
    syn_clean = np.linalg.norm(ell - P @ ell)
    print(f"clean system: ||syndrome|| = {syn_clean:.3e}  (0 to machine eps)")

    print("\n  per-edge detection strength  1 - R_eff  "
          "(= squared syndrome per unit corruption)")
    rows = []
    a = 1.0
    for k, e in enumerate(edges):
        ell_bad = ell.copy()
        ell_bad[k] += a
        syn = np.linalg.norm(ell_bad - P @ ell_bad) ** 2
        rows.append((e, Reff[k], 1 - Reff[k], syn))
    # show the bridge and the extremes
    rows.sort(key=lambda r: r[2])
    for e, R, d, syn in rows[:3] + rows[-3:]:
        tag = "  <-- BRIDGE" if set(e) == set(bridge) else ""
        print(f"    {str(e):22s} R_eff={R:.4f}  1-R_eff={d:.4f}  "
              f"||syn||^2={syn:.4f}{tag}")
    err = max(abs((1 - R) - syn) for _, R, _, syn in rows)
    print(f"  identity ||syn||^2 = a^2 (1 - R_eff) verified to {err:.2e}")

    # Foster: sum of R_eff over edges = |V| - 1, so the total
    # detection strength of the code is exactly beta_1.
    tot = float((1 - Reff).sum())
    print(f"  TOTAL detection strength sum_e (1-R_eff) = {tot:.6f}"
          f"   beta_1 = {b1}   (Foster)")
    for name, Gx in (("K5", nx.complete_graph(5)),
                     ("C6", nx.cycle_graph(6)),
                     ("grid 5x5", nx.grid_2d_graph(5, 5)),
                     ("3-reg n=60", nx.random_regular_graph(3, 60, seed=7))):
        _, Rx, _ = edge_resistances(Gx)
        print(f"    {name:12s} sum(1-R_eff)={float((1-Rx).sum()):9.5f}"
              f"  beta_1={betti(Gx)}")

    # locatability: are the cycle-space signatures pairwise non-parallel?
    Q = np.eye(len(edges)) - P
    sig = Q[:, :]          # column e of Q is Pi_cyc 1_e
    def unit(x):
        n = np.linalg.norm(x)
        return x / n if n > 1e-12 else x
    confusable = []
    for i, j in itertools.combinations(range(len(edges)), 2):
        u, v = unit(sig[:, i]), unit(sig[:, j])
        if np.linalg.norm(u) < 1e-9 or np.linalg.norm(v) < 1e-9:
            continue
        if abs(abs(u @ v) - 1.0) < 1e-9:
            confusable.append((edges[i], edges[j]))
    print(f"  confusable edge pairs in barbell(K5): {len(confusable)}")

    # a series chain: two engines with a degree-2 vertex between them
    H = nx.cycle_graph(6)
    eH, RH, PH = edge_resistances(H)
    QH = np.eye(len(eH)) - PH
    pairs = 0
    for i, j in itertools.combinations(range(len(eH)), 2):
        u, v = unit(QH[:, i]), unit(QH[:, j])
        if abs(abs(u @ v) - 1.0) < 1e-9:
            pairs += 1
    print(f"  C6 (every vertex of degree 2): beta_1={betti(H)}, "
          f"confusable pairs={pairs} of {len(eH)*(len(eH)-1)//2}"
          f"  -> series engines are indistinguishable")

    # detection strength versus density, d-regular expanders
    print("\n  mean detection strength of a random d-regular graph, n=60")
    for d in (3, 4, 6, 10, 20):
        Gd = nx.random_regular_graph(d, 60, seed=7)
        _, R, _ = edge_resistances(Gd)
        print(f"    d={d:2d}: mean 1-R_eff = {1 - R.mean():.4f}   "
              f"(2/d bound gives {1 - 2/d:.4f})")
    return G


# --------------------------------------------------------------------
# (2) individuation
# --------------------------------------------------------------------

def isoperimetric_profile(G, source, radii):
    """|dB(r)| / |B(r)| for balls around `source`."""
    dist = nx.single_source_shortest_path_length(G, source)
    out = []
    for r in radii:
        ball = {v for v, dd in dist.items() if dd <= r}
        if len(ball) == G.number_of_nodes():
            out.append((r, len(ball), 0, 0.0))
            continue
        bd = sum(1 for (u, v) in G.edges()
                 if (u in ball) != (v in ball))
        out.append((r, len(ball), bd, bd / len(ball)))
    return out


def individuation_demo():
    print()
    print("=" * 68)
    print("(2) INDIVIDUATION: h(B) >= eps*theta*r/rho")
    print("=" * 68)

    fams = {
        "cycle C_400 (growth 1)": (nx.cycle_graph(400), 0),
        "grid 20x20 (growth 2)": (nx.grid_2d_graph(20, 20), (10, 10)),
        "3-regular expander n=400": (nx.random_regular_graph(3, 400, seed=3), 0),
        "6-regular expander n=400": (nx.random_regular_graph(6, 400, seed=3), 0),
    }
    for name, (G, src) in fams.items():
        prof = isoperimetric_profile(G, src, [1, 2, 3, 4, 6, 8])
        s = "  ".join(f"r={r}:h={h:.3f}" for r, _, _, h in prof)
        print(f"  {name:28s} {s}")

    print("\n  merged population at the threshold, kappa := eps*theta/rho")
    print("  (largest ball with h(B(r)) >= kappa*r)")
    print(f"  {'kappa':>8s} {'C_400':>10s} {'grid':>10s} "
          f"{'3-reg':>10s} {'6-reg':>10s}")
    for kappa in (0.30, 0.10, 0.03, 0.01):
        row = []
        for name, (G, src) in fams.items():
            best = 1
            for r, size, _, h in isoperimetric_profile(
                    G, src, range(1, 25)):
                if h >= kappa * r:
                    best = size
            row.append(best)
        print(f"  {kappa:8.2f} " + " ".join(f"{x:10d}" for x in row))

    print("\n  reading: polynomial growth gives |B(r*)| ~ (r*)^d;")
    print("  expansion gives |B(r*)| ~ exp(c r*).  Density buys")
    print("  exponentially larger individuals for the same eps, theta, rho.")


# --------------------------------------------------------------------
# (3) value decay and the profitable radius
# --------------------------------------------------------------------

def decay_demo():
    print()
    print("=" * 68)
    print("(3) VALUE DECAY AND THE PROFITABLE RADIUS")
    print("=" * 68)
    print("  delivered value  Y e^{-theta d};  round-trip toll  c d;")
    print("  profitable iff  Y e^{-theta d} > c d.")
    Y, c = 1000.0, 20.0
    print(f"\n  Y={Y:.0f}, c={c:.0f}")
    print(f"  {'theta':>8s} {'d*':>8s}  {'interpretation'}")
    for theta, tag in ((0.05, "cheap: within a component"),
                       (0.20, "moderate"),
                       (2.303, "trophic step, r=1/10"),
                       (5.00, "thin interstellar bridge")):
        lo, hi = 0.0, 500.0
        for _ in range(200):
            mid = (lo + hi) / 2
            if Y * np.exp(-theta * mid) > c * mid:
                lo = mid
            else:
                hi = mid
        print(f"  {theta:8.3f} {lo:8.3f}  {tag}")
    print("\n  a chain of trophic engines at r=1/10 has theta=ln10=2.303;")
    print("  the ecological pyramid IS exponential decay of nu along a path.")


# --------------------------------------------------------------------
# (4) inventory engines: arbitrage as a property of the marking
# --------------------------------------------------------------------

def amm_demo():
    print()
    print("=" * 68)
    print("(4) INVENTORY ENGINES: ARBITRAGE IS A PROPERTY OF THE MARKING")
    print("=" * 68)
    print("  three colours a,b,c; three engines, each holding a reserve")
    print("  pair; constant-product rate r(x->y) = R_y / R_x.")

    # a triangle: beta_1 = 1, exactly one arbitrage degree of freedom
    res = {("a", "b"): [Fraction(100), Fraction(100)],
           ("b", "c"): [Fraction(100), Fraction(100)],
           ("c", "a"): [Fraction(100), Fraction(100)]}

    def cycle_yield(res):
        y = Fraction(1)
        for (x, yy) in (("a", "b"), ("b", "c"), ("c", "a")):
            Rx, Ry = res[(x, yy)]
            y *= Ry / Rx
        return y

    print(f"\n  balanced marking: cycle yield = {float(cycle_yield(res)):.6f}"
          f"   (arbitrage-free)")

    print("\n  now drain one engine by trading a->b through it:")
    print(f"  {'trades':>8s} {'R_a':>8s} {'R_b':>8s} {'cycle yield':>14s}"
          f" {'log-arb':>10s}")
    for t in range(0, 6):
        r = {k: v[:] for k, v in res.items()}
        r[("a", "b")][0] += Fraction(10 * t)
        r[("a", "b")][1] -= Fraction(10 * t) * Fraction(9, 10)
        y = cycle_yield(r)
        print(f"  {t:8d} {float(r[('a','b')][0]):8.1f} "
              f"{float(r[('a','b')][1]):8.1f} {float(y):14.6f} "
              f"{float(np.log(float(y))):10.4f}")
    print("\n  the log-rate cochain is a function of the marking, so")
    print("  ell = ell(M) and the Hodge split must be taken pointwise:")
    print("  arbitrage-freedom is a condition on a STATE, not a system.")




# --------------------------------------------------------------------
# (5) porosity: integration depth against a lossy internal medium
# --------------------------------------------------------------------

def integration_demo():
    print()
    print("=" * 68)
    print("(5) POROSITY: HOW DEEP AN INTEGRATION THE MEDIUM AFFORDS")
    print("=" * 68)
    print("  a percept assembled along a redex pathway of depth d from a")
    print("  boundary channel composes information from more of the")
    print("  surface, with saturating gain Y(d) = Y0 (1 - 2^-d); it")
    print("  survives d hops of an internal medium of per-hop reliability")
    print("  p; and it costs c per hop.  Net = Y0 (1-2^-d) p^d - c d.")
    Y0, c = 100.0, 2.0
    print(f"\n  Y0={Y0:.0f}, c={c:.0f}")
    print(f"  {'p':>6s} {'d*':>4s} {'net':>9s} {'reliability':>12s}"
          f"  {'reading'}")
    for p, tag in ((0.999, "owned medium, repaired"),
                   (0.99,  "owned medium"),
                   (0.95,  "shared medium"),
                   (0.90,  "boundary medium"),
                   (0.75,  "thin bridge")):
        best, bestd = -1e9, 0
        for d in range(1, 60):
            net = Y0 * (1 - 2.0 ** (-d)) * p ** d - c * d
            if net > best:
                best, bestd = net, d
        print(f"  {p:6.3f} {bestd:4d} {best:9.3f} {p**bestd:12.4f}"
              f"  {tag}")
    print("\n  integration depth is not bought with budget alone: it is")
    print("  bought with reliability, and reliability is available only")
    print("  on a medium the individual can name and therefore repair.")


if __name__ == "__main__":
    price_code_demo()
    individuation_demo()
    decay_demo()
    amm_demo()
    integration_demo()
