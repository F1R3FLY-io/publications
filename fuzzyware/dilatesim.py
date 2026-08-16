"""
dilatesim.py --- companion to "Graded Where-Clauses", version 4.

Computes every number in the section on dilation, heralds and the price of
erasure.  Standard library only, matching gradedsim.py.

Contents
  * complex matrix arithmetic, inverse, and a Denman--Beavers square root
  * the defect operators and the Julia--Halmos two-defect block
  * verification that the dilated step is an isometry and that its ok-branch
    reproduces the undilated step exactly
  * the herald statistics of the post-selection hazard, hence the retry cost
  * a check that heralding is inert at an isometric site
  * the block-diagonality of the weakening-free ("linear fragment") gadget

Run:  python3 dilatesim.py
"""

import cmath
import math
import random

# ---------------------------------------------------------------------------
# minimal complex matrix arithmetic
# ---------------------------------------------------------------------------


def eye(n):
    return [[1.0 + 0j if i == j else 0j for j in range(n)] for i in range(n)]


def zeros(m, n):
    return [[0j for _ in range(n)] for _ in range(m)]


def dag(A):
    return [[A[i][j].conjugate() for i in range(len(A))] for j in range(len(A[0]))]


def mul(A, B):
    m, k, n = len(A), len(B), len(B[0])
    C = zeros(m, n)
    for i in range(m):
        for t in range(k):
            a = A[i][t]
            if a == 0:
                continue
            for j in range(n):
                C[i][j] += a * B[t][j]
    return C


def add(A, B):
    return [[A[i][j] + B[i][j] for j in range(len(A[0]))] for i in range(len(A))]


def sub(A, B):
    return [[A[i][j] - B[i][j] for j in range(len(A[0]))] for i in range(len(A))]


def scale(c, A):
    return [[c * A[i][j] for j in range(len(A[0]))] for i in range(len(A))]


def maxabs(A):
    return max(abs(x) for row in A for x in row)


def inv(A):
    """Gauss--Jordan inverse of a square complex matrix."""
    n = len(A)
    M = [row[:] + eye(n)[i][:] for i, row in enumerate(A)]
    for col in range(n):
        piv = max(range(col, n), key=lambda r: abs(M[r][col]))
        if abs(M[piv][col]) < 1e-14:
            raise ZeroDivisionError("singular")
        M[col], M[piv] = M[piv], M[col]
        p = M[col][col]
        M[col] = [x / p for x in M[col]]
        for r in range(n):
            if r == col:
                continue
            f = M[r][col]
            if f == 0:
                continue
            M[r] = [x - f * y for x, y in zip(M[r], M[col])]
    return [row[n:] for row in M]


def _jacobi_sym(S, sweeps=100, tol=1e-15):
    """
    Cyclic Jacobi eigendecomposition of a real symmetric matrix.
    Returns (eigenvalues, V) with S = V diag(w) V^T.  Handles singular S.
    """
    n = len(S)
    A = [row[:] for row in S]
    V = [[1.0 if i == j else 0.0 for j in range(n)] for i in range(n)]
    for _ in range(sweeps):
        off = math.sqrt(sum(A[i][j] ** 2 for i in range(n) for j in range(n) if i != j))
        if off < tol:
            break
        for p in range(n - 1):
            for q in range(p + 1, n):
                if abs(A[p][q]) < 1e-18:
                    continue
                theta = (A[q][q] - A[p][p]) / (2.0 * A[p][q])
                t = math.copysign(1.0, theta) / (abs(theta) + math.sqrt(theta * theta + 1.0))
                c = 1.0 / math.sqrt(t * t + 1.0)
                s = t * c
                for k in range(n):
                    akp, akq = A[k][p], A[k][q]
                    A[k][p] = c * akp - s * akq
                    A[k][q] = s * akp + c * akq
                for k in range(n):
                    apk, aqk = A[p][k], A[q][k]
                    A[p][k] = c * apk - s * aqk
                    A[q][k] = s * apk + c * aqk
                for k in range(n):
                    vkp, vkq = V[k][p], V[k][q]
                    V[k][p] = c * vkp - s * vkq
                    V[k][q] = s * vkp + c * vkq
    return [A[i][i] for i in range(n)], V


