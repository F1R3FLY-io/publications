"""Phase-two erection under maximal progress: the literal ERECT of draft 3
Def. 6.8 versus single-input context instantiation."""
import random
from rhocombmat import NameTable, State, run_parallel, run_sequential
from experiments import fresh_names


def leaves(nt, sig):
    K = nt.quote((('m', nt.nil, nt.nil),))
    L = nt.quote((('m', sig, nt.nil),))
    R = nt.quote((('m', sig, K),))
    return L, R, K


def owner(nt, leaf):
    """The address a leaf lies beneath, or None if it is not a leaf."""
    comps = nt.drop(leaf)
    if len(comps) == 1 and comps[0][0] == 'm':
        return comps[0][1]
    return None


def literal_image(nt, S):
    """Def. 6.8 for the unit {fw(z1,z2), k(z1)}, with per-node gadget
    channels and the leaf fan-out of v0.4 Req. 6.10.  S holds static names."""
    K = leaves(nt, nt.nil)[2]
    return [('d', S['r'], S['u1'], S['u2']),
            ('cons_m', S['u1'], S['z0'], S['t1']), ('m', S['z0'], nt.nil),
            ('cons_m', S['u2'], S['zK'], S['t2']), ('m', S['zK'], K),
            ('d', S['t1'], S['t1a'], S['t1b']),                    # fan-out
            ('cons_fw', S['t1a'], S['t2'], S['f1']), ('e', S['f1']),
            ('cons_k', S['t1b'], S['f2']), ('e', S['f2'])]


def context_image(nt, S):
    h = nt.hole
    Lh, Rh, _ = leaves(nt, h)
    C = nt.quote((('fw', Lh, Rh), ('k', Lh)))
    return [('inst', S['r'], S['f'], C), ('e', S['f'])]


def run(kind, n_inst, seed, parallel=True):
    nt = NameTable()
    names = fresh_names(nt, 16)
    S = dict(zip(['r', 'u1', 'u2', 'z0', 'zK', 't1', 't2', 't1a', 't1b',
                  'f1', 'f2', 'k0', 'c0', 'f'], names))
    addrs = fresh_names(nt, n_inst)
    img = literal_image(nt, S) if kind == 'literal' else context_image(nt, S)
    atoms = []
    for a in addrs:
        atoms += img + [('m', S['r'], a)]
    st = State(atoms)
    if parallel:
        steps, fired, widths, _ = run_parallel(st, nt, seed=seed)
    else:
        steps = run_sequential(st, nt, seed=seed)
    fws = [a for a in st.occ.values() if a[0] == 'fw']
    ks = [a for a in st.occ.values() if a[0] == 'k']
    assert len(fws) == n_inst and len(ks) == n_inst, (len(fws), len(ks))
    # mixed: an erected atom spans two addresses, or the k and the fw of one
    # address do not share its left leaf
    mixed = sum(1 for a in fws if owner(nt, a[1]) != owner(nt, a[2]))
    return steps, len(fws), mixed


if __name__ == '__main__':
    print('=' * 78)
    for kind in ('literal', 'context'):
        for n in (2, 4, 8, 16):
            for par in (True, False):
                runs = [run(kind, n, s, par) for s in range(50)]
                erected = sum(r[1] for r in runs)
                mixed = sum(r[2] for r in runs)
                bad = sum(1 for r in runs if r[2])
                steps = sum(r[0] for r in runs) / len(runs)
                print(f"E5 {kind:8s} instances={n:3d} "
                      f"{'parallel' if par else 'sequential':10s} "
                      f"steps={steps:8.2f} erected fw={erected:5d} "
                      f"mixed={mixed:5d} runs-with-mixing={bad:2d}/50")
        print('-' * 78)


# ---------------------------------------------------------------- E6: D_x

def dx_unfoldings(n_unfold, seed=0):
    """draft 3 Ex. 6.9, the recursion combinator D_x = for(y<-x){x!(*y) | *y},
    with phase two by single-input context instantiation.  Returns per-
    unfolding step counts and name-table growth."""
    import rhocombmat as R
    nt = NameTable()
    x, r, f = fresh_names(nt, 3)
    h = nt.hole
    K = nt.quote((('m', nt.nil, nt.nil),))

    def leaf(path):
        v = h
        for bit in path:
            v = nt.quote((('m', v, K if bit else nt.nil),))
        return v
    paths = [(a, b, c) for a in (0, 1) for b in (0, 1) for c in (0, 1)]
    p0, q1, p1, p2, c, b, c2, nxt = (leaf(p) for p in paths)
    C = nt.quote((('d', x, p0, q1), ('d', q1, p1, p2), ('s', p0, b, c2),
                  ('q', b, nt.quote((('e', c),))), ('e', c2),
                  ('fw', p1, x), ('fw', p2, c), ('m', r, nxt)))
    image = (('inst', r, f, C), ('e', f))
    v = nt.quote(image)                         # @[D_x]#
    sigma = fresh_names(nt, 1)[0]               # root address, larger
    st = State(list(image) + [('m', x, v), ('m', r, sigma)])
    rng = random.Random(seed)
    marks, steps = [], 0
    names0 = len(nt)
    while len(marks) < n_unfold:
        Rx = R.redexes_matrix(st, nt)
        if not Rx:
            break
        Cg, _ = R.conflict_graph(Rx, st)
        for i in R.mis(Rx, Cg, rng):
            if Rx[i][0] == 'inst':
                marks.append((steps, len(nt) - names0))
            R.fire(st, nt, Rx[i])
        steps += 1
    return marks


if __name__ == '__main__':
    marks = dx_unfoldings(200)
    d_steps = {marks[i + 1][0] - marks[i][0] for i in range(len(marks) - 1)}
    d_names = {marks[i + 1][1] - marks[i][1] for i in range(len(marks) - 1)}
    print(f"E6 D_x, 200 unfoldings: parallel steps per unfolding {sorted(d_steps)}, "
          f"new names per unfolding {sorted(d_names)}")
