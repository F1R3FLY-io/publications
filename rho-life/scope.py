#!/usr/bin/env python3
r"""
scope.py --- computations for the fifth rho-life note,
"Scope and Manufacture".

Blocks:
  (1) extension growth of a generated namespace mu X.@((phi \/ X) | (psi \/ X))
  (2) overcounting: CN_n in a population separating at depth h with branching b
  (3) the sampling floor: copies needed to sustain a depth-d percept
  (4) the worked composite: a four-skill individual and its copy series

Exact integer / rational arithmetic wherever it is available.
"""

from fractions import Fraction
from math import comb, ceil, log


# ---------------------------------------------------------------- (1)

def extension_growth(levels=5, atoms=2):
    r"""
    u_j = number of names of stratum <= j in the extension of
          mu X . @( (phi \/ X) | (psi \/ X) )
    A name is either an atom (stratum 0) or @(P|Q) with *P, *Q names of
    lower stratum.  Parallel composition is commutative, so the composites
    at stratum <= j+1 are unordered pairs with repetition drawn from u_j.
    """
    rows = []
    u_prev = atoms
    rows.append((0, atoms, atoms))
    for j in range(1, levels):
        u = atoms + comb(u_prev + 1, 2)
        rows.append((j, u, u - u_prev))
        u_prev = u
    return rows


# ---------------------------------------------------------------- (2)

def overcount(b=3, h=8):
    """
    A population whose members are the leaves of a depth-h, branching-b
    behaviour tree.  Two leaves are n-step bisimilar iff their paths agree
    for n steps.  An observer of resolution n therefore counts the whole
    depth-n subtree containing P as copies of P:
        CN_n(P) = b^(h-n),      CN_h(P) = 1.
    """
    return [(n, b ** (h - n)) for n in range(h + 1)]


# ---------------------------------------------------------------- (3)

def optimal_depth(p, Y0=100.0, c=2.0, dmax=12):
    """engine-note Prop 'deep integration is exponentially fragile':
       maximise Y(d) p^d - c d with Y(d) = Y0 (1 - 2^-d)."""
    best, arg = None, 0
    for d in range(1, dmax + 1):
        v = Y0 * (1 - 2.0 ** (-d)) * p ** d - c * d
        if best is None or v > best:
            best, arg = v, d
    return arg, best


def sampling_floor(p, d, m=20):
    """copies needed for m independent depth-d confirmations per round."""
    return ceil(m * p ** (-d))


def sampling_table(ps, ds, m=1):
    return [(p, [ceil(m * p ** (-d)) for d in ds]) for p in ps]


# ---------------------------------------------------------------- (4)

SKILLS = [
    # name        p     divergence depth from the kernel
    ("Forage",  0.95,  4),
    ("Pod",     0.99,  2),
    ("Mate",    0.90,  3),
    ("Rear",    0.99,  3),
]

M_CONF = 20          # independent confirmations wanted per hypothesis
POD = 12             # individuals in the pod

# assembly floor, in joining steps, from the cheap bounds of section 5
KERNEL = {
    "inside/boundary media (L1,L2)": 2,
    "germ: one level for the quotation (L3)": 1,
    "located stack + reclaim cycle": 1,
    "assay: complementary guard pair": 1,
}


