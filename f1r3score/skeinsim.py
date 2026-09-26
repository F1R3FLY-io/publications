#!/usr/bin/env python3
"""skeinsim.py -- reference simulator for the skein calculus.

Stdlib only (mpmath optional, used only by T4 for the digits of pi and e).

The configuration is a soup of receipts and messages indexed by LOCATION, a
location being (quote id, timbre).  Quoted processes are hash-consed so that
name equality up to structural congruence is integer equality.  Candidates are
generated generically from the soup: a receipt and a message at one location
with opposite polarities.  Contention components, maximal matchings and the
graded where-clause are implemented as in the note; nothing in the resolver
knows about voices, fans or machines.

Run:  python3 skeinsim.py            (all tests)
"""
import itertools, math, random, sys
from fractions import Fraction as F

# --------------------------------------------------------------------------
# Hash-consed quoted processes.  Only the forms that ever sit under a quote
# are needed: the base marker B_i (an inert receipt on a dead channel) and
# the record Rec(H,v,t) = <@H,tau,v>!(H,tau,t).
# --------------------------------------------------------------------------
_TAB, _NODES = {}, []
def intern(node):
    i = _TAB.get(node)
    if i is None:
        i = len(_NODES); _TAB[node] = i; _NODES.append(node)
    return i
def node(i): return _NODES[i]

def base(tag):                       # B_tag : for(_ <- <@0, dead, tag>) 0
    return intern(('recv', ('nil',), 'dead', tag))

def rec(H, tau, subj_datum, carried):  # <@H,tau,subj>!(H, tau, carried)
    return intern(('send', H, tau, subj_datum, H, tau, carried))

def P(i): return ('p', i)            # pitch datum  -> channel
def D(j): return ('d', j)            # duration datum -> co-channel
def pol(d): return d[0]

# --------------------------------------------------------------------------
# Graded where-clauses over R>=0 (the graded sort of Paper II, without
# modalities).  A candidate view exposes: the note (pitch,dur), the datum the
# message carries, and the location's quote (for namespace atoms).
# --------------------------------------------------------------------------
def ev(psi, c):
    k = psi[0]
    if k == 'const': return psi[1]
    if k == 'crisp': return 1.0 if atom(psi[1], c) else 0.0
    if k == 'tensor':
        v = 1.0
        for s in psi[1:]:
            v *= ev(s, c)
            if v == 0.0: return 0.0
        return v
    if k == 'sum': return sum(ev(s, c) for s in psi[1])
    if k == 'table':
        # (+)_{keys} [key atoms] (x) w : a sum over MUTUALLY EXCLUSIVE crisp
        # conjunctions, evaluated by lookup instead of term by term.  Same value.
        return psi[2].get(KEYS[psi[1]](c), 0.0)
    raise ValueError(k)

def atom(a, c):
    k = a[0]
    if k == 'pitch': return c['pitch'] == a[1]
    if k == 'dur':   return c['dur'] == a[1]
    if k == 'carry': return c['carry'] == a[1]
    if k == 'step':  # the carried pitch lies k above the held pitch
        return c['carry'][0] == 'p' and c['pitch'][0] == 'p' and \
               c['carry'][1] - c['pitch'][1] == a[1]
    if k == 'back':  # spatial: the pitch j notes back in the recorded past is e
        n = node(c['quote'])
        for _ in range(a[1]):
            n = node(n[4])
            if n[0] != 'send': return False
        return n[6] == a[2]
    if k == 'repeat':  # carry = back(0) = back(1): the finite disjunction over
        n0 = node(c['quote'])                      # t of back(0,t)/\back(1,t)/\carry(t)
        if n0[0] != 'send': return False
        n1 = node(n0[4])
        return n1[0] == 'send' and n0[6] == n1[6] == c['carry']
    if k == 'not': return not atom(a[1], c)
    if k == 'and': return all(atom(b, c) for b in a[1:])
    if k == 'or':  return any(atom(b, c) for b in a[1:])
    if k == 'prev':  # @( out(<_,_,a1>) T ) : the location is a record whose
        n = node(c['quote'])                    # subject carries datum a1
        return n[0] == 'send' and n[3] == a[1]
    raise ValueError(k)

def _prev(c):
    n = node(c['quote']); return n[3] if n[0] == 'send' else None
KEYS = {
    'pitch,carry': lambda c: (c['pitch'], c['carry']),
    'prev,dur':    lambda c: (_prev(c), c['dur']),
    'step':        lambda c: c['carry'][1] - c['pitch'][1]
                   if c['carry'][0] == 'p' and c['pitch'][0] == 'p' else None,
}

