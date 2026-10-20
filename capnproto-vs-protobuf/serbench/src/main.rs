//! prost vs Cap'n Proto on f1r3node-rust message shapes.
//!
//! The prost types are generated from the real CasperMessage.proto and
//! RhoTypes.proto (dev branch) with the same prost-build settings that matter
//! for codec cost.  The Cap'n Proto schema (schema/f1r3.capnp) mirrors the
//! fields the generators below populate.  The prost structs double as the
//! stand-in "domain" representation for both codecs.

#![allow(clippy::all)]
use std::hint::black_box;
use std::time::{Duration, Instant};

use bytes::Bytes;
use prost::Message;

pub mod casper {
    include!(concat!(env!("OUT_DIR"), "/casper.rs"));
}
pub mod rhoapi {
    include!(concat!(env!("OUT_DIR"), "/rhoapi.rs"));
}
pub mod f1r3_capnp {
    include!(concat!(env!("OUT_DIR"), "/f1r3_capnp.rs"));
}

use casper::*;
use f1r3_capnp as cp;
use rhoapi::*;

// ---------------------------------------------------------------- data gen

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn bytes(&mut self, n: usize) -> Bytes {
        let mut v = Vec::with_capacity(n);
        while v.len() < n {
            v.extend_from_slice(&self.next().to_le_bytes());
        }
        v.truncate(n);
        Bytes::from(v)
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

const TERM: &str = r#"new rl(`rho:registry:lookup`), RevVaultCh, vaultCh, toVaultCh, deployerId(`rho:rchain:deployerId`), revVaultKeyCh, resultCh in {
  rl!(`rho:rchain:revVault`, *RevVaultCh) |
  for (@(_, RevVault) <- RevVaultCh) {
    @RevVault!("findOrCreate", "1111pdZDG4MZ3eBfaipaD22VXdbFY6PNCBeAhgsQyzbp6Ftus7ZdU", *vaultCh) |
    @RevVault!("findOrCreate", "11112ZM9yrfaTrzCCbKjPbxBncjNCkMFsPqtcLFvhBf4Kqx6rpir2w", *toVaultCh) |
    @RevVault!("deployerAuthKey", *deployerId, *revVaultKeyCh) |
    for (@(true, vault) <- vaultCh; key <- revVaultKeyCh; @(true, toVault) <- toVaultCh) {
      @vault!("transfer", "11112ZM9yrfaTrzCCbKjPbxBncjNCkMFsPqtcLFvhBf4Kqx6rpir2w", 100000, *key, *resultCh) |
      for (@result <- resultCh) {
        match result {
          (true, _) => { Nil }
          (false, reason) => { @"transferFailed"!(reason) }
        }
      }
    }
  }
}"#;

fn gen_event(r: &mut Rng) -> EventProto {
    use event_proto::EventInstance as E;
    let prod = |r: &mut Rng| ProduceEventProto {
        channels_hash: r.bytes(32),
        hash: r.bytes(32),
        persistent: r.below(4) == 0,
        times_repeated: 0,
        is_deterministic: true,
        output_value: vec![],
        failed: false,
    };
    let cons = |r: &mut Rng| ConsumeEventProto {
        channels_hashes: (0..1 + r.below(3)).map(|_| r.bytes(32)).collect(),
        hash: r.bytes(32),
        persistent: r.below(4) == 0,
    };
    let inst = match r.below(3) {
        0 => E::Produce(prod(r)),
        1 => E::Consume(cons(r)),
        _ => E::Comm(CommEventProto {
            consume: Some(cons(r)),
            produces: (0..1 + r.below(2)).map(|_| prod(r)).collect(),
            peeks: vec![],
        }),
    };
    EventProto { event_instance: Some(inst) }
}

fn gen_block(n_deploys: usize, events_per_deploy: usize, seed: u64) -> BlockMessageProto {
    let mut r = Rng(seed);
    let validators: Vec<Bytes> = (0..5).map(|_| r.bytes(65)).collect();
    let deploys = (0..n_deploys)
        .map(|_| ProcessedDeployProto {
            deploy: Some(DeployDataProto {
                deployer: r.bytes(65),
                term: TERM.to_string(),
                timestamp: 1_760_000_000_000 + r.below(1 << 20) as i64,
                sig: r.bytes(71),
                sig_algorithm: "secp256k1".into(),
                phlo_price: 1,
                phlo_limit: 500_000,
                valid_after_block_number: r.below(10_000) as i64,
                shard_id: "root".into(),
                language: "rholang".into(),
                expiration_timestamp: 0,
                parameters: vec![],
            }),
            cost: Some(PCost { cost: r.below(400_000) }),
            deploy_log: (0..events_per_deploy).map(|_| gen_event(&mut r)).collect(),
            errored: false,
            system_deploy_error: String::new(),
        })
        .collect();
    BlockMessageProto {
        block_hash: r.bytes(32),
        header: Some(HeaderProto {
            parents_hash_list: (0..3).map(|_| r.bytes(32)).collect(),
            timestamp: 1_760_000_000_000,
            version: 1,
            extra_bytes: Bytes::new(),
        }),
        body: Some(BodyProto {
            state: Some(RChainStateProto {
                pre_state_hash: r.bytes(32),
                post_state_hash: r.bytes(32),
                bonds: validators
                    .iter()
                    .map(|v| BondProto { validator: v.clone(), stake: 1_000_000 })
                    .collect(),
                block_number: 123_456,
            }),
            deploys,
            system_deploys: vec![],
            extra_bytes: Bytes::new(),
            rejected_deploys: vec![],
            applied_from_scope: vec![],
            merge_base: Bytes::new(),
        }),
        justifications: validators
            .iter()
            .map(|v| JustificationProto { validator: v.clone(), latest_block_hash: r.bytes(32) })
            .collect(),
        sender: validators[0].clone(),
        seq_num: 42,
        sig: r.bytes(71),
        sig_algorithm: "secp256k1".into(),
        shard_id: "root".into(),
        extra_bytes: Bytes::new(),
    }
}

