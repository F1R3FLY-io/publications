"""
forager.py -- the worked PLN example for the graded-where note.

A single mortal forager meets a stream of specimens.  A cheap structural
predicate chi is EVIDENCE, not a decision procedure.  The forager's belief
Inheritance(chi, Rich) is a PLN simple truth value (s, c) with c = n/(n+k), and
the graded where-clause on the break-in receipt is valued at the power s*c.
That receipt races an outside option -- grazing the wall -- of fixed weight g.

Three arms:
  crisp      -- the guard is chi, weight 1 when satisfied   (the note as it stands)
  individual -- the hypothesis is about the specimen; n never exceeds 1
  pln        -- the hypothesis is about the class; n accumulates
"""

import random, statistics, math
from dataclasses import dataclass

@dataclass(frozen=True)
class World:
    name: str = "A"
    p_rich: float = 0.40
    p_chi_given_rich: float = 0.80
    p_chi_given_decoy: float = 0.30
    sigma_rich: int = 120
    sigma_decoy: int = 0
    kappa_sense: int = 4          # 2^d for a depth-2 structural predicate
    kappa_break: int = 40
    graze: int = 5                # the wall: yield of the outside option
    g: float = 0.5                # weight of the graze branch in the race
    endowment: int = 120
    horizon: int = 400

    def posterior(self):
        a = self.p_rich * self.p_chi_given_rich
        b = (1 - self.p_rich) * self.p_chi_given_decoy
        return a / (a + b)

    def ev_open(self):
        p = self.posterior()
        return p * self.sigma_rich + (1 - p) * self.sigma_decoy - self.kappa_break


class Belief:
    """PLN simple truth value, count-based confidence c = n/(n+k)."""
    def __init__(self, k, s0=0.5, n0=1, cap=None):
        self.k, self.s, self.n, self.cap = k, s0, n0, cap
        self.pos = s0 * n0
    @property
    def c(self):
        return self.n / (self.n + self.k)
    @property
    def power(self):
        return self.s * self.c
    def revise(self, rich):
        if self.cap is not None and self.n >= self.cap:
            return
        self.n += 1
        self.pos += 1 if rich else 0
        self.s = self.pos / self.n


def live(world, arm, k, rng, n0=1, record=None):
    """A single life.  `world` may be a pair, in which case one is drawn."""
    if isinstance(world, tuple):
        world = world[0] if rng.random() < 0.5 else world[1]
    stack = world.endowment
    bel = Belief(k, n0=(0 if arm == "naive" else n0),
                 cap=(1 + n0 if arm == "individual" else None))
    opened = decoys = rich = 0
    for t in range(world.horizon):
        if stack < world.kappa_sense:
            break
        stack -= world.kappa_sense
        is_rich = rng.random() < world.p_rich
        chi = rng.random() < (world.p_chi_given_rich if is_rich
                              else world.p_chi_given_decoy)
        if not chi:
            stack += world.graze
            continue
        w = 1.0 if arm == "crisp" else bel.power
        if rng.random() >= w / (w + world.g):        # the graze branch won
            stack += world.graze
            continue
        if stack < world.kappa_break:
            break
        stack -= world.kappa_break
        stack += world.sigma_rich if is_rich else world.sigma_decoy
        opened += 1
        rich += 1 if is_rich else 0
        decoys += 0 if is_rich else 1
        bel.revise(is_rich)
        if record is not None:
            record.append((t, bel.s, bel.c, bel.power, stack))
    else:
        return dict(alive=True, t=world.horizon, stack=stack, opened=opened,
                    decoys=decoys, rich=rich, belief=bel)
    return dict(alive=False, t=t, stack=stack, opened=opened,
                decoys=decoys, rich=rich, belief=bel)


def trial(world, arm, k, n=20000, seed=0, n0=1):
    rng = random.Random(seed)
    runs = [live(world, arm, k, rng, n0=n0) for _ in range(n)]
    alive = [r for r in runs if r["alive"]]
    return dict(survival=len(alive)/n,
                terminal=statistics.fmean(r["stack"] for r in runs),
                lifetime=statistics.fmean(r["t"] for r in runs),
                opened=statistics.fmean(r["opened"] for r in runs),
                decoys=statistics.fmean(r["decoys"] for r in runs),
                logfit=statistics.fmean(math.log1p(max(r["stack"], 0))
                                        if r["alive"] else 0.0 for r in runs),
                s_final=statistics.fmean(r["belief"].s for r in runs))


