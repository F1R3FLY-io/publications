"""
fibre.py -- the computations for the fibre-side (gradient) sections of
"Graded Where-Clauses over the Reals".

Everything here is stdlib only, matching gradedsim.py and forager.py.

  E1  PLN revision IS a natural-gradient step (the headline proposition),
      with a negative control that fixes where the metric is evaluated.
  E2  the step is IMPLICIT, hence unconditionally stable: the explicit
      forms leave the unit interval and the implicit one cannot.
  E3  the confidence schedule satisfies the Robbins-Monro conditions for
      every personality parameter, so k governs the transient only.
  E4  a modality-free clause is a posynomial: log-convex in log-parameters.
      Its normalisation is not; the fibre is smooth but not convex.
  E5  the score of a causal history decomposes over firing sites.
  E6  the modal clause: differentiating through the graded fixed point,
      and the coincidence of the two convergence conditions.
"""

import math
import random

TOL = 1e-12


# ----------------------------------------------------------------- utilities

def jacobi_eigenvalues(A, sweeps=60):
    """Eigenvalues of a small real symmetric matrix by cyclic Jacobi."""
    n = len(A)
    M = [row[:] for row in A]
    for _ in range(sweeps):
        off = math.sqrt(sum(M[i][j] ** 2 for i in range(n)
                            for j in range(n) if i != j))
        if off < 1e-14:
            break
        for p in range(n - 1):
            for q in range(p + 1, n):
                if abs(M[p][q]) < 1e-16:
                    continue
                theta = (M[q][q] - M[p][p]) / (2.0 * M[p][q])
                t = (1.0 if theta >= 0 else -1.0) / \
                    (abs(theta) + math.sqrt(theta * theta + 1.0))
                c = 1.0 / math.sqrt(t * t + 1.0)
                s = t * c
                for k in range(n):
                    mkp, mkq = M[k][p], M[k][q]
                    M[k][p] = c * mkp - s * mkq
                    M[k][q] = s * mkp + c * mkq
                for k in range(n):
                    mpk, mqk = M[p][k], M[q][k]
                    M[p][k] = c * mpk - s * mqk
                    M[q][k] = s * mpk + c * mqk
    return sorted(M[i][i] for i in range(n))


def hessian(f, u, h=1e-4):
    """Central-difference Hessian of a scalar function of a vector."""
    n = len(u)
    H = [[0.0] * n for _ in range(n)]
    for i in range(n):
        for j in range(i, n):
            up = list(u)
            if i == j:
                up[i] = u[i] + h
                a = f(up)
                up[i] = u[i] - h
                b = f(up)
                H[i][i] = (a - 2.0 * f(u) + b) / (h * h)
            else:
                def at(di, dj):
                    v = list(u)
                    v[i] += di * h
                    v[j] += dj * h
                    return f(v)
                H[i][j] = H[j][i] = (at(1, 1) - at(1, -1)
                                     - at(-1, 1) + at(-1, -1)) / (4.0 * h * h)
    return H


def solve(A, b):
    """Gauss-Jordan solve of a small dense system."""
    n = len(A)
    M = [A[i][:] + [b[i]] for i in range(n)]
    for col in range(n):
        piv = max(range(col, n), key=lambda r: abs(M[r][col]))
        if abs(M[piv][col]) < 1e-15:
            raise ZeroDivisionError("singular")
        M[col], M[piv] = M[piv], M[col]
        d = M[col][col]
        M[col] = [x / d for x in M[col]]
        for r in range(n):
            if r != col and M[r][col] != 0.0:
                f = M[r][col]
                M[r] = [x - f * y for x, y in zip(M[r], M[col])]
    return [M[i][n] for i in range(n)]


def spectral_radius(T, iters=4000):
    n = len(T)
    v = [1.0 / math.sqrt(n)] * n
    lam = 0.0
    for _ in range(iters):
        w = [sum(T[i][j] * v[j] for j in range(n)) for i in range(n)]
        nrm = math.sqrt(sum(x * x for x in w))
        if nrm < 1e-300:
            return 0.0
        v = [x / nrm for x in w]
        lam = nrm
    return lam


# ------------------------------------------------- E1  revision = natural gradient

def revise(s, n, x, m=1.0):
    """PLN revision of (s, n) by evidence of strength x and count m."""
    return (n * s + m * x) / (n + m), n + m


