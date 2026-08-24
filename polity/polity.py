#!/usr/bin/env python3
"""
polity.py --- computations for "Votes and Capital".

Every number quoted in the research note is printed by this script.
Standard library only.  Deterministic: seed 20260823.

Sections
  M1  Foster / effective resistance on the political conversion graph
  M2  The cycle yield, homogeneous reservation prices
  M3  The cycle yield, heterogeneous reservation prices (order statistics)
  M4  Inequality sweep: where the yield crosses one
  M5  Districting and packing
  M6  The purchasable set as a function of the purse
  M7  The ratchet: stock against flow
  M8  Sale against delegation
"""

import math
import random
from statistics import mean, median

SEED = 20260823

# ---------------------------------------------------------------- utilities

def solve(A, b):
    """Gaussian elimination with partial pivoting.  A is a list of rows."""
    n = len(A)
    M = [row[:] + [b[i]] for i, row in enumerate(A)]
    for c in range(n):
        p = max(range(c, n), key=lambda r: abs(M[r][c]))
        if abs(M[p][c]) < 1e-14:
            raise ValueError("singular")
        M[c], M[p] = M[p], M[c]
        pc = M[c][c]
        for r in range(n):
            if r == c:
                continue
            f = M[r][c] / pc
            if f == 0.0:
                continue
            for k in range(c, n + 1):
                M[r][k] -= f * M[c][k]
    return [M[i][n] / M[i][i] for i in range(n)]


def effective_resistances(nv, edges):
    """R_eff for every edge of a connected unit-weight graph on nv vertices."""
    L = [[0.0] * nv for _ in range(nv)]
    for (u, v) in edges:
        L[u][u] += 1.0
        L[v][v] += 1.0
        L[u][v] -= 1.0
        L[v][u] -= 1.0
    # ground the last vertex
    red = [[L[i][j] for j in range(nv - 1)] for i in range(nv - 1)]
    out = []
    for (u, v) in edges:
        b = [0.0] * (nv - 1)
        if u < nv - 1:
            b[u] += 1.0
        if v < nv - 1:
            b[v] -= 1.0
        x = solve([r[:] for r in red], b)
        xu = x[u] if u < nv - 1 else 0.0
        xv = x[v] if v < nv - 1 else 0.0
        out.append(xu - xv)
    return out


def political_graph(m):
    """Vertices: 0 = C, 1 = V, 2..m+1 = offices O_1..O_m.
    Edges: the bribe edge C--V, then for each office V--O_i and O_i--C."""
    edges = [(0, 1)]
    labels = ["b (C->V)"]
    for i in range(m):
        edges.append((1, 2 + i))
        labels.append("e_%d (V->O_%d)" % (i + 1, i + 1))
        edges.append((2 + i, 0))
        labels.append("r_%d (O_%d->C)" % (i + 1, i + 1))
    return 2 + m, edges, labels


def lognormal_fixed_mean(rng, n, target_mean, sigma):
    mu = math.log(target_mean) - sigma * sigma / 2.0
    return [math.exp(rng.gauss(mu, sigma)) for _ in range(n)]


def cheapest_k(xs, k):
    return sum(sorted(xs)[:k]) if k > 0 else 0.0


# ------------------------------------------------------------- parameters

N = 101                 # electorate
Q = 51                  # simple majority quorum
R = 1200.0              # rent released to the office holder per epoch
T = 4                   # tenure, in epochs
RT = R * T              # total rent of one tenure
MEAN_THETA = 100.0      # mean reservation price of a vote
SIGMA = 0.8             # dispersion of reservation prices

lines = []


def say(s=""):
    print(s)
    lines.append(s)


say("=" * 72)
say("polity.py --- Votes and Capital.  seed = %d" % SEED)
say("=" * 72)
say("electorate n = %d, quorum q = %d, rent R = %.1f/epoch, tenure T = %d"
    % (N, Q, R, T))
say("total rent of a tenure  RT = %.1f" % RT)
say("break-even vote price   theta* = RT/q = %.4f" % (RT / Q))
say()

# --------------------------------------------------------------------- M1
say("-" * 72)
say("M1  Foster's theorem on the political conversion graph")
say("-" * 72)
say("m = number of independently rent-bearing offices")
say("%3s %6s %8s %12s %14s %14s" %
    ("m", "|V|", "|E|", "beta_1", "sum(1-Reff)", "1-Reff(bribe)"))