def crisp(*atoms):
    return ('tensor',) + tuple(('crisp', a) for a in atoms)

def machine_clause(EP, ED):
    """psi_M for the COMM1 (pitch-leads) encoding."""
    fp = ('table', 'pitch,carry', {(P(s), P(t)): w for (s, t), w in EP.items()})
    fd = ('table', 'prev,dur', {(D(u), D(v)): w for (u, v), w in ED.items()})
    return ('tensor', fp, fd)

def machine_clause_formula(EP, ED):
    """The same clause written term by term (used to cross-check 'table')."""
    fp = ('sum', [('tensor', crisp(('pitch', P(s)), ('carry', P(t))), ('const', w))
                  for (s, t), w in EP.items()])
    fd = ('sum', [('tensor', crisp(('prev', D(u)), ('dur', D(v))), ('const', w))
                  for (u, v), w in ED.items()])
    return ('tensor', fp, fd)

def dual_clause(EP, ED):
    """psi for the COMM2 (rhythm-leads) encoding."""
    fp = ('sum', [('tensor', crisp(('prev', P(s)), ('pitch', P(t))), ('const', w))
                  for (s, t), w in EP.items()])
    fd = ('sum', [('tensor', crisp(('dur', D(u)), ('carry', D(v))), ('const', w))
                  for (u, v), w in ED.items()])
    return ('tensor', fp, fd)

# --------------------------------------------------------------------------
# The soup.
# --------------------------------------------------------------------------
class Soup:
    def __init__(self):
        self.R = {}   # loc -> list of receipts  {id,datum,psi,cont,stamp,tag}
        self.M = {}   # loc -> list of messages  {id,subj,payload,ptau,carry}
        self.n = 0
        self.touched = set()
    def fresh(self): self.n += 1; return self.n
    def receipt(self, quote, tau, datum, psi, cont, stamp, tag=None):
        self.R.setdefault((quote, tau), []).append(
            dict(id=self.fresh(), datum=datum, psi=psi, cont=cont, stamp=stamp, tag=tag))
    def send(self, quote, tau, subj, payload, ptau, carry, stamp=0):
        self.M.setdefault((quote, tau), []).append(
            dict(id=self.fresh(), subj=subj, payload=payload, ptau=ptau, carry=carry,
                 stamp=stamp))

    def candidates(self, loc):
        out = []
        for r in self.R.get(loc, []):
            for m in self.M.get(loc, []):
                if pol(r['datum']) == pol(m['subj']): continue      # no rule
                if m['ptau'] != loc[1]: continue                    # timbre
                pitch, dur = (r['datum'], m['subj']) if pol(r['datum']) == 'p' \
                             else (m['subj'], r['datum'])
                view = dict(pitch=pitch, dur=dur, carry=m['carry'], quote=loc[0])
                w = ev(r['psi'], view)
                if w > 0.0:
                    out.append((r, m, w, (pitch, dur)))
        return out

    def components(self, loc):
        cands = self.candidates(loc)
        parent = list(range(len(cands)))
        def f(i):
            while parent[i] != i: parent[i] = parent[parent[i]]; i = parent[i]
            return i
        byr, bym = {}, {}
        for i, (r, m, _, _) in enumerate(cands):
            for key, d in ((r['id'], byr), (m['id'], bym)):
                if key in d: parent[f(i)] = f(d[key])
                else: d[key] = i
        comps = {}
        for i in range(len(cands)): comps.setdefault(f(i), []).append(cands[i])
        return list(comps.values())

    def live_locations(self):
        return [l for l in self.R if self.R[l] and self.M.get(l)]

    def remove(self, loc, r, m):
        self.R[loc].remove(r); self.M[loc].remove(m)

def maximal_matchings(comp):
    """All maximal sets of pairwise non-contending candidates (brute force)."""
    n = len(comp)
    if all(c[0] is comp[0][0] for c in comp):          # a star: fast path
        return [[c] for c in comp]
    best = []
    def ok(S):
        rs = [c[0]['id'] for c in S]; ms = [c[1]['id'] for c in S]
        return len(set(rs)) == len(rs) and len(set(ms)) == len(ms)
    for k in range(n, 0, -1):
        for S in itertools.combinations(comp, k):
            if ok(S) and not any(set(map(id, S)) < set(map(id, T)) for T in best):
                best.append(S)
    return [list(S) for S in best
            if not any(ok(list(S) + [c]) for c in comp if c not in S)]

