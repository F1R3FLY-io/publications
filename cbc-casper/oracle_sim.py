"""Stale-edge scenario for the clique oracle.

Re-implements, outside the node, (i) LMD-GHOST over main-parent spines with
stake flowing down each latest message's spine, and (ii) two edge rules:
  impl   -- the f1r3node-rust rule: walk b's self-justification chain from
            self(lm_b) to the stopper self(lm_a_j_b) (or lm_a_j_b when that has
            no self-justification), with the two-sided-by-height test;
  model  -- the rule of the note: additionally require lm_a_j_b to support
            the target, and every visited b-block to support it.
Then runs an asynchronous schedule with no equivocation.
"""
from itertools import combinations

B = {}  # hash -> dict(sender, parent, self, just, height)


def mk(h, sender, parent, just):
    selfj = just.get(sender)
    B[h] = dict(sender=sender, parent=parent, self=selfj, just=dict(just),
                height=0 if parent is None else B[parent]["height"] + 1)
    return h


def spine(h):
    out = []
    while h is not None:
        out.append(h)
        h = B[h]["parent"]
    return out


def on_spine(target, h):
    return target in spine(h)


def ghost(latest, w):
    score = {}
    for v, h in latest.items():
        for x in spine(h):
            score[x] = score.get(x, 0) + w[v]
    cur = "G"
    while True:
        kids = [k for k in B if B[k]["parent"] == cur and k in score]
        if not kids:
            return cur
        cur = max(kids, key=lambda k: (score[k], k))


def latest_in(view):
    lm = {}
    for h in view:
        s = B[h]["sender"]
        if s is None:
            continue
        if s not in lm or B[h]["height"] > B[lm[s]]["height"]:
            lm[s] = h
    return lm


def closure(h):
    seen, st = set(), [h]
    while st:
        x = st.pop()
        if x in seen:
            continue
        seen.add(x)
        if B[x]["parent"]:
            st.append(B[x]["parent"])
        st.extend(B[x]["just"].values())
    return seen


def no_disagree_impl(lm_b, lm_ajb, target):
    stopper = B[lm_ajb]["self"] or lm_ajb
    cur = B[lm_b]["self"]
    th = B[target]["height"]
    while cur is not None and cur != stopper:
        if B[cur]["height"] < th:
            ok = on_spine(cur, target)
        else:
            ok = on_spine(target, cur)
        if not ok:
            return False
        cur = B[cur]["self"]
    return True


def no_disagree_model(lm_b, lm_ajb, target):
    cur = lm_b
    while True:
        if not on_spine(target, cur):
            return False
        if cur == lm_ajb:
            return True
        cur = B[cur]["self"]
        if cur is None:
            return False


def ft(view, target, w, rule):
    lm = latest_in(view)
    W = sum(w.values())
    agree = [v for v in lm if on_spine(target, lm[v])]
    if 2 * sum(w[v] for v in agree) <= W:
        return None
    edges = set()
    for a, b in combinations(agree, 2):
        ja, jb = B[lm[a]]["just"].get(b), B[lm[b]]["just"].get(a)
        if ja is None or jb is None:
            continue
        if rule(lm[b], ja, target) and rule(lm[a], jb, target):
            edges.add(frozenset((a, b)))
    best = 0
    for r in range(1, len(agree) + 1):
        for Q in combinations(agree, r):
            if all(frozenset(p) in edges for p in combinations(Q, 2)):
                best = max(best, sum(w[v] for v in Q))
    return (2 * best - W) / W


def run(k=10):
    B.clear()
    bs = [f"b{i}" for i in range(k)]
    w = {"a": 1.0, "c": 1.01, **{b: 1.0 for b in bs}}
    mk("G", None, None, {})
    prev, just = "G", {}
    for b in bs:
        prev = mk(f"{b}_1", b, prev, just)
        just[b] = prev
    mk("T", "a", prev, just)
    mk("T2", "c", prev, just)
    for b in bs:
        mk(f"{b}_2", b, "T", {**just, "a": "T", b: f"{b}_1"})
    sigma = set().union(*(closure(f"{b}_2") for b in bs), closure("T"))
    fi = ft(sigma, "T", w, no_disagree_impl)
    fm = ft(sigma, "T", w, no_disagree_model)
    print(f"k={k}  W={sum(w.values()):.2f}  observer estimate: {ghost(latest_in(sigma), w)}")
    print(f"  FT(T) impl rule  = {fi}")
    print(f"  FT(T) model rule = {fm}")
    va = closure("T2") | closure("T")
    est = ghost(latest_in(va), w)
    mk("A3", "a", est, {**just, "a": "T", "c": "T2"})
    print(f"  a, seeing T2 but no b*_2, builds on {est}")
    for b in bs:
        vb = closure(f"{b}_2") | closure("A3")
        est = ghost(latest_in(vb), w)
        mk(f"{b}_3", b, est, {**just, "a": "A3", "c": "T2", b: f"{b}_2"})
    final = set(B)
    lm = latest_in(final)
    print(f"  each b_i, seeing A3 but no other b*_2, builds on: "
          f"{sorted({B[f'{b}_3']['parent'] for b in bs})}")
    print(f"  union of all views: estimate {ghost(lm, w)}, T on its spine: "
          f"{on_spine('T', ghost(lm, w))}, equivocations: 0 (every validator's "
          f"blocks form one self-justification chain)")
    lm = latest_in(set(B))
    head = ghost(lm, w)
    for v in w:
        mk(f"{v}_4", v, head, dict(lm))
    allv = set(B)
    print(f"  after one synchronous round on {head}: "
          f"FT(T2) impl rule = {ft(allv, 'T2', w, no_disagree_impl)}, "
          f"model rule = {ft(allv, 'T2', w, no_disagree_model)}")
    print(f"  T and T2 are siblings; both certified; equivocating weight 0")


if __name__ == "__main__":
    run(10)
    run(3)
