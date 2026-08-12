"""
zxemit.py -- emission of ZX diagrams for cwrho programs, per the note's
Definitions 5.7/5.8 (SELECT, one-step block encoding).

Built on PyZX.  Everything here is verified against an independent numpy
reference in the driver scripts; nothing is asserted that is not contracted.
"""

import numpy as np
import pyzx as zx
from pyzx import VertexType, EdgeType
from fractions import Fraction

# ---------------------------------------------------------------- utilities

def circuit(n):
    return zx.Circuit(n)

def _boundary_vertex(g, qubit, which):
    lst = list(g.inputs()) if which == "in" else list(g.outputs())
    for v in lst:
        if g.qubit(v) == qubit:
            return v
    raise KeyError(f"no {which} boundary on qubit {qubit}")

SPIDER = {
    # name : (vertex type, phase)
    "0":  (VertexType.X, 0),          # |0> as a state, <0| as an effect
    "1":  (VertexType.X, Fraction(1)),
    "+":  (VertexType.Z, 0),
    "-":  (VertexType.Z, Fraction(1)),
}

def plug(g, qubit, which, kind):
    """Replace a boundary vertex by a one-legged spider: a state (which='in')
    or an effect (which='out').  kind in {'0','1','+','-'}."""
    v = _boundary_vertex(g, qubit, which)
    ty, ph = SPIDER[kind]
    g.set_type(v, ty)
    g.set_phase(v, ph)
    if which == "in":
        g.set_inputs(tuple(x for x in g.inputs() if x != v))
    else:
        g.set_outputs(tuple(x for x in g.outputs() if x != v))
    return g

def graph_of(circ, plugs_in=None, plugs_out=None):
    g = circ.to_graph()
    for q, k in (plugs_in or {}).items():
        plug(g, q, "in", k)
    for q, k in (plugs_out or {}).items():
        plug(g, q, "out", k)
    return g

def tensor(g):
    return zx.tensorfy(g)

def statevector(g):
    """Tensor of a graph with no inputs, flattened, normalised."""
    t = np.asarray(zx.tensorfy(g)).flatten()
    n = np.linalg.norm(t)
    return t / n if n > 1e-12 else t

# ------------------------------------------------- Euler / state prep

def _frac(x, denom=8192):
    """Nearest fraction of pi, in PyZX's units."""
    return Fraction(int(round(x * denom)), denom)

def _apply_1q(circ, q, U):
    """Append the ZX Euler form of a 1-qubit unitary: Zsp(g) Xsp(b) Zsp(a),
    three spiders on a wire.  PyZX's ZPhase(p) = diag(1, e^{i pi p}) and
    XPhase(p) is the same in the X basis, so both are spider phases."""
    U = np.asarray(U, dtype=complex)
    d = np.linalg.det(U)
    U = U / np.sqrt(d + 0j)                       # into SU(2)
    u00, u01 = U[0, 0], U[0, 1]
    beta = 2 * np.arctan2(abs(u01), abs(u00))
    if abs(u01) < 1e-12:
        alpha = 0.0
        gamma = np.angle(U[1, 1]) - np.angle(U[0, 0])
    elif abs(u00) < 1e-12:
        alpha = 0.0
        gamma = np.angle(U[1, 0]) - np.angle(U[0, 1])
    else:
        a0, a1 = np.angle(u00), np.angle(u01)
        gamma = -a0 - a1 - np.pi / 2
        alpha = -a0 + a1 + np.pi / 2
    circ.add_gate("ZPhase", q, phase=_frac(alpha / np.pi))
    circ.add_gate("XPhase", q, phase=_frac(beta / np.pi))
    circ.add_gate("ZPhase", q, phase=_frac(gamma / np.pi))
    return circ

def _unitary_with_first_column(amp):
    a, b = complex(amp[0]), complex(amp[1])
    n = np.sqrt(abs(a) ** 2 + abs(b) ** 2)
    a, b = a / n, b / n
    return np.array([[a, -np.conj(b)], [b, np.conj(a)]], dtype=complex)

def prep_1q(circ, q, amp):
    """|0> on wire q  ->  the normalised 1-qubit state amp."""
    return _apply_1q(circ, q, _unitary_with_first_column(amp))

def _ry(theta):
    c, s = np.cos(theta / 2), np.sin(theta / 2)
    return np.array([[c, -s], [s, c]], dtype=complex)