foster_rows = []
for m in range(1, 7):
    nv, edges, labels = political_graph(m)
    reff = effective_resistances(nv, edges)
    total = sum(1.0 - r for r in reff)
    beta1 = len(edges) - nv + 1
    say("%3d %6d %8d %12d %14.6f %14.6f"
        % (m, nv, len(edges), beta1, total, 1.0 - reff[0]))
    foster_rows.append((m, beta1, total, 1.0 - reff[0]))
say()
say("closed form for the bribe edge: 1 - Reff(b) = m/(m+2)")
for m in range(1, 7):
    say("   m = %d :  m/(m+2) = %.6f" % (m, m / (m + 2.0)))
say()
say("per-edge detection strengths at m = 3:")
nv, edges, labels = political_graph(3)
reff = effective_resistances(nv, edges)
for lab, r in zip(labels, reff):
    say("   %-16s  Reff = %.6f   1-Reff = %.6f" % (lab, r, 1.0 - r))
say()

# --------------------------------------------------------------------- M2
say("-" * 72)
say("M2  The cycle yield, homogeneous reservation prices")
say("-" * 72)
K_hom = Q * MEAN_THETA
Y_hom = RT / K_hom
say("every citizen prices the franchise at theta = %.2f" % MEAN_THETA)
say("capture cost K = q*theta            = %.2f" % K_hom)
say("cycle yield  Y = RT/K               = %.6f" % Y_hom)
say("verdict: %s" % ("captured (Y > 1)" if Y_hom > 1 else "rule of law holds (Y <= 1)"))
say()

# --------------------------------------------------------------------- M3
say("-" * 72)
say("M3  The cycle yield, heterogeneous reservation prices")
say("-" * 72)
rng = random.Random(SEED)
theta = lognormal_fixed_mean(rng, N, MEAN_THETA, SIGMA)
theta_sorted = sorted(theta)
K_het = cheapest_k(theta, Q)
Y_het = RT / K_het
say("lognormal reservation prices, mean fixed at %.2f, sigma = %.2f"
    % (MEAN_THETA, SIGMA))
say("   realised mean   = %.4f" % mean(theta))
say("   realised median = %.4f" % median(theta))
say("   cheapest        = %.4f" % theta_sorted[0])
say("   dearest         = %.4f" % theta_sorted[-1])
say("capture cost K = sum of the %d cheapest = %.4f" % (Q, K_het))
say("   naive estimate q * mean            = %.4f" % (Q * mean(theta)))
say("   ratio K / (q*mean)                 = %.6f" % (K_het / (Q * mean(theta))))
say("cycle yield Y = RT/K                  = %.6f" % Y_het)
say("verdict: %s" % ("captured (Y > 1)" if Y_het > 1 else "rule of law holds"))
say()
say("the dearest %d citizens are irrelevant to the price of the polity:" % (N - Q))
say("   their aggregate reservation price = %.4f" % (sum(theta) - K_het))
say("   a %.1f%% share of civic virtue that buys no protection at all"
    % (100.0 * (sum(theta) - K_het) / sum(theta)))
say()

# --------------------------------------------------------------------- M4
say("-" * 72)
say("M4  Inequality of virtue: where the yield crosses one")
say("-" * 72)
say("%8s %14s %14s %12s" % ("sigma", "K", "Y = RT/K", "captured"))
sweep = []
rz = random.Random(SEED + 5)
zs = [rz.gauss(0.0, 1.0) for _ in range(N)]      # common random numbers
for i in range(0, 17):
    s = i * 0.1
    mu = math.log(MEAN_THETA) - s * s / 2.0
    th = [math.exp(mu + s * z) for z in zs]
    K = cheapest_k(th, Q)
    Y = RT / K
    sweep.append((s, K, Y))
    say("%8.2f %14.4f %14.6f %12s" % (s, K, Y, "yes" if Y > 1 else "no"))
cross = None
for a, b in zip(sweep, sweep[1:]):
    if a[2] <= 1.0 < b[2]:
        # linear interpolation in sigma
        cross = a[0] + (1.0 - a[2]) * (b[0] - a[0]) / (b[2] - a[2])
        break
say()
say("mean reservation price is CONSTANT across the whole sweep (%.2f)" % MEAN_THETA)
say("crossing Y = 1 at sigma ~ %.4f" % cross)
say()

