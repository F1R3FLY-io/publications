"""Experiments for the note.  Every number in the paper is produced here."""

import copy
import random
from rhocombmat import (NameTable, State, redexes_matrix, redexes_naive,
                        conflict_graph, run_parallel, run_sequential, SHAPES,
                        ARITY, incidence)


def fresh_names(nt, n):
    """n distinct names, built by iterated quotation.  The cursor lives on the
    name table, so successive calls on one table do not collide."""
    cur = getattr(nt, 'cursor', nt.nil)
    out = []
    for _ in range(n):
        cur = nt.quote((('k', cur),))
        out.append(cur)
    nt.cursor = cur
    return out


# ----------------------------------------------------------------- E1

def fan(nt, k, d):
    """k independent forwarder chains of length d, each carrying one message."""
    names = fresh_names(nt, k * (d + 1) + 1)
    v = names[-1]
    atoms = []
    for i in range(k):
        base = i * (d + 1)
        for j in range(d):
            atoms.append(('fw', names[base + j], names[base + j + 1]))
        atoms.append(('m', names[base], v))
    return State(atoms)


# ----------------------------------------------------------------- E2

def contended(nt, c):
    """c messages and c forwarders, all on one channel."""
    a, b, v = fresh_names(nt, 3)
    atoms = [('fw', a, b) for _ in range(c)] + [('m', a, v) for _ in range(c)]
    return State(atoms)


# ----------------------------------------------------------------- E3

def dup_tree(nt, h):
    """a binary tree of duplicators of height h, one message at the root,
    kills at the leaves."""
    names = fresh_names(nt, 2 ** (h + 2))
    atoms = []
    idx = [0]

    def build(node, depth):
        if depth == h:
            atoms.append(('k', names[node]))
            return
        l, r = 2 * node + 1, 2 * node + 2
        atoms.append(('d', names[node], names[l], names[r]))
        build(l, depth + 1)
        build(r, depth + 1)

    build(0, 0)
    atoms.append(('m', names[0], names[-1]))
    return State(atoms)


# ----------------------------------------------------------------- E4

def construct_then_eval(nt, k):
    """k independent gadgets, each of which builds the name @m(u,v) with
    cons_m, delivering it at c, where e opens it."""
    atoms = []
    for _ in range(k):
        a, b, c, u, v = fresh_names(nt, 5)
        atoms += [('cons_m', a, b, c), ('m', a, u), ('m', b, v),
                  ('e', c)]
    return State(atoms)


# ----------------------------------------------------------------- driver

def measure(name, mk, seeds=(0, 1, 2, 3, 4)):
    results = []
    for s in seeds:
        nt = NameTable()
        st = mk(nt)
        n0 = len(st)
        p_steps, fired, widths, counts = run_parallel(st, nt, seed=s)
        nt2 = NameTable()
        st2 = mk(nt2)
        q_steps = run_sequential(st2, nt2, seed=s)
        results.append((n0, p_steps, fired, q_steps,
                        max(widths) if widths else 0,
                        max(counts) if counts else 0))
    n0 = results[0][0]
    p = sum(r[1] for r in results) / len(results)
    f = sum(r[2] for r in results) / len(results)
    q = sum(r[3] for r in results) / len(results)
    w = max(r[4] for r in results)
    R = max(r[5] for r in results)
    print(f"{name:30s} atoms={n0:5d} par={p:7.2f} fired={f:8.2f} "
          f"seq={q:8.2f} width={w:4d} redexes={R:5d} ratio={q/p if p else 0:6.2f}")
    return dict(atoms=n0, par=p, fired=f, seq=q, width=w, redexes=R)


def validate(trials=400, seed=17):
    """The matrix formulation of redex finding against naive enumeration."""
    rng = random.Random(seed)
    bad = 0
    for _ in range(trials):
        nt = NameTable()
        names = fresh_names(nt, rng.randint(2, 5))
        atoms = []
        for _ in range(rng.randint(2, 12)):
            sh = rng.choice(SHAPES)
            atoms.append(tuple([sh] + [rng.choice(names)
                                       for _ in range(ARITY[sh])]))
        st = State(atoms)
        A = sorted(redexes_matrix(st, nt))
        B = sorted(redexes_naive(st, nt))
        if A != B:
            bad += 1
    print(f"validation: {trials - bad}/{trials} random states agree "
          f"(matrix join vs naive enumeration)")
    return bad


if __name__ == '__main__':
    print("=" * 78)
    validate()
    print("=" * 78)
    for k in (1, 4, 16, 64):
        measure(f"E1 fan k={k:3d} d=8", lambda nt, k=k: fan(nt, k, 8))
    print("-" * 78)
    for c in (2, 4, 8, 16):
        measure(f"E2 contended c={c:3d}", lambda nt, c=c: contended(nt, c))
    print("-" * 78)
    for h in (2, 4, 6):
        measure(f"E3 duplicator tree h={h}", lambda nt, h=h: dup_tree(nt, h))
    print("-" * 78)
    for k in (1, 8, 32):
        measure(f"E4 construct+eval k={k:3d}",
                lambda nt, k=k: construct_then_eval(nt, k))