def accuracy(world, k=4.0, n=20000, seed=5):
    """Mean |s - true posterior| among survivors, individual vs pln."""
    out = {}
    truth = world.posterior()
    for arm in ("individual", "pln"):
        rng = random.Random(seed)
        errs, ns = [], []
        for _ in range(n):
            r = live(world, arm, k, rng)
            if r["alive"]:
                errs.append(abs(r["belief"].s - truth))
                ns.append(r["belief"].n)
        out[arm] = (statistics.fmean(errs), statistics.fmean(ns), len(errs))
    return truth, out


def table(world, k=4.0, n=20000):
    print(f"--- world {world.name}: P(rich|chi) = {world.posterior():.4f}, "
          f"E[net | open] = {world.ev_open():+.2f}")
    hdr = (f"  {'arm':<12}{'survival':>10}{'terminal':>11}{'lifetime':>10}"
           f"{'opened':>9}{'decoys':>9}{'final s':>9}")
    print(hdr); print("  " + "-" * (len(hdr) - 2))
    for arm in ("naive", "crisp", "individual", "pln"):
        r = trial(world, arm, k, n=n)
        print(f"  {arm:<12}{r['survival']:>10.4f}{r['terminal']:>11.1f}"
              f"{r['lifetime']:>10.1f}{r['opened']:>9.2f}{r['decoys']:>9.2f}"
              f"{r['s_final']:>9.4f}")


def k_sweep(world, ks, n=20000, key="logfit"):
    print(f"  {'k':>7}{'survival':>11}{'terminal':>11}{'log-fitness':>13}{'opened':>9}")
    print("  " + "-" * 51)
    best = None
    for k in ks:
        r = trial(world, "pln", k, n=n, seed=1)
        print(f"  {k:>7.1f}{r['survival']:>11.4f}{r['terminal']:>11.1f}"
              f"{r['logfit']:>13.4f}{r['opened']:>9.2f}")
        if best is None or r[key] > best[1]:
            best = (k, r[key])
    print(f"  best k = {best[0]:g} at {key} {best[1]:.4f}")
    return best[0]


def belief_trace(world, k=4.0, seed=11):
    rng = random.Random(seed)
    rec = []
    live(world, "pln", k, rng, record=rec)
    print(f"  {'opening':>9}{'s':>9}{'c':>9}{'power':>9}{'P(open)':>10}")
    print("  " + "-" * 46)
    for i, (t, s, c, pw, st) in enumerate(rec, start=1):
        if i in (1, 2, 3, 5, 10, 20, 40, 80):
            print(f"  {i:>9}{s:>9.4f}{c:>9.4f}{pw:>9.4f}"
                  f"{pw/(pw+world.g):>10.4f}")


if __name__ == "__main__":
    A = World(name="A")
    B = World(name="B", p_rich=0.15, p_chi_given_rich=0.80, p_chi_given_decoy=0.40)

    print("=== The bootstrap trap: n0 = 0 (arm 'naive') ===")
    for W in (A, B):
        table(W)
        print()

    print("=== Belief trajectory in world B, one life (k=4) ===")
    belief_trace(B)

    print("\n=== k sweep, world A (chi is good evidence) ===")
    k_sweep(A, [0.5, 1, 2, 4, 8, 16, 32, 64])

    print("\n=== k sweep, world B (chi is a trap) ===")
    k_sweep(B, [0.5, 1, 2, 4, 8, 16, 32, 64])

    print("\n=== k sweep, ENSEMBLE: each life draws world A or B with prob 1/2 ===")
    k_sweep((A, B), [0.5, 1, 2, 4, 6, 8, 12, 16, 24, 32, 64])

    print("\n=== Hypothesis accuracy among survivors (world A, k=4) ===")
    truth, acc = accuracy(A)
    print(f"  true posterior = {truth:.4f}")
    for arm, (e, nn, m) in acc.items():
        print(f"  {arm:<12} mean |s - truth| = {e:.4f}   mean n = {nn:6.2f}   "
              f"survivors = {m}")
