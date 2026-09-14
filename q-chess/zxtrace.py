"""
zxtrace.py -- verification for "A ZX semantics for game traces".

Checks that the ZX denotation of the Italian transposition trace agrees with a
direct evaluation of the game's amplitude semantics, for every assignment of
signs to the two Q-move forks, and for the measurement variant.

Conventions.  Unnormalised ZX spiders throughout:

    green 0->1 phase p   =  |0> + e^{ip} |1>      (column [1, e^{ip}])
    green 1->0 phase 0   =  <0| + <1|             (row    [1, 1])
    red   1->0 phase 0   =  sqrt(2) <0|           (we use the bare <0| and
    red   1->0 phase pi  =  sqrt(2) <1|            note the factor in the text)

With this convention the amplitude of a final position is exactly the number of
histories reaching it, signed.  No normalisation is applied anywhere: the game
never samples, so norms carry no meaning.

Board register for this trace: four commitment bits

    n = Nf6 played,  b = Bc5 played,  d = d3 played,  c = c3 played

Branch registers: rB (Black's fork), rW (White's fork).
"""

import itertools
import numpy as np

# ----------------------------------------------------------------------------
# Side A: the game's amplitude semantics, evaluated directly.
#
# A position is a dict from board bitstring (n,b,d,c) to amplitude.  A Q-move
# offers a list of (effect, sign); an effect is feasible at s if its source
# pattern matches, modelled here by the precondition on the commitment bits.
# ----------------------------------------------------------------------------

NBDC = ("n", "b", "d", "c")


def commit(bit):
    """A classical move that sets `bit`, feasible only when `bit` is unset.

    `bit` unset is the source pattern matching: the piece is still on its
    origin square.  Once set, the source is empty and the move is infeasible.
    """
    i = NBDC.index(bit)

    def pre(s):
        return s[i] == 0

    def post(s):
        t = list(s)
        t[i] = 1
        return tuple(t)

    return pre, post


def qmove(state, branches):
    """Apply a Q-move.  `branches` is a list of (bit, sign)."""
    out = {}
    for s, amp in state.items():
        for bit, sign in branches:
            pre, post = commit(bit)
            if pre(s):
                t = post(s)
                out[t] = out.get(t, 0) + sign * amp
    return {s: a for s, a in out.items() if a != 0}


def measure(state, bit, answer):
    """Filter on an atom.  `answer` True keeps realisations with bit set."""
    i = NBDC.index(bit)
    return {s: a for s, a in state.items() if (s[i] == 1) == answer}


def game_trace(sB, sW):
    """The nine-ply trace.  sB, sW in {+1,-1} are the signs on the second
    branch of Black's and White's opening Q-move respectively."""
    state = {(0, 0, 0, 0): 1}                       # after ply 5 (classical)
    state = qmove(state, [("n", 1), ("b", sB)])     # ply 6  Black's fork
    state = qmove(state, [("d", 1), ("c", sW)])     # ply 7  White's fork
    state = qmove(state, [("n", 1), ("b", 1)])      # ply 8  closes Black
    state = qmove(state, [("d", 1), ("c", 1)])      # ply 9  closes White
    return state


def game_trace_measured(sB, sW, answer):
    """Variant: ply 8 is `measure (f6)!(B,Knight(R))` instead of a Q-move."""
    state = {(0, 0, 0, 0): 1}
    state = qmove(state, [("n", 1), ("b", sB)])
    state = qmove(state, [("d", 1), ("c", sW)])
    state = measure(state, "n", answer)             # ply 8  adversarial answer
    state = qmove(state, [("d", 1), ("c", 1)])      # ply 9  closes White
    return state


# ----------------------------------------------------------------------------
# Side B: the ZX denotation, evaluated as a tensor contraction.
#
# Six qubits, ordered (n, b, d, c, rB, rW).  Every turn map is a permutation of
# basis states, i.e. a reversible circuit; the branch wires are closed at the
# end with green 1->0 effects, which is the coherent sum.
# ----------------------------------------------------------------------------

NQ = 6
IDX = {"n": 0, "b": 1, "d": 2, "c": 3, "rB": 4, "rW": 5}


def basis_states():
    return list(itertools.product([0, 1], repeat=NQ))


def apply_perm(vec, f):
    """Apply a basis permutation f: bitstring -> bitstring."""
    out = np.zeros_like(vec)
    for k, s in enumerate(basis_states()):
        out[basis_states().index(f(s))] += vec[k]
    return out


def bit(s, name):
    return s[IDX[name]]


def setbit(s, name, v):
    t = list(s)
    t[IDX[name]] = v
    return tuple(t)


def X(name):
    return lambda s: setbit(s, name, 1 - bit(s, name))


def CNOT(ctrl, tgt):
    def f(s):
        if bit(s, ctrl) == 1:
            return setbit(s, tgt, 1 - bit(s, tgt))
        return s
    return f