fn gpriv(r: &mut Rng) -> GUnforgeable {
    GUnforgeable {
        unf_instance: Some(g_unforgeable::UnfInstance::GPrivateBody(GPrivate {
            id: r.bytes(32).to_vec(),
        })),
    }
}

fn leaf_par(r: &mut Rng) -> Par {
    use expr::ExprInstance as X;
    let e = match r.below(4) {
        0 => X::GInt(r.next() as i64 >> 20),
        1 => X::GString(format!("str-{}", r.below(1_000_000))),
        2 => X::GBool(r.below(2) == 0),
        _ => X::EVarBody(EVar {
            v: Some(Var { var_instance: Some(var::VarInstance::BoundVar(r.below(8) as i32)) }),
        }),
    };
    Par { exprs: vec![Expr { expr_instance: Some(e) }], ..Default::default() }
}

fn gen_par(r: &mut Rng, depth: u32, fanout: u64) -> Par {
    if depth == 0 {
        return leaf_par(r);
    }
    let mut p = Par::default();
    for _ in 0..1 + r.below(fanout) {
        match r.below(5) {
            0 | 1 => p.sends.push(Send {
                chan: Some(Par { unforgeables: vec![gpriv(r)], ..Default::default() }),
                data: (0..1 + r.below(3)).map(|_| gen_par(r, depth - 1, fanout)).collect(),
                persistent: false,
                ..Default::default()
            }),
            2 => p.receives.push(Receive {
                binds: vec![ReceiveBind {
                    patterns: vec![Par {
                        exprs: vec![Expr {
                            expr_instance: Some(expr::ExprInstance::EVarBody(EVar {
                                v: Some(Var { var_instance: Some(var::VarInstance::FreeVar(0)) }),
                            })),
                        }],
                        ..Default::default()
                    }],
                    source: Some(Par { unforgeables: vec![gpriv(r)], ..Default::default() }),
                    remainder: None,
                    free_count: 1,
                }],
                body: Some(gen_par(r, depth - 1, fanout)),
                bind_count: 1,
                ..Default::default()
            }),
            3 => p.news.push(New {
                bind_count: 2,
                p: Some(gen_par(r, depth - 1, fanout)),
                uri: vec!["rho:registry:lookup".into()],
                ..Default::default()
            }),
            _ => p.exprs.push(Expr {
                expr_instance: Some(expr::ExprInstance::ETupleBody(ETuple {
                    ps: (0..2).map(|_| gen_par(r, depth - 1, fanout)).collect(),
                    ..Default::default()
                })),
            }),
        }
    }
    p
}

// ------------------------------------------------------- capnp: build

fn cp_prod(mut b: cp::produce_event::Builder, p: &ProduceEventProto) {
    b.set_channels_hash(&p.channels_hash);
    b.set_hash(&p.hash);
    b.set_persistent(p.persistent);
    b.set_times_repeated(p.times_repeated);
    b.set_is_deterministic(p.is_deterministic);
    let mut ov = b.reborrow().init_output_value(p.output_value.len() as u32);
    for (i, v) in p.output_value.iter().enumerate() {
        ov.set(i as u32, v);
    }
    b.set_failed(p.failed);
}
fn cp_cons(mut b: cp::consume_event::Builder, c: &ConsumeEventProto) {
    let mut l = b.reborrow().init_channels_hashes(c.channels_hashes.len() as u32);
    for (i, h) in c.channels_hashes.iter().enumerate() {
        l.set(i as u32, h);
    }
    b.set_hash(&c.hash);
    b.set_persistent(c.persistent);
}

fn cp_header(mut h: cp::header::Builder, hp: &HeaderProto) {
    let mut ph = h.reborrow().init_parents_hash_list(hp.parents_hash_list.len() as u32);
    for (i, x) in hp.parents_hash_list.iter().enumerate() {
        ph.set(i as u32, x);
    }
    h.set_timestamp(hp.timestamp);
    h.set_version(hp.version);
    h.set_extra_bytes(&hp.extra_bytes);
}

