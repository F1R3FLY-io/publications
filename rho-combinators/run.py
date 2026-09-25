import subprocess, re, sys, json, statistics as st
B="/home/claude/Saph1r3/f1r3comb/target/release/f1r3comb"
def measure(kind, args, seeds=(0,1,2), maxsteps=100000, extra=()):
    src=subprocess.run(["python3","gen.py",kind,*map(str,args)],capture_output=True,text=True).stdout
    open("cur.rho","w").write(src)
    comp=subprocess.run([B,"compile","cur.rho","-o","cur.comb"],capture_output=True,text=True)
    if comp.returncode: return {"kind":kind,"args":args,"err":comp.stderr[-300:]}
    res=[]
    for s in seeds:
        out=subprocess.run([B,"run","cur.comb","--trace","--seed",str(s),"--max-steps",str(maxsteps),*extra],capture_output=True,text=True).stdout
        steps=[]; 
        for line in out.splitlines():
            m=re.match(r"\s*(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(.*?)\s+([0-9a-f]{16})$",line)
            if m:
                rules=dict((k,int(v)) for k,v in re.findall(r"(\w+):(\d+)",m.group(5)))
                steps.append((int(m.group(2)),int(m.group(3)),int(m.group(4)),rules))
        stop=re.search(r"stop (\w+)",out).group(1)
        fired=sum(f for _,f,_,_ in steps); enum=sum(e for e,_,_,_ in steps)
        inst=sum(r.get("inst",0) for *_,r in steps)
        erect_steps=sum(1 for *_,r in steps if set(r)=={"inst"})
        res.append(dict(steps=len(steps),fired=fired,enum=enum,maxw=max(f for _,f,_,_ in steps),
                        inst=inst,stop=stop,names=sum(n for _,_,n,_ in steps)))
    r0=res[0]
    return {"kind":kind,"args":args,"src_bytes":len(src),"atoms":None,
            "steps":st.mean(r["steps"] for r in res),"fired":st.mean(r["fired"] for r in res),
            "maxw":max(r["maxw"] for r in res),"enum_per_fired":st.mean(r["enum"]/r["fired"] for r in res),
            "inst":r0["inst"],"names":r0["names"],"stop":r0["stop"],
            "steps_range":(min(r["steps"] for r in res),max(r["steps"] for r in res))}
if __name__=="__main__":
    print(measure(sys.argv[1],[int(a) for a in sys.argv[2:]]))
