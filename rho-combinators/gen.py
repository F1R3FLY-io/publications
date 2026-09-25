import sys
def name(k):
    s = "@{Nil}"
    for _ in range(k): s = "@{" + s + "!(Nil)}"
    return s
def par(ps): return " | ".join(ps) if ps else "Nil"

def fan(n, d):
    # n independent relay chains of depth d: c_{i,0} -> c_{i,1} -> ... -> c_{i,d}
    ps=[]; k=0
    ids={}
    def c(i,j):
        if (i,j) not in ids:
            nonlocal_k[0]+=1; ids[(i,j)]=nonlocal_k[0]
        return name(ids[(i,j)])
    nonlocal_k=[0]
    for i in range(n):
        ps.append(f"{c(i,0)}!(Nil)")
        for j in range(d):
            ps.append(f"for(y <- {c(i,j)}){{ {c(i,j+1)}!(*y) }}")
    return par(ps)

def bcast(h):
    # binary broadcast tree of height h
    ps=["%s!(Nil)" % name(1)]
    for v in range(1, 2**h):
        ps.append(f"for(y <- {name(v)}){{ {name(2*v)}!(*y) | {name(2*v+1)}!(*y) }}")
    return par(ps)

def contend(c):
    ps=[]
    ch=name(0)
    for i in range(c):
        ps.append(f"{ch}!(Nil)")
        ps.append(f"for(y <- {ch}){{ {name(i+1)}!(*y) }}")
    return par(ps)

def ship(n):
    # higher-order: one piece of code with its own input is shipped and run n times,
    # each copy then serves its own client message
    b=name(1); a=name(2)
    body=" | ".join(["*z"]*n)
    R=f"for(y <- {a}){{ {name(3)}!(*y) }}"
    ps=[f"for(z <- {b}){{ {body} }}", f"{b}!({R})"]
    for i in range(n): ps.append(f"{a}!(Nil)")
    return par(ps)

def nested(n, d):
    # n clients each running a d-deep chain of nested inputs on a private channel
    ps=[]
    for i in range(n):
        ch=name(i+1)
        t=f"{name(n+1+i)}!(Nil)"
        for j in range(d): t=f"for(y{j} <- {ch}){{ {t} }}"
        ps.append(t)
        for j in range(d): ps.append(f"{ch}!(Nil)")
    return par(ps)

def serve(n):
    # replicated server via the recursion combinator: !for(y<-c){ out!(*y) } serving n clients
    x=name(1); c=name(2); out=name(3)
    P=f"for(w <- {c}){{ {out}!(*w) }}"
    D=f"for(y <- {x}){{ {x}!(*y) | *y }}"
    ps=[D, f"{x}!({D} | {P})"]
    for i in range(n): ps.append(f"{c}!(Nil)")
    return par(ps)

kind=sys.argv[1]; args=[int(a) for a in sys.argv[2:]]
print(globals()[kind](*args))