fn cp_body(mut b: cp::body::Builder, bp: &BodyProto) {
    {
        let sp = bp.state.as_ref().unwrap();
        let mut s = b.reborrow().init_state();
        s.set_pre_state_hash(&sp.pre_state_hash);
        s.set_post_state_hash(&sp.post_state_hash);
        s.set_block_number(sp.block_number);
        let mut bl = s.init_bonds(sp.bonds.len() as u32);
        for (i, bo) in sp.bonds.iter().enumerate() {
            let mut x = bl.reborrow().get(i as u32);
            x.set_validator(&bo.validator);
            x.set_stake(bo.stake);
        }
    }
    let mut dl = b.reborrow().init_deploys(bp.deploys.len() as u32);
    for (i, pd) in bp.deploys.iter().enumerate() {
        let mut d = dl.reborrow().get(i as u32);
        {
            let dd = pd.deploy.as_ref().unwrap();
            let mut x = d.reborrow().init_deploy();
            x.set_deployer(&dd.deployer);
            x.set_term(dd.term.as_str());
            x.set_timestamp(dd.timestamp);
            x.set_sig(&dd.sig);
            x.set_sig_algorithm(dd.sig_algorithm.as_str());
            x.set_phlo_price(dd.phlo_price);
            x.set_phlo_limit(dd.phlo_limit);
            x.set_valid_after_block_number(dd.valid_after_block_number);
            x.set_shard_id(dd.shard_id.as_str());
            x.set_language(dd.language.as_str());
            x.set_expiration_timestamp(dd.expiration_timestamp);
        }
        d.set_cost(pd.cost.as_ref().map(|c| c.cost).unwrap_or(0));
        d.set_errored(pd.errored);
        d.set_system_deploy_error(pd.system_deploy_error.as_str());
        let mut el = d.init_deploy_log(pd.deploy_log.len() as u32);
        for (j, ev) in pd.deploy_log.iter().enumerate() {
            let e = el.reborrow().get(j as u32);
            match ev.event_instance.as_ref().unwrap() {
                event_proto::EventInstance::Produce(p) => cp_prod(e.init_produce(), p),
                event_proto::EventInstance::Consume(c) => cp_cons(e.init_consume(), c),
                event_proto::EventInstance::Comm(c) => {
                    let mut cb = e.init_comm();
                    cp_cons(cb.reborrow().init_consume(), c.consume.as_ref().unwrap());
                    let mut pl = cb.reborrow().init_produces(c.produces.len() as u32);
                    for (k, p) in c.produces.iter().enumerate() {
                        cp_prod(pl.reborrow().get(k as u32), p);
                    }
                    let mut pk = cb.init_peeks(c.peeks.len() as u32);
                    for (k, p) in c.peeks.iter().enumerate() {
                        pk.set(k as u32, p.channel_index);
                    }
                }
            }
        }
    }
    b.set_extra_bytes(&bp.extra_bytes);
    b.set_merge_base(&bp.merge_base);
}

fn cp_build_block(m: &BlockMessageProto) -> capnp::message::Builder<capnp::message::HeapAllocator> {
    let mut msg = capnp::message::Builder::new_default();
    {
        let mut b = msg.init_root::<cp::block_message::Builder>();
        b.set_block_hash(&m.block_hash);
        cp_header(b.reborrow().init_header(), m.header.as_ref().unwrap());
        cp_body(b.reborrow().init_body(), m.body.as_ref().unwrap());
        let mut jl = b.reborrow().init_justifications(m.justifications.len() as u32);
        for (i, j) in m.justifications.iter().enumerate() {
            let mut x = jl.reborrow().get(i as u32);
            x.set_validator(&j.validator);
            x.set_latest_block_hash(&j.latest_block_hash);
        }
        b.set_sender(&m.sender);
        b.set_seq_num(m.seq_num);
        b.set_sig(&m.sig);
        b.set_sig_algorithm(m.sig_algorithm.as_str());
        b.set_shard_id(m.shard_id.as_str());
        b.set_extra_bytes(&m.extra_bytes);
    }
    msg
}

fn cp_var(mut b: cp::var::Builder, v: &Var) {
    match v.var_instance.unwrap() {
        var::VarInstance::BoundVar(i) => b.set_bound_var(i),
        var::VarInstance::FreeVar(i) => b.set_free_var(i),
        var::VarInstance::Wildcard(_) => b.set_wildcard(()),
    }
}