NULL = ('d', 'z')                     # the null duration: length zero
REST = ('p', 'r')                     # the rest pitch used by null notes
def dur_value(d):                     # index 0 = whole note = 16 ticks
    return 0 if d == NULL else 16 >> d[1]

# --------------------------------------------------------------------------
# One resolution.  mode='max': a component fires one maximal matching.
#                  mode='one': a component fires one candidate (interleaving).
# rng_for(receipt) supplies the chance -- per voice if the caller wants it.
# --------------------------------------------------------------------------
def resolve(soup, rng_for, mode='max', schedule=None, gc=False, log=None):
    comps = []
    for loc in soup.live_locations():
        for c in soup.components(loc):
            t = min(max(x[0]['stamp'], x[1]['stamp']) for x in c)
            comps.append((t, loc, c))
    if not comps: return False
    tmin = min(t for t, _, _ in comps)
    due = [(loc, c) for t, loc, c in comps if t == tmin]
    if schedule: schedule(due)
    loc, comp = due[0]
    rng = rng_for(comp[0][0])
    if mode == 'max':
        opts = [(S, math.prod(c[2] for c in S)) for S in maximal_matchings(comp)]
    else:
        opts = [([c], c[2]) for c in comp]
    tot = sum(w for _, w in opts)
    x, acc = rng.random() * tot, 0.0
    for S, w in opts:
        acc += w
        if x < acc: break
    for (r, m, w, note) in S:
        soup.remove(loc, r, m)
        soup.touched.add(loc)
        onset = max(r['stamp'], m['stamp'])
        if log is not None: log.append((onset, r['tag'], loc[1], note))
        z = (m['payload'], loc[1], m['carry'])       # <@Q, tau, d>
        r['cont'](soup, z, onset + dur_value(note[1]))
    # Dead-location collection.  A location that is the quote of a RECORD is never
    # re-entered once its voice has moved on (freshness), so once nothing there
    # has a positive candidate its contents are inert forever.  Base locations
    # (servers, code channels, rendezvous) are never collected.
    if gc and node(loc[0])[0] == 'send' and not soup.candidates(loc):
        soup.M.pop(loc, None); soup.R.pop(loc, None)
    return True

# --------------------------------------------------------------------------
# Voices.
#   Voice(x) := for(y <- x where psi) Voice(y)  |  Fan(x)
#   Fan(x)   := prod_{v,t} <@*x,tau,v>!( Rec(x,v,t), tau, t )
# --------------------------------------------------------------------------
def voice(soup, z, stamp, psi, NP, ND, tag, dual=False):
    H, tau, d = z
    def cont(s, z2, t2): voice(s, z2, t2, psi, NP, ND, tag, dual)
    soup.receipt(H, tau, d, psi, cont, stamp, tag)
    if not dual:     # hand on pitch channel; fan of duration co-channels
        fan(soup, H, tau, NP, ND, stamp)
    else:            # hand on duration co-channel; fan of pitch channels
        for t in range(NP):
            for v in range(ND):
                soup.send(H, tau, P(t), rec(H, tau, P(t), D(v)), tau, D(v), stamp)

def fan(soup, H, tau, NP, ND, stamp):
    for v in range(ND):
        for t in range(NP):
            soup.send(H, tau, D(v), rec(H, tau, D(v), P(t)), tau, P(t), stamp)

# --------------------------------------------------------------------------
# The REFLECTIVE voice: no process constants.  Recursion comes from a server
# whose code is kept on a code channel, fetched and re-sent at each use.  Every
# communication of the machinery is a NULL NOTE (rest, length zero).
#
#   Server   := for(w <- <@S,tau,r>) ( Handle(w) | Refresh )
#   Refresh  := for(c <- <@K,tau,r>) ( *c | <@K,tau,0>!(*c, tau, r) )
#   Handle(w):= prod_t for(z <- <@*w,tau,t> where psi (x) [back(0)=t]) Req(z)
#               | Fan(w)
#   Req(z)   := <@S,tau,0>!(*z, tau, r)
#
# The text of Server mentions only the fixed names S and K: no self-reference.
# The code message's payload is the quoted text of Server; running it (*c) is
# looked up in CODE, the only place Python stands in for process text.
# --------------------------------------------------------------------------
CODE = {}

