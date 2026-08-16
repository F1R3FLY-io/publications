"""
shorterm.py --- emits the full graded-rholang term for Shor's algorithm at
N = 15, a = 7, and checks that the emitted gate list is the one the simulator
of gradedsim.py executes.

The point of this file is that the listing printed in the note is not
transcribed by hand.  It is generated from the same gate list that produces the
counting-register distribution, so the code in the paper and the numbers in the
paper cannot drift apart.

Run:  python3 shorterm.py            # prints the term
      python3 shorterm.py --check    # prints the verification only
"""

import math
import sys
from cmath import pi

import gradedsim as G

N, A = 15, 7
N_COUNT, N_WORK = 4, 4


# ---------------------------------------------------------------------------
# the gate list --- structurally identical to gradedsim.shor + gradedsim.qft
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
# executing the gate list, using gradedsim's gadgets
# ---------------------------------------------------------------------------


def run(gates):
    width = N_COUNT + N_WORK
    init = G.int_to_bits(0, N_COUNT) + G.int_to_bits(1, N_WORK)
    state = {init: 1.0 + 0j}

    def modexp(cfg):
        x = G.bits_to_int(cfg[:N_COUNT])
        return cfg[:N_COUNT] + G.int_to_bits(pow(A, x, N), N_WORK)

    for g in gates:
        if g[0] == "H":
            state = G.one_qubit_gadget(state, g[1], G.hadamard_clause)
        elif g[0] == "CP":
            state = G.two_qubit_gadget(state, g[1], g[2], G.cphase_clause(g[3]))
        elif g[0] == "SWAP":
            state = G.two_qubit_gadget(state, g[1], g[2], G.swap_clause)
        elif g[0] == "MODEXP":
            state = G.deterministic(state, modexp)
        else:
            raise ValueError(g[0])
    probs = G.marginal(state, list(range(N_COUNT)))
    return state, {G.bits_to_int(k): v for k, v in probs.items()}


# ---------------------------------------------------------------------------
# the rholang modular exponentiation, checked against pow()
# ---------------------------------------------------------------------------


def modexp_reference(base, expn, modulus):
    """Right-to-left repeated squaring: the algorithm the emitted contract runs."""
    acc, b, e = 1, base % modulus, expn
    while e > 0:
        if e % 2 == 1:
            acc = (acc * b) % modulus
        b = (b * b) % modulus
        e = e // 2
    return acc


# ---------------------------------------------------------------------------
# emission
# ---------------------------------------------------------------------------