fn cp_par(mut b: cp::par::Builder, p: &Par) {
    let mut sl = b.reborrow().init_sends(p.sends.len() as u32);
    for (i, s) in p.sends.iter().enumerate() {
        let mut x = sl.reborrow().get(i as u32);
        cp_par(x.reborrow().init_chan(), s.chan.as_ref().unwrap());
        let mut dl = x.reborrow().init_data(s.data.len() as u32);
        for (k, d) in s.data.iter().enumerate() {
            cp_par(dl.reborrow().get(k as u32), d);
        }
        x.set_persistent(s.persistent);
        x.set_locally_free(&s.locally_free);
        x.set_connective_used(s.connective_used);
    }
    let mut rl = b.reborrow().init_receives(p.receives.len() as u32);
    for (i, rc) in p.receives.iter().enumerate() {
        let mut x = rl.reborrow().get(i as u32);
        let mut bl = x.reborrow().init_binds(rc.binds.len() as u32);
        for (k, bd) in rc.binds.iter().enumerate() {
            let mut y = bl.reborrow().get(k as u32);
            let mut pl = y.reborrow().init_patterns(bd.patterns.len() as u32);
            for (q, pt) in bd.patterns.iter().enumerate() {
                cp_par(pl.reborrow().get(q as u32), pt);
            }
            cp_par(y.reborrow().init_source(), bd.source.as_ref().unwrap());
            y.set_free_count(bd.free_count);
        }
        cp_par(x.reborrow().init_body(), rc.body.as_ref().unwrap());
        x.set_persistent(rc.persistent);
        x.set_peek(rc.peek);
        x.set_bind_count(rc.bind_count);
        x.set_locally_free(&rc.locally_free);
        x.set_connective_used(rc.connective_used);
    }
    let mut nl = b.reborrow().init_news(p.news.len() as u32);
    for (i, n) in p.news.iter().enumerate() {
        let mut x = nl.reborrow().get(i as u32);
        x.set_bind_count(n.bind_count);
        cp_par(x.reborrow().init_p(), n.p.as_ref().unwrap());
        let mut ul = x.reborrow().init_uri(n.uri.len() as u32);
        for (k, u) in n.uri.iter().enumerate() {
            ul.set(k as u32, u.as_str());
        }
        x.set_locally_free(&n.locally_free);
    }
    let mut el = b.reborrow().init_exprs(p.exprs.len() as u32);
    for (i, e) in p.exprs.iter().enumerate() {
        let mut x = el.reborrow().get(i as u32);
        use expr::ExprInstance as X;
        match e.expr_instance.as_ref().unwrap() {
            X::GBool(v) => x.set_g_bool(*v),
            X::GInt(v) => x.set_g_int(*v),
            X::GString(s) => x.set_g_string(s.as_str()),
            X::GByteArray(v) => x.set_g_byte_array(v),
            X::EVarBody(v) => cp_var(x.init_e_var(), v.v.as_ref().unwrap()),
            X::EListBody(l) => {
                let mut ll = x.init_e_list(l.ps.len() as u32);
                for (k, q) in l.ps.iter().enumerate() {
                    cp_par(ll.reborrow().get(k as u32), q);
                }
            }
            X::ETupleBody(l) => {
                let mut ll = x.init_e_tuple(l.ps.len() as u32);
                for (k, q) in l.ps.iter().enumerate() {
                    cp_par(ll.reborrow().get(k as u32), q);
                }
            }
            _ => unreachable!(),
        }
    }
    let mut ul = b.reborrow().init_unforgeables(p.unforgeables.len() as u32);
    for (i, u) in p.unforgeables.iter().enumerate() {
        if let Some(g_unforgeable::UnfInstance::GPrivateBody(g)) = &u.unf_instance {
            ul.set(i as u32, &g.id);
        }
    }
    b.set_locally_free(&p.locally_free);
    b.set_connective_used(p.connective_used);
}

fn cp_build_par(p: &Par) -> capnp::message::Builder<capnp::message::HeapAllocator> {
    let mut msg = capnp::message::Builder::new_default();
    cp_par(msg.init_root::<cp::par::Builder>(), p);
    msg
}

// ------------------------------------------------ capnp: read -> owned

type R<T> = capnp::Result<T>;

fn b(d: &[u8]) -> Bytes {
    Bytes::copy_from_slice(d)
}

fn rd_prod(p: cp::produce_event::Reader) -> R<ProduceEventProto> {
    Ok(ProduceEventProto {
        channels_hash: b(p.get_channels_hash()?),
        hash: b(p.get_hash()?),
        persistent: p.get_persistent(),
        times_repeated: p.get_times_repeated(),
        is_deterministic: p.get_is_deterministic(),
        output_value: p.get_output_value()?.iter().map(|x| x.map(b)).collect::<R<_>>()?,
        failed: p.get_failed(),
    })
}
fn rd_cons(c: cp::consume_event::Reader) -> R<ConsumeEventProto> {
    Ok(ConsumeEventProto {
        channels_hashes: c.get_channels_hashes()?.iter().map(|x| x.map(b)).collect::<R<_>>()?,
        hash: b(c.get_hash()?),
        persistent: c.get_persistent(),
    })
}

