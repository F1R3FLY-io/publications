"""
zxtikz.py -- render a PyZX graph as TikZ, so the figures in the note are
generated from the compiler's output rather than drawn by hand.

Also defines the minimal divergent example (one spike, two somas, one step),
whose emitted and reduced diagrams are small enough to print in full.
"""

from fractions import Fraction
import numpy as np
import pyzx as zx
from pyzx import VertexType, EdgeType
import zxemit as E


# ------------------------------------------------------------------ TikZ

def _phase_label(ph):
    f = Fraction(ph)
    if f == 0:
        return ""
    if f == 1:
        return r"$\pi$"
    if f.denominator == 1:
        return rf"${f.numerator}\pi$"
    if f.numerator == 1:
        return rf"$\frac{{\pi}}{{{f.denominator}}}$"
    return rf"$\frac{{{f.numerator}\pi}}{{{f.denominator}}}$"


def _layout(g):
    """(x, y) per vertex.  PyZX carries row/qubit for circuit-shaped graphs;
    after simplification those can drift, so rows are re-ranked and stray
    vertices are pulled onto the wire of the boundary they attach to."""
    rows = sorted({float(g.row(v)) for v in g.vertices()})
    rank = {r: i for i, r in enumerate(rows)}
    ycoord = {}
    for v in g.vertices():
        if g.type(v) == VertexType.BOUNDARY:
            ycoord[v] = float(g.qubit(v))
    for v in g.vertices():
        if v in ycoord:
            continue
        bn = [ycoord[u] for u in g.neighbors(v) if u in ycoord]
        ycoord[v] = sum(bn) / len(bn) if bn else float(g.qubit(v))
    pos, used = {}, set()
    for v in sorted(g.vertices()):
        x, y = rank[float(g.row(v))], ycoord[v]
        while (round(x, 3), round(y, 3)) in used:
            y += 0.55
        used.add((round(x, 3), round(y, 3)))
        pos[v] = (x, y)
    return pos


def to_tikz(g, xscale=1.5, yscale=1.1, label=None, boundary_names=None,
            row_labels=None):
    pos = _layout(g)
    out = [rf"\begin{{tikzpicture}}[xscale={xscale},yscale={-yscale},baseline]"]
    for e in g.edges():
        s, t = g.edge_st(e)
        (x0, y0), (x1, y1) = pos[s], pos[t]
        if g.edge_type(e) == EdgeType.HADAMARD:
            out.append(rf"  \draw[zxw] ({x0},{y0}) -- ({x1},{y1});")
            mx, my = (x0 + x1) / 2, (y0 + y1) / 2
            out.append(rf"  \node[zxh] at ({mx},{my}) {{}};")
        else:
            out.append(rf"  \draw[zxw] ({x0},{y0}) -- ({x1},{y1});")
    for v in g.vertices():
        x, y = pos[v]
        ty = g.type(v)
        if ty == VertexType.BOUNDARY:
            nm = (boundary_names or {}).get(v, "")
            out.append(rf"  \node[zxb,label={{[font=\tiny]above:{nm}}}] "
                       rf"at ({x},{y}) {{}};")
        else:
            sty = "zxz" if ty == VertexType.Z else "zxx"
            out.append(rf"  \node[{sty}] at ({x},{y}) "
                       rf"{{{_phase_label(g.phase(v))}}};")
    for y, txt in (row_labels or {}).items():
        out.append(rf"  \node[font=\small,anchor=east] at (-0.55,{y}) {{{txt}}};")
    if label:
        xs = [p[0] for p in pos.values()]
        ys = [p[1] for p in pos.values()]
        out.append(rf"  \node[font=\small] at "
                   rf"({(min(xs)+max(xs))/2},{max(ys)+0.9}) {{{label}}};")
    out.append(r"\end{tikzpicture}")
    return "\n".join(out)


# ------------------------------------------------- the minimal race

S, A, B, BY, F, VV = 0, 1, 2, 3, 4, 5
NQ = 6


def race(wa, wb, cap):
    """One spike, two somas, one step, two jumps: the minimal DIVERGENT
    branch.  relay(y) : s=1, y=0 -> s=0, y=1, for y in {a,b}."""
    c = E.circuit(NQ)
    E.prep_1q(c, BY, np.array([wa, wb], dtype=complex))       # PREPARE
    # MATCH: v := y_{by}; f := s AND NOT v; uncompute v
    E.mcx(c, [BY, A], VV, [], negs=[BY])
    E.mcx(c, [BY, B], VV, [])
    E.mcx(c, [S, VV], F, [], negs=[VV])
    E.mcx(c, [BY, B], VV, [])
    E.mcx(c, [BY, A], VV, [], negs=[BY])
    # WRITE
    c.add_gate("CNOT", F, S)
    E.mcx(c, [F, BY], A, [], negs=[BY])
    E.mcx(c, [F, BY], B, [])
    g = c.to_graph()
    for q, k in [(S, "1"), (A, "0"), (B, "0")]:
        E.plug(g, q, "in", k)
    for q in [BY, F, VV]:
        E.plug(g, q, "in", "0")
    E.plug(g, F, "out", "1")          # the projection onto enabled jumps
    E.plug(g, VV, "out", "0")
    E.plug(g, BY, "out", "+" if cap == "X" else ("0" if cap == 0 else "1"))
    return g


def race_amplitudes(wa, wb, cap):
    g = race(wa, wb, cap)
    h = g.copy()
    zx.full_reduce(h)
    T, order = E.contract_graph(h)
    return T, order, g, h


if __name__ == "__main__":
    r2 = 1 / np.sqrt(2)
    for cap in ["X", 0, 1]:
        T, order, g, h = race_amplitudes(r2, r2, cap)
        T = np.asarray(T).flatten()
        nz = {i: np.round(v, 4) for i, v in enumerate(T) if abs(v) > 1e-9}
        print(f"cap={cap!s:2s}  emitted {g.num_vertices():3d}v/{g.num_edges():3d}e"
              f"  reduced {h.num_vertices():2d}v/{h.num_edges():2d}e"
              f"  outputs(qubits)={order}  nonzero={nz}")