PREAMBLE = r"""// ---------------------------------------------------------------------------
// Gadgets.  Each `contract` is a persistent receipt on its own installer
// channel; the receipt it installs on a wire is linear, so the persistence
// never touches a race.  Clause scalars are constants of V = C, and the crisp
// guards are written [ ... ].
// ---------------------------------------------------------------------------

contract gate1( qIn, qOut, m00, m01, m10, m11 ) = {
  new w in {
      w!(0) | w!(1)
    | for( v <- *qIn & u1 <- w & u2 <- w
           where [ *v == 0 and *u1 == 0 ] * *m00
               + [ *v == 0 and *u1 == 1 ] * *m01
               + [ *v == 1 and *u1 == 0 ] * *m10
               + [ *v == 1 and *u1 == 1 ] * *m11 )
      { *qOut!(*u1) }
  }
}

contract hadamard( qIn, qOut ) = {
  gate1!( *qIn, *qOut,  0.7071067811865476,  0.7071067811865476,
                        0.7071067811865476, -0.7071067811865476 )
}

contract cphase( qaIn, qbIn, qaOut, qbOut, theta ) = {
  new wa, wb in {
      wa!(0) | wa!(1) | wb!(0) | wb!(1)
    | for( va <- *qaIn & vb <- *qbIn
         & ua1 <- wa & ua2 <- wa & ub1 <- wb & ub2 <- wb
           where [ *ua1 == *va and *ub1 == *vb ]
               * ( [ *va == 1 and *vb == 1 ] * cis(*theta)
                 + [ not (*va == 1 and *vb == 1) ] * 1 ) )
      { *qaOut!(*ua1) | *qbOut!(*ub1) }
  }
}

contract swap( qaIn, qbIn, qaOut, qbOut ) = {
  new wa, wb in {
      wa!(0) | wa!(1) | wb!(0) | wb!(1)
    | for( va <- *qaIn & vb <- *qbIn
         & ua1 <- wa & ua2 <- wa & ub1 <- wb & ub2 <- wb
           where [ *ua1 == *vb and *ub1 == *va ] * 1 )
      { *qaOut!(*ua1) | *qbOut!(*ub1) }
  }
}

// ---------------------------------------------------------------------------
// Modular exponentiation.  One candidate at every step, clause value 1, so by
// Proposition 9.8 the amplitudes are untouched: ordinary rholang, running
// inside every branch of the superposition.
// ---------------------------------------------------------------------------

contract modexp( ret, base, expn, modulus ) = {
  new loop in {
      contract loop( acc, b, e ) = {
        if ( *e == 0 ) { *ret!(*acc) }
        else {
          if ( *e % 2 == 1 ) {
            loop!( (*acc * *b) % *modulus, (*b * *b) % *modulus, *e / 2 )
          } else {
            loop!( *acc, (*b * *b) % *modulus, *e / 2 )
          }
        }
      }
    | loop!( 1, *base % *modulus, *expn )
  }
}
"""


def emit(gates):
    stage = {w: 0 for w in range(N_COUNT + N_WORK)}

    def chan(w):
        return f"q{w}_{stage[w]}"

    def bump(w):
        stage[w] += 1
        return chan(w)

    lines = []
    h = "1.0 / sqrt(2.0)"

    lines.append("// --- the instance: N = 15, a = 7, four counting and four work wires ---")
    lines.append("")
    lines.append("NEWLINE_PLACEHOLDER")
    lines.append("")
    # initial state |0000>|0001>
    inits = []
    for w in range(N_COUNT):
        inits.append(f"{chan(w)}!(0)")
    for i, b in enumerate(G.int_to_bits(1, N_WORK)):
        inits.append(f"{chan(N_COUNT + i)}!({b})")
    lines.append("    // 1. the input register")
    for s in inits:
        lines.append(f"    {s} |")
    lines.append("")

    layer = 0
    for g in gates:
        if g[0] == "H":
            w = g[1]
            a, b = chan(w), bump(w)
            lines.append(f"    hadamard!( *{a}, *{b} ) |")
        elif g[0] == "CP":
            wa, wb, theta = g[1], g[2], g[3]
            a0, b0 = chan(wa), chan(wb)
            a1, b1 = bump(wa), bump(wb)
            k = int(round(2 * math.pi / theta.real))
            lines.append(
                f"    cphase!( *{a0}, *{b0}, *{a1}, *{b1}, 2*pi/{k} ) |"
            )
        elif g[0] == "SWAP":
            wa, wb = g[1], g[2]
            a0, b0 = chan(wa), chan(wb)
            a1, b1 = bump(wa), bump(wb)
            lines.append(f"    swap!( *{a0}, *{b0}, *{a1}, *{b1} ) |")
        elif g[0] == "MODEXP":
            lines.append("")
            lines.append("    // 2. modular exponentiation, entangling work with counting")
            src = [chan(w) for w in range(N_COUNT)]
            dst = [bump(w) for w in range(N_COUNT)]
            wsrc = [chan(N_COUNT + i) for i in range(N_WORK)]
            wdst = [bump(N_COUNT + i) for i in range(N_WORK)]
            pats = " & ".join(f"x{i} <- *{src[i]}" for i in range(N_COUNT))
            wpats = " & ".join(f"y{i} <- *{wsrc[i]}" for i in range(N_WORK))
            lines.append("    //    y0..y3 are discarded: the work register is overwritten")
            lines.append(f"    for( {pats}")
            lines.append(f"       & {wpats} ) {{")
            lines.append("      new r in {")
            weights = " + ".join(
                (f"*x{i}" if i == N_COUNT - 1 else f"{2 ** (N_COUNT - 1 - i)} * *x{i}")
                for i in range(N_COUNT)
            )
            lines.append(f"        modexp!( *r, {A}, {weights}, {N} ) |")
            lines.append("        for( m <- r ) {")
            emits = " | ".join(
                f"{dst[i]}!(*x{i})" for i in range(N_COUNT)
            )
            lines.append(f"          {emits} |")
            wemits = [
                (f"{wdst[i]}!( *m % 2 )" if i == N_WORK - 1
                 else f"{wdst[i]}!( (*m / {2 ** (N_WORK - 1 - i)}) % 2 )")
                for i in range(N_WORK)
            ]
            lines.append("          " + " | ".join(wemits[:2]) + " |")
            lines.append("          " + " | ".join(wemits[2:]))
            lines.append("        }")
            lines.append("      }")
            lines.append("    } |")
            lines.append("")
            lines.append("    // 3. the transform on the counting register")
        layer += 1

    lines.append("")
    lines.append("    // 4. read the counting register")
    reads = " & ".join(f"c{i} <- *{chan(i)}" for i in range(N_COUNT))
    lines.append(f"    for( {reads} ) {{")
    val = " + ".join(
        (f"*c{i}" if i == N_COUNT - 1 else f"{2 ** (N_COUNT - 1 - i)} * *c{i}")
        for i in range(N_COUNT)
    )
    lines.append(f"      done!( {val} )")
    lines.append("    }")
    lines.append("}")

    # the new-binder, emitted once every wire stage is known
    names = [f"q{w}_{t}" for w in range(N_COUNT + N_WORK)
             for t in range(stage[w] + 1)] + ["done"]
    decl, cur = [], "new "
    for i, nm in enumerate(names):
        piece = nm + ("," if i < len(names) - 1 else " in {")
        if len(cur) + len(piece) > 74:
            decl.append(cur.rstrip())
            cur = "    "
        cur += piece + " "
    decl.append(cur.rstrip())
    body = "\n".join(lines).replace("NEWLINE_PLACEHOLDER", "\n".join(decl))
    return PREAMBLE + "\n" + body + "\n"