def prepare_branch(vec, name, phase_sign):
    """Green 0->1 spider with phase 0 or pi on a wire currently in |0>.

    Unnormalised: |0> + sign|1>.  Implemented on an all-|0> register by
    writing the superposition in directly.
    """
    out = np.zeros_like(vec)
    for k, s in enumerate(basis_states()):
        if vec[k] == 0:
            continue
        assert bit(s, name) == 0, "branch wire must start at |0>"
        out[basis_states().index(s)] += vec[k]
        out[basis_states().index(setbit(s, name, 1))] += phase_sign * vec[k]
    return out


def close_branch(vec, name):
    """Green 1->0 spider, phase 0: the coherent sum <0| + <1| on that wire."""
    out = np.zeros_like(vec)
    for k, s in enumerate(basis_states()):
        if vec[k] == 0:
            continue
        out[basis_states().index(setbit(s, name, 0))] += vec[k]
    return out


def project(vec, name, value):
    """Red 1->0 spider, phase 0 or pi (up to the sqrt(2) of the convention):
    basis projection <value| on a board qubit."""
    out = np.zeros_like(vec)
    for k, s in enumerate(basis_states()):
        if vec[k] != 0 and bit(s, name) == value:
            out[k] += vec[k]
    return out


def zx_trace(sB, sW, measured=None):
    vec = np.zeros(2 ** NQ, dtype=complex)
    vec[basis_states().index((0,) * NQ)] = 1

    # ply 6: split on rB, then commit.  rB=0 -> Nf6, rB=1 -> Bc5.
    vec = prepare_branch(vec, "rB", sB)
    vec = apply_perm(vec, CNOT("rB", "b"))
    vec = apply_perm(vec, X("n"))
    vec = apply_perm(vec, CNOT("rB", "n"))

    # ply 7: split on rW, then commit.  rW=0 -> d3, rW=1 -> c3.
    vec = prepare_branch(vec, "rW", sW)
    vec = apply_perm(vec, CNOT("rW", "c"))
    vec = apply_perm(vec, X("d"))
    vec = apply_perm(vec, CNOT("rW", "d"))

    if measured is None:
        # ply 8: close Black's diamond -- uncomputes rB from n and b.
        vec = apply_perm(vec, CNOT("rB", "n"))
        vec = apply_perm(vec, CNOT("rB", "b"))
        vec = apply_perm(vec, X("b"))
    else:
        # ply 8 variant: project on the atom (f6)!(B,Knight(R)).
        vec = project(vec, "n", 1 if measured else 0)

    # ply 9: close White's diamond -- uncomputes rW from d and c.
    vec = apply_perm(vec, CNOT("rW", "d"))
    vec = apply_perm(vec, CNOT("rW", "c"))
    vec = apply_perm(vec, X("c"))

    # close the branch wires: coherent sum.
    vec = close_branch(vec, "rB")
    vec = close_branch(vec, "rW")

    out = {}
    for k, s in enumerate(basis_states()):
        if abs(vec[k]) > 1e-12:
            out[s[:4]] = vec[k]
    return out


# ----------------------------------------------------------------------------

def fmt(state):
    if not state:
        return "(empty support)"
    return ", ".join(
        "|%s%s%s%s> : %+g" % (s + (a.real if isinstance(a, complex) else a,))
        for s, a in sorted(state.items())
    )


def main():
    print("=" * 74)
    print("Unsigned and signed traces: game semantics vs ZX denotation")
    print("=" * 74)
    ok = True
    for sB, sW in itertools.product([1, -1], repeat=2):
        g = game_trace(sB, sW)
        z = {s: complex(a) for s, a in zx_trace(sB, sW).items()}
        g = {s: complex(a) for s, a in g.items()}
        agree = (set(g) == set(z)) and all(
            abs(g[s] - z[s]) < 1e-12 for s in g)
        ok = ok and agree
        print("\n  signs  Black %+d   White %+d" % (sB, sW))
        print("    game : %s" % fmt(g))
        print("    ZX   : %s" % fmt(z))
        print("    agree: %s" % agree)
        if g:
            amp = list(g.values())[0].real
            print("    scalar (1+sB)(1+sW) = %+g   amplitude = %+g"
                  % ((1 + sB) * (1 + sW), amp))

    print("\n" + "=" * 74)
    print("Measurement variant: ply 8 is  measure (f6)!(B,Knight(R))")
    print("=" * 74)
    for answer in (True, False):
        g = {s: complex(a) for s, a in
             game_trace_measured(1, 1, answer).items()}
        z = {s: complex(a) for s, a in
             zx_trace(1, 1, measured=answer).items()}
        agree = (set(g) == set(z)) and all(
            abs(g[s] - z[s]) < 1e-12 for s in g)
        ok = ok and agree
        print("\n  opponent answers %s" % ("YES" if answer else "NO"))
        print("    game : %s" % fmt(g))
        print("    ZX   : %s" % fmt(z))
        print("    agree: %s" % agree)

    print("\n" + "=" * 74)
    print("ALL CHECKS PASS" if ok else "MISMATCH")
    print("=" * 74)


if __name__ == "__main__":
    main()
