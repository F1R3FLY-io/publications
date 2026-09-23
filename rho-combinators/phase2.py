"""Phase-two erection under maximal progress.  Three arms, as F1R3Comb v0.5
Rem. 5.20 asks: the literal ERECT of draft 3 Def. 6.8, the curried path-indexed
constructors cons*_A[w] of draft 3 S6.7, and single-input context
instantiation `inst` of draft 3 S6.7.

E5  a unit of two top-level atoms, n concurrent instances: is any erected atom
    split between two addresses?
E6  the recursion combinator D_x of draft 3 Ex. 6.9: cost per unfolding.
E7  a unit whose gate store holds a body mentioning scoped names in two
    atoms: can the store be erected without a join at a static channel?
"""
import random
import rhocombmat as R
from rhocombmat import NameTable, State, run_parallel, run_sequential
from experiments import fresh_names


def consts(nt):
    K = nt.quote((('m', nt.nil, nt.nil),))
    return K


def leaf_of(nt, base, path):
    K = consts(nt)
    v = base
    for bit in path:
        v = nt.quote((('m', v, K if bit else nt.nil),))
    return v


def owner(nt, leaf):
    """The address a leaf lies beneath (following L/R to the bottom)."""
    n = leaf
    seen = False
    while True:
        comps = nt.drop(n)
        if len(comps) == 1 and comps[0][0] == 'm' and comps[0][2] in (
                nt.nil, consts(nt)) and len(comps[0]) == 3:
            n = comps[0][1]
            seen = True
            continue
        return n if seen else None


PATHS = [(a, b, c) for a in (0, 1) for b in (0, 1) for c in (0, 1)]


def fanout(S, src, outs, tag):
    """A d-tree on static channels delivering a copy of the message at src to
    each channel in outs."""
    atoms, frontier, k = [], [src], 0
    need = len(outs)
    while len(frontier) < need:
        ch = frontier.pop(0)
        a, b = S[f'{tag}{k}'], S[f'{tag}{k+1}']
        k += 2
        atoms.append(('d', ch, a, b))
        frontier += [a, b]
    # rename the frontier to the requested outs with forwarders is avoidable:
    # the caller uses the frontier channels directly
    return atoms, frontier[:need]


def statics(nt, n=64):
    names = fresh_names(nt, n)
    return {f's{i}': x for i, x in enumerate(names)}


# ---------------------------------------------------------------- E5

def e5_image(nt, S, kind):
    h = nt.hole
    L, Rt = leaf_of(nt, h, (0,)), leaf_of(nt, h, (1,))
    if kind == 'literal':
        K = consts(nt)
        return [('d', S['s0'], S['s1'], S['s2']),
                ('cons_m', S['s1'], S['s3'], S['s5']), ('m', S['s3'], nt.nil),
                ('cons_m', S['s2'], S['s4'], S['s6']), ('m', S['s4'], K),
                ('d', S['s5'], S['s7'], S['s8']),
                ('cons_fw', S['s7'], S['s6'], S['s9']), ('e', S['s9']),
                ('cons_k', S['s8'], S['s10']), ('e', S['s10'])]
    if kind == 'curried':
        Tfw = nt.quote((('fw', L, Rt),))
        Tk = nt.quote((('k', L),))
        return [('d', S['s0'], S['s1'], S['s2']),
                ('cstar', S['s1'], S['s9'], Tfw), ('e', S['s9']),
                ('cstar', S['s2'], S['s10'], Tk), ('e', S['s10'])]
    C = nt.quote((('fw', L, Rt), ('k', L)))
    return [('inst', S['s0'], S['s9'], C), ('e', S['s9'])]


def e5(kind, n_inst, seed, parallel=True):
    nt = NameTable()
    S = statics(nt)
    img = e5_image(nt, S, kind)
    addrs = fresh_names(nt, n_inst)
    atoms = []
    for a in addrs:
        atoms += img + [('m', S['s0'], a)]
    st = State(atoms)
    if parallel:
        steps = run_parallel(st, nt, seed=seed)[0]
    else:
        steps = run_sequential(st, nt, seed=seed)
    fws = [a for a in st.occ.values() if a[0] == 'fw']
    assert len(fws) == n_inst
    mixed = sum(1 for a in fws if owner(nt, a[1]) != owner(nt, a[2]))
    return steps, len(fws), mixed


# ---------------------------------------------------------------- E6: D_x

def dx_image(nt, kind, x, r, S):
    """D_x per draft 3 Ex. 6.9: bound names p0,q1,p1,p2,c,b,c' and a spare.
    Under the curried scheme b is left static (it is unmarked, draft 3
    Rem. 6.20), since q(b, @e(c)) with b scoped would need a join."""
    h = nt.hole
    p0, q1, p1, p2, c, b, c2, nxt = (leaf_of(nt, h, p) for p in PATHS)
    body = nt.quote((('e', c),))
    if kind == 'inst':
        C = nt.quote((('d', x, p0, q1), ('d', q1, p1, p2), ('s', p0, b, c2),
                      ('q', b, body), ('e', c2), ('fw', p1, x), ('fw', p2, c),
                      ('m', r, nxt)))
        return [('inst', r, S['s0'], C), ('e', S['s0'])]
    bs = S['s63']                                   # static b
    tmpl = [('d', x, p0, q1), ('d', q1, p1, p2), ('s', p0, bs, c2),
            ('e', c2), ('fw', p1, x), ('fw', p2, c), ('m', r, nxt)]
    # the store: payload @e(c) by cons*_e, then cons_q with the static b as a
    # constant input -- one address input, so it conforms
    tmpl_payload = ('e', c)
    targets = len(tmpl) + 1
    atoms, outs = fanout(S, r, list(range(targets)), 's')
    k = 40
    for t, A in zip(outs, tmpl):
        f = S[f's{k}']; k += 1
        atoms += [('cstar', t, f, nt.quote((A,))), ('e', f)]
    g, kb, f = S[f's{k}'], S[f's{k+1}'], S[f's{k+2}']
    atoms += [('cstar', outs[-1], g, nt.quote((tmpl_payload,))),
              ('m', kb, bs), ('cons_q', kb, g, f), ('e', f)]
    return atoms


