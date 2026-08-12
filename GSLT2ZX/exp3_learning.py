"""
Experiment 3 -- a plasticity read is a which-route read.

A Hebbian update needs to know WHICH synapse carried the spike, so it must
couple a trace register to the branch record.  We model a partial read: a
controlled rotation by alpha copies the "which soma" bit of the step-1
branch into a trace register, which is then traced out.  The overlap of the
two trace states is c = cos(alpha), so c = 1 is no read and c = 0 is a
complete one.

Theorem 8.1 says a branch record is a which-path record.  The prediction is
that the correlation produced by interference degrades monotonically in the
information extracted, and that a complete read returns the network to the
behaviour of the Z cap.
"""

import itertools
import numpy as np
import exp2_network as N


def outcome_law(t, lam, alpha):
    """P(a,b) under the X cap with a partial which-route read of strength
    alpha, obtained by tracing out the trace register."""
    g = N.build(t, lam, "X", trace_angle=alpha)
    T = N.term_vector(g)                      # axes (s1,s2,a,b,TR)
    law = {}
    for idx in itertools.product(*[range(2)] * 4):
        p = float(np.sum(np.abs(T[idx]) ** 2))
        if p > 1e-12:
            law[idx] = p
    tot = sum(law.values())
    return {k: v / tot for k, v in law.items()}


def correlation(law):
    """Pearson correlation of the two firing indicators."""
    ea = sum(v for k, v in law.items() if k[2] == 1)
    eb = sum(v for k, v in law.items() if k[3] == 1)
    eab = sum(v for k, v in law.items() if k[2] == 1 and k[3] == 1)
    cov = eab - ea * eb
    va = ea * (1 - ea)
    vb = eb * (1 - eb)
    if va < 1e-12 or vb < 1e-12:
        return float("nan")
    return cov / np.sqrt(va * vb)


def co_fire(law):
    return sum(v for k, v in law.items() if k[2] == 1 and k[3] == 1)


if __name__ == "__main__":
    r2 = 1 / np.sqrt(2)
    bs = [[r2, r2], [1j * r2, -1j * r2]]
    lam = r2

    print("Experiment 3: correlation versus the strength of the plasticity read")
    print()
    print(f"    {'alpha':>8s} {'c=cos a':>9s} {'P(both fire)':>13s} {'corr(a,b)':>11s}")
    rows = []
    for k in range(9):
        alpha = k * np.pi / 16
        law = outcome_law(bs, lam, alpha)
        c = np.cos(alpha)
        rows.append((alpha, c, co_fire(law), correlation(law)))
        print(f"    {alpha:8.4f} {c:9.6f} {co_fire(law):13.6f} "
              f"{correlation(law):11.6f}")

    # the classical reference: the same theory under the Z cap
    zdist, _ = N.zx_distributions(bs, lam)
    zl = N.normalise(zdist)
    print()
    print(f"    Z cap (no coherence at all):        "
          f"P(both)={co_fire(zl):.6f}  corr={correlation(zl):.6f}")

    # closed form:  P(both) = eta^2 / (1 + eta^2)  with eta = 1 - c
    print()
    print("    against the closed form  eta^2/(1+eta^2),  eta = 1 - c :")
    worst = 0.0
    for alpha, c, p, _ in rows:
        eta = 1 - c
        pred = eta ** 2 / (1 + eta ** 2)
        worst = max(worst, abs(p - pred))
        print(f"      eta = {eta:8.6f}   measured {p:8.6f}   predicted {pred:8.6f}")
    print(f"    worst deviation: {worst:.2e}")