def prep_2q(circ, q0, q1, amp4):
    """|00> on (q0,q1) -> a normalised 2-qubit state, via Schmidt form:
    one rotation and one CNOT for the entangling core, then local Euler."""
    M = np.array(amp4, dtype=complex).reshape(2, 2)
    M = M / np.linalg.norm(M)
    U, sv, Vh = np.linalg.svd(M)
    theta = 2 * np.arctan2(sv[1], sv[0])
    _apply_1q(circ, q0, _ry(theta))
    circ.add_gate("CNOT", q0, q1)
    _apply_1q(circ, q0, U)
    _apply_1q(circ, q1, Vh.T)
    return circ

# ------------------------------------------------- reference semantics

def matrix_unit(nloc, foot_in, foot_out):
    """|R><L| (x) id : a matrix unit on the footprint, identity elsewhere.

    foot_in / foot_out : dicts loc -> bit, giving the demanded and written
    tags on the footprint.  Locations absent from both are untouched.
    """
    dim = 2 ** nloc
    K = np.zeros((dim, dim), dtype=complex)
    for x in range(dim):
        bits = [(x >> (nloc - 1 - i)) & 1 for i in range(nloc)]
        if any(bits[l] != v for l, v in foot_in.items()):
            continue
        out = list(bits)
        for l, v in foot_out.items():
            out[l] = v
        y = 0
        for bqq in out:
            y = (y << 1) | bqq
        K[y, x] = 1
    return K

# ------------------------------------------------- multi-controlled gates

def _x_conj(circ, qs):
    for q in qs:
        circ.add_gate("NOT", q)

def mcx(circ, controls, target, anc, negs=()):
    """Multi-controlled NOT.  `anc` is a list of clean ancillas (returned to
    |0>); len(anc) >= len(controls)-2 suffices."""
    negs = tuple(negs)
    _x_conj(circ, negs)
    cs = list(controls)
    if len(cs) == 0:
        circ.add_gate("NOT", target)
    elif len(cs) == 1:
        circ.add_gate("CNOT", cs[0], target)
    elif len(cs) == 2:
        circ.add_gate("TOF", cs[0], cs[1], target)
    else:
        k = len(cs)
        need = k - 2
        a = list(anc)[:need]
        circ.add_gate("TOF", cs[0], cs[1], a[0])
        for i in range(2, k - 1):
            circ.add_gate("TOF", cs[i], a[i - 2], a[i - 1])
        circ.add_gate("TOF", cs[k - 1], a[need - 1], target)
        for i in reversed(range(2, k - 1)):
            circ.add_gate("TOF", cs[i], a[i - 2], a[i - 1])
        circ.add_gate("TOF", cs[0], cs[1], a[0])
    _x_conj(circ, negs)
    return circ

def mcphase(circ, controls, phase_pi, anc, negs=()):
    """Apply e^{i pi * phase_pi} on the basis state where all controls are 1
    (after negation of `negs`).  Uses one extra clean ancilla anc[-1]."""
    if abs(phase_pi) < 1e-12:
        return circ
    t = anc[-1]
    rest = anc[:-1]
    mcx(circ, controls, t, rest, negs)
    circ.add_gate("ZPhase", t, phase=_frac(phase_pi))
    mcx(circ, controls, t, rest, negs)
    return circ

def mcry(circ, controls, target, theta, anc, negs=()):
    """R_y(theta) on target, controlled on all `controls` (negs negated)."""
    if abs(theta) < 1e-12:
        return circ
    _apply_1q(circ, target, _ry(theta / 2))
    mcx(circ, controls, target, anc, negs)
    _apply_1q(circ, target, _ry(-theta / 2))
    mcx(circ, controls, target, anc, negs)
    return circ

# ------------------------------------------------- general state prep

