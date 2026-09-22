"""
rhocombmat.py -- a reference machine for the rho combinators in which every
step of redex finding and conflict detection is a Boolean sparse matrix
product, and a step fires a maximal independent set of redexes.

Presentation A: every atom is a fixed-width tuple of interned name indices.
The payload of q is stored by its quote, so q(a,p) is held as q(a, @p).

Names are interned: a name is the quote of a process, a process is a multiset
of atoms, so the name table maps a name index to a sorted tuple of atoms and
a hash-cons table maps back.

No dependency beyond numpy/scipy.
"""

import random
from collections import Counter
import numpy as np
import scipy.sparse as sp

# ---------------------------------------------------------------- shapes

# shape -> arity (number of name arguments)
ARITY = {
    'm': 2, 'd': 3, 'k': 1, 'fw': 2, 'bl': 2, 'br': 2,
    's': 3, 'e': 1, 'q': 2,
    'cons': 3, 'consm': 3, 'consd': 4, 'conss': 4,
}
SHAPES = list(ARITY)

# ---------------------------------------------------------------- names


class NameTable:
    """Interns quoted processes. A process is a canonical sorted tuple of
    atoms; an atom is (shape, n1, ..., nk) with the n's name indices."""

    def __init__(self):
        self._fwd = {}       # canonical process -> name index
        self._bwd = []       # name index -> canonical process
        self.nil = self.quote(())          # @0

    def quote(self, atoms):
        key = tuple(sorted(atoms))
        i = self._fwd.get(key)
        if i is None:
            i = len(self._bwd)
            self._fwd[key] = i
            self._bwd.append(key)
        return i

    def drop(self, n):
        """*n : the components of the process the name quotes."""
        return self._bwd[n]

    def __len__(self):
        return len(self._bwd)


# ---------------------------------------------------------------- state

class State:
    """A multiset of atom occurrences.  Occurrences carry identities so that
    conflict between redexes is well defined; identity is machine-level only
    and is invisible to the semantics."""

    def __init__(self, atoms=()):
        self.occ = {}         # occurrence id -> atom
        self._next = 0
        for a in atoms:
            self.add(a)

    def add(self, atom):
        i = self._next
        self._next += 1
        self.occ[i] = atom
        return i

    def remove(self, i):
        del self.occ[i]

    def components(self):
        return tuple(sorted(self.occ.values()))

    def counter(self):
        return Counter(self.occ.values())

    def __len__(self):
        return len(self.occ)


# ---------------------------------------------------------------- matrices

def incidence(state, nt):
    """For each shape and slot, the Boolean matrix M[shape][j][i, n] = 1 iff
    the i-th occurrence of that shape carries name n in slot j.
    Returns (ids, M) where ids[shape][i] is the occurrence id of row i."""
    ids = {sh: [] for sh in SHAPES}
    rows = {sh: [] for sh in SHAPES}
    for oid, atom in state.occ.items():
        sh = atom[0]
        ids[sh].append(oid)
        rows[sh].append(atom[1:])
    N = len(nt)
    M = {}
    for sh in SHAPES:
        k = ARITY[sh]
        M[sh] = []
        n = len(rows[sh])
        for j in range(k):
            if n == 0:
                M[sh].append(sp.csr_matrix((0, N), dtype=bool))
                continue
            data = np.ones(n, dtype=bool)
            r = np.arange(n)
            c = np.array([t[j] for t in rows[sh]])
            M[sh].append(sp.csr_matrix((data, (r, c)), shape=(n, N)))
    return ids, M


def _pairs(A, B):
    """Nonzeros of A B^T : rows of A matched to rows of B on a shared name."""
    P = (A.astype(np.int8) @ B.T.astype(np.int8)).tocoo()
    return list(zip(P.row.tolist(), P.col.tolist()))


# consumer shapes whose rule is  C(a,...) | m(a,v)  ->  ...
BINARY_M = ['d', 'k', 'fw', 'bl', 'br', 's', 'e']