def natural_gradient_step(s, n_metric, x, eta=1.0):
    """
    Ascent on the Bernoulli log-likelihood with the Fisher metric.
        score  = dl/ds  = (x - s) / (s(1-s))
        Fisher = I(s;n) = n / (s(1-s))
    """
    score = (x - s) / (s * (1.0 - s))
    fisher = n_metric / (s * (1.0 - s))
    return s + eta * score / fisher


def e1(trials=20000, seed=20260816):
    rng = random.Random(seed)
    worst_post = worst_prior = 0.0
    for _ in range(trials):
        s = rng.uniform(0.02, 0.98)
        n = rng.randint(1, 500)
        x = rng.choice([0.0, 1.0])
        target, _ = revise(s, n, x)
        worst_post = max(worst_post, abs(target - natural_gradient_step(s, n + 1, x)))
        worst_prior = max(worst_prior, abs(target - natural_gradient_step(s, n, x)))
    print("E1  PLN revision as a natural-gradient step   (%d random (s,n,x))" % trials)
    print("      Fisher metric at the POSTERIOR count n+1 : worst |diff| = %.3e"
          % worst_post)
    print("      Fisher metric at the PRIOR count n       : worst |diff| = %.3e"
          % worst_prior)
    return worst_post, worst_prior


# ------------------------------------------------------- E2  implicit stability

def e2(trials=200000, seed=11):
    """
    Three discretisations of the same fibre gradient.  The implicit one
    (which is revision) stays strictly interior; the explicit one is
    absorbed at the boundary when the metric is weak, and overshoots
    outright once the step exceeds one; the metric-free one is wild.
    """
    rng = random.Random(seed)
    keys = ("implicit", "explicit", "explicit15", "euclid")
    out = {k: [0, 0, 0.0] for k in keys}          # escapes, absorptions, worst
    for _ in range(trials):
        s = rng.uniform(1e-3, 1 - 1e-3)
        n = rng.randint(1, 50)
        x = rng.choice([0.0, 1.0])
        cands = {
            "implicit": natural_gradient_step(s, n + 1, x),
            "explicit": natural_gradient_step(s, n, x),
            "explicit15": natural_gradient_step(s, n, x, eta=1.5),
            "euclid": s + (1.0 / (n + 1)) * (x - s) / (s * (1.0 - s)),
        }
        for key, v in cands.items():
            if v < -1e-12 or v > 1.0 + 1e-12:
                out[key][0] += 1
                out[key][2] = max(out[key][2], max(-v, v - 1.0))
            elif v <= 1e-12 or v >= 1.0 - 1e-12:
                out[key][1] += 1
    print("E2  three discretisations of the fibre gradient  (%d random steps)"
          % trials)
    print("      %-42s %9s %11s %14s"
          % ("", "escapes", "absorbed", "worst excursion"))
    for key, label in (("implicit", "implicit natural gradient (= revision)"),
                       ("explicit", "explicit natural gradient, step 1"),
                       ("explicit15", "explicit natural gradient, step 1.5"),
                       ("euclid", "ascent on the raw score, step 1/(n+1)")):
        esc, ab, exc = out[key]
        print("      %-42s %9d %11d %14.3e" % (label, esc, ab, exc))
    print("      absorbed = landed exactly on 0 or 1, where the score is undefined")
    return out


# --------------------------------------------------- E3  Robbins-Monro schedule

def e3(N=200000, ks=(1, 4, 9, 64, 512)):
    print("E3  the confidence schedule eta_n = 1/(n+k), truncated at N = %d" % N)
    print("      %6s %14s %16s" % ("k", "sum eta_n", "sum eta_n^2"))
    rows = []
    for k in ks:
        s1 = sum(1.0 / (n + k) for n in range(1, N))
        s2 = sum(1.0 / (n + k) ** 2 for n in range(1, N))
        print("      %6d %14.4f %16.6f" % (k, s1, s2))
        rows.append((k, s1, s2))
    print("      (sum eta_n grows like log N for every k; sum eta_n^2 is bounded)")
    return rows


# ------------------------------------------- E4  posynomial clause, log-convexity

