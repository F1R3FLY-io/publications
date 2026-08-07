//! The simulator's state (DR-W7).
//!
//! The interpreter's state lives in RSpace: hot store, cold store, history
//! trie, checkpoints. That machinery exists for persistence, replay and merging
//! under consensus. A simulator wants none of it and is harmed by it, because
//! the thing a study does constantly — fork the state, explore, discard — is
//! exactly what checkpointing makes expensive.
//!
//! So the simulator owns a plain immutable index: channels to pending data and
//! waiting continuations, shared structurally so [`Space::fork`] is O(1) and
//! the first mutation after a fork is the only copy. [`Space::fire`] is
//! non-destructive, which is what makes exhaustive graph construction possible
//! at all.
//!
//! The *shape* is RSpace's, because the shape is semantically load-bearing:
//! `persistent` and `peek` change multiplicity (F4).

use std::collections::BTreeMap;
use std::sync::Arc;

use crate::syntax::{Bind, Name, Term};

pub type ChanKey = String;
pub type DatumId = u64;
pub type ContId = u64;

/// A pending message.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Datum {
    pub id: DatumId,
    pub chan: ChanKey,
    /// The original name, kept because a name predicate must be able to see
    /// into its structure (`n ⊨ @φ`). Reconstructing a name from its key string
    /// loses exactly that, and silently: every key predicate then fails.
    pub name: Name,
    pub data: Vec<Term>,
    pub persistent: bool,
}

/// A waiting continuation. Multi-bind continuations are joins.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Cont {
    pub id: ContId,
    pub binds: Vec<Bind>,
    pub body: Term,
    pub persistent: bool,
    pub peek: bool,
}

impl Cont {
    /// The join set: the channels this continuation waits on.
    pub fn join_set(&self) -> Vec<ChanKey> {
        self.binds.iter().map(|b| b.source.key()).collect()
    }
}

/// The observable state of a space: how many messages sit on each channel, and
/// which continuations are installed. Two spaces with the same marking are the
/// same state.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct Marking {
    pub occupancy: BTreeMap<ChanKey, usize>,
    pub conts: Vec<String>,
}

impl Marking {
    /// A stable key, used to identify nodes of a transition graph.
    pub fn key(&self) -> String {
        let occ = self
            .occupancy
            .iter()
            .map(|(c, n)| format!("{c}={n}"))
            .collect::<Vec<_>>()
            .join(",");
        format!("[{occ}][{}]", self.conts.join(";"))
    }

    pub fn total_tokens(&self) -> usize {
        self.occupancy.values().sum()
    }
}

#[derive(Clone, Debug, Default)]
pub struct Space {
    data: Arc<BTreeMap<ChanKey, Vec<Datum>>>,
    conts: Arc<BTreeMap<ContId, Cont>>,
    by_chan: Arc<BTreeMap<ChanKey, Vec<ContId>>>,
    next_id: u64,
    /// Names introduced by `new`, alpha-renamed to fresh keys on install.
    pub fresh_counter: u64,
}

impl Space {
    pub fn new() -> Space {
        Space::default()
    }

    /// O(1). The copy happens on the next mutation, and only for the maps that
    /// mutation touches.
    pub fn fork(&self) -> Space {
        self.clone()
    }

    /// Build a space from a term: walk the top-level parallel decomposition and
    /// index sends as data, receives as continuations. This is the only piece
    /// of genuinely new semantics in the crate, which is why
    /// `law_marking_matches_term` covers it.
    pub fn install(term: &Term) -> Space {
        let mut s = Space::new();
        s.install_into(term);
        s
    }

    pub fn install_into(&mut self, term: &Term) {
        let renamed = self.alpha_rename_new(term);
        for c in renamed.components() {
            match c {
                Term::Send {
                    chan,
                    data,
                    persistent,
                } => {
                    let id = self.fresh();
                    let key = chan.key();
                    let d = Datum {
                        id,
                        chan: key.clone(),
                        name: chan,
                        data,
                        persistent,
                    };
                    Arc::make_mut(&mut self.data).entry(key).or_default().push(d);
                }
                Term::Receive {
                    binds,
                    body,
                    persistent,
                    peek,
                } => {
                    let id = self.fresh();
                    let cont = Cont {
                        id,
                        binds,
                        body: *body,
                        persistent,
                        peek,
                    };
                    for k in cont.join_set() {
                        Arc::make_mut(&mut self.by_chan)
                            .entry(k)
                            .or_default()
                            .push(id);
                    }
                    Arc::make_mut(&mut self.conts).insert(id, cont);
                }
                Term::Zero => {}
                // Ground values, drops and variables at top level are inert.
                _ => {}
            }
        }
    }