fn rd_block(m: cp::block_message::Reader) -> R<BlockMessageProto> {
    let h = m.get_header()?;
    let bd = m.get_body()?;
    let st = bd.get_state()?;
    let mut deploys = Vec::with_capacity(bd.get_deploys()?.len() as usize);
    for d in bd.get_deploys()?.iter() {
        let dd = d.get_deploy()?;
        let mut log = Vec::with_capacity(d.get_deploy_log()?.len() as usize);
        for e in d.get_deploy_log()?.iter() {
            use event_proto::EventInstance as E;
            let inst = match e.which()? {
                cp::event::Produce(p) => E::Produce(rd_prod(p?)?),
                cp::event::Consume(c) => E::Consume(rd_cons(c?)?),
                cp::event::Comm(c) => {
                    let c = c?;
                    E::Comm(CommEventProto {
                        consume: Some(rd_cons(c.get_consume()?)?),
                        produces: c.get_produces()?.iter().map(rd_prod).collect::<R<_>>()?,
                        peeks: c
                            .get_peeks()?
                            .iter()
                            .map(|i| PeekProto { channel_index: i })
                            .collect(),
                    })
                }
            };
            log.push(EventProto { event_instance: Some(inst) });
        }
        deploys.push(ProcessedDeployProto {
            deploy: Some(DeployDataProto {
                deployer: b(dd.get_deployer()?),
                term: dd.get_term()?.to_string()?,
                timestamp: dd.get_timestamp(),
                sig: b(dd.get_sig()?),
                sig_algorithm: dd.get_sig_algorithm()?.to_string()?,
                phlo_price: dd.get_phlo_price(),
                phlo_limit: dd.get_phlo_limit(),
                valid_after_block_number: dd.get_valid_after_block_number(),
                shard_id: dd.get_shard_id()?.to_string()?,
                language: dd.get_language()?.to_string()?,
                expiration_timestamp: dd.get_expiration_timestamp(),
                parameters: vec![],
            }),
            cost: Some(PCost { cost: d.get_cost() }),
            deploy_log: log,
            errored: d.get_errored(),
            system_deploy_error: d.get_system_deploy_error()?.to_string()?,
        });
    }
    Ok(BlockMessageProto {
        block_hash: b(m.get_block_hash()?),
        header: Some(HeaderProto {
            parents_hash_list: h.get_parents_hash_list()?.iter().map(|x| x.map(b)).collect::<R<_>>()?,
            timestamp: h.get_timestamp(),
            version: h.get_version(),
            extra_bytes: b(h.get_extra_bytes()?),
        }),
        body: Some(BodyProto {
            state: Some(RChainStateProto {
                pre_state_hash: b(st.get_pre_state_hash()?),
                post_state_hash: b(st.get_post_state_hash()?),
                bonds: st
                    .get_bonds()?
                    .iter()
                    .map(|x| Ok(BondProto { validator: b(x.get_validator()?), stake: x.get_stake() }))
                    .collect::<R<_>>()?,
                block_number: st.get_block_number(),
            }),
            deploys,
            system_deploys: vec![],
            extra_bytes: b(bd.get_extra_bytes()?),
            rejected_deploys: vec![],
            applied_from_scope: vec![],
            merge_base: b(bd.get_merge_base()?),
        }),
        justifications: m
            .get_justifications()?
            .iter()
            .map(|j| {
                Ok(JustificationProto {
                    validator: b(j.get_validator()?),
                    latest_block_hash: b(j.get_latest_block_hash()?),
                })
            })
            .collect::<R<_>>()?,
        sender: b(m.get_sender()?),
        seq_num: m.get_seq_num(),
        sig: b(m.get_sig()?),
        sig_algorithm: m.get_sig_algorithm()?.to_string()?,
        shard_id: m.get_shard_id()?.to_string()?,
        extra_bytes: b(m.get_extra_bytes()?),
    })
}

fn rd_var(v: cp::var::Reader) -> R<Var> {
    use var::VarInstance as V;
    Ok(Var {
        var_instance: Some(match v.which()? {
            cp::var::BoundVar(i) => V::BoundVar(i),
            cp::var::FreeVar(i) => V::FreeVar(i),
            cp::var::Wildcard(()) => V::Wildcard(var::WildcardMsg {}),
        }),
    })
}

fn rd_par(p: cp::par::Reader) -> R<Par> {
    let mut out = Par::default();
    for s in p.get_sends()?.iter() {
        out.sends.push(Send {
            chan: Some(rd_par(s.get_chan()?)?),
            data: s.get_data()?.iter().map(rd_par).collect::<R<_>>()?,
            persistent: s.get_persistent(),
            locally_free: s.get_locally_free()?.to_vec(),
            connective_used: s.get_connective_used(),
        });
    }
    for rc in p.get_receives()?.iter() {
        let mut binds = Vec::new();
        for bd in rc.get_binds()?.iter() {
            binds.push(ReceiveBind {
                patterns: bd.get_patterns()?.iter().map(rd_par).collect::<R<_>>()?,
                source: Some(rd_par(bd.get_source()?)?),
                remainder: None,
                free_count: bd.get_free_count(),
            });
        }
        out.receives.push(Receive {
            binds,
            body: Some(rd_par(rc.get_body()?)?),
            persistent: rc.get_persistent(),
            peek: rc.get_peek(),
            bind_count: rc.get_bind_count(),
            locally_free: rc.get_locally_free()?.to_vec(),
            connective_used: rc.get_connective_used(),
            condition: None,
        });
    }
    for n in p.get_news()?.iter() {
        out.news.push(New {
            bind_count: n.get_bind_count(),
            p: Some(rd_par(n.get_p()?)?),
            uri: n.get_uri()?.iter().map(|u| u?.to_string().map_err(Into::into)).collect::<R<_>>()?,
            locally_free: n.get_locally_free()?.to_vec(),
            ..Default::default()
        });
    }
    for e in p.get_exprs()?.iter() {
        use expr::ExprInstance as X;
        let inst = match e.which()? {
            cp::expr::GBool(v) => X::GBool(v),
            cp::expr::GInt(v) => X::GInt(v),
            cp::expr::GString(s) => X::GString(s?.to_string()?),
            cp::expr::GByteArray(d) => X::GByteArray(d?.to_vec()),
            cp::expr::EVar(v) => X::EVarBody(EVar { v: Some(rd_var(v?)?) }),
            cp::expr::EList(l) => X::EListBody(EList {
                ps: l?.iter().map(rd_par).collect::<R<_>>()?,
                ..Default::default()
            }),
            cp::expr::ETuple(l) => X::ETupleBody(ETuple {
                ps: l?.iter().map(rd_par).collect::<R<_>>()?,
                ..Default::default()
            }),
        };
        out.exprs.push(Expr { expr_instance: Some(inst) });
    }
    for u in p.get_unforgeables()?.iter() {
        out.unforgeables.push(GUnforgeable {
            unf_instance: Some(g_unforgeable::UnfInstance::GPrivateBody(GPrivate { id: u?.to_vec() })),
        });
    }
    out.locally_free = p.get_locally_free()?.to_vec();
    out.connective_used = p.get_connective_used();
    Ok(out)
}