def random_clause(rng, nparam=3, nterms=5, maxdeg=3):
    """
    A modality-free graded clause over R>=0 in disjunctive form: a sum of
    products of parameters, with non-negative coefficients supplied by the
    crisp predicates.  That is a posynomial in the parameters.
    """
    terms = []
    for _ in range(nterms):
        coef = rng.uniform(0.2, 3.0)
        expo = [rng.randint(0, maxdeg) for _ in range(nparam)]
        terms.append((coef, expo))
    return terms


def clause_value(terms, theta):
    return sum(c * math.prod(t ** e for t, e in zip(theta, ex))
               for c, ex in terms)


def e4(trials=400, seed=7, nparam=3):
    rng = random.Random(seed)
    worst_clause = float("inf")      # min eigenvalue of Hess log psi in log-coords
    indefinite = 0
    worst_neg = 0.0
    for _ in range(trials):
        terms = random_clause(rng, nparam=nparam)
        other = random_clause(rng, nparam=nparam)
        u = [rng.uniform(-1.0, 1.0) for _ in range(nparam)]

        def logpsi(uu):
            return math.log(clause_value(terms, [math.exp(z) for z in uu]))

        def logp(uu):
            th = [math.exp(z) for z in uu]
            a = clause_value(terms, th)
            b = clause_value(other, th)
            return math.log(a / (a + b))

        ev = jacobi_eigenvalues(hessian(logpsi, u))
        worst_clause = min(worst_clause, ev[0])
        ev2 = jacobi_eigenvalues(hessian(logp, u))
        if ev2[0] < -1e-6 and ev2[-1] > 1e-6:
            indefinite += 1
            worst_neg = min(worst_neg, ev2[0])
    print("E4  curvature in log-parameters   (%d random clauses, %d parameters)"
          % (trials, nparam))
    print("      Hess log(clause)        : least eigenvalue over all trials = %+.3e"
          % worst_clause)
    print("        -> log-convex, so the clause is a posynomial")
    print("      Hess log(branch prob.)  : indefinite in %d of %d trials"
          % (indefinite, trials))
    print("        -> the normalisation is signomial; most negative eigenvalue %+.3e"
          % worst_neg)
    return worst_clause, indefinite, trials


# ------------------------------------------------- E5  the score decomposes

class Site:
    """A contention set: a list of candidates, each a posynomial clause."""

    def __init__(self, clauses):
        self.clauses = clauses

    def weights(self, theta):
        return [clause_value(c, theta) for c in self.clauses]

    def prob(self, theta, j):
        w = self.weights(theta)
        return w[j] / sum(w)

    def local_score(self, theta, j, i, h=1e-6):
        """d/dtheta_i  log P(candidate j at this site)."""
        up, dn = list(theta), list(theta)
        up[i] += h
        dn[i] -= h
        return (math.log(self.prob(up, j)) - math.log(self.prob(dn, j))) / (2 * h)

    def analytic_score(self, theta, j, i):
        """The closed form: dlog w_j - (sum_k dw_k)/(sum_k w_k)."""
        w = self.weights(theta)
        tot = sum(w)
        dw = [dclause(c, theta, i) for c in self.clauses]
        return dw[j] / w[j] - sum(dw) / tot


def dclause(terms, theta, i):
    """Exact partial derivative of a posynomial."""
    tot = 0.0
    for c, ex in terms:
        if ex[i] == 0:
            continue
        p = c * ex[i] * theta[i] ** (ex[i] - 1)
        for k, (t, e) in enumerate(zip(theta, ex)):
            if k != i:
                p *= t ** e
        tot += p
    return tot


def e5(trials=300, seed=3, nsites=6, nparam=3):
    rng = random.Random(seed)
    worst = 0.0
    for _ in range(trials):
        theta = [rng.uniform(0.3, 2.5) for _ in range(nparam)]
        history = []
        for _ in range(nsites):
            k = rng.randint(2, 4)
            site = Site([random_clause(rng, nparam=nparam, nterms=3) for _ in range(k)])
            history.append((site, rng.randrange(k)))

        def loglik(th):
            return sum(math.log(s.prob(th, j)) for s, j in history)

        for i in range(nparam):
            h = 1e-6
            up, dn = list(theta), list(theta)
            up[i] += h
            dn[i] -= h
            fd = (loglik(up) - loglik(dn)) / (2 * h)
            local = sum(s.analytic_score(theta, j, i) for s, j in history)
            worst = max(worst, abs(fd - local))
    print("E5  the score of a causal history decomposes over firing sites")
    print("      %d histories of %d sites, %d parameters" % (trials, nsites, nparam))
    print("      worst |sum of local scores  -  gradient of the log-likelihood|"
          " = %.3e" % worst)
    return worst