def dx_unfoldings(n_unfold, kind='inst', seed=0, sequential=False):
    nt = NameTable()
    x, r = fresh_names(nt, 2)
    S = statics(nt)
    image = dx_image(nt, kind, x, r, S)
    # the constant m(kb, b) must be re-supplied per instance; it is part of
    # the image, so the image as a whole is what the drop re-erects
    v = nt.quote(tuple(image))
    sigma = fresh_names(nt, 1)[0]
    st = State(list(image) + [('m', x, v), ('m', r, sigma)])
    rng = random.Random(seed)
    marks, steps = [], 0
    names0 = len(nt)
    trigger = 'inst' if kind == 'inst' else 'd'
    while len(marks) < n_unfold and steps < 100000:
        Rx = R.redexes_matrix(st, nt)
        if not Rx:
            break
        if sequential:
            sel = [rng.randrange(len(Rx))]
        else:
            Cg, _ = R.conflict_graph(Rx, st)
            sel = R.mis(Rx, Cg, rng)
        for i in sel:
            rule, occs = Rx[i]
            if rule == trigger and st.occ[occs[1]][1] == r:
                marks.append((steps, len(nt) - names0))
            R.fire(st, nt, Rx[i])
        steps += 1
    d_steps = [marks[i + 1][0] - marks[i][0] for i in range(len(marks) - 1)]
    d_names = [marks[i + 1][1] - marks[i][1] for i in range(len(marks) - 1)]
    return d_steps, d_names


# ---------------------------------------------------------------- E7

def e7_image(nt, S, kind):
    """A unit whose store is q(b, @(fw(c1,c3) | k(c2))) with b static and
    c1, c2, c3 scoped: the stored body mentions scoped names in two atoms,
    as it does whenever an input's body uses its bound name in two atoms."""
    h = nt.hole
    c1, c2, c3 = (leaf_of(nt, h, p) for p in PATHS[:3])
    b = S['s63']
    if kind == 'inst':
        payload = nt.quote((('fw', c1, c3), ('k', c2)))
        C = nt.quote((('q', b, payload),))
        return [('inst', S['s0'], S['s9'], C), ('e', S['s9'])]
    # curried: each atom of the payload is one curried constructor, but the
    # two must be joined into one quotation by cons_|, whose two inputs both
    # carry the address -- the join draft 3 Prop. 6.17 excludes
    return [('d', S['s0'], S['s1'], S['s2']),
            ('cstar', S['s1'], S['s3'], nt.quote((('fw', c1, c3),))),
            ('cstar', S['s2'], S['s4'], nt.quote((('k', c2),))),
            ('cons', S['s3'], S['s4'], S['s5']),
            ('m', S['s6'], b), ('cons_q', S['s6'], S['s5'], S['s7']),
            ('e', S['s7'])]


def e7(kind, n_inst, seed):
    nt = NameTable()
    S = statics(nt)
    img = e7_image(nt, S, kind)
    atoms = []
    for a in fresh_names(nt, n_inst):
        atoms += img + [('m', S['s0'], a)]
    st = State(atoms)
    run_parallel(st, nt, seed=seed)
    qs = [a for a in st.occ.values() if a[0] == 'q']
    assert len(qs) == n_inst
    mixed = 0
    for q in qs:
        owners = set()
        for at in nt.drop(q[2]):
            owners.update(owner(nt, x) for x in at[1:])
        mixed += len(owners) > 1
    return mixed


if __name__ == '__main__':
    print('=' * 78)
    for kind in ('literal', 'curried', 'inst'):
        for n in (2, 4, 8, 16):
            for par in (True, False):
                runs = [e5(kind, n, s, par) for s in range(50)]
                mixed = sum(r[2] for r in runs)
                bad = sum(1 for r in runs if r[2])
                steps = sum(r[0] for r in runs) / len(runs)
                print(f"E5 {kind:8s} instances={n:3d} "
                      f"{'parallel' if par else 'sequential':10s} "
                      f"steps={steps:7.2f} erected fw={sum(r[1] for r in runs):4d} "
                      f"mixed={mixed:4d} runs-with-mixing={bad:2d}/50")
        print('-' * 78)
    for kind in ('curried', 'inst'):
        ds, dn = dx_unfoldings(200, kind)
        ss, _ = dx_unfoldings(200, kind, sequential=True)
        print(f"E6 D_x {kind:8s} 200 unfoldings: parallel steps/unfolding "
              f"{sorted(set(ds))}, sequential mean {sum(ss)/len(ss):.2f} "
              f"(range {min(ss)}-{max(ss)}), new names/unfolding {sorted(set(dn))}")
    print('-' * 78)
    for kind in ('curried', 'inst'):
        for n in (2, 8):
            m = [e7(kind, n, s) for s in range(50)]
            print(f"E7 store with two scoped atoms {kind:8s} instances={n:2d}: "
                  f"mixed stores={sum(m):4d}/{50*n}, "
                  f"runs-with-mixing={sum(1 for x in m if x):2d}/50")