def sqrtm_psd(M):
    """
    Principal square root of a Hermitian positive-semidefinite complex matrix,
    via the real symmetric embedding X + iY |-> [[X, -Y], [Y, X]].  The
    embedding is a ring homomorphism, so the square root of the embedded
    matrix is the embedding of the square root, and Jacobi copes with the
    singular case that a Newton iteration does not.
    """
    n = len(M)
    X = [[M[i][j].real for j in range(n)] for i in range(n)]
    Y = [[M[i][j].imag for j in range(n)] for i in range(n)]
    S = [[0.0] * (2 * n) for _ in range(2 * n)]
    for i in range(n):
        for j in range(n):
            S[i][j] = X[i][j]
            S[i][n + j] = -Y[i][j]
            S[n + i][j] = Y[i][j]
            S[n + i][n + j] = X[i][j]
    w, V = _jacobi_sym(S)
    r = [math.sqrt(max(x, 0.0)) for x in w]
    R = [[sum(V[i][k] * r[k] * V[j][k] for k in range(2 * n)) for j in range(2 * n)]
         for i in range(2 * n)]
    return [[complex(R[i][j], R[n + i][j]) for j in range(n)] for i in range(n)]


# ---------------------------------------------------------------------------
# defects and the Julia--Halmos block
# ---------------------------------------------------------------------------


def defect(A):
    """D_A = sqrt(1 - A^dag A)."""
    n = len(A)
    return sqrtm_psd(sub(eye(n), mul(dag(A), A)))


def julia_halmos(A):
    """
    The two-defect block

        U = [ A            D_{A^dag} ]
            [ D_A         -A^dag     ]

    which is unitary for any contraction A, without assuming A normal.
    """
    n = len(A)
    Da = defect(A)
    Dad = defect(dag(A))
    Aad = dag(A)
    U = zeros(2 * n, 2 * n)
    for i in range(n):
        for j in range(n):
            U[i][j] = A[i][j]
            U[i][n + j] = Dad[i][j]
            U[n + i][j] = Da[i][j]
            U[n + i][n + j] = -Aad[i][j]
    return U


def unitarity_defect(U):
    n = len(U)
    return maxabs(sub(mul(dag(U), U), eye(n)))


# ---------------------------------------------------------------------------
# experiment 1: the dilation is exact, and the ok-branch is unchanged
# ---------------------------------------------------------------------------


def exp1_dilation(eps=1e-3, trials=200, seed=20260815):
    """
    The filter clause diag(1, eps), plus random contractions, dilated.
    Reports the worst deviation of U^dag U from the identity, and the worst
    deviation of the ok-block of U from A itself.
    """
    rng = random.Random(seed)
    filt = [[1.0 + 0j, 0j], [0j, eps + 0j]]
    cases = [filt]
    for _ in range(trials):
        A = [[complex(rng.uniform(-1, 1), rng.uniform(-1, 1)) for _ in range(2)]
             for _ in range(2)]
        # rescale to a strict contraction: divide by a bound on the top
        # singular value
        s = math.sqrt(maxabs(mul(dag(A), A)) * 2) + 1e-9
        A = scale(1.0 / (s * 1.05), A)
        cases.append(A)

    worst_u, worst_ok = 0.0, 0.0
    for A in cases:
        U = julia_halmos(A)
        worst_u = max(worst_u, unitarity_defect(U))
        ok = [[U[i][j] for j in range(2)] for i in range(2)]
        worst_ok = max(worst_ok, maxabs(sub(ok, A)))
    return len(cases), worst_u, worst_ok


# ---------------------------------------------------------------------------
# experiment 2: herald statistics of the post-selection hazard
# ---------------------------------------------------------------------------