# ------------------------------------- E6  differentiating the graded fixed point

def modal_fixpoint(b, T):
    """v = b + T v, solved directly."""
    n = len(b)
    A = [[(1.0 if i == j else 0.0) - T[i][j] for j in range(n)] for i in range(n)]
    return solve(A, b)


def e6(trials=200, seed=5, n=5, nparam=2):
    """
    A modal clause  psi = mu X. ( beta (+) <K> X )  over R>=0 on a finite
    transition system is  v = b + T v.  Its parameter derivative is the
    ADJOINT recursion  dv = (I - T)^{-1} ( db + (dT) v ), which converges by
    the same Neumann series as the fixed point itself.
    """
    rng = random.Random(seed)
    worst = 0.0
    rho_min, rho_max = 10.0, 0.0
    for _ in range(trials):
        theta = [rng.uniform(0.4, 1.2) for _ in range(nparam)]
        b0 = [rng.uniform(0.1, 1.0) for _ in range(n)]
        T0 = [[rng.uniform(0.0, 1.0) for _ in range(n)] for _ in range(n)]
        scale = rng.uniform(0.25, 0.85) / spectral_radius(T0)

        def build(th):
            b = [b0[i] * th[0] for i in range(n)]
            T = [[T0[i][j] * scale * th[1] for j in range(n)] for i in range(n)]
            return b, T

        b, T = build(theta)
        rho = spectral_radius(T)
        if rho >= 1.0:
            continue
        rho_min, rho_max = min(rho_min, rho), max(rho_max, rho)
        v = modal_fixpoint(b, T)

        for i in range(nparam):
            # adjoint / implicit-function derivative
            db = [b0[j] * (1.0 if i == 0 else 0.0) for j in range(n)]
            dT = [[T0[p][q] * scale * (1.0 if i == 1 else 0.0) for q in range(n)]
                  for p in range(n)]
            rhs = [db[p] + sum(dT[p][q] * v[q] for q in range(n)) for p in range(n)]
            A = [[(1.0 if p == q else 0.0) - T[p][q] for q in range(n)]
                 for p in range(n)]
            dv = solve(A, rhs)
            # central difference on the fixed point itself
            h = 1e-6
            up, dn = list(theta), list(theta)
            up[i] += h
            dn[i] -= h
            vu = modal_fixpoint(*build(up))
            vd = modal_fixpoint(*build(dn))
            fd = [(vu[p] - vd[p]) / (2 * h) for p in range(n)]
            worst = max(worst, max(abs(a - c) for a, c in zip(dv, fd)))
    print("E6  the gradient of a modal clause, through the graded fixed point")
    print("      %d systems of dimension %d, spectral radius in [%.3f, %.3f]"
          % (trials, n, rho_min, rho_max))
    print("      worst |adjoint solution  -  finite difference| = %.3e" % worst)
    return worst, rho_min, rho_max


def e6_divergence():
    """
    The Neumann series for the derivative is the same series as for the value.
    Below one they both converge; at or above one neither does.
    """
    print("      Neumann truncation at spectral radius r, 60 terms:")
    n = 3
    for r in (0.5, 0.9, 0.99, 1.0, 1.05):
        T = [[r / n] * n for _ in range(n)]
        b = [1.0] * n
        val = [0.0] * n
        term = b[:]
        for _ in range(60):
            val = [val[i] + term[i] for i in range(n)]
            term = [sum(T[i][j] * term[j] for j in range(n)) for i in range(n)]
        ex = "diverges" if r >= 1.0 else "%.6f" % modal_fixpoint(b, T)[0]
        print("        r = %.2f   truncated value %12.6f   exact %s"
              % (r, val[0], ex))


if __name__ == "__main__":
    print("=" * 74)
    e1()
    print()
    e2()
    print()
    e3()
    print()
    e4()
    print()
    e5()
    print()
    e6()
    e6_divergence()
    print("=" * 74)
