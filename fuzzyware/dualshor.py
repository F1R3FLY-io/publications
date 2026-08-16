"""
dualshor.py --- Shor's algorithm in the dual-rail (occupation) encoding.

Two jobs, both from one gate list, so the listing printed in the note and the
numbers reported in the note cannot drift apart:

  1. realise every gate of the circuit THROUGH THE DUAL-RAIL SITE, by
     enumerating that site's candidates, and run the whole algorithm;
  2. emit the corresponding graded-rholang term.

The dual-rail site, with every bound name used exactly once and nothing
discarded:

    for( y o- qa & z o- qb & u1 o- w & u2 o- w  where psi ) { u1!(*y) | u2!(*z) }

The ancilla channel w carries the two OUTPUT RAIL NAMES; reflection makes a
received name usable as the channel of a send, so the candidate chooses where
each token lands.  A qubit is one token distributed over two rails: rail 0 holds
`one` and rail 1 holds `zero`, or the other way about.

Run:  python3 dualshor.py
"""

import math
import sys
from cmath import pi

import gradedsim as G
import dualrail as D

N, A = 15, 7
N_COUNT, N_WORK = 4, 4


# ---------------------------------------------------------------------------
# the gate list (identical in structure to gradedsim.shor + gradedsim.qft)
# ---------------------------------------------------------------------------