def composite():
    out = {}
    rows = []
    total_learners = 0
    for name, p, sep in SKILLS:
        d, net = optimal_depth(p)
        N = sampling_floor(p, d, M_CONF)
        total_learners += N
        rows.append(dict(name=name, p=p, dstar=d, net=net, N=N, sep=sep))
    out["skills"] = rows
    out["learners_per_individual"] = total_learners

    # copy series for the pod's generated namespace
    out["copy_series"] = {
        0: POD * total_learners,     # mortal scientists
        1: POD * len(SKILLS),        # skill-ecologies
        2: POD,                      # individuals
        3: 1,                        # the pod
    }
    out["flat_collapse"] = sum(out["copy_series"].values())

    # assembly floor
    kernel_total = sum(KERNEL.values())
    differentia = sum(r["dstar"] for r in rows)
    wiring_join = ceil(log(len(SKILLS), 2))          # joining k parts
    wiring_engines = len(SKILLS)                     # a cycle, so beta_1 >= 1
    closure = 1
    shared = kernel_total + differentia + wiring_join + wiring_engines + closure
    unshared = (kernel_total * len(SKILLS) + differentia
                + wiring_join + wiring_engines + closure)
    out["floor"] = dict(kernel=kernel_total, differentia=differentia,
                        wiring_join=wiring_join, wiring_engines=wiring_engines,
                        closure=closure, shared=shared, unshared=unshared,
                        saving=unshared - shared,
                        saving_pct=100.0 * (unshared - shared) / unshared,
                        content_pct=100.0 * differentia / shared)

    # what an observer of resolution n reports about the learner stratum
    seps = sorted(r["sep"] for r in rows)
    reports = []
    for n in range(0, max(seps) + 2):
        # skills not yet separated pool into one reported class
        pooled = [r for r in rows if r["sep"] > n]
        resolved = [r for r in rows if r["sep"] <= n]
        classes = (1 if pooled else 0) + len(resolved)
        sizes = ([sum(r["N"] for r in pooled)] if pooled else []) \
              + [r["N"] for r in resolved]
        reports.append((n, classes, POD * max(sizes)))
    out["observer"] = reports
    out["true_classes"] = len(rows)
    out["true_largest"] = POD * max(r["N"] for r in rows)
    return out


# ---------------------------------------------------------------- (5)

def individuation(eps, theta, rho, h_iso):
    """engine-note S7: a region may close its namespace iff
       h(B) >= (eps*theta/rho) * r.  Returns the critical radius."""
    kappa = eps * theta / rho
    return h_iso / kappa, kappa


def individuation_block():
    rows = []
    for label, eps, h_iso, r in [
            ("individual (internal media, p=0.99)", 0.01, 0.50, 4),
            ("pod (cross-individual media, p=0.90)", 0.10, 0.30, 12),
    ]:
        rstar, kappa = individuation(eps, 0.2, 5.0, h_iso)
        rows.append((label, eps, h_iso, r, kappa, rstar, r <= rstar))
    return rows


# ---------------------------------------------------------------- report

def main():
    print("=" * 66)
    print("(1) extension of a three-atom generator")
    print("=" * 66)
    for j, u, s in extension_growth(5):
        print(f"  stratum <= {j}:  |N| = {u:>8}   new at this stratum: {s:>8}")

    print()
    print("=" * 66)
    print("(2) overcounting, b = 3, h = 8")
    print("=" * 66)
    for n, cn in overcount(3, 8):
        print(f"  resolution n = {n}:  CN_n = {cn:>6}   overcount x{cn}")

    print()
    print("=" * 66)
    print("(3) sampling floor: copies for m = %d depth-d confirmations" % M_CONF)
    print("=" * 66)
    ds = list(range(1, 9))
    print("      p  " + "".join(f"{d:>7}" for d in ds))
    for p, row in sampling_table([0.99, 0.95, 0.90, 0.80, 0.70, 0.50], ds, m=M_CONF):
        print(f"  {p:>5.2f}  " + "".join(f"{v:>7}" for v in row))

    print()
    print("=" * 66)
    print("(4) the worked composite")
    print("=" * 66)
    c = composite()
    print("  skill      p     d*     net     N")
    for r in c["skills"]:
        print(f"  {r['name']:<9}{r['p']:<6.2f}{r['dstar']:<7}"
              f"{r['net']:<8.1f}{r['N']}")
    print(f"  learners per individual: {c['learners_per_individual']}")
    print()
    print("  copy series over the pod namespace:")
    for j, v in c["copy_series"].items():
        print(f"    stratum {j}: {v}")
    print(f"    flat collapse: {c['flat_collapse']}")
    print()
    f = c["floor"]
    print("  assembly floor (joining steps):")
    for k, v in f.items():
        print(f"    {k}: {v}")
    print()
    print("  individuation (theta=0.2, rho=5.0):")
    for lab, eps, hi, r, kappa, rstar, ok in individuation_block():
        print(f"    {lab}: eps={eps}, h={hi}, r={r}, "
              f"kappa={kappa:.2e}, r*={rstar:.1f}, closes={ok}")
    print()
    print("  observer of resolution n reports:")
    print(f"    truth: {c['true_classes']} classes, largest {c['true_largest']}")
    for n, cl, lg in c["observer"]:
        print(f"    n = {n}: {cl} classes, largest reported {lg}")


if __name__ == "__main__":
    main()