def reflective_voice(soup, tag, tau, psi, NP, ND, p0, u0, stamp=0):
    S, K = base(('server', tag)), base(('code', tag))
    code = intern(('code', 'Server', tag))
    def run_server(s, t):                       # *c  with c the Server text
        s.receipt(S, tau, REST, ('const', 1.0), handle, t, ('null', tag))
    CODE[code] = run_server
    def refresh(s, c, t):                       # for(c <- K) ( *c | K!(*c) )
        CODE[c[0]](s, t)
        s.send(K, tau, NULL, c[0], tau, REST, t)
    def req(s, z, t):                           # the hand's continuation
        s.send(S, tau, NULL, z[0], tau, REST, t)
    def handle(s, w, t):                        # the server's body
        H = w[0]                                # location @*w; datum unused
        for pt in range(NP):                    # one guarded hand per pitch
            s.receipt(H, tau, P(pt), ('tensor', psi, ('crisp', ('back', 0, P(pt)))),
                      req, t, tag)
        fan(s, H, tau, NP, ND, t)
        s.receipt(K, tau, REST, ('const', 1.0), refresh, t, ('null', tag))
    # initial configuration: Server | code message | request carrying the seed
    run_server(soup, stamp)
    soup.send(K, tau, NULL, code, tau, REST, stamp)
    H0 = rec(base(tag), tau, D(u0), P(p0))
    soup.send(S, tau, NULL, H0, tau, REST, stamp)


def seed(tag, tau, p0, u0, dual=False):
    B = base(tag)
    if not dual:
        return (rec(B, tau, D(u0), P(p0)), tau, P(p0))
    return (rec(B, tau, P(p0), D(u0)), tau, D(u0))

# --------------------------------------------------------------------------
# Machines for the tests.
# --------------------------------------------------------------------------
def random_machine(N, rng, deg=(2, 4)):
    E = {}
    for s in range(N):
        for t in rng.sample(range(N), rng.randint(*deg)):
            E[(s, t)] = rng.choice([1, 1, 1, 2, 3, 0.5])
    return E

def rownorm(E, N):
    P_ = {}
    for s in range(N):
        tot = sum(w for (a, _), w in E.items() if a == s)
        for (a, b), w in E.items():
            if a == s: P_[(a, b)] = w / tot
    return P_

# --------------------------------------------------------------------------
# Tests
# --------------------------------------------------------------------------
def T0_inertness(notes=400):
    rng = random.Random(1)
    NP, ND = 16, 5
    EP, ED = random_machine(NP, rng), random_machine(ND, rng, (1, 3))
    soup = Soup(); psi = machine_clause(EP, ED)
    voice(soup, seed('A', 'piano', 0, 2), 0, psi, NP, ND, 'A')
    worst = 0
    for _ in range(notes):
        live = soup.live_locations()
        worst = max(worst, len(live))
        resolve(soup, lambda r: rng)
    stale = sum(len(v) for k, v in soup.M.items() if k in soup.touched)
    return dict(notes=notes, max_live_locations=worst,
                stale_messages=stale, per_note=stale / notes)

def T1_law(notes=20000, seed_=7):
    rng = random.Random(seed_)
    NP, ND = 16, 5
    EP, ED = random_machine(NP, rng), random_machine(ND, rng, (2, 3))
    PP, PD = rownorm(EP, NP), rownorm(ED, ND)
    soup = Soup(); psi = machine_clause(EP, ED); log = []
    voice(soup, seed('A', 'piano', 0, 2), 0, psi, NP, ND, 'A')
    for _ in range(notes): resolve(soup, lambda r: rng, gc=True, log=log)
    ps = [n[3][0][1] for n in log]; ds = [n[3][1][1] for n in log]
    cP, cD, cJ = {}, {}, {}
    for i in range(1, len(log)):
        cP[(ps[i-1], ps[i])] = cP.get((ps[i-1], ps[i]), 0) + 1
        cD[(ds[i-1], ds[i])] = cD.get((ds[i-1], ds[i]), 0) + 1
    def dev(C, Pm, N):
        worst, zmax, n_off = 0.0, 0.0, 0
        for s in range(N):
            tot = sum(v for (a, _), v in C.items() if a == s)
            if tot < 200: continue
            for t in range(N):
                emp = C.get((s, t), 0) / tot; p = Pm.get((s, t), 0.0)
                if p == 0 and emp > 0: n_off += 1
                worst = max(worst, abs(emp - p))
                if 0 < p < 1: zmax = max(zmax, abs(emp - p) / math.sqrt(p * (1 - p) / tot))
        return worst, zmax, n_off
    # independence: joint of (next pitch, this duration) given (pitch, prev dur)
    J = {}
    for i in range(1, len(log) - 1):
        key = (ps[i], ds[i-1]); J.setdefault(key, []).append((ps[i+1], ds[i]))
    wj, zj, nj = 0.0, 0.0, 0
    for (p, u), L in J.items():
        if len(L) < 400: continue
        for t in range(NP):
            for v in range(ND):
                emp = sum(1 for x in L if x == (t, v)) / len(L)
                q = PP.get((p, t), 0) * PD.get((u, v), 0)
                wj = max(wj, abs(emp - q))
                if 0 < q < 1:
                    nj += 1; zj = max(zj, abs(emp - q) / math.sqrt(q * (1 - q) / len(L)))
    (dp, zp, op), (dd, zd, od) = dev(cP, PP, NP), dev(cD, PD, ND)
    cells = sum(1 for k in PP if 0 < PP[k] < 1) + sum(1 for k in PD if 0 < PD[k] < 1)
    return dict(notes=notes, pitch_dev=round(dp, 4), pitch_zmax=round(zp, 2),
                dur_dev=round(dd, 4), dur_zmax=round(zd, 2),
                off_machine=op + od, joint_dev=round(wj, 4), joint_zmax=round(zj, 2), joint_cells=nj,
                edges=(len(EP), len(ED)), stochastic_cells=cells)