def redexes_matrix(state, nt):
    """All redexes, found by Boolean sparse matrix products.
    A redex is (rule, tuple of consumed occurrence ids)."""
    ids, M = incidence(state, nt)
    out = []

    # two-premise rules against a message, joined on the subject
    for sh in BINARY_M:
        if M[sh][0].shape[0] == 0 or M['m'][0].shape[0] == 0:
            continue
        for i, j in _pairs(M[sh][0], M['m'][0]):
            out.append((sh, (ids[sh][i], ids['m'][j])))

    # fw(a,b) | q(a,p)  -> m(b,@p)
    if M['fw'][0].shape[0] and M['q'][0].shape[0]:
        for i, j in _pairs(M['fw'][0], M['q'][0]):
            out.append(('quote', (ids['fw'][i], ids['q'][j])))

    # three-premise constructors: join slot 0 and slot 1 against m's subject,
    # then take the row-wise Cartesian product with distinctness masking
    for sh in ['cons', 'consm']:
        if M[sh][0].shape[0] == 0 or M['m'][0].shape[0] < 2:
            continue
        X, Y = {}, {}
        for i, j in _pairs(M[sh][0], M['m'][0]):
            X.setdefault(i, []).append(j)
        for i, j in _pairs(M[sh][1], M['m'][0]):
            Y.setdefault(i, []).append(j)
        for i in X:
            for j in X[i]:
                for kk in Y.get(i, ()):
                    if j != kk:
                        out.append((sh, (ids[sh][i], ids['m'][j], ids['m'][kk])))

    # four-premise constructors
    for sh in ['consd', 'conss']:
        if M[sh][0].shape[0] == 0 or M['m'][0].shape[0] < 3:
            continue
        J = []
        for slot in range(3):
            d = {}
            for i, j in _pairs(M[sh][slot], M['m'][0]):
                d.setdefault(i, []).append(j)
            J.append(d)
        for i in J[0]:
            for a in J[0][i]:
                for b in J[1].get(i, ()):
                    if b == a:
                        continue
                    for c in J[2].get(i, ()):
                        if c in (a, b):
                            continue
                        out.append((sh, (ids[sh][i], ids['m'][a],
                                         ids['m'][b], ids['m'][c])))
    return out


def redexes_naive(state, nt):
    """Independent enumeration, used only to validate redexes_matrix."""
    occ = state.occ
    out = []
    msgs = [(i, a) for i, a in occ.items() if a[0] == 'm']
    for i, a in occ.items():
        sh = a[0]
        if sh in BINARY_M:
            for j, b in msgs:
                if b[1] == a[1]:
                    out.append((sh, (i, j)))
        elif sh == 'fw':
            pass
        if sh == 'fw':
            for j, b in occ.items():
                if b[0] == 'q' and b[1] == a[1]:
                    out.append(('quote', (i, j)))
        if sh in ('cons', 'consm'):
            for j, b in msgs:
                if b[1] != a[1]:
                    continue
                for k2, c in msgs:
                    if k2 != j and c[1] == a[2]:
                        out.append((sh, (i, j, k2)))
        if sh in ('consd', 'conss'):
            for j, b in msgs:
                if b[1] != a[1]:
                    continue
                for k2, c in msgs:
                    if k2 == j or c[1] != a[2]:
                        continue
                    for l, e in msgs:
                        if l in (j, k2) or e[1] != a[3]:
                            continue
                        out.append((sh, (i, j, k2, l)))
    return out


