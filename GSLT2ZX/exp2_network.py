"""
Experiment 2 -- two release sites, two neurons, correlated firing.

Theory (one-constructor signature, q = 1 per location):
  locations : s1, s2  release sites;  a, b  somas
  rules, for x in {1,2}, y in {a,b}:
     relay(x,y) :  s_x=1, y=0  ->  s_x=0, y=1     (transmission)
     leak (x,y) :  s_x=1, y=1  ->  s_x=0, y=1     (spike lost on a soma
                                                   that is already spiking)
  Jump index j = (r,x,y), eight jumps, so three branch qubits per step.
  Budget beta = 2: exactly enough to consume both spikes.
  Weight map: w(relay,x,y) = t[x][y],  w(leak,x,y) = lam.

The configuration (a=1,b=1) is reached by four derivations -- two pairings
of spikes to somas, each in two orders -- so it is a closed diamond and its
amplitudes interfere.  (1,0) and (0,1) are reached by two derivations each.

Every number is contracted out of a ZX diagram and cross-checked against an
independent reference implementation of Definition 5.9.
"""

import itertools
import numpy as np
import pyzx as zx
import zxemit as E

S1, S2, A, B = 0, 1, 2, 3
B1 = [4, 5, 6]
B2 = [7, 8, 9]
F1, F2 = 10, 11
U, V = 12, 13
AN = [14, 15]
TR = [16, 17]                # plasticity trace registers (which x, which y)
NQ = 18


def emit_step(c, Breg, flag, weights, trace_angle=None):
    br, bx, by = Breg
    E.prep_nq(c, Breg, weights, anc=AN)               # PREPARE

    # MATCH: flag := chi_j(gamma) for the selected j
    E.mcx(c, [bx, S1], U, AN, negs=[bx])
    E.mcx(c, [bx, S2], U, AN)
    E.mcx(c, [by, A], V, AN, negs=[by])
    E.mcx(c, [by, B], V, AN)
    c.add_gate("NOT", V)
    c.add_gate("CNOT", br, V)
    c.add_gate("TOF", U, V, flag)
    c.add_gate("CNOT", br, V)
    c.add_gate("NOT", V)
    E.mcx(c, [by, B], V, AN)
    E.mcx(c, [by, A], V, AN, negs=[by])
    E.mcx(c, [bx, S2], U, AN)
    E.mcx(c, [bx, S1], U, AN, negs=[bx])

    if trace_angle is not None:                       # plasticity read
        # A synapse-specific Hebbian update must know BOTH the presynaptic
        # and the postsynaptic index, so it couples to both bits of the
        # branch record.  Partial strength: overlap cos(trace_angle).
        E.mcry(c, [bx], TR[0], 2 * trace_angle, AN)
        E.mcry(c, [by], TR[1], 2 * trace_angle, AN)

    # WRITE: the permutation part of K_j, gated by the flag
    E.mcx(c, [flag, bx], S1, AN, negs=[bx])
    E.mcx(c, [flag, bx], S2, AN)
    E.mcx(c, [flag, br, by], A, AN, negs=[br, by])
    E.mcx(c, [flag, br, by], B, AN, negs=[br])
    return c


def weight_vector(t, lam):
    w = np.zeros(8, dtype=complex)
    for x in range(2):
        for y in range(2):
            w[(0 << 2) | (x << 1) | y] = t[x][y]
            w[(1 << 2) | (x << 1) | y] = lam
    return w


def build(t, lam, cap, trace_angle=None):
    """cap = 'X' (post-select <+| on every branch wire) or a pair (j1, j2)
    of branch records (post-select the computational basis)."""
    c = E.circuit(NQ)
    w = weight_vector(t, lam)
    emit_step(c, B1, F1, w, trace_angle)
    emit_step(c, B2, F2, w, None)
    g = c.to_graph()
    for q, k in [(S1, "1"), (S2, "1"), (A, "0"), (B, "0")]:
        E.plug(g, q, "in", k)
    for q in B1 + B2 + [F1, F2, U, V] + TR + AN:
        E.plug(g, q, "in", "0")
    E.plug(g, F1, "out", "1")
    E.plug(g, F2, "out", "1")
    for q in [U, V] + AN:
        E.plug(g, q, "out", "0")
    if cap == "X":
        for q in B1 + B2:
            E.plug(g, q, "out", "+")
    else:
        j1, j2 = cap
        for i, q in enumerate(B1):
            E.plug(g, q, "out", "1" if (j1 >> (2 - i)) & 1 else "0")
        for i, q in enumerate(B2):
            E.plug(g, q, "out", "1" if (j2 >> (2 - i)) & 1 else "0")
    if trace_angle is None:
        for q in TR:
            E.plug(g, q, "out", "0")
    return g