def T2_harmony(notes=3000):
    NP, ND = 16, 5
    r0 = random.Random(3)
    EA, EDA = random_machine(NP, r0), random_machine(ND, r0, (1, 3))
    EB, EDB = random_machine(NP, r0), random_machine(ND, r0, (1, 3))
    perfs, sims = [], None
    for sched in ('A-first', 'B-first', 'random-1', 'random-2', 'random-3'):
        soup = Soup(); log = []
        rngs = {'A': random.Random(11), 'B': random.Random(12)}
        srng = random.Random(hash(sched) & 0xffff)
        voice(soup, seed('A', 'piano', 0, 2), 0, machine_clause(EA, EDA), NP, ND, 'A')
        voice(soup, seed('B', 'piano', 7, 1), 0, machine_clause(EB, EDB), NP, ND, 'B')
        def schedule(due, s=sched):
            if s == 'A-first': due.sort(key=lambda lc: lc[1][0][0]['tag'])
            elif s == 'B-first': due.sort(key=lambda lc: lc[1][0][0]['tag'], reverse=True)
            else: srng.shuffle(due)
        for _ in range(notes):
            resolve(soup, lambda r: rngs[r['tag']], schedule=schedule, gc=True, log=log)
        perf = sorted(log, key=lambda e: (e[0], e[1]))
        perfs.append(perf)
        if sims is None:
            by = {}
            for e in log: by.setdefault(e[0], []).append(e[1])
            sims = sum(1 for v in by.values() if len(set(v)) == 2)
    same = all(p == perfs[0] for p in perfs)
    cross = sum(1 for e in perfs[0] if e[1] == 'A') , sum(1 for e in perfs[0] if e[1] == 'B')
    return dict(schedules=len(perfs), identical=same,
                simultaneous_strikes=sims, notes_A_B=cross)

def T3_two_sided(m=3, n=4, trials=2000):
    rng = random.Random(5)
    res = {}
    for mode in ('max', 'one'):
        first, total, together = [], [], 0
        for _ in range(trials):
            soup = Soup(); log = []
            H = base('K')
            for i in range(m):
                soup.receipt(H, 'piano', P(i), ('const', 1.0),
                             lambda s, z, t: None, 0, 'K')
            for j in range(n):
                soup.send(H, 'piano', D(j), base(('q', j)), 'piano', P(0))
            resolve(soup, lambda r: rng, mode=mode, log=log)
            first.append(len(log))
            while resolve(soup, lambda r: rng, mode=mode, log=log): pass
            total.append(len(log))
        res[mode] = (sum(first) / trials, sum(total) / trials)
    return dict(m=m, n=n, max_first=res['max'][0], max_total=res['max'][1],
                one_first=res['one'][0], one_total=res['one'][1])

def interval_draw(weights, digits, b, KMAX=60):
    """Exact lazy interval decoding (the drawn-distributions design, Alg. 1)."""
    tot = sum(weights); cum = [F(0)]
    for w in weights: cum.append(cum[-1] + F(w) / tot)
    A, k = 0, 0
    while True:
        A = A * b + next(digits); k += 1
        lo, hi = F(A, b**k), F(A + 1, b**k)
        for j in range(len(weights)):
            if cum[j] <= lo and hi <= cum[j+1]: return j, k
        if k >= KMAX: raise RuntimeError('undecided')