def gate_list():
    count = list(range(N_COUNT))
    gates = [("H", w) for w in count]
    gates.append(("MODEXP",))
    for j in range(N_COUNT):
        gates.append(("H", count[j]))
        for k in range(j + 1, N_COUNT):
            gates.append(("CP", count[k], count[j], 2 * pi / (2 ** (k - j + 1))))
    for i in range(N_COUNT // 2):
        gates.append(("SWAP", count[i], count[N_COUNT - 1 - i]))
    return gates


# ---------------------------------------------------------------------------
# realising a gate matrix through the dual-rail site
# ---------------------------------------------------------------------------


def realised_1q(target):
    """The 2x2 the dual-rail site actually produces for this clause."""
    return D.site_1q(D.clause_for(target))


def realised_2q(target):
    M, _, _ = D.site_2q(target)
    return M


def matrices_of(gates):
    """Every gate's matrix, as realised by the site rather than as written."""
    out = []
    for g in gates:
        if g[0] == "H":
            out.append(("1q", g[1], realised_1q(D.H)))
        elif g[0] == "CP":
            th = g[3]
            cp = [[1 + 0j, 0j, 0j, 0j], [0j, 1 + 0j, 0j, 0j],
                  [0j, 0j, 1 + 0j, 0j], [0j, 0j, 0j, __import__("cmath").exp(1j * th)]]
            out.append(("2q", (g[1], g[2]), realised_2q(cp)))
        elif g[0] == "SWAP":
            sw = [[1 + 0j, 0j, 0j, 0j], [0j, 0j, 1 + 0j, 0j],
                  [0j, 1 + 0j, 0j, 0j], [0j, 0j, 0j, 1 + 0j]]
            out.append(("2q", (g[1], g[2]), realised_2q(sw)))
        elif g[0] == "MODEXP":
            out.append(("modexp", None, None))
    return out


# ---------------------------------------------------------------------------
# running the circuit with the realised matrices
# ---------------------------------------------------------------------------


def apply_1q(state, wire, M):
    out = {}
    for cfg, amp in state.items():
        i = cfg[wire]
        for o in (0, 1):
            v = M[o][i]
            if v == 0:
                continue
            new = list(cfg)
            new[wire] = o
            k = tuple(new)
            out[k] = out.get(k, 0) + amp * v
    return {k: a for k, a in out.items() if abs(a) > 1e-15}


def apply_2q(state, wa, wb, M):
    out = {}
    for cfg, amp in state.items():
        i = 2 * cfg[wa] + cfg[wb]
        for o in range(4):
            v = M[o][i]
            if v == 0:
                continue
            new = list(cfg)
            new[wa], new[wb] = o // 2, o % 2
            k = tuple(new)
            out[k] = out.get(k, 0) + amp * v
    return {k: a for k, a in out.items() if abs(a) > 1e-15}


def run(gates):
    init = G.int_to_bits(0, N_COUNT) + G.int_to_bits(1, N_WORK)
    state = {init: 1.0 + 0j}

    def modexp(cfg):
        x = G.bits_to_int(cfg[:N_COUNT])
        return cfg[:N_COUNT] + G.int_to_bits(pow(A, x, N), N_WORK)

    for kind, where, M in matrices_of(gates):
        if kind == "1q":
            state = apply_1q(state, where, M)
        elif kind == "2q":
            state = apply_2q(state, where[0], where[1], M)
        else:
            state = G.deterministic(state, modexp)
    probs = G.marginal(state, list(range(N_COUNT)))
    return state, {G.bits_to_int(k): v for k, v in probs.items()}


# ---------------------------------------------------------------------------
# emission
# ---------------------------------------------------------------------------

PREAMBLE = r"""// Rails are bound by `new`, hence quantum, and every receipt on a quantum
// channel uses `o-`.  Installer and result channels are bound by `newc`, the
// derived binder for fresh CLASSICAL names, so their receipts carry no
// linearity obligation -- which is what lets a gadget contract mention its
// parameters only inside a clause.  A qubit is ONE token distributed over TWO
// rails.

contract gate1( r0In, r1In, r0Out, r1Out, m00, m01, m10, m11 ) = {
  new w in {
      w!(*r0Out) | w!(*r1Out)                 // the two output rail names
    | for( y o- *r0In & z o- *r1In & u1 o- w & u2 o- w
           where [ *y == one and *u1 == *r0Out ] * *m00
               + [ *y == one and *u1 == *r1Out ] * *m10
               + [ *y == zero and *u1 == *r0Out ] * *m01
               + [ *y == zero and *u1 == *r1Out ] * *m11 )
      { u1!(*y) | u2!(*z) }
  }
}

contract hadamard( r0In, r1In, r0Out, r1Out ) = {
  gate1!( *r0In, *r1In, *r0Out, *r1Out,
           0.7071067811865476,  0.7071067811865476,
           0.7071067811865476, -0.7071067811865476 )
}

// Two-qubit gates join four input rails with four ancilla patterns carrying the
// four output rail names.  The clause reads the tokens and the assignment, so
// it determines both the input and the output basis state; entries off the
// intended support return 0 and those candidates do not fire.

// `bit` reads a rail token as 0 or 1 and `rail` writes one; both are ordinary
// rholang and neither touches an amplitude.
contract bit( t ) = { if (*t == one) { 0 } else { 1 } }
contract rail( r0, r1, b ) = {
  if (*b == 0) { *r0!(one) | *r1!(zero) } else { *r0!(zero) | *r1!(one) }
}

contract gate2( a0In, a1In, b0In, b1In, a0Out, a1Out, b0Out, b1Out, m ) = {
  new w in {
      w!(*a0Out) | w!(*a1Out) | w!(*b0Out) | w!(*b1Out)
    | for( ya o- *a0In & za o- *a1In & yb o- *b0In & zb o- *b1In
         & u1 o- w & u2 o- w & u3 o- w & u4 o- w
           where entry( *m, basisIn(*ya, *yb), basisOut(*u1, *u3) ) )
      { u1!(*ya) | u2!(*za) | u3!(*yb) | u4!(*zb) }
  }
}
"""


def emit(gates):
    stage = {w: 0 for w in range(N_COUNT + N_WORK)}

    def rails(w):
        return f"q{w}_{stage[w]}_0", f"q{w}_{stage[w]}_1"

    def bump(w):
        stage[w] += 1
        return rails(w)

    lines, names = [], []

    def note(*ns):
        for n in ns:
            if n not in names:
                names.append(n)

    for w in range(N_COUNT + N_WORK):
        note(*rails(w))

    body = []
    body.append("    // 1. the input register, one token per rail pair")
    for w in range(N_COUNT):
        r0, r1 = rails(w)
        body.append(f"    {r0}!(one) | {r1}!(zero) |")
    for i, b in enumerate(G.int_to_bits(1, N_WORK)):
        r0, r1 = rails(N_COUNT + i)
        body.append(f"    {r0}!({'one' if b == 0 else 'zero'}) | "
                    f"{r1}!({'zero' if b == 0 else 'one'}) |")
    body.append("")

    for g in gates:
        if g[0] == "H":
            w = g[1]
            a0, a1 = rails(w)
            b0, b1 = bump(w)
            note(b0, b1)
            body.append(f"    hadamard!( *{a0}, *{a1}, *{b0}, *{b1} ) |")
        elif g[0] in ("CP", "SWAP"):
            wa, wb = g[1], g[2]
            a0, a1 = rails(wa)
            b0, b1 = rails(wb)
            c0, c1 = bump(wa)
            d0, d1 = bump(wb)
            note(c0, c1, d0, d1)
            if g[0] == "CP":
                k = int(round(2 * math.pi / g[3].real))
                m = f"cphase(2*pi/{k})"
            else:
                m = "swapMatrix"
            body.append(f"    gate2!( *{a0}, *{a1}, *{b0}, *{b1},")
            body.append(f"            *{c0}, *{c1}, *{d0}, *{d1}, {m} ) |")
        elif g[0] == "MODEXP":
            body.append("")
            body.append("    // 2. modular exponentiation: one candidate, clause value 1,")
            body.append("    //    so amplitudes pass through untouched.  Every rail is")
            body.append("    //    consumed and every token re-emitted: no discard.")
            src = [rails(w) for w in range(N_COUNT)]
            dst = [bump(w) for w in range(N_COUNT)]
            wsrc = [rails(N_COUNT + i) for i in range(N_WORK)]
            wdst = [bump(N_COUNT + i) for i in range(N_WORK)]
            for r in dst + wdst:
                note(*r)
            pats = [f"x{i} o- *{src[i][0]} & x{i}' o- *{src[i][1]}"
                    for i in range(N_COUNT)]
            wpats = [f"y{i} o- *{wsrc[i][0]} & y{i}' o- *{wsrc[i][1]}"
                     for i in range(N_WORK)]
            body.append(f"    for( {pats[0]}")
            for q in pats[1:] + wpats[:-1]:
                body.append(f"       & {q}")
            body.append(f"       & {wpats[-1]} ) {{")
            weights = " + ".join(
                (f"bit(*x{i})" if i == N_COUNT - 1
                 else f"{2 ** (N_COUNT - 1 - i)} * bit(*x{i})") for i in range(N_COUNT))
            body.append(f"      newc r in {{")
            body.append(f"        modexp!( *r, {A},")
            body.append(f"                 {weights}, {N} ) |")
            body.append("        for( m <- r ) {")
            outs = " | ".join(
                f"{dst[i][0]}!(*x{i}) | {dst[i][1]}!(*x{i}')" for i in range(2))
            body.append(f"          {outs} |")
            outs = " | ".join(
                f"{dst[i][0]}!(*x{i}) | {dst[i][1]}!(*x{i}')" for i in range(2, 4))
            body.append(f"          {outs} |")
            for i in range(N_WORK):
                sh = 2 ** (N_WORK - 1 - i)
                b = f"(*m / {sh}) % 2" if sh > 1 else "*m % 2"
                body.append(f"          rail( {wdst[i][0]}, {wdst[i][1]}, {b} ) |")
            body.append("          done!(Nil)")
            body.append("        }")
            body.append("      }")
            body.append("    } |")
            body.append("")
            body.append("    // 3. the transform on the counting register")

    body.append("")
    body.append("    // 4. read the counting register")
    reads = " & ".join(
        f"c{i} o- *{rails(i)[0]}" for i in range(N_COUNT))
    body.append(f"    for( {reads} ) {{")
    val = " + ".join(
        (f"bit(*c{i})" if i == N_COUNT - 1 else f"{2 ** (N_COUNT - 1 - i)} * bit(*c{i})")
        for i in range(N_COUNT))
    body.append(f"      out!( {val} )")
    body.append("    }")

    names.append("__SPLIT__")
    names.append("out")
    q_names = names[:names.index("__SPLIT__")]
    c_names = names[names.index("__SPLIT__") + 1:]

    def binder(kw, ns, tail):
        out, cur = [], kw + " "
        for i, nm in enumerate(ns):
            piece = nm + ("," if i < len(ns) - 1 else tail)
            if len(cur) + len(piece) > 72:
                out.append(cur.rstrip())
                cur = "    "
            cur += piece + " "
        out.append(cur.rstrip())
        return out

    decl = binder("new", q_names, " in") + binder("newc", c_names, " in {")

    lines = decl + [""] + body + ["}"]
    return PREAMBLE + "\n" + "\n".join(lines) + "\n"


# ---------------------------------------------------------------------------


def check():
    gates = gate_list()
    ours, ourprobs = run(gates)
    theirs, theirprobs = G.shor(N, A, N_COUNT, N_WORK)
    worst = max(abs(ourprobs.get(k, 0.0) - theirprobs.get(k, 0.0))
                for k in set(ourprobs) | set(theirprobs))

    # how far each realised matrix is from the one that was asked for
    wm = 0.0
    for kind, _, M in matrices_of(gates):
        if kind == "1q":
            wm = max(wm, D.maxdiff(M, D.H))
    n1 = sum(1 for k, _, _ in matrices_of(gates) if k == "1q")
    n2 = sum(1 for k, _, _ in matrices_of(gates) if k == "2q")

    print(f"  dual-rail sites: {n1} one-qubit, {n2} two-qubit")
    print(f"  worst |realised gate - intended gate| : {wm:.3e}")
    print("  counting register, from the dual-rail realisation:")
    for k in sorted(ourprobs):
        if ourprobs[k] > 1e-12:
            print(f"    c = {k:2d}   {ourprobs[k]:.9f}")
    print(f"    total                    {sum(ourprobs.values()):.12f}")
    print(f"  worst deviation from the payload-encoded run : {worst:.3e}")


if __name__ == "__main__":
    if "--check" not in sys.argv:
        print(emit(gate_list()))
        print()
    check()
