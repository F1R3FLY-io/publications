"""
Experiment 1 -- convergent input onto one soma.

Theory (a cwrho theory with a one-constructor signature, so q = 1):
  locations : s  (presynaptic release site), a (soma of neuron A)
  rules     : syn1 :  s=1, a=0  ->  s=0, a=1
              syn2 :  s=1, a=0  ->  s=0, a=1
  Two distinct rules -- two synaptic routes -- with the SAME left- and
  right-hand sides.  They are distinct jumps (distinct basis vectors of B)
  with identical K, so the local diamond closes in one step: this is the
  minimal closed diamond a cwrho theory admits.

Weights w1, w2 in C.  We build ONE ZX diagram with the branch wire open and
cap it two ways.
"""

import numpy as np
import pyzx as zx
import zxemit as E

NS, NA, NB = 0, 1, 2          # wires: s, a, branch


def build(w1, w2):
    """Emit the one-step diagram with the branch wire left open."""
    c = E.circuit(3)
    # PREPARE on the branch wire: amplitudes ARE the weight map.
    E.prep_1q(c, NB, np.array([w1, w2], dtype=complex))
    # SELECT.  Both jumps have the same K = |0_s 1_a><1_s 0_a|, so the
    # controlled part is trivial and the matrix unit acts unconditionally.
    # The cut on s: effect <1| on the incoming wire (plugged below) and a
    # fresh |0> which nothing reads.  The cut on a: effect <0| meets the
    # initial |0>, and a fresh |1> is emitted -- realised on this input as X.
    c.add_gate("NOT", NA)
    g = c.to_graph()
    E.plug(g, NS, "in", "1")      # initial configuration: spike present
    E.plug(g, NS, "out", "1")     # the matrix unit's effect <1| on s
    E.plug(g, NA, "in", "0")      # initial configuration: soma silent
    E.plug(g, NB, "in", "0")      # PREPARE acts on |0>
    return g


def cap(g, kind, branch_value=None):
    """Cap the branch wire.  kind='Z' with a value, or kind='X' (<+|)."""
    h = g.copy()
    if kind == "Z":
        E.plug(h, NB, "out", "0" if branch_value == 0 else "1")
    else:
        E.plug(h, NB, "out", "+")
    return h


def amplitudes(w1, w2):
    g = build(w1, w2)
    out = {}
    for k, v in [("Z:j1", 0), ("Z:j2", 1)]:
        t = np.asarray(zx.tensorfy(cap(g, "Z", v))).flatten()
        out[k] = t
    t = np.asarray(zx.tensorfy(cap(g, "X"))).flatten()
    out["X"] = t
    return out


def report(name, w1, w2):
    a = amplitudes(w1, w2)
    # Normalise all three against the same scalar convention: the diagram is
    # identical apart from the cap, so we divide by the diagram's own norm
    # with a Z cap on an equal-weight instance.
    ref = np.linalg.norm(np.asarray(zx.tensorfy(cap(build(1, 0), "Z", 0))).flatten())
    pz = (np.linalg.norm(a["Z:j1"]) ** 2 + np.linalg.norm(a["Z:j2"]) ** 2) / ref ** 2
    px = np.linalg.norm(a["X"]) ** 2 / ref ** 2 * 2   # <+| carries 1/sqrt(2)
    print(f"  {name:28s} w1={w1:+.4g} w2={w2:+.4g}   "
          f"Z-cap P(fire)={pz:.6f}   X-cap P(fire)={px:.6f}")
    return pz, px


def reduction_stats(w1, w2):
    g = cap(build(w1, w2), "X")
    before = (g.num_vertices(), g.num_edges())
    t0 = zx.tensorfy(g)
    h = g.copy()
    zx.full_reduce(h)
    after = (h.num_vertices(), h.num_edges())
    t1 = zx.tensorfy(h)
    return before, after, zx.compare_tensors(t0, t1)


if __name__ == "__main__":
    r2 = 1 / np.sqrt(2)
    print("Experiment 1: two synaptic routes onto one soma (a closed diamond)")
    print()
    rows = []
    rows.append(report("excitatory, in phase", r2, r2))
    rows.append(report("inhibitory, phase pi", r2, -r2))
    rows.append(report("quadrature, phase pi/2", r2, 1j * r2))
    rows.append(report("single route only", 1.0, 0.0))
    print()
    b, a, ok = reduction_stats(r2, -r2)
    print(f"  full_reduce: {b[0]} vertices/{b[1]} edges  ->  "
          f"{a[0]} vertices/{a[1]} edges;  tensor preserved: {ok}")