# ---------------------------------------------------------------------------


def check():
    gates = gate_list()
    ours, ourprobs = run(gates)
    theirs, theirprobs = G.shor(N, A, N_COUNT, N_WORK)

    counts = {}
    for g in gates:
        counts[g[0]] = counts.get(g[0], 0) + 1

    worst = 0.0
    for k in set(ourprobs) | set(theirprobs):
        worst = max(worst, abs(ourprobs.get(k, 0.0) - theirprobs.get(k, 0.0)))

    modexp_ok = all(
        modexp_reference(A, x, N) == pow(A, x, N) for x in range(2 ** N_COUNT)
    )

    print("gate list emitted by this file:")
    for k in ("H", "CP", "SWAP", "MODEXP"):
        print(f"  {k:7s} {counts.get(k, 0)}")
    print(f"  one-qubit gadget firings : {counts.get('H', 0)}")
    print(f"  two-qubit gadget firings : {counts.get('CP', 0) + counts.get('SWAP', 0)}")
    print()
    print("counting register, from the emitted gate list:")
    for k in sorted(ourprobs):
        if ourprobs[k] > 1e-12:
            print(f"  c = {k:2d}   {ourprobs[k]:.9f}")
    print(f"  total                     {sum(ourprobs.values()):.12f}")
    print(f"  worst deviation from gradedsim.shor : {worst:.3e}")
    print(f"  emitted modexp == pow(a,x,N) for all x : {modexp_ok}")


if __name__ == "__main__":
    if "--check" in sys.argv:
        check()
    else:
        print(emit(gate_list()))
        print()
        check()
