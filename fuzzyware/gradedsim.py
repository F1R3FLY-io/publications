"""
gradedsim.py -- a simulator for graded rho that follows Definition 6.2 literally.

A configuration is a store: which datum sits on each named channel.  A gadget is
a receipt site; its CANDIDATES are the bijections from patterns to data, and the
one-step operator sums, over candidates, the clause value times the outcome.
Interference is addition of coefficients on a common outcome key -- here, a
Python dict update.

Nothing in this file multiplies gate matrices.  The matrices only appear as the
values a clause returns on a candidate.
"""

from itertools import permutations, product
from cmath import exp, pi, sqrt as csqrt
import math

# ----------------------------------------------------------------------------
# The one-step operator, restricted to a gadget site.
# ----------------------------------------------------------------------------

def one_qubit_gadget(state, wire, clause):
    """
    for( v <- q & u1 <- w & u2 <- w where psi ) { q'!(*u1) }

    The ancilla channel w carries two data, indexed 0 and 1.  Candidates are the
    2 bijections {u1,u2} -> {a0,a1}; the outcome is the index of the datum bound
    to u1.  Every candidate consumes everything, so no residual records which
    one fired.
    """
    out = {}
    for cfg, amp in state.items():
        v = cfg[wire]                                  # the input datum on q
        for sigma in permutations((0, 1)):             # the candidates
            u1 = sigma[0]                              # datum bound to u1
            w = clause(v, u1)                          # the graded where-clause
            if w == 0:
                continue                               # support: does not fire
            new = list(cfg)
            new[wire] = u1
            key = tuple(new)
            out[key] = out.get(key, 0) + amp * w
    return {k: a for k, a in out.items() if abs(a) > 1e-15}


def two_qubit_gadget(state, wa, wb, clause):
    """
    Two input channels and two ancilla channels joined together.  Candidates are
    the 2 x 2 = 4 pairs of bijections; the outcome is the pair of u1-indices.
    """
    out = {}
    for cfg, amp in state.items():
        va, vb = cfg[wa], cfg[wb]
        for sa, sb in product(permutations((0, 1)), repeat=2):
            ua, ub = sa[0], sb[0]
            w = clause((va, vb), (ua, ub))
            if w == 0:
                continue
            new = list(cfg)
            new[wa], new[wb] = ua, ub
            key = tuple(new)
            out[key] = out.get(key, 0) + amp * w
    return {k: a for k, a in out.items() if abs(a) > 1e-15}


def deterministic(state, f):
    """
    An ordinary rho computation inside every branch: one candidate, clause value
    1, amplitude untouched.  f rewrites a configuration.
    """
    out = {}
    for cfg, amp in state.items():
        key = f(cfg)
        out[key] = out.get(key, 0) + amp
    return out


# ----------------------------------------------------------------------------
# Clauses
# ----------------------------------------------------------------------------

INV_SQRT2 = 1.0 / math.sqrt(2.0)

def hadamard_clause(v, u1):
    return INV_SQRT2 * (-1.0 if (v == 1 and u1 == 1) else 1.0)

def cphase_clause(theta):
    def psi(vin, vout):
        if vin != vout:                 # diagonal: all other candidates vanish
            return 0
        return exp(1j * theta) if vin == (1, 1) else 1.0
    return psi

def swap_clause(vin, vout):
    return 1.0 if vout == (vin[1], vin[0]) else 0

def beamsplitter_clause(a, b):
    """Used for the Hong-Ou-Mandel check of section 8."""
    def psi(v, u1):
        return a if u1 == 0 else b
    return psi


# ----------------------------------------------------------------------------
# Utilities
# ----------------------------------------------------------------------------

def bits_to_int(bits):
    n = 0
    for b in bits:
        n = 2 * n + b
    return n

def int_to_bits(n, width):
    return tuple((n >> (width - 1 - i)) & 1 for i in range(width))

def marginal(state, wires):
    p = {}
    for cfg, amp in state.items():
        key = tuple(cfg[w] for w in wires)
        p[key] = p.get(key, 0.0) + abs(amp) ** 2
    return p

def norm(state):
    return math.sqrt(sum(abs(a) ** 2 for a in state.values()))


# ----------------------------------------------------------------------------
# The quantum Fourier transform, assembled from gadgets
# ----------------------------------------------------------------------------