// ----------------------------------------- traversal (touch every field)

fn walk_block_owned(m: &BlockMessageProto) -> usize {
    let mut acc = m.block_hash.len() + m.sender.len() + m.sig.len() + m.seq_num as usize;
    let body = m.body.as_ref().unwrap();
    for d in &body.deploys {
        let dd = d.deploy.as_ref().unwrap();
        acc += dd.term.len() + dd.sig.len() + dd.deployer.len() + dd.phlo_limit as usize;
        for e in &d.deploy_log {
            acc += match e.event_instance.as_ref().unwrap() {
                event_proto::EventInstance::Produce(p) => p.hash.len() + p.channels_hash.len(),
                event_proto::EventInstance::Consume(c) => c.hash.len() + c.channels_hashes.len(),
                event_proto::EventInstance::Comm(c) => c.produces.len() + c.consume.as_ref().unwrap().hash.len(),
            };
        }
    }
    acc
}

fn walk_block_cp(m: cp::block_message::Reader) -> R<usize> {
    let mut acc = m.get_block_hash()?.len() + m.get_sender()?.len() + m.get_sig()?.len() + m.get_seq_num() as usize;
    for d in m.get_body()?.get_deploys()?.iter() {
        let dd = d.get_deploy()?;
        acc += dd.get_term()?.len() as usize
            + dd.get_sig()?.len()
            + dd.get_deployer()?.len()
            + dd.get_phlo_limit() as usize;
        for e in d.get_deploy_log()?.iter() {
            acc += match e.which()? {
                cp::event::Produce(p) => {
                    let p = p?;
                    p.get_hash()?.len() + p.get_channels_hash()?.len()
                }
                cp::event::Consume(c) => {
                    let c = c?;
                    c.get_hash()?.len() + c.get_channels_hashes()?.len() as usize
                }
                cp::event::Comm(c) => {
                    let c = c?;
                    c.get_produces()?.len() as usize + c.get_consume()?.get_hash()?.len()
                }
            };
        }
    }
    Ok(acc)
}

// ---------------------------------------------------------------- harness

fn bench<F: FnMut() -> usize>(name: &str, mut f: F) -> f64 {
    // calibrate
    let mut iters = 1u64;
    loop {
        let t = Instant::now();
        for _ in 0..iters {
            black_box(f());
        }
        if t.elapsed() > Duration::from_millis(40) {
            break;
        }
        iters *= 2;
    }
    let mut samples = Vec::new();
    for _ in 0..11 {
        let t = Instant::now();
        for _ in 0..iters {
            black_box(f());
        }
        samples.push(t.elapsed().as_nanos() as f64 / iters as f64);
    }
    samples.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let med = samples[samples.len() / 2];
    println!("  {:<46} {:>14.1} ns  (min {:>.1}, max {:>.1})", name, med, samples[0], samples[10]);
    med
}

fn opts() -> capnp::message::ReaderOptions {
    let mut o = capnp::message::ReaderOptions::new();
    o.traversal_limit_in_words(None);
    o.nesting_limit(512);
    o
}

fn blake(bytes: &[u8]) -> [u8; 32] {
    use blake2::Digest;
    let mut h = blake2::Blake2b::<blake2::digest::consts::U32>::new();
    h.update(bytes);
    h.finalize().into()
}

fn csv(rows: &mut Vec<String>, wl: &str, op: &str, codec: &str, ns: f64) {
    rows.push(format!("{wl},{op},{codec},{ns:.1}"));
}

