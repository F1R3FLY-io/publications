"""
ecology.py -- the robustness sweep and the testimony experiment for
"Graded Where-Clauses over the Reals".

The forager of the earlier note lived in a world specified by a payoff table.
Here it lives in an ECOLOGY, and the robustness question is asked over the
three distributions that specify one:

  (T)  the distribution over TROPHIC TYPES among the edible computations
       -- autotrophs, which eat only the reservoir; heterotrophs, which eat
       other computations and have therefore accumulated; and composites;
  (I)  the distribution over INEDIBLES -- computations that cannot be
       opened at a profit;
  (R)  the distribution over TOKEN RESERVOIRS not housed in a trophic type
       -- free stacks, the outside option.

Two objectives are reported side by side throughout: mean terminal stack
(expected yield) and mean log terminal stack (the criterion appropriate to a
multiplicative absorbing process).

Stdlib only.
"""

import math
import random
import statistics
from dataclasses import dataclass, replace

# --------------------------------------------------------------- the ecology

SIGMA = {"auto": 30, "hetero": 130, "comp": 70, "inedible": 0}
P_CHI = {"auto": 0.45, "hetero": 0.85, "comp": 0.75, "inedible": 0.30}

_CUM = {}


@dataclass(frozen=True)
class Ecology:
    name: str = "baseline"
    hetero: float = 0.40          # (T) heterotroph share among edibles
    inedible: float = 0.40        # (I) inedible share of computations
    reservoir: float = 0.25       # (R) chance of meeting a free stack
    graze: int = 5                # yield of a free stack
    g: float = 0.5                # weight of the graze branch in the race
    kappa_sense: int = 4
    kappa_break: int = 60
    endowment: int = 120
    horizon: int = 400

    def kinds(self):
        """P(kind | a computation is met)."""
        e = 1.0 - self.inedible
        rest = (1.0 - self.hetero) / 2.0
        return {"inedible": self.inedible,
                "hetero": e * self.hetero,
                "auto": e * rest,
                "comp": e * rest}

    def cum(self):
        key = (self.hetero, self.inedible)
        c = _CUM.get(key)
        if c is None:
            acc, c = 0.0, []
            for k, p in self.kinds().items():
                acc += p
                c.append((acc, k, P_CHI[k], SIGMA[k]))
            _CUM[key] = c
        return c

    def draw_kind(self, rng):
        u = rng.random()
        for acc, k, _, _ in self.cum():
            if u < acc:
                return k
        return "inedible"

    def profitable(self, kind):
        return SIGMA[kind] - self.kappa_break > 0

    def truth(self):
        """P(profitable | chi): what the learner's hypothesis is about."""
        num = den = 0.0
        for k, p in self.kinds().items():
            w = p * P_CHI[k]
            den += w
            if self.profitable(k):
                num += w
        return num / den if den > 0 else 0.0

    def ev_open(self):
        num = den = 0.0
        for k, p in self.kinds().items():
            w = p * P_CHI[k]
            den += w
            num += w * SIGMA[k]
        return num / den - self.kappa_break if den > 0 else -float(self.kappa_break)


class Belief:
    """A PLN simple truth value with count-based confidence c = n/(n+k)."""

    def __init__(self, k, s0=0.5, n0=1.0, cap=None):
        self.k, self.s, self.n, self.cap = k, s0, n0, cap
        self.pos = s0 * n0
        self.independent = n0          # what the evidence would warrant

    @property
    def c(self):
        return self.n / (self.n + self.k)

    @property
    def power(self):
        return self.s * self.c

    @property
    def step(self):
        return 1.0 / (self.n + 1.0)

    def revise_observation(self, good):
        if self.cap is not None and self.n >= self.cap:
            return
        self.n += 1.0
        self.independent += 1.0
        self.pos += 1.0 if good else 0.0
        self.s = self.pos / self.n

    def revise_belief(self, other, independent):
        """PLN revision by another agent's truth value."""
        self.pos += other.pos
        self.n += other.n
        self.s = self.pos / self.n
        if independent:
            self.independent += other.independent


def live(eco, arm, k, rng, n0=1.0, shift_at=None, eco2=None, bel=None,
         report=None):
    """One life.  Returns the outcome dictionary."""
    stack = eco.endowment
    if bel is None:
        bel = Belief(k, n0=(0.0 if arm == "naive" else n0),
                     cap=(1.0 + n0 if arm == "individual" else None))
    world = eco
    cum = world.cum()
    for t in range(eco.horizon):
        if shift_at is not None and t == shift_at:
            world = eco2
            cum = world.cum()
        if stack < world.kappa_sense:
            return dict(alive=False, t=t, stack=stack, belief=bel)
        stack -= world.kappa_sense
        u = rng.random()
        if u < world.reservoir:
            stack += world.graze
            continue
        u = rng.random()
        sig = pchi = None
        for acc, _k, pc, sg in cum:
            if u < acc:
                pchi, sig = pc, sg
                break
        if pchi is None:
            pchi, sig = cum[-1][2], cum[-1][3]
        if rng.random() >= pchi:
            stack += world.graze
            continue
        w = 1.0 if arm == "crisp" else bel.s * bel.n / (bel.n + bel.k)
        if rng.random() >= w / (w + world.g):
            stack += world.graze
            continue
        if stack < world.kappa_break:
            return dict(alive=False, t=t, stack=stack, belief=bel)
        stack -= world.kappa_break
        stack += sig
        bel.revise_observation(sig - world.kappa_break > 0)
        if report is not None:
            report.append((t, bel.s, bel.n, bel.independent))
    return dict(alive=True, t=world.horizon, stack=stack, belief=bel)