# --------------------------------------------------------------------- M5
say("-" * 72)
say("M5  Districting")
say("-" * 72)
DISTRICT_SIZES = [10, 10] + [9] * 9
assert sum(DISTRICT_SIZES) == N
DIST_WIN = 6            # districts needed


def district_capture(assignment):
    """assignment: list of lists of reservation prices, one per district."""
    costs = []
    voters = []
    for d in assignment:
        need = len(d) // 2 + 1
        costs.append(cheapest_k(d, need))
        voters.append(need)
    order = sorted(range(len(costs)), key=lambda i: costs[i])[:DIST_WIN]
    return sum(costs[i] for i in order), sum(voters[i] for i in order), order


# random assignment
r3 = random.Random(SEED + 991)
perm = theta[:]
r3.shuffle(perm)
rand_assign, p = [], 0
for s in DISTRICT_SIZES:
    rand_assign.append(perm[p:p + s])
    p += s
K_rand, v_rand, _ = district_capture(rand_assign)

# packed (gerrymandered) assignment: six districts of nine, each holding a
# bare majority of the cheapest citizens and padded with the dearest, who are
# never bought.  The remaining districts absorb everyone else.
asc = sorted(theta)
v_pack = DIST_WIN * (9 // 2 + 1)
K_pack = cheapest_k(theta, v_pack)

say("11 districts, sizes %s" % DISTRICT_SIZES)
say("a majority of districts = %d of 11" % DIST_WIN)
say()
say("%-24s %14s %10s %14s" % ("rule", "K", "voters", "Y = RT/K"))
say("%-24s %14.4f %10d %14.6f" % ("popular majority", K_het, Q, RT / K_het))
say("%-24s %14.4f %10d %14.6f"
    % ("districts, random draw", K_rand, v_rand, RT / K_rand))
say("%-24s %14.4f %10d %14.6f"
    % ("districts, packed", K_pack, v_pack, RT / K_pack))
say()
say("packing buys the office for %.2f%% of the popular-majority price"
    % (100.0 * K_pack / K_het))
say("and needs the consent of %d of %d citizens (%.2f%%)"
    % (v_pack, N, 100.0 * v_pack / N))
say()

# --------------------------------------------------------------------- M6
say("-" * 72)
say("M6  The purchasable set as a function of the purse")
say("-" * 72)
NOUT = 2000
r4 = random.Random(SEED + 77)
costs = []
free = 0
for k in range(NOUT):
    p_support = r4.uniform(0.15, 0.85)
    supporters = [i for i in range(N) if r4.random() < p_support]
    if len(supporters) >= Q:
        free += 1
        costs.append(0.0)
        continue
    opp = [theta[i] for i in range(N) if i not in set(supporters)]
    costs.append(cheapest_k(opp, Q - len(supporters)))
costs.sort()
say("%d collectively enabled outcomes, each with its own alignment" % NOUT)
say("outcomes already commanding a natural majority: %d (%.2f%%)"
    % (free, 100.0 * free / NOUT))
say()
say("%14s %16s" % ("purse W", "purchasable %"))
for W in [0, 250, 500, 1000, 1500, 2000, 2500, 3000, 4000, 5000, 6000]:
    frac = sum(1 for c in costs if c <= W) / NOUT
    say("%14d %15.2f%%" % (W, 100.0 * frac))
for target in [0.50, 0.90, 0.99, 1.00]:
    idx = min(int(math.ceil(target * NOUT)) - 1, NOUT - 1)
    say("purse buying %5.0f%% of all outcomes: %12.4f" % (100 * target, costs[idx]))
say()
say("HOMOGENEOUS CONTROL: identical reservation prices, identical alignment")
say("   every outcome costs the same; the purchasable set is empty below")
say("   K = %.2f and the whole of it above.  No intermediate purse exists."
    % K_hom)
say()

# --------------------------------------------------------------------- M7
say("-" * 72)
say("M7  The ratchet: a stock against a flow")
say("-" * 72)
r5 = random.Random(SEED + 4242)
citizen_capital = lognormal_fixed_mean(r5, N, 500.0, 1.0)
W_electorate = sum(citizen_capital)
say("electorate's combined capital stock W_e = %.2f" % W_electorate)
say("capture cost per tenure            K   = %.4f" % K_het)
say("rent per tenure                    RT  = %.2f" % RT)
say("net per tenure                RT - K   = %.4f" % (RT - K_het))
W = K_het
say()
say("%8s %16s %16s" % ("tenure", "incumbent stock", "vs electorate"))
tenure_over = None
t = 0
while True:
    if t <= 12 or (tenure_over is not None and t <= tenure_over):
        say("%8d %16.2f %15.2f%%" % (t, W, 100.0 * W / W_electorate))
    if tenure_over is None and W > W_electorate:
        tenure_over = t
        break
    W = W - K_het + RT
    t += 1
say()
say("incumbent's stock exceeds the whole electorate's at tenure %d (= %d epochs)"
    % (tenure_over, tenure_over * T))
say("the electorate's franchise over those %d epochs: %d votes, none storable"
    % (tenure_over * T, tenure_over * T * N))
say()

# --------------------------------------------------------------------- M8
say("-" * 72)
say("M8  Sale against delegation")
say("-" * 72)
say("V is re-issued at each registration, so a SALE must be repurchased")
say("every epoch; a DELEGATION is a standing capability drawn from the")
say("unrestricted zone and is bought once per tenure.")
say()
say("%-28s %16s %14s" % ("arrangement", "cost per tenure", "Y = RT/cost"))
say("%-28s %16.4f %14.6f" % ("sale, T epochs", T * K_het, RT / (T * K_het)))
say("%-28s %16.4f %14.6f" % ("delegation, once", K_het, RT / K_het))
say()
say("ratio of yields  Y_del / Y_sale = %.6f  (= T = %d)"
    % ((RT / K_het) / (RT / (T * K_het)), T))
say()
say("with the sale arrangement the polity is %s"
    % ("captured" if RT / (T * K_het) > 1 else "SAFE: Y = %.6f <= 1"
       % (RT / (T * K_het))))
say("with the delegation arrangement it is %s"
    % ("CAPTURED: Y = %.6f > 1" % (RT / K_het) if RT / K_het > 1 else "safe"))
say()
say("tenure at which co-authentication flips the verdict: T > %.4f"
    % (K_het / R * 1.0))
say("   (sale is unprofitable while RT < T*K, i.e. always, since R < K;")
say("    delegation is profitable as soon as RT > K, i.e. T > K/R = %.4f)"
    % (K_het / R))
say()
# --------------------------------------------------------------------- M9
say("-" * 72)
say("M9  Dividing the office: shared against disjoint electorates")
say("-" * 72)
say("total rent held fixed at RT = %.1f, split equally among m offices" % RT)
say()
say("%4s %10s %16s %14s %16s %14s"
    % ("m", "beta_1", "K (shared)", "Y (shared)", "K (disjoint)", "Y (disjoint)"))
for m in [1, 3, 5, 11]:
    # shared electorate: one purchased quorum votes in every election
    K_shared = K_het
    Y_shared = RT / K_shared
    # disjoint electorates: office j is elected by its own block
    blocks, p = [], 0
    base, extra = divmod(N, m)
    for j in range(m):
        s_j = base + (1 if j < extra else 0)
        blocks.append(sorted(theta)[p:p + s_j] if False else theta[p:p + s_j])
        p += s_j
    per = []
    for blk in blocks:
        need = len(blk) // 2 + 1
        per.append(cheapest_k(blk, need))
    K_disj = sum(per)
    # the cheapest single office, which is the relevant profitability test
    Y_disj = (RT / m) / min(per)
    say("%4d %10d %16.4f %14.6f %16.4f %14.6f"
        % (m, m, K_shared, Y_shared, K_disj, Y_disj))
say()
say("shared electorate: dividing the rent does not divide the yield,")
say("because one purchased quorum wins every election at once.")
say("disjoint electorates: the cheapest single office carries the yield;")
say("the figure quoted is (RT/m) divided by the cheapest block's price.")
say()

# ------------------------------------------------------------- derived ratios
say("-" * 72)
say("Derived ratios quoted in the note")
say("-" * 72)
W_all = costs[-1]
say("purse buying every outcome        W* = %.4f" % W_all)
say("   as a share of a bare quorum  W*/K = %.6f" % (W_all / K_het))
say("   as a share of one tenure's rent   = %.6f" % (W_all / RT))
say("K (heterogeneous) / K (homogeneous)  = %.6f" % (K_het / K_hom))
say("packed districting / popular majority = %.6f" % (K_pack / K_het))
say()

say("=" * 72)
say("end of computations")
say("=" * 72)

with open("polity-output.txt", "w") as f:
    f.write("\n".join(lines) + "\n")