def prep_nq(circ, qubits, amps, anc):
    """|0...0> on `qubits` -> the normalised state `amps` (length 2**n),
    qubits[0] the most significant.  Magnitudes by a binary tree of
    uniformly-controlled R_y; phases by multi-controlled phase gates.

    This is PREPARE of Definition 5.3: the weight map, as a state."""
    n = len(qubits)
    v = np.array(amps, dtype=complex)
    v = v / np.linalg.norm(v)
    mag = np.abs(v)
    # magnitudes: tree of controlled rotations
    for level in range(n):
        block = 2 ** (n - level)          # size of a subtree at this level
        for idx in range(2 ** level):
            seg = mag[idx * block:(idx + 1) * block]
            lo = np.linalg.norm(seg[: block // 2])
            hi = np.linalg.norm(seg[block // 2:])
            if lo < 1e-12 and hi < 1e-12:
                continue
            theta = 2 * np.arctan2(hi, lo)
            controls = [qubits[i] for i in range(level)]
            negs = [qubits[i] for i in range(level)
                    if ((idx >> (level - 1 - i)) & 1) == 0]
            mcry(circ, controls, qubits[level], theta, anc, negs)
    # phases
    ph = np.angle(v)
    ref = ph[int(np.argmax(mag))]
    for k in range(2 ** n):
        if mag[k] < 1e-12:
            continue
        d = (ph[k] - ref) / np.pi
        if abs(d) < 1e-9:
            continue
        negs = [qubits[i] for i in range(n) if ((k >> (n - 1 - i)) & 1) == 0]
        mcphase(circ, list(qubits), d, anc, negs)
    return circ

# ------------------------------------------------- contraction

_H = np.array([[1, 1], [1, -1]], dtype=complex) / np.sqrt(2)

def _spider(vtype, phase_pi, degree):
    """The spider tensor: 1 on all-zero, e^{i pi phase} on all-one, else 0.
    X-spiders are the same in the X basis, i.e. conjugated by H on every leg."""
    T = np.zeros([2] * degree, dtype=complex)
    if degree == 0:
        return np.array(1 + np.exp(1j * np.pi * float(phase_pi)), dtype=complex)
    T[(0,) * degree] = 1.0
    T[(1,) * degree] = np.exp(1j * np.pi * float(phase_pi))
    if vtype == VertexType.X:
        for k in range(degree):
            T = np.tensordot(_H, T, axes=([1], [k]))
            T = np.moveaxis(T, 0, k)
    return T

def _emit_spider(vtype, phase_pi, legs, ops, subs, fresh):
    """Append the tensors for one spider.  A spider of degree d is
    sum_k c_k |k>^{(x)d}, i.e. bond dimension two, so we emit a chain of
    rank-3 COPY tensors rather than a dense rank-d array."""
    d = len(legs)
    ph = np.exp(1j * np.pi * float(phase_pi))
    if d == 0:
        ops.append(np.array(1 + ph, dtype=complex)); subs.append([])
        return
    phys = list(legs)
    if vtype == VertexType.X:                 # H on every physical leg
        newphys = []
        for l in phys:
            m = fresh()
            ops.append(_H); subs.append([l, m])
            newphys.append(m)
        phys = newphys
    if d == 1:
        ops.append(np.array([1.0, ph], dtype=complex)); subs.append([phys[0]])
        return
    COPY = np.zeros((2, 2, 2), dtype=complex)
    COPY[0, 0, 0] = 1.0; COPY[1, 1, 1] = 1.0
    PH = np.diag([1.0, ph]).astype(complex)
    b = fresh()
    ops.append(COPY); subs.append([phys[0], phys[1], b])
    for k in range(2, d - 1):
        b2 = fresh()
        ops.append(COPY); subs.append([b, phys[k], b2])
        b = b2
    ops.append(PH); subs.append([b, phys[d - 1]])


def contract_graph(g, out_order=None):
    """Contract a ZX graph to a tensor.  Spiders are emitted in bond-dimension
    two form and the contraction path is chosen by numpy's optimiser, so the
    cost is governed by the graph's treewidth rather than by vertex order."""
    counter = [0]
    def fresh():
        counter[0] += 1
        return counter[0] - 1

    slots = {v: [] for v in g.vertices()}
    ops, subs = [], []
    for e in g.edges():
        s_, t_ = g.edge_st(e)
        i0 = fresh()
        if g.edge_type(e) == EdgeType.HADAMARD:
            i1 = fresh()
            ops.append(_H); subs.append([i0, i1])
            slots[s_].append(i0); slots[t_].append(i1)
        else:
            slots[s_].append(i0); slots[t_].append(i0)

    boundaries = [v for v in g.vertices() if g.type(v) == VertexType.BOUNDARY]
    order = out_order if out_order is not None else \
        sorted(boundaries, key=lambda v: (g.qubit(v), v))
    out_ix = []
    for v in order:
        assert len(slots[v]) == 1, "boundary of degree != 1"
        out_ix.append(slots[v][0])

    for v in g.vertices():
        if g.type(v) == VertexType.BOUNDARY:
            continue
        _emit_spider(g.type(v), g.phase(v), slots[v], ops, subs, fresh)

    args = []
    for o, sb in zip(ops, subs):
        args += [o, sb]
    args.append(out_ix)
    try:
        import opt_einsum
        T = opt_einsum.contract(*args, optimize="auto")
    except ImportError:
        T = np.einsum(*args, optimize="greedy")
    try:
        T = T * g.scalar.to_number()
    except Exception:
        pass
    return T, [g.qubit(v) for v in order]
