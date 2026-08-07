//! Exhaustive construction of the reachable graph and its generator.
//!
//! This is the thing a simulator can do and an interpreter cannot: enumerate
//! *all* successors of every reachable configuration and assemble the CTMC
//! generator `Q` directly, with no sampling. It tests the enumeration
//! independently of the sampler, and it is what the quantum construction needs
//! (a finite basis).
//!
//! The state budget is enforced and its exhaustion is reported, never hidden.

use std::collections::BTreeMap;

use crate::explore::Stop;
use crate::graph::{Generator, LabelledTransitionGraph, TransitionEdge};
use crate::logic::Budget;
use crate::matching::Matching;
use crate::propensity::propensities;
use crate::theory::{Configuration, UpdateCtx, WeightedTheory};

pub struct Exploration {
    pub graph: LabelledTransitionGraph,
    pub generator: Generator,
    pub stop: Stop,
    pub states_visited: usize,
}

impl Exploration {
    /// Whether the graph is the whole reachable set or a truncation.
    pub fn is_complete(&self) -> bool {
        !self.stop.is_truncation()
    }
}

/// Build the reachable labelled transition graph and the generator matrix.
///
/// Nodes are configurations — space *and* weight map — because two runs
/// reaching the same term with different maps have different futures. A study
/// that only cares about the term can project afterwards.
pub fn exhaustive_graph<M: Matching>(
    theory: &WeightedTheory,
    init: &Configuration,
    m: &M,
    max_states: usize,
) -> Exploration {
    let mut budget = Budget {
        states: max_states as u32,
        ..Budget::default()
    };

    let mut index: BTreeMap<String, usize> = BTreeMap::new();
    let mut labels: Vec<String> = Vec::new();
    let mut configs: Vec<Configuration> = Vec::new();
    let mut edges: Vec<TransitionEdge> = Vec::new();

    let k0 = init.key();
    index.insert(k0.clone(), 0);
    labels.push(init.space.marking().key());
    configs.push(init.clone());

    let mut frontier = vec![0usize];
    let mut stop = Stop::Absorbing;

    while let Some(i) = frontier.pop() {
        if configs.len() > max_states {
            stop = Stop::StateBudget;
            break;
        }
        let cfg = configs[i].clone();
        let props = propensities(theory, &cfg, m, &mut budget);
        for w in &props.items {
            if w.rate <= 0.0 {
                continue;
            }
            let next_space = cfg.space.fire(&w.redex);
            let key = (w.redex.rule, w.class);
            let position = w.redex.position();
            let ctx = UpdateCtx {
                key,
                bindings: &w.redex.bindings,
                position: &position,
                // Exhaustive construction has no clock; time-dependent update
                // functions are therefore not admissible here, and a theory
                // using one should be explored by sampling instead.
                time: 0.0,
                trace: &cfg.trace,
            };
            let rule = match theory.rule(w.redex.rule) {
                Some(r) => r,
                None => continue,
            };
            let new_weights = (rule.entries[w.class].update)(&cfg.weights, &ctx);
            let next = Configuration {
                space: next_space,
                weights: new_weights,
                trace: cfg.trace.clone(),
            };
            let nk = next.key();
            let j = match index.get(&nk) {
                Some(j) => *j,
                None => {
                    let j = configs.len();
                    if j > max_states {
                        stop = Stop::StateBudget;
                        break;
                    }
                    index.insert(nk.clone(), j);
                    labels.push(next.space.marking().key());
                    configs.push(next);
                    frontier.push(j);
                    j
                }
            };
            edges.push(TransitionEdge {
                from: i,
                to: j,
                rule: w.redex.rule,
                class: w.class,
                position: position.clone(),
                rate: w.rate,
            });
        }
        if stop == Stop::StateBudget {
            break;
        }
    }

    let n = configs.len();
    let graph = LabelledTransitionGraph {
        node_labels: labels,
        node_keys: {
            let mut ks = vec![String::new(); n];
            for (k, i) in &index {
                if *i < n {
                    ks[*i] = k.clone();
                }
            }
            ks
        },
        edges,
    };
    let generator = Generator::from_graph(&graph);
    Exploration {
        graph,
        generator,
        stop,
        states_visited: n,
    }
}
