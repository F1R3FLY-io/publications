from functools import lru_cache
from fractions import Fraction as F

def moves(pos):
    out=[]
    for i,h in enumerate(pos):
        for take in range(1,h+1):
            q=list(pos); q[i]=h-take
            out.append(tuple(sorted(q,reverse=True)))
    return out
def terminal(pos): return all(h==0 for h in pos)
def nimsum(pos):
    s=0
    for h in pos: s^=h
    return s
KAPPA={1:2,2:2,3:1}
def rung_cost(r): return sum(KAPPA[i] for i in range(1,r+1))
def rule1(pos,ms):
    if sum(1 for h in pos if h>0)==1: return [m for m in ms if terminal(m)]
    return []
def rule2(pos,ms):
    ne=[h for h in pos if h>0]
    if len(ne)==2 and ne[0]!=ne[1]:
        return [m for m in ms if len([h for h in m if h>0])==2 and sorted(h for h in m if h>0)[0]==sorted(h for h in m if h>0)[1]]
    return []
def rule3(pos,ms): return [m for m in ms if nimsum(m)==0]
RULES={1:rule1,2:rule2,3:rule3}
def choose(pos,r):
    ms=moves(pos)
    for i in range(1,r+1):
        c=RULES[i](pos,ms)
        if c: return c,len(ms)
    return ms,len(ms)

def ladder(start):
    def ev(r,first):
        @lru_cache(maxsize=None)
        def rec(pos,me):
            if terminal(pos): return (F(0),F(0)) if me else (F(1),F(0))
            if me:
                cand,n=choose(pos,r); cost=F(n*rung_cost(r)); p=F(0); c=F(0)
                for m in cand:
                    a,b=rec(m,False); p+=a; c+=b
                k=len(cand); return (p/k, cost+c/k)
            ms=moves(pos); p=F(0); c=F(0)
            for m in ms:
                a,b=rec(m,True); p+=a; c+=b
            k=len(ms); return (p/k, c/k)
        return rec(start,first)
    out=[]
    for r in range(4):
        a=ev(r,True); b=ev(r,False)
        out.append((float((a[0]+b[0])/2), float((a[1]+b[1])/2), float(a[0]), float(b[0])))
    return out

for start in [(3,4,5),(1,2,3),(1,2)]:
    L=ladder(start)
    print(f"Nim{start}  nim-sum={nimsum(start)}  branching={sum(start)}")
    pw=pc=None
    for r,(w,c,w1,w2) in enumerate(L):
        s = "" if pw is None else f"  dwin {w-pw:+.4f}  dcost {c-pc:+6.1f}  return {(w-pw)/(c-pc):+.5f}"
        print(f"   rung {r}: win {w:.4f} loss {1-w:.4f} cost {c:6.1f}  (1st {w1:.4f} / 2nd {w2:.4f}){s}")
        pw,pc=w,c
    print()

# ---- competition
R0,Q,P=12000,6,0.05
TTT={0:0.0,1:40.9,2:77.8,3:134.9,4:245.7,5:255.9,6:679.2}
N=R0/Q
cA=TTT[2]; DEEP=5; deficit=P*TTT[DEEP]-Q; net=Q-P*cA
def depth(cB):
    s=(1/cA)/(1/cA+1/cB); g=N*s; surp=g*net
    return s,g,surp,surp/deficit
print("A = noughts rung 2 (deepest self-funding), cost 77.8, net +%.2f/game"%net)
print("deep play = games of rung-5 noughts the terminal surplus funds (deficit %.3f/game)"%deficit)
print(f"{'c_B':>9} {'share_A':>8} {'games_A':>8} {'surplus':>8} {'deep':>7} {'retained':>9}")
base=N*net/deficit
print(f"{'alone':>9} {1.0:8.3f} {N:8.0f} {N*net:8.0f} {base:7.0f} {1.0:9.3f}")
for cB in [1,2,5,10,20,30,50,80,99.6,120,200,400]:
    s,g,surp,d=depth(cB)
    print(f"{cB:9.1f} {s:8.3f} {g:8.0f} {surp:8.0f} {d:7.0f} {d/base:9.3f}")
print()
for cB in [99.6]:
    s,g,surp,d=depth(cB)
    print(f"with B = chi_Nim(3,4,5) at {cB}: A retains {d/base:.3f} of its deep play ({d:.0f} vs {base:.0f} games); identical to its share {s:.3f}")
    print(f"B: share {1-s:.3f}, games {N*(1-s):.0f}, net/game {Q-P*cB:+.2f}, terminal surplus {N*(1-s)*(Q-P*cB):.0f}")
    print(f"B's best rung is self-funding ({cB:.1f} < {Q/P:.0f}) so B loses NO depth, only games")

print()
print("="*70)
print("WORKED EXAMPLE: A = noughts rung 2 (77.8), B = chi_Nim(1,2,3) (38.5)")
print("="*70)
cB=38.5
sA=(1/cA)/(1/cA+1/cB); sB=1-sA
gA=N*sA; gB=N*sB
surpA=gA*net; surpB=gB*(Q-P*cB)
print(f"shares: A {sA:.4f}  B {sB:.4f}   (identity: share_A = c_B/(c_A+c_B) = {cB/(cA+cB):.4f})")
print(f"A: {gA:.0f} games, net {net:+.3f}/game, terminal surplus {surpA:.0f}")
print(f"B: {gB:.0f} games, net {Q-P*cB:+.3f}/game, terminal surplus {surpB:.0f}")
print(f"A deep play alone {base:.0f} games -> composed {surpA/deficit:.0f} games  (retains {sA:.3f}, loses {100*(1-sA):.1f}%)")
print(f"B loses NO depth: its best rung costs {cB} < {Q/P:.0f}, self-funding")
print(f"at exhaustion B's stack is {surpB/surpA:.2f}x A's")
print(f"foraging inequality for A on B: sigma_B = {surpB:.0f} must exceed kappa_sense+kappa_assay+kappa_break")
for d in [1,2,3,4,5]:
    print(f"   at concealment depth delta={d}, kappa_sense=2^{d}={2**d} priced at {P}: "
          f"{P*(2**d):.2f} tokens; total outlay even at 100x that is {100*P*(2**d):.0f} << {surpB:.0f}")
    break
