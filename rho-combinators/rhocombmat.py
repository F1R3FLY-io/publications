"""
rhocombmat.py -- a reference machine for the rho combinators in which every
step of redex finding and conflict detection is a Boolean sparse matrix
product, and a step fires a maximal independent set of redexes.

Version 1.3.  The rule table is generated from a shape table, as the
constructor family of draft 3 (Def. 3.8) requires: one constructor cons_A per
atom shape A, of arity ar(A)+1, and optionally a second level cons_{cons_A}
for erecting the constructors that phase one emits in (o5) chains.  An
optional context-instantiation primitive `inst` (draft 3, S6.7) and the
curried, path-indexed constructors cons*_A[w] (draft 3, S6.7) are included
because the phase-two experiments in phase2.py compare them.

Presentation A: every atom is a fixed-width tuple of interned name indices.
The payload of q is stored by its quote, so q(a,p) is held as q(a, @p), and
the constructor cons_q is then uniform with every other member:
    cons_q(a,b,f) | m(a,v1) | m(b,v2)  ->  m(f, @q(v1, *v2)),
whose normal form is the row (q, v1, v2).

No dependency beyond numpy/scipy.
"""

import random
from collections import Counter
import numpy as np
import scipy.sparse as sp

# ---------------------------------------------------------------- shapes

BASE = {'m': 2, 'd': 3, 'k': 1, 'fw': 2, 'bl': 2, 'br': 2, 's': 3, 'e': 1,
        'q': 2}
PAR = 'cons'            # cons_| : parallel composition of two quotations
INST = 'inst'           # inst(a, f, C) | m(a, v) -> m(f, @C[v])
CSTAR = 'cstar'         # cons*_A[w](t, f), draft 3 S6.7: the template T is one
                        # atom whose arguments are paths below the hole or
                        # static names; cstar(t, f, T) | m(t, s) -> m(f, @T[s])


def family(levels=1):
    """The generated shape table: base atoms, cons_|, and `levels` levels of
    the constructor family.  Returns {shape: arity}."""
    ar = dict(BASE)
    ar[PAR] = 3
    frontier = list(BASE) + [PAR]
    for _ in range(levels):
        nxt = []
        for a in frontier:
            c = 'cons_' + a
            if c not in ar:
                ar[c] = ar[a] + 1
                nxt.append(c)
        frontier = nxt
    ar[INST] = 3
    ar[CSTAR] = 3
    return ar


ARITY = family(2)
SHAPES = list(ARITY)


def is_member(sh):
    return sh.startswith('cons_')


def built_shape(sh):
    return sh[len('cons_'):]


def premises(sh):
    """Premise slots joined against the message table, for a consumer shape.
    None if the shape consumes nothing (m, q, and the hole marker)."""
    if sh in ('d', 'k', 'fw', 'bl', 'br', 's', 'e', INST, CSTAR):
        return [0]
    if sh == PAR:
        return [0, 1]
    if is_member(sh):
        return list(range(ARITY[sh] - 1))
    return None


# ---------------------------------------------------------------- names

class NameTable:
    """Interns quoted processes. A process is a canonical sorted tuple of
    atoms; an atom is (shape, n1, ..., nk) with the n's name indices."""

    def __init__(self):
        self._fwd = {}
        self._bwd = []
        self.nil = self.quote(())                  # @0
        self.hole = self.quote((('hole',),))       # context hole marker

    def quote(self, atoms):
        key = tuple(sorted(atoms))
        i = self._fwd.get(key)
        if i is None:
            i = len(self._bwd)
            self._fwd[key] = i
            self._bwd.append(key)
        return i

    def drop(self, n):
        return self._bwd[n]

    def subst(self, n, v, memo=None):
        """C[v]: replace the hole by v, hereditarily under quotation."""
        if memo is None:
            memo = {}
        if n == self.hole:
            return v
        if n in memo:
            return memo[n]
        comps = self.drop(n)
        out = self.quote(tuple((a[0],) + tuple(self.subst(x, v, memo)
                                              for x in a[1:]) for a in comps))
        memo[n] = out
        return out

    def is_path(self, n):
        """True iff n is the hole or L/R applied to a path."""
        while n != self.hole:
            comps = self.drop(n)
            if len(comps) != 1 or comps[0][0] != 'm':
                return False
            n = comps[0][1]
        return True

    def mentions_hole(self, n, memo=None):
        if memo is None:
            memo = {}
        if n == self.hole:
            return True
        if n not in memo:
            memo[n] = False
            memo[n] = any(self.mentions_hole(x, memo)
                          for a in self.drop(n) for x in a[1:])
        return memo[n]

    def curried_template(self, T):
        """A curried template is one atom whose every argument is a path
        beneath the hole or mentions no hole at all (a static name)."""
        comps = self.drop(T)
        if len(comps) != 1:
            return False
        return all(self.is_path(x) or not self.mentions_hole(x)
                   for x in comps[0][1:])

    def __len__(self):
        return len(self._bwd)


# ---------------------------------------------------------------- state