def T4_embedding(notes=400):
    try:
        import mpmath
    except ImportError:
        return dict(skipped='mpmath unavailable')
    mpmath.mp.dps = int(notes * 1.3) + 60
    def digs(x, b, n):
        x = x % 1; o = []
        for _ in range(n):
            x *= b; d = int(mpmath.floor(x)); o.append(d); x -= d
        return o
    NP, ND = 16, 5
    pd = digs(+mpmath.pi, NP, notes + 2); dd = digs(+mpmath.e, ND, notes + 2)
    EP = {(s, t): F(1, NP - 1) for s in range(NP) for t in range(NP)}
    ED = {(u, v): F(1, ND - 1) for u in range(ND) for v in range(ND)}
    psi = machine_clause({k: float(w) for k, w in EP.items()},
                         {k: float(w) for k, w in ED.items()})
    soup = Soup(); log = []
    pit, dur = iter(pd), iter(dd)
    p0 = next(pit)
    voice(soup, seed('A', 'piano', p0, 0), 0, psi, NP, ND, 'A')
    consumed = []
    # factored resolver: one draw per ribbon, each from its own constant
    for _ in range(notes):
        loc = soup.live_locations()[0]
        comp = soup.components(loc)[0]
        r = comp[0][0]
        p_now, u = r['datum'][1], node(loc[0])[3][1]
        wd = [ED.get((u, v), F(0)) for v in range(ND)]
        wp = [EP.get((p_now, t), F(0)) for t in range(NP)]
        (v, kd), (t, kp) = interval_draw(wd, dur, ND), interval_draw(wp, pit, NP)
        consumed.append((kd, kp))
        pick = [c for c in comp if c[3][1] == D(v) and c[1]['carry'] == P(t)][0]
        soup.remove(loc, pick[0], pick[1])
        log.append(pick[3])
        z = (pick[1]['payload'], loc[1], pick[1]['carry'])
        pick[0]['cont'](soup, z, 0)
        soup.M.pop(loc, None); soup.R.pop(loc, None)
    ok_p = all(log[i][0][1] == pd[i] for i in range(notes))
    ok_d = all(log[i][1][1] == dd[i] for i in range(notes))
    one = all(c == (1, 1) for c in consumed)
    return dict(notes=notes, pitch_equals_pi_hex=ok_p,
                duration_equals_e_base5=ok_d, one_digit_each=one)

def T5_dual(notes=20000):
    rng = random.Random(7)
    NP, ND = 16, 5
    EP, ED = random_machine(NP, rng), random_machine(ND, rng, (2, 3))
    PP, PD = rownorm(EP, NP), rownorm(ED, ND)
    soup = Soup(); log = []
    voice(soup, seed('A', 'piano', 0, 2, dual=True), 0,
          dual_clause(EP, ED), NP, ND, 'A', dual=True)
    for _ in range(notes): resolve(soup, lambda r: rng, gc=True, log=log)
    ps = [n[3][0][1] for n in log]; ds = [n[3][1][1] for n in log]
    def dev(xs, Pm, N):
        C = {}
        for i in range(1, len(xs)): C[(xs[i-1], xs[i])] = C.get((xs[i-1], xs[i]), 0) + 1
        w, z = 0.0, 0.0
        for s in range(N):
            tot = sum(v for (a, _), v in C.items() if a == s)
            if tot < 200: continue
            for t in range(N):
                p = Pm.get((s, t), 0); e = C.get((s, t), 0) / tot
                w = max(w, abs(e - p))
                if 0 < p < 1: z = max(z, abs(e - p) / math.sqrt(p * (1 - p) / tot))
        return round(w, 4), round(z, 2)
    (dp, zp), (dd, zd) = dev(ps, PP, NP), dev(ds, PD, ND)
    return dict(notes=notes, pitch_dev=dp, pitch_zmax=zp, dur_dev=dd, dur_zmax=zd)

def T6_coupled(trials=20000):
    """Two hands at ONE location: contention, not independence."""
    rng = random.Random(9)
    same = 0; strikes = 0
    for _ in range(trials):
        soup = Soup(); log = []
        H = base('shared')
        for i in (0, 4):
            soup.receipt(H, 'piano', P(i), ('const', 1.0), lambda s, z, t: None, 0, i)
        for j in range(5):
            soup.send(H, 'piano', D(j), base(('q', j)), 'piano', P(0))
        resolve(soup, lambda r: rng, mode='max', log=log)
        strikes += len(log)
        if len(log) == 2 and log[0][3][1] == log[1][3][1]: same += 1
    return dict(trials=trials, notes_per_resolution=strikes / trials,
                P_same_duration=same / trials, independent_would_be=0.2)