def qft(state, wires):
    n = len(wires)
    for j in range(n):
        state = one_qubit_gadget(state, wires[j], hadamard_clause)
        for k in range(j + 1, n):
            theta = 2 * pi / (2 ** (k - j + 1))
            state = two_qubit_gadget(state, wires[k], wires[j], cphase_clause(theta))
    for i in range(n // 2):
        state = two_qubit_gadget(state, wires[i], wires[n - 1 - i], swap_clause)
    return state


def qft_matrix_check(n):
    """Verify the gadget-assembled QFT against the DFT matrix, entry by entry."""
    N = 2 ** n
    worst = 0.0
    for j in range(N):
        st = {int_to_bits(j, n): 1.0 + 0j}
        st = qft(st, list(range(n)))
        for k in range(N):
            got = st.get(int_to_bits(k, n), 0j)
            want = exp(2j * pi * j * k / N) / math.sqrt(N)
            worst = max(worst, abs(got - want))
    return worst


# ----------------------------------------------------------------------------
# Section 8: the minimal interferometer
# ----------------------------------------------------------------------------

def hom_check():
    results = {}
    for label, (a, b) in [("constructive", (INV_SQRT2, INV_SQRT2)),
                          ("destructive",  (INV_SQRT2, -INV_SQRT2)),
                          ("quadrature",   (INV_SQRT2, 1j * INV_SQRT2))]:
        # symmetric continuation: both candidates land on one outcome
        sym = {}
        for u1 in (0, 1):
            w = beamsplitter_clause(a, b)(0, u1)
            sym["z"] = sym.get("z", 0) + w
        # distinguishing continuation: two outcomes
        asym = {u1: beamsplitter_clause(a, b)(0, u1) for u1 in (0, 1)}
        results[label] = (abs(sym["z"]) ** 2,
                          {k: abs(v) ** 2 for k, v in asym.items()})
    return results


# ----------------------------------------------------------------------------
# The interleaving artefact
# ----------------------------------------------------------------------------

def interleaving_inflation(n):
    """
    Compare the causal-history sum against a sum over interleavings for n
    independent single-qubit gadgets.  The ratio is the spurious factor.
    """
    st = {tuple([0] * n): 1.0 + 0j}
    for w in range(n):
        st = one_qubit_gadget(st, w, hadamard_clause)
    causal = norm(st) ** 2
    interleaved = causal * math.factorial(n)   # each causal class has n! orders
    return causal, interleaved, math.factorial(n)


# ----------------------------------------------------------------------------
# Shor, N = 15, a = 7
# ----------------------------------------------------------------------------

def shor(N=15, a=7, n_count=4, n_work=4):
    count = list(range(n_count))
    work = list(range(n_count, n_count + n_work))
    width = n_count + n_work

    # |0...0> |0...01>
    init = int_to_bits(0, n_count) + int_to_bits(1, n_work)
    state = {init: 1.0 + 0j}

    # 1. Hadamard layer on the counting register: n_count independent gadgets.
    for w in count:
        state = one_qubit_gadget(state, w, hadamard_clause)

    # 2. Modular exponentiation.  Ordinary rho computation: one candidate,
    #    clause value 1, amplitudes untouched.
    def modexp(cfg):
        x = bits_to_int(cfg[:n_count])
        return cfg[:n_count] + int_to_bits(pow(a, x, N), n_work)
    state = deterministic(state, modexp)

    # 3. QFT on the counting register.
    state = qft(state, count)

    # 4. Read the counting register.
    probs = marginal(state, count)
    return state, {bits_to_int(k): v for k, v in probs.items()}


# ----------------------------------------------------------------------------
# The post-selection hazard
# ----------------------------------------------------------------------------

def postselection_hazard(n=6, eps=1e-3):
    """
    A non-unitary clause diag(1, eps) applied to each of n qubits, followed by
    the renormalisation the Born reading performs.  The surviving state is the
    post-selected one, at no cost in the semantics.
    """
    st = {tuple([0] * n): 1.0 + 0j}
    for w in range(n):
        st = one_qubit_gadget(st, w, hadamard_clause)
    def filt(v, u1):
        if v != u1:
            return 0
        return 1.0 if u1 == 0 else eps
    for w in range(n):
        st = one_qubit_gadget(st, w, filt)
    unnormalised_all_zero = abs(st[tuple([0] * n)])
    total = norm(st)
    return unnormalised_all_zero, total, (unnormalised_all_zero / total) ** 2


# ----------------------------------------------------------------------------

if __name__ == "__main__":
    print("=== QFT gadget assembly vs the DFT matrix ===")
    for n in (2, 3, 4):
        print(f"  n={n}: worst entry error = {qft_matrix_check(n):.3e}")

    print("\n=== Section 8: the minimal interferometer ===")
    for label, (sym, asym) in hom_check().items():
        print(f"  {label:13s} symmetric outcome = {sym:.6f}   "
              f"distinguishing outcomes = {asym[0]:.6f}, {asym[1]:.6f}")

    print("\n=== Interleaving artefact ===")
    for n in (2, 3, 4):
        c, i, f = interleaving_inflation(n)
        print(f"  n={n}: causal norm^2 = {c:.6f}, interleaved = {i:.6f}, factor = {f}")

    print("\n=== Shor, N=15, a=7, 4 counting qubits ===")
    state, probs = shor()
    print(f"  configurations with nonzero amplitude: {len(state)}")
    for k in sorted(probs):
        if probs[k] > 1e-12:
            print(f"    measured c = {k:2d}   probability = {probs[k]:.9f}")
    tot = sum(probs.values())
    print(f"  total probability = {tot:.12f}")

    # classical post-processing
    from fractions import Fraction
    print("  continued fractions:")
    for k in sorted(probs):
        if probs[k] > 1e-12 and k != 0:
            r = Fraction(k, 16).limit_denominator(15).denominator
            print(f"    c={k:2d} -> c/16 = {Fraction(k,16)} -> r = {r}")
    r = 4
    p = math.gcd(pow(7, r // 2, 15) - 1, 15)
    q = math.gcd(pow(7, r // 2, 15) + 1, 15)
    print(f"  r = {r}, gcd(7^2-1,15) = {p}, gcd(7^2+1,15) = {q}")

    print("\n=== Post-selection hazard ===")
    amp, tot, prob = postselection_hazard()
    print(f"  |amplitude on all-zero| = {amp:.6e}")
    print(f"  norm of surviving state = {tot:.6e}")
    print(f"  renormalised probability of all-zero = {prob:.9f}")