fn main() {
    let mut rows: Vec<String> = vec!["workload,op,codec,ns".into()];
    let mut sizes: Vec<String> = vec!["workload,codec,raw_bytes,lz4_bytes".into()];

    let blocks = [
        ("block-1x40", gen_block(1, 40, 7)),
        ("block-32x40", gen_block(32, 40, 11)),
        ("block-32x400", gen_block(32, 400, 13)),
        ("block-200x40", gen_block(200, 40, 17)),
    ];

    for (wl, blk) in &blocks {
        println!("== {wl}");
        let pb = blk.encode_to_vec();
        let cmsg = cp_build_block(blk);
        let cbytes = capnp::serialize::write_message_to_words(&cmsg);
        let mut cpacked = Vec::new();
        capnp::serialize_packed::write_message(&mut cpacked, &cmsg).unwrap();
        // round-trip check
        {
            let r = capnp::serialize::read_message_from_flat_slice(&mut &cbytes[..], opts()).unwrap();
            let back = rd_block(r.get_root().unwrap()).unwrap();
            assert_eq!(&back, blk, "capnp round trip");
            assert_eq!(&BlockMessageProto::decode(&pb[..]).unwrap(), blk);
        }
        let lz = |v: &[u8]| lz4_flex::compress_prepend_size(v).len();
        println!(
            "  size: prost {} B (lz4 {}), capnp {} B (lz4 {}), capnp-packed {} B (lz4 {})",
            pb.len(), lz(&pb), cbytes.len(), lz(&cbytes), cpacked.len(), lz(&cpacked)
        );
        sizes.push(format!("{wl},prost,{},{}", pb.len(), lz(&pb)));
        sizes.push(format!("{wl},capnp,{},{}", cbytes.len(), lz(&cbytes)));
        sizes.push(format!("{wl},capnp-packed,{},{}", cpacked.len(), lz(&cpacked)));

        let t = bench("encode prost (struct -> Vec)", || blk.encode_to_vec().len());
        csv(&mut rows, wl, "encode", "prost", t);
        let t = bench("encode capnp (build + flat words)", || {
            capnp::serialize::write_message_to_words(&cp_build_block(blk)).len()
        });
        csv(&mut rows, wl, "encode", "capnp", t);
        let t = bench("encode capnp packed", || {
            let mut v = Vec::new();
            capnp::serialize_packed::write_message(&mut v, &cp_build_block(blk)).unwrap();
            v.len()
        });
        csv(&mut rows, wl, "encode", "capnp-packed", t);

        let t = bench("decode prost -> owned", || BlockMessageProto::decode(&pb[..]).unwrap().seq_num as usize);
        csv(&mut rows, wl, "decode_owned", "prost", t);
        let pbb = Bytes::from(pb.clone());
        let t = bench("decode prost from Bytes (zero-copy bytes fields)", || {
            BlockMessageProto::decode(pbb.clone()).unwrap().seq_num as usize
        });
        csv(&mut rows, wl, "decode_owned", "prost-Bytes", t);
        let t = bench("prost encoded_len only (size metric)", || blk.encoded_len());
        csv(&mut rows, wl, "encoded_len", "prost", t);
        let t = bench("decode capnp -> owned", || {
            let r = capnp::serialize::read_message_from_flat_slice(&mut &cbytes[..], opts()).unwrap();
            rd_block(r.get_root().unwrap()).unwrap().seq_num as usize
        });
        csv(&mut rows, wl, "decode_owned", "capnp", t);
        let t = bench("decode capnp-packed -> owned", || {
            let r = capnp::serialize_packed::read_message(&mut &cpacked[..], opts()).unwrap();
            rd_block(r.get_root().unwrap()).unwrap().seq_num as usize
        });
        csv(&mut rows, wl, "decode_owned", "capnp-packed", t);

        let t = bench("read-all prost (decode + walk)", || {
            walk_block_owned(&BlockMessageProto::decode(&pb[..]).unwrap())
        });
        csv(&mut rows, wl, "read_all", "prost", t);
        let t = bench("read-all capnp (zero-copy walk)", || {
            let r = capnp::serialize::read_message_from_flat_slice(&mut &cbytes[..], opts()).unwrap();
            walk_block_cp(r.get_root().unwrap()).unwrap()
        });
        csv(&mut rows, wl, "read_all", "capnp", t);

        let t = bench("partial: deploy sigs, prost full decode", || {
            let m = BlockMessageProto::decode(&pb[..]).unwrap();
            m.body.unwrap().deploys.iter().map(|d| d.deploy.as_ref().unwrap().sig.len()).sum()
        });
        csv(&mut rows, wl, "partial_sigs", "prost", t);
        let t = bench("partial: deploy sigs, capnp zero-copy", || {
            let r = capnp::serialize::read_message_from_flat_slice(&mut &cbytes[..], opts()).unwrap();
            let m: cp::block_message::Reader = r.get_root().unwrap();
            m.get_body().unwrap().get_deploys().unwrap().iter()
                .map(|d| d.get_deploy().unwrap().get_sig().unwrap().len()).sum()
        });
        csv(&mut rows, wl, "partial_sigs", "capnp", t);

        // hash_block-shaped pipeline: encode header + body, blake2b256
        let (h, bd) = (blk.header.clone().unwrap(), blk.body.clone().unwrap());
        let t = bench("hash: prost(header)+prost(body) -> blake2b", || {
            let mut v = h.encode_to_vec();
            v.extend(bd.encode_to_vec());
            blake(&v)[0] as usize
        });
        csv(&mut rows, wl, "hash_pipeline", "prost", t);
        let t = bench("hash: capnp canonical(header,body) -> blake2b", || {
            let mut mh = capnp::message::Builder::new_default();
            cp_header(mh.init_root(), &h);
            let mut mb = capnp::message::Builder::new_default();
            cp_body(mb.init_root(), &bd);
            let ch = mh.into_reader().canonicalize().unwrap();
            let cb = mb.into_reader().canonicalize().unwrap();
            let mut v: Vec<u8> = capnp::Word::words_to_bytes(&ch).to_vec();
            v.extend_from_slice(capnp::Word::words_to_bytes(&cb));
            blake(&v)[0] as usize
        });
        csv(&mut rows, wl, "hash_pipeline", "capnp", t);
        // Merkle-body alternative: hash each processed deploy independently,
        // then a binary Merkle tree over the leaf hashes plus a small header record.
        let leaves_seq = || -> Vec<[u8; 32]> {
            bd.deploys.iter().map(|d| blake(&d.encode_to_vec())).collect()
        };
        fn merkle(mut v: Vec<[u8; 32]>) -> [u8; 32] {
            if v.is_empty() { return [0u8; 32]; }
            while v.len() > 1 {
                v = v.chunks(2).map(|c| {
                    let mut x = c[0].to_vec();
                    x.extend_from_slice(c.get(1).unwrap_or(&c[0]));
                    blake(&x)
                }).collect();
            }
            v[0]
        }
        let t = bench("merkle: per-deploy prost+blake2b, then root", || merkle(leaves_seq())[0] as usize);
        csv(&mut rows, wl, "merkle_full", "prost", t);
        let leaves = leaves_seq();
        let t = bench("merkle: root only (leaves cached at replay)", || {
            let mut x = h.encode_to_vec();
            x.extend_from_slice(&merkle(leaves.clone()));
            blake(&x)[0] as usize
        });
        csv(&mut rows, wl, "merkle_root_only", "prost", t);
        let one = &bd.deploys[0];
        let t = bench("merkle: one deploy leaf (per-deploy, at replay)", || blake(&one.encode_to_vec())[0] as usize);
        csv(&mut rows, wl, "merkle_one_leaf", "prost", t);
        let t = bench("blake2b256 alone over prost bytes", || blake(&pb)[0] as usize);
        csv(&mut rows, wl, "blake2b_only", "-", t);
        let t = bench("lz4 compress prost bytes", || lz4_flex::compress_prepend_size(&pb).len());
        csv(&mut rows, wl, "lz4_compress", "prost", t);
        let t = bench("clone owned struct (to_proto stand-in)", || blk.clone().seq_num as usize);
        csv(&mut rows, wl, "clone", "-", t);
    }

    let pars = [
        ("par-small", { let mut r = Rng(3); gen_par(&mut r, 2, 2) }),
        ("par-medium", { let mut r = Rng(5); gen_par(&mut r, 4, 3) }),
        ("par-large", { let mut r = Rng(9); gen_par(&mut r, 6, 3) }),
    ];
    for (wl, par) in &pars {
        println!("== {wl}");
        let pb = par.encode_to_vec();
        let cmsg = cp_build_par(par);
        let cbytes = capnp::serialize::write_message_to_words(&cmsg);
        let mut cpacked = Vec::new();
        capnp::serialize_packed::write_message(&mut cpacked, &cmsg).unwrap();
        {
            let r = capnp::serialize::read_message_from_flat_slice(&mut &cbytes[..], opts()).unwrap();
            let back = rd_par(r.get_root().unwrap()).unwrap();
            assert_eq!(back.encode_to_vec(), pb, "capnp par round trip");
        }
        let lz = |v: &[u8]| lz4_flex::compress_prepend_size(v).len();
        println!(
            "  size: prost {} B (lz4 {}), capnp {} B (lz4 {}), capnp-packed {} B (lz4 {})",
            pb.len(), lz(&pb), cbytes.len(), lz(&cbytes), cpacked.len(), lz(&cpacked)
        );
        sizes.push(format!("{wl},prost,{},{}", pb.len(), lz(&pb)));
        sizes.push(format!("{wl},capnp,{},{}", cbytes.len(), lz(&cbytes)));
        sizes.push(format!("{wl},capnp-packed,{},{}", cpacked.len(), lz(&cpacked)));

        let t = bench("encode prost", || par.encode_to_vec().len());
        csv(&mut rows, wl, "encode", "prost", t);
        let t = bench("encode capnp", || capnp::serialize::write_message_to_words(&cp_build_par(par)).len());
        csv(&mut rows, wl, "encode", "capnp", t);
        let t = bench("decode prost -> owned", || Par::decode(&pb[..]).unwrap().sends.len());
        csv(&mut rows, wl, "decode_owned", "prost", t);
        let t = bench("decode capnp -> owned", || {
            let r = capnp::serialize::read_message_from_flat_slice(&mut &cbytes[..], opts()).unwrap();
            rd_par(r.get_root().unwrap()).unwrap().sends.len()
        });
        csv(&mut rows, wl, "decode_owned", "capnp", t);
        let t = bench("hash: prost -> blake2b (toByteArray/registry)", || blake(&par.encode_to_vec())[0] as usize);
        csv(&mut rows, wl, "hash_pipeline", "prost", t);
        let t = bench("hash: capnp canonical -> blake2b", || {
            let c = cp_build_par(par).into_reader().canonicalize().unwrap();
            blake(capnp::Word::words_to_bytes(&c))[0] as usize
        });
        csv(&mut rows, wl, "hash_pipeline", "capnp", t);
        let t = bench("hash: bincode -> blake2b (rspace channel hash)", || {
            blake(&bincode::serialize(par).unwrap())[0] as usize
        });
        csv(&mut rows, wl, "hash_pipeline", "bincode", t);
        let t = bench("encode bincode", || bincode::serialize(par).unwrap().len());
        csv(&mut rows, wl, "encode", "bincode", t);
        println!("  size: bincode {} B", bincode::serialize(par).unwrap().len());
        sizes.push(format!("{wl},bincode,{},{}", bincode::serialize(par).unwrap().len(), lz(&bincode::serialize(par).unwrap())));
        let t = bench("clone owned Par", || par.clone().sends.len());
        csv(&mut rows, wl, "clone", "-", t);
    }

    std::fs::write("results.csv", rows.join("\n") + "\n").unwrap();
    std::fs::write("sizes.csv", sizes.join("\n") + "\n").unwrap();
    println!("wrote results.csv, sizes.csv");
}