def contract(g, reduce_first=False):
    h = g.copy()
    if reduce_first:
        zx.full_reduce(h)
    return E.contract_graph(h)


def term_vector(g):
    T, order = contract(g)
    want = [S1, S2, A, B] + [q for q in TR if q in order]
    perm = [order.index(q) for q in want]
    return np.transpose(T, perm)


def reference(t, lam):
    """Definition 5.9 computed directly."""
    w = weight_vector(t, lam)

    def step(cfg, j):
        s1, s2, a, b = cfg
        r, x, y = (j >> 2) & 1, (j >> 1) & 1, j & 1
        if (s1, s2)[x] != 1:
            return None
        soma = (a, b)[y]
        if (r == 0 and soma != 0) or (r == 1 and soma != 1):
            return None
        ns = [s1, s2]; ns[x] = 0
        nm = [a, b]; nm[y] = 1
        return (ns[0], ns[1], nm[0], nm[1])

    amps = {}
    for j1 in range(8):
        c1 = step((1, 1, 0, 0), j1)
        if c1 is None:
            continue
        for j2 in range(8):
            c2 = step(c1, j2)
            if c2 is None:
                continue
            amps[(j1, j2, c2)] = amps.get((j1, j2, c2), 0) + w[j1] * w[j2]
    return amps


def ref_distributions(t, lam):
    amps = reference(t, lam)
    z, x = {}, {}
    for (j1, j2, cfg), v in amps.items():
        z[cfg] = z.get(cfg, 0.0) + abs(v) ** 2
        x[cfg] = x.get(cfg, 0j) + v
    return z, {k: abs(v) ** 2 for k, v in x.items()}


def zx_distributions(t, lam, verbose=False):
    zdist, xdist = {}, {}
    Tx = term_vector(build(t, lam, "X"))
    for idx in itertools.product(*[range(2)] * 4):
        if abs(Tx[idx]) > 1e-9:
            xdist[idx] = xdist.get(idx, 0j) + Tx[idx]
    nz = 0
    for j1, j2 in itertools.product(range(8), range(8)):
        T = term_vector(build(t, lam, (j1, j2)))
        if np.max(np.abs(T)) < 1e-9:
            continue
        nz += 1
        for idx in itertools.product(*[range(2)] * 4):
            if abs(T[idx]) > 1e-9:
                zdist[idx] = zdist.get(idx, 0.0) + abs(T[idx]) ** 2
    if verbose:
        print(f"      {nz} of 64 branch records survive the flag effect")
    return zdist, {k: abs(v) ** 2 for k, v in xdist.items()}


def normalise(d):
    tot = sum(d.values())
    return {k: v / tot for k, v in d.items()} if tot > 1e-12 else dict(d)


def show(name, t, lam):
    print(f"\n  {name}")
    zdist, xdist = zx_distributions(t, lam, verbose=True)
    rz, rx = ref_distributions(t, lam)
    nz, nx = normalise(zdist), normalise(xdist)
    rnz, rnx = normalise(rz), normalise(rx)
    print(f"      {'(a,b)':>8s} {'Z cap':>11s} {'X cap':>11s}")
    for a, b in [(1, 1), (1, 0), (0, 1)]:
        pz = sum(v for k, v in nz.items() if k[2] == a and k[3] == b)
        px = sum(v for k, v in nx.items() if k[2] == a and k[3] == b)
        print(f"      {'(%d,%d)' % (a, b):>8s} {pz:11.6f} {px:11.6f}")
    ez = max(abs(nz.get(k, 0) - rnz.get(k, 0)) for k in set(nz) | set(rnz))
    ex = max(abs(nx.get(k, 0) - rnx.get(k, 0)) for k in set(nx) | set(rnx))
    print(f"      max deviation from the reference:  Z {ez:.2e}   X {ex:.2e}")
    return nz, nx


if __name__ == "__main__":
    r2 = 1 / np.sqrt(2)
    bs = [[r2, r2], [1j * r2, -1j * r2]]
    flat = [[r2, r2], [r2, r2]]
    print("Experiment 2: two spikes, two neurons, budget 2")
    show("beamsplitter coupling  t = [[1,1],[i,-i]]/sqrt2", bs, r2)
    show("phase-free coupling    t = [[1,1],[1,1]]/sqrt2", flat, r2)