class State:
    """A multiset of atom occurrences.  Occurrence identities are machine-level
    only and invisible to the semantics."""

    def __init__(self, atoms=()):
        self.occ = {}
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
    """M[shape][j][i, n] = 1 iff the i-th occurrence of shape carries name n
    in slot j.  Returns (ids, M)."""
    ids = {sh: [] for sh in SHAPES}
    rows = {sh: [] for sh in SHAPES}
    for oid, atom in state.occ.items():
        sh = atom[0]
        if sh not in ids:
            continue
        ids[sh].append(oid)
        rows[sh].append(atom[1:])
    N = len(nt)
    M = {}
    for sh in SHAPES:
        M[sh] = []
        n = len(rows[sh])
        for j in range(ARITY[sh]):
            if n == 0:
                M[sh].append(sp.csr_matrix((0, N), dtype=bool))
                continue
            data = np.ones(n, dtype=bool)
            M[sh].append(sp.csr_matrix(
                (data, (np.arange(n), np.array([t[j] for t in rows[sh]]))),
                shape=(n, N)))
    return ids, M


def _pairs(A, B):
    """Nonzeros of A B^T : rows of A matched to rows of B on a shared name."""
    P = (A.astype(np.int8) @ B.T.astype(np.int8)).tocoo()
    return list(zip(P.row.tolist(), P.col.tolist()))


def _ordered(state, occs):
    """Symmetry mask: a redex is an unordered match, so when two premise
    positions carry equal atoms keep only ascending occurrence ids."""
    for x in range(len(occs)):
        for y in range(x + 1, len(occs)):
            if state.occ[occs[x]] == state.occ[occs[y]] and occs[x] > occs[y]:
                return False
    return True


def _expand(state, cons_oid, lists, out, rule):
    """Row-wise Cartesian product of per-slot candidate lists, with the
    distinctness and symmetry masks."""
    def go(i, acc):
        if i == len(lists):
            if _ordered(state, acc):
                out.append((rule, (cons_oid,) + tuple(acc)))
            return
        for o in lists[i]:
            if o not in acc:
                go(i + 1, acc + [o])
    go(0, [])


def redexes_matrix(state, nt):
    """All redexes, by Boolean sparse products derived from the shape table."""
    ids, M = incidence(state, nt)
    out = []
    nm = M['m'][0].shape[0]
    for sh in SHAPES:
        prem = premises(sh)
        if prem is None or M[sh][0].shape[0] == 0 or nm == 0:
            continue
        per = []
        for slot in prem:
            d = {}
            for i, j in _pairs(M[sh][slot], M['m'][0]):
                d.setdefault(i, []).append(ids['m'][j])
            per.append(d)
        for i in range(M[sh][0].shape[0]):
            lists = [d.get(i, []) for d in per]
            if all(lists):
                _expand(state, ids[sh][i], lists, out, sh)
    # fw(a,b) | q(a,p) -> m(b,@p)
    if M['fw'][0].shape[0] and M['q'][0].shape[0]:
        for i, j in _pairs(M['fw'][0], M['q'][0]):
            out.append(('quote', (ids['fw'][i], ids['q'][j])))
    return out


def redexes_naive(state, nt):
    """Independent enumeration, used only to validate redexes_matrix."""
    occ = state.occ
    msgs = [(i, a) for i, a in occ.items() if a[0] == 'm']
    out = []
    for i, a in occ.items():
        sh = a[0]
        prem = premises(sh) if sh in ARITY else None
        if prem is not None:
            lists = [[j for j, b in msgs if b[1] == a[1 + s]] for s in prem]
            if all(lists):
                _expand(state, i, lists, out, sh)
        if sh == 'fw':
            for j, b in occ.items():
                if b[0] == 'q' and b[1] == a[1]:
                    out.append(('quote', (i, j)))
    return out


def conflict_graph(redexes, state):
    """B[r, o] = 1 iff redex r consumes occurrence o; conflicts are the
    off-diagonal nonzeros of B B^T."""
    if not redexes:
        return sp.coo_matrix((0, 0), dtype=bool), 0
    oidx = {o: i for i, o in enumerate(state.occ)}
    rows, cols = [], []
    for r, (_, occs) in enumerate(redexes):
        for o in occs:
            rows.append(r)
            cols.append(oidx[o])
    B = sp.csr_matrix((np.ones(len(rows), dtype=np.int8), (rows, cols)),
                      shape=(len(redexes), len(state.occ)))
    return (B @ B.T).tocoo(), B.nnz


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
    chosen, blocked = [], set()
    for r in sorted(range(n), key=lambda r: prio[r]):
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
    c, msgs = atoms[0], atoms[1:]
    vals = [m[2] for m in msgs]
    if rule == 'd':
        produced = [('m', c[2], vals[0]), ('m', c[3], vals[0])]
    elif rule == 'k':
        produced = []
    elif rule == 'fw':
        produced = [('m', c[2], vals[0])]
    elif rule == 'bl':
        produced = [('fw', vals[0], c[2])]
    elif rule == 'br':
        produced = [('fw', c[2], vals[0])]
    elif rule == 's':
        produced = [('fw', c[2], c[3])]
    elif rule == 'e':
        produced = list(nt.drop(vals[0]))      # decode: a gather
    elif rule == 'quote':
        produced = [('m', c[2], atoms[1][2])]
    elif rule == PAR:
        produced = [('m', c[3], nt.quote(nt.drop(vals[0]) + nt.drop(vals[1])))]
    elif rule == INST:
        produced = [('m', c[2], nt.subst(c[3], vals[0]))]
    elif rule == CSTAR:
        assert nt.curried_template(c[3]), 'not a curried template'
        produced = [('m', c[2], nt.subst(c[3], vals[0]))]
    elif is_member(rule):
        a = built_shape(rule)
        produced = [('m', c[-1], nt.quote(((a,) + tuple(vals),)))]
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
        C, _ = conflict_graph(R, state)
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