def trial(eco, arm, k, n=4000, seed=0, n0=1.0):
    rng = random.Random(seed)
    runs = [live(eco, arm, k, rng, n0=n0) for _ in range(n)]
    return dict(
        survival=statistics.fmean(1.0 if r["alive"] else 0.0 for r in runs),
        wealth=statistics.fmean(r["stack"] for r in runs),
        logfit=statistics.fmean(math.log1p(max(r["stack"], 0)) if r["alive"]
                                else 0.0 for r in runs),
        s_final=statistics.fmean(r["belief"].s for r in runs),
        n_final=statistics.fmean(r["belief"].n for r in runs),
    )


# ----------------------------------------------------- the four arms, baseline

KGRID = (0.5, 1, 2, 4, 9, 16, 32, 64, 128, 256, 512)


def arms_table(eco, k=4.0, n=8000):
    print("--- ecology %s:  P(profitable | chi) = %.4f,  E[net | open] = %+.2f"
          % (eco.name, eco.truth(), eco.ev_open()))
    print("    %-12s %10s %11s %12s %10s"
          % ("arm", "survival", "wealth", "log-fitness", "final s"))
    print("    " + "-" * 58)
    for arm in ("naive", "crisp", "individual", "pln"):
        r = trial(eco, arm, k, n=n, seed=2)
        print("    %-12s %10.4f %11.1f %12.4f %10.4f"
              % (arm, r["survival"], r["wealth"], r["logfit"], r["s_final"]))


def k_star(eco, n=3000, seed=1, grid=KGRID):
    """Optimal personality parameter under both objectives."""
    best_w = best_l = None
    rows = []
    for k in grid:
        r = trial(eco, "pln", k, n=n, seed=seed)
        rows.append((k, r))
        if best_w is None or r["wealth"] > best_w[1]["wealth"]:
            best_w = (k, r)
        if best_l is None or r["logfit"] > best_l[1]["logfit"]:
            best_l = (k, r)
    return best_w, best_l, rows


def k_table(eco, n=6000):
    print("    %8s %10s %11s %12s" % ("k", "survival", "wealth", "log-fitness"))
    print("    " + "-" * 43)
    bw, bl, rows = k_star(eco, n=n)
    for k, r in rows:
        print("    %8g %10.4f %11.1f %12.4f"
              % (k, r["survival"], r["wealth"], r["logfit"]))
    print("    argmax wealth      k* = %-6g (survival %.4f)"
          % (bw[0], bw[1]["survival"]))
    print("    argmax log-fitness k* = %-6g (survival %.4f)"
          % (bl[0], bl[1]["survival"]))
    return bw, bl


# ------------------------------------------------------------ the three sweeps

def sweep(axis, values, base, n=3000):
    print("  sweeping %s" % axis)
    print("    %8s %8s %12s %10s %12s %10s %10s"
          % (axis, "truth", "k*(wealth)", "surv", "k*(log-fit)", "surv", "E[net]"))
    print("    " + "-" * 76)
    out = []
    for v in values:
        eco = replace(base, **{axis: v}, name="%s=%g" % (axis, v))
        bw, bl, _ = k_star(eco, n=n)
        print("    %8.2f %8.4f %12g %10.4f %12g %10.4f %10.2f"
              % (v, eco.truth(), bw[0], bw[1]["survival"],
                 bl[0], bl[1]["survival"], eco.ev_open()))
        out.append((v, eco.truth(), bw[0], bl[0]))
    return out


def corner_grid(base, hs=(0.15, 0.40, 0.75), ins=(0.15, 0.40, 0.70), n=1200):
    print("    %6s %6s %8s %9s %12s %12s"
          % ("T", "I", "truth", "E[net]", "k*(log-fit)", "k*(wealth)"))
    print("    " + "-" * 57)
    for h in hs:
        for i in ins:
            eco = replace(base, hetero=h, inedible=i)
            bw, bl, _ = k_star(eco, n=n)
            print("    %6.2f %6.2f %8.3f %9.1f %12g %12g"
                  % (h, i, eco.truth(), eco.ev_open(), bl[0], bw[0]),
                  flush=True)