def exp2_retry_cost(ns=(4, 6, 8, 10), eps=1e-3):
    """
    A Hadamard layer on n wires followed by diag(1, eps) on each wire.  The
    dilated step heralds ok with probability |A psi|^2 per site; the whole run
    succeeds when every herald reads ok.  Reported: surviving norm, success
    probability, and the expected number of attempts of a memoryless retry.

    Reproduces the figures of the hazard subsection from the amplitudes rather
    than importing them.
    """
    out = []
    for n in ns:
        # a Hadamard layer takes |0..0> to the uniform superposition;
        # diag(1, eps) then keeps amplitude 1 on the 0 branch of each wire.
        # Surviving norm factorises over wires.
        per_wire = math.sqrt((1.0 + eps * eps) / 2.0)
        norm = per_wire ** n
        p = norm * norm
        out.append((n, norm, p, 1.0 / p))
    return out


# ---------------------------------------------------------------------------
# experiment 3: heralding is inert at an isometric site
# ---------------------------------------------------------------------------


def exp3_inert_herald(phases=(0.0, math.pi, math.pi / 2)):
    """
    The minimal interferometer of the recombiner section, with and without a
    herald message appended to the outcome.  At an isometric site the herald
    reads ok on every candidate, so the outcomes remain congruent and the
    coherent sum is unchanged; if instead the herald carried the candidate
    index, the outcomes would separate and the sum would go incoherent.
    """
    rows = []
    for theta in phases:
        h = 1.0 / math.sqrt(2.0)
        a, b = h + 0j, h * cmath.exp(1j * theta)
        bare = abs(a + b) ** 2                      # outcomes coincide
        with_ok = abs(a + b) ** 2                   # herald ok on both
        with_index = abs(a) ** 2 + abs(b) ** 2      # herald names the candidate
        rows.append((theta, bare, with_ok, with_index))
    return rows


# ---------------------------------------------------------------------------
# experiment 4: the weakening-free gadget is block-diagonal
# ---------------------------------------------------------------------------


def exp4_block_diagonal():
    """
    The echo-patched gate gadget

        for( v <- q & u1 <- w & u2 <- w ){ q'!(*u1) | v!(*v) | u1!(*u1) }

    re-emits the input datum, so the outcome records it.  Outcomes are indexed
    by (i, j) rather than by j alone.  Reported: the 2x4 matrix of the site for
    a clause that would have been a Hadamard, its row supports, and the rank of
    the induced map on the input basis.
    """
    h = 1.0 / math.sqrt(2.0)
    clause = [[h, h], [h, -h]]           # what one wanted
    outcomes = [(i, j) for i in range(2) for j in range(2)]
    M = zeros(2, 4)
    for i in range(2):
        for j in range(2):
            M[i][outcomes.index((i, j))] = clause[i][j]
    supports = [set(k for k in range(4) if abs(M[i][k]) > 1e-12) for i in range(2)]
    isom = maxabs(sub(mul(M, dag(M)), eye(2)))
    return M, supports, supports[0] & supports[1], isom


# ---------------------------------------------------------------------------


if __name__ == "__main__":
    print("=== 1. Julia--Halmos dilation ===")
    n, wu, wok = exp1_dilation()
    print(f"  cases                        : {n}")
    print(f"  worst |U^dag U - 1|          : {wu:.3e}")
    print(f"  worst |ok-block of U  -  A|  : {wok:.3e}")

    print("\n=== 2. Heralds and the retry cost ===")
    print("   n   surviving norm   P[all heralds ok]   E[attempts]")
    for n, norm, p, att in exp2_retry_cost():
        print(f"  {n:2d}   {norm:.6f}         {p:.9f}        {att:.1f}")

    print("\n=== 3. An inert herald leaves the interferometer alone ===")
    print("   phase      bare      herald=ok    herald=candidate")
    for theta, bare, ok, idx in exp3_inert_herald():
        print(f"  {theta:7.4f}   {bare:.6f}   {ok:.6f}     {idx:.6f}")

    print("\n=== 4. The weakening-free gadget is block-diagonal ===")
    M, supports, overlap, isom = exp4_block_diagonal()
    for i, row in enumerate(M):
        print(f"  row {i}: " + "  ".join(f"{x.real:+.6f}" for x in row))
    print(f"  row supports                 : {supports[0]}, {supports[1]}")
    print(f"  overlap                      : {overlap if overlap else '(empty)'}")
    print(f"  |M M^dag - 1|                : {isom:.3e}")