    /// Replace `new x in { P }` by `P` with `x` renamed to a fresh unforgeable
    /// name. Freshness is manufactured by a structural scheme from material the
    /// computation already holds, not drawn from a registry.
    fn alpha_rename_new(&mut self, term: &Term) -> Term {
        match term {
            Term::New { names, body } => {
                let mut env = BTreeMap::new();
                for n in names {
                    self.fresh_counter += 1;
                    let fresh = Name::quote(Term::Ground(crate::syntax::Ground::Str(format!(
                        "$new{}_{}",
                        self.fresh_counter, n
                    ))));
                    env.insert(n.clone(), Term::Drop(Box::new(fresh)));
                }
                let inner = body.substitute(&env);
                self.alpha_rename_new(&inner)
            }
            Term::Par(ts) => {
                Term::Par(ts.iter().map(|t| self.alpha_rename_new(t)).collect()).canonical()
            }
            other => other.clone(),
        }
    }

    fn fresh(&mut self) -> u64 {
        self.next_id += 1;
        self.next_id
    }

    pub fn data_on(&self, chan: &str) -> &[Datum] {
        self.data.get(chan).map(|v| v.as_slice()).unwrap_or(&[])
    }

    pub fn conts_on(&self, chan: &str) -> Vec<&Cont> {
        self.by_chan
            .get(chan)
            .map(|ids| ids.iter().filter_map(|i| self.conts.get(i)).collect())
            .unwrap_or_default()
    }

    pub fn all_conts(&self) -> impl Iterator<Item = &Cont> {
        self.conts.values()
    }

    pub fn channels(&self) -> Vec<ChanKey> {
        let mut ks: Vec<ChanKey> = self.data.keys().cloned().collect();
        for k in self.by_chan.keys() {
            if !ks.contains(k) {
                ks.push(k.clone());
            }
        }
        ks.sort();
        ks
    }

    pub fn occupancy(&self, chan: &str) -> usize {
        self.data.get(chan).map(|v| v.len()).unwrap_or(0)
    }

    pub fn marking(&self) -> Marking {
        let mut occupancy = BTreeMap::new();
        for (k, v) in self.data.iter() {
            if !v.is_empty() {
                occupancy.insert(k.clone(), v.len());
            }
        }
        let mut conts: Vec<String> = self
            .conts
            .values()
            .map(|c| {
                format!(
                    "{}{}",
                    if c.persistent { "!" } else { "" },
                    c.join_set().join("&")
                )
            })
            .collect();
        conts.sort();
        Marking { occupancy, conts }
    }

    /// Remove one datum by id (used by `fire`).
    fn remove_datum(&mut self, chan: &str, id: DatumId) {
        if let Some(v) = Arc::make_mut(&mut self.data).get_mut(chan) {
            v.retain(|d| d.id != id);
        }
    }

    fn remove_cont(&mut self, id: ContId) {
        Arc::make_mut(&mut self.conts).remove(&id);
        for (_, ids) in Arc::make_mut(&mut self.by_chan).iter_mut() {
            ids.retain(|i| *i != id);
        }
    }

    /// Apply a redex, returning a new space. The receiver is untouched — this
    /// is what `law_fire_is_pure` asserts and what makes branching cheap.
    pub fn fire(&self, r: &crate::redex::Redex) -> Space {
        let mut next = self.fork();
        let cont = match self.conts.get(&r.cont) {
            Some(c) => c.clone(),
            None => return next,
        };

        // A peeking receive leaves the data in place; otherwise a
        // non-persistent datum is consumed.
        if !cont.peek {
            for (chan, id, persistent) in &r.consumed {
                if !*persistent {
                    next.remove_datum(chan, *id);
                }
            }
        }
        // A persistent continuation survives; a linear one is consumed.
        if !cont.persistent {
            next.remove_cont(cont.id);
        }

        let body = cont.body.substitute(&r.bindings);
        next.install_into(&body);
        next
    }

    /// The space read back as a term, for evaluating structural formulae
    /// against the whole configuration.
    pub fn to_term(&self) -> Term {
        let mut parts: Vec<Term> = Vec::new();
        for (_k, ds) in self.data.iter() {
            for d in ds {
                parts.push(Term::Send {
                    chan: d.name.clone(),
                    data: d.data.clone(),
                    persistent: d.persistent,
                });
            }
        }
        for c in self.conts.values() {
            parts.push(Term::Receive {
                binds: c.binds.clone(),
                body: Box::new(c.body.clone()),
                persistent: c.persistent,
                peek: c.peek,
            });
        }
        Term::Par(parts).canonical()
    }
}