# ---------------------------------------------------------------- E: testimony

def testimony(eco_pre, eco_post, protocol, k=9.0, epochs=8, seed=0,
              lives=1500):
    """
    Two learners with a shared belief namespace.  Each epoch they forage a
    stretch of the horizon and then exchange truth values.

      isolated    -- no exchange
      honest      -- exchange once, evidence is disjoint, counts add
      plagiarism  -- exchange every epoch, so each learner's count is
                     re-transmitted and re-added on the next exchange
    """
    rng = random.Random(seed)
    infl, err_pre, err_post, surv, wealth, steps = [], [], [], [], [], []
    span = eco_pre.horizon // epochs
    for _ in range(lives):
        bels = [Belief(k, n0=1.0), Belief(k, n0=1.0)]
        stacks = [eco_pre.endowment, eco_pre.endowment]
        alive = [True, True]
        for ep in range(epochs):
            world = eco_pre if ep < epochs // 2 else eco_post
            for i in (0, 1):
                if not alive[i]:
                    continue
                sub = replace(world, horizon=span, endowment=stacks[i])
                r = live(sub, "pln", k, rng, bel=bels[i])
                stacks[i], alive[i] = r["stack"], r["alive"]
            if protocol == "honest" and ep == epochs // 2 - 1:
                a, b = Belief(k), Belief(k)
                a.pos, a.n, a.independent = bels[1].pos, bels[1].n, bels[1].independent
                b.pos, b.n, b.independent = bels[0].pos, bels[0].n, bels[0].independent
                bels[0].revise_belief(a, independent=True)
                bels[1].revise_belief(b, independent=True)
            if protocol == "plagiarism":
                a, b = Belief(k), Belief(k)
                a.pos, a.n, a.independent = bels[1].pos, bels[1].n, 0.0
                b.pos, b.n, b.independent = bels[0].pos, bels[0].n, 0.0
                bels[0].revise_belief(a, independent=False)
                bels[1].revise_belief(b, independent=False)
            if ep == epochs // 2 - 1:
                for bl in bels:
                    err_pre.append(abs(bl.s - eco_pre.truth()))
        for i in (0, 1):
            infl.append(bels[i].n / max(bels[i].independent, 1e-9))
            err_post.append(abs(bels[i].s - eco_post.truth()))
            steps.append(bels[i].step)
            surv.append(1.0 if alive[i] else 0.0)
            wealth.append(stacks[i])
    return dict(inflation=statistics.fmean(infl),
                err_pre=statistics.fmean(err_pre),
                err_post=statistics.fmean(err_post),
                step=statistics.fmean(steps),
                survival=statistics.fmean(surv),
                wealth=statistics.fmean(wealth))


def testimony_table(pre, post, k=9.0):
    print("    pre-shift  truth P(profitable | chi) = %.4f" % pre.truth())
    print("    post-shift truth P(profitable | chi) = %.4f" % post.truth())
    print("    %-12s %11s %10s %11s %10s %10s %10s"
          % ("protocol", "inflation", "|s-truth|", "|s-truth|", "step", "survival",
             "wealth"))
    print("    %-12s %11s %10s %11s %10s %10s %10s"
          % ("", "n / n_indep", "pre-shift", "post-shift", "1/(n+1)", "", ""))
    print("    " + "-" * 78)
    rows = {}
    for p in ("isolated", "honest", "plagiarism"):
        r = testimony(pre, post, p, k=k)
        rows[p] = r
        print("    %-12s %11.3f %10.4f %11.4f %10.5f %10.4f %10.1f"
              % (p, r["inflation"], r["err_pre"], r["err_post"], r["step"],
                 r["survival"], r["wealth"]))
    return rows


# --------------------------------------------------------------------- driver

if __name__ == "__main__":
    base = Ecology()
    print("=" * 78)
    print("=== The four arms in the baseline ecology (8000 lives) ===")
    arms_table(base)

    print()
    print("=== The personality parameter, both objectives (6000 lives) ===")
    k_table(base)

    print()
    print("=== Robustness over the three distributions (3000 lives) ===")
    sweep("hetero", (0.0, 0.20, 0.40, 0.60, 0.80, 1.0), base)
    print()
    sweep("inedible", (0.0, 0.20, 0.40, 0.60, 0.80), base)
    print()
    sweep("reservoir", (0.0, 0.15, 0.30, 0.45, 0.60), base)

    print()
    print("=== Interaction of (T) and (I) (1200 lives) ===")
    corner_grid(base, n=1200)

    print()
    print("=== Testimony: two learners sharing a belief namespace ===")
    pre = replace(base, name="pre")
    post = replace(base, hetero=0.05, inedible=0.65, name="post")
    print("  shifting ecology")
    testimony_table(pre, post)
    print()
    print("  stationary control")
    testimony_table(pre, pre)
    print("=" * 78)