def run_voice(psi, NP, ND, notes, seed_=7, p0=0, u0=2):
    rng = random.Random(seed_); soup = Soup(); log = []
    voice(soup, seed('A', 'piano', p0, u0), 0, psi, NP, ND, 'A')
    for _ in range(notes):
        if not resolve(soup, lambda r: rng, gc=True, log=log): break
    return [(n[3][0][1], n[3][1][1]) for n in log]

def T7_convolution(notes=20000):
    """Style (x) physicality: the product of factors is normalised pointwise."""
    rng = random.Random(21); NP, ND = 21, 5
    style = random_machine(NP, rng, (4, 7))
    ED = {(u, v): 1.0 for u in range(ND) for v in (2, 3)}
    fdur = ('table', 'prev,dur', {(D(u), D(v)): w for (u, v), w in ED.items()})
    fsty = ('table', 'pitch,carry', {(P(s), P(t)): w for (s, t), w in style.items()})
    phys = {'stepwise': {k: (3.0 if abs(k) <= 2 else 0.3) for k in range(-NP, NP)},
            'leaping':  {k: (3.0 if abs(k) in (4, 7) else 0.3) for k in range(-NP, NP)}}
    out = {}
    for name, f in phys.items():
        fphy = ('table', 'step', dict(f))
        ps = [p for p, _ in run_voice(('tensor', fsty, fphy, fdur), NP, ND, notes)]
        target = {}
        for s in range(NP):
            row = {t: w * f[t - s] for (a, t), w in style.items() if a == s}
            tot = sum(row.values())
            for t, w in row.items(): target[(s, t)] = w / tot
        C = {}
        for i in range(1, len(ps)): C[(ps[i-1], ps[i])] = C.get((ps[i-1], ps[i]), 0) + 1
        zmax, off = 0.0, 0
        for s in range(NP):
            tot = sum(v for (a, _), v in C.items() if a == s)
            if tot < 200: continue
            for t in range(NP):
                p = target.get((s, t), 0.0); e = C.get((s, t), 0) / tot
                if p == 0 and e > 0: off += 1
                if 0 < p < 1: zmax = max(zmax, abs(e - p) / math.sqrt(p * (1 - p) / tot))
        leaps = [abs(ps[i] - ps[i-1]) for i in range(1, len(ps))]
        out[name] = dict(zmax=round(zmax, 2), off=off,
                         mean_interval=round(sum(leaps) / len(leaps), 2),
                         share_step_le2=round(sum(1 for x in leaps if x <= 2) / len(leaps), 3),
                         share_4_or_7=round(sum(1 for x in leaps if x in (4, 7)) / len(leaps), 3))
    return out

def T8_idioms(notes=10000):
    """Crisp enforcement and graded encouragement, both read from the past."""
    rng = random.Random(33); NP, ND = 16, 5
    EP, ED = random_machine(NP, rng, (3, 5)), random_machine(ND, rng, (2, 3))
    base_ = machine_clause(EP, ED)
    # (a) enforcement: never three equal pitches in a row
    #     the held pitch is back(0); the previous one is back(1)
    forbid = ('crisp', ('not', ('repeat',)))
    # (b) encouragement: complete the motif 0 -> 2 -> 4
    motif = ('sum', [('crisp', ('and', ('back', 1, P(0)), ('back', 0, P(2)), ('carry', P(4)))),
                     ('const', 0.0)])
    boost = ('sum', [('tensor', motif, ('const', 20.0)), ('const', 1.0)])
    def count(ps):
        triples = sum(1 for i in range(2, len(ps)) if ps[i] == ps[i-1] == ps[i-2])
        mot = sum(1 for i in range(2, len(ps)) if (ps[i-2], ps[i-1], ps[i]) == (0, 2, 4))
        ctx = sum(1 for i in range(2, len(ps)) if (ps[i-2], ps[i-1]) == (0, 2))
        return triples, mot, ctx
    EP2 = dict(EP); EP2[(0, 2)] = EP2.get((0, 2), 1); EP2[(2, 4)] = EP2.get((2, 4), 1)
    EP2[(2, 2)] = EP2.get((2, 2), 1); EP2[(5, 5)] = 3
    base2 = machine_clause(EP2, ED)
    r = {}
    for name, psi in [('unconstrained', base2), ('forbid_triples', ('tensor', base2, forbid)),
                      ('encourage_motif', ('tensor', base2, boost))]:
        ps = [p for p, _ in run_voice(psi, NP, ND, notes, seed_=5)]
        t, m, c = count(ps)
        r[name] = dict(notes=len(ps), triple_repeats=t,
                       motif_completion_rate=round(m / c, 3) if c else None)
    return r