def conflict_graph(redexes, state):
    """B[r, o] = 1 iff redex r consumes occurrence o.  The conflict graph is
    the nonzero pattern of B B^T off the diagonal -- another sparse product."""
    if not redexes:
        return sp.csr_matrix((0, 0), dtype=bool), 0
    oidx = {o: i for i, o in enumerate(state.occ)}
    rows, cols = [], []
    for r, (_, occs) in enumerate(redexes):
        for o in occs:
            rows.append(r)
            cols.append(oidx[o])
    B = sp.csr_matrix((np.ones(len(rows), dtype=np.int8), (rows, cols)),
                      shape=(len(redexes), len(state.occ)))
    C = (B @ B.T).tocoo()
    return C, B.nnz


def mis(redexes, C, rng):
    """Maximal independent set of redexes by random priority."""
    n = len(redexes)
    if n == 0:
        return []
    adj = [[] for _ in range(n)]
    for r, c in zip(C.row.tolist(), C.col.tolist()):
        if r != c:
            adj[r].append(c)
    prio = list(range(n))
    rng.shuffle(prio)
    order = sorted(range(n), key=lambda r: prio[r])
    chosen, blocked = [], set()
    for r in order:
        if r in blocked:
            continue
        chosen.append(r)
        blocked.update(adj[r])
        blocked.add(r)
    return chosen


# ---------------------------------------------------------------- firing

def fire(state, nt, redex):
    rule, occs = redex
    atoms = [state.occ[o] for o in occs]
    produced = []
    if rule == 'd':
        (_, a, b, c), (_, _, v) = atoms
        produced = [('m', b, v), ('m', c, v)]
    elif rule == 'k':
        produced = []
    elif rule == 'fw':
        (_, a, b), (_, _, v) = atoms
        produced = [('m', b, v)]
    elif rule == 'bl':
        (_, a, b), (_, _, v) = atoms
        produced = [('fw', v, b)]
    elif rule == 'br':
        (_, a, b), (_, _, v) = atoms
        produced = [('fw', b, v)]
    elif rule == 's':
        (_, a, b, c), (_, _, v) = atoms
        produced = [('fw', b, c)]
    elif rule == 'e':
        (_, a), (_, _, v) = atoms
        produced = list(nt.drop(v))          # decode: a gather from the table
    elif rule == 'quote':
        (_, a, b), (_, _, pq) = atoms
        produced = [('m', b, pq)]
    elif rule == 'cons':
        (_, a, b, c), (_, _, p), (_, _, r) = atoms
        produced = [('m', c, nt.quote(nt.drop(p) + nt.drop(r)))]
    elif rule == 'consm':
        (_, a, b, c), (_, _, u), (_, _, v) = atoms
        produced = [('m', c, nt.quote((('m', u, v),)))]
    elif rule == 'consd':
        (_, a, b, c, f), (_, _, u), (_, _, v), (_, _, w) = atoms
        produced = [('m', f, nt.quote((('d', u, v, w),)))]
    elif rule == 'conss':
        (_, a, b, c, f), (_, _, u), (_, _, v), (_, _, w) = atoms
        produced = [('m', f, nt.quote((('s', u, v, w),)))]
    else:
        raise ValueError(rule)
    for o in occs:
        state.remove(o)
    for at in produced:
        state.add(at)


# ---------------------------------------------------------------- drivers

def run_parallel(state, nt, seed=0, cap=10000):
    """Bulk-synchronous: each step fires a maximal independent set."""
    rng = random.Random(seed)
    steps, fired, widths, counts = 0, 0, [], []
    while steps < cap:
        R = redexes_matrix(state, nt)
        if not R:
            break
        C, nnz = conflict_graph(R, state)
        sel = mis(R, C, rng)
        widths.append(len(sel))
        counts.append(len(R))
        for r in sel:
            fire(state, nt, R[r])
        fired += len(sel)
        steps += 1
    return steps, fired, widths, counts


def run_sequential(state, nt, seed=0, cap=1000000):
    rng = random.Random(seed)
    steps = 0
    while steps < cap:
        R = redexes_matrix(state, nt)
        if not R:
            break
        fire(state, nt, R[rng.randrange(len(R))])
        steps += 1
    return steps