def T9_table_equals_formula(trials=3000):
    rng = random.Random(2); NP, ND = 16, 5
    EP, ED = random_machine(NP, rng), random_machine(ND, rng, (2, 3))
    a, b = machine_clause(EP, ED), machine_clause_formula(EP, ED)
    worst = 0.0
    for _ in range(trials):
        H = rec(base('x'), 'piano', D(rng.randrange(ND)), P(rng.randrange(NP)))
        c = dict(pitch=P(rng.randrange(NP)), dur=D(rng.randrange(ND)),
                 carry=P(rng.randrange(NP)), quote=H)
        worst = max(worst, abs(ev(a, c) - ev(b, c)))
    return dict(trials=trials, worst_difference=worst)

def T10_reflective(notes=3000):
    """The reflective voice plays exactly the constant-based voice's notes."""
    rng0 = random.Random(7); NP, ND = 16, 5
    EP, ED = random_machine(NP, rng0), random_machine(ND, rng0, (2, 3))
    psi = machine_clause(EP, ED)
    def run(reflective):
        soup = Soup(); log = []
        music, nulls = random.Random(99), random.Random(4)
        pick = lambda r: nulls if (r['tag'] or ('',))[0] == 'null' else music
        if reflective:
            reflective_voice(soup, 'A', 'piano', psi, NP, ND, 0, 2)
        else:
            voice(soup, seed('A', 'piano', 0, 2), 0, psi, NP, ND, 'A')
        while sum(1 for e in log if e[3][1] != NULL) < notes:
            if not resolve(soup, pick, gc=True, log=log): break
        mus = [(e[0], e[3]) for e in log if e[3][1] != NULL]
        nul = [e for e in log if e[3][1] == NULL]
        return mus, nul
    a, _ = run(False)
    b, nul = run(True)
    null_onsets_ok = all(e[3] == (REST, NULL) for e in nul)
    return dict(notes=len(b), identical_performance=(a == b),
                null_notes=len(nul), null_per_note=round(len(nul) / len(b), 3),
                null_notes_are_zero_length_rests=null_onsets_ok,
                final_onset_whole_notes=b[-1][0] / 16)

def T11_reflective_inert(notes=200):
    """Without collection: only one location ever has a positive candidate."""
    rng = random.Random(1); NP, ND = 16, 5
    EP, ED = random_machine(NP, rng), random_machine(ND, rng, (2, 3))
    soup = Soup(); log = []; worst = 0; worst_all = 0
    reflective_voice(soup, 'A', 'piano', machine_clause(EP, ED), NP, ND, 0, 2)
    while sum(1 for e in log if e[3][1] != NULL) < notes:
        live = [l for l in soup.live_locations() if soup.candidates(l)]
        worst = max(worst, sum(1 for l in live if node(l[0])[0] == 'send'))
        worst_all = max(worst_all, len(live))
        if not resolve(soup, lambda r: rng, log=log): break
    live_ids = {c[0]['id'] for k in soup.R if node(k[0])[0] == 'send'
                for c in soup.candidates(k)}
    dead_hands = sum(1 for k, v in soup.R.items() if node(k[0])[0] == 'send'
                     for r in v if r['id'] not in live_ids)
    return dict(notes=notes, max_note_locations_with_a_candidate=worst,
                max_locations_with_a_candidate_incl_machinery=worst_all,
                dead_hands_left=dead_hands, per_note=round(dead_hands / notes, 2))

if __name__ == '__main__':
    for name, fn in [('T0 inertness of the unplayed', T0_inertness),
                     ('T1 law of one voice (pitch leads)', T1_law),
                     ('T2 harmony by independence', T2_harmony),
                     ('T3 two-sided fitting', T3_two_sided),
                     ('T4 embedding of the spigot instrument', T4_embedding),
                     ('T5 law of the dual (rhythm leads)', T5_dual),
                     ('T6 simultaneity by contention', T6_coupled),
                     ('T7 style (x) physicality', T7_convolution),
                     ('T8 idioms read from the past', T8_idioms),
                     ('T9 lookup factor = its formula', T9_table_equals_formula),
                     ('T10 reflective voice = constant voice', T10_reflective),
                     ('T11 reflective voice: the unplayed is inert', T11_reflective_inert)]:
        print(name); print('   ', fn()); sys.stdout.flush()
