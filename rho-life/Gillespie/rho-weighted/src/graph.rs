//! Graphs — the simulator's product.
//!
//! The interpreter's output is a new state. The simulator's output is a graph:
//! the labelled transition system, its generator, and whatever a study
//! projects out of them. These are artifacts meant to be looked at, diffed and
//! cited, so they carry provenance and they declare truncation.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use crate::redex::RuleId;

#[derive(Clone, Debug)]
pub struct TransitionEdge {
    pub from: usize,
    pub to: usize,
    pub rule: RuleId,
    pub class: usize,
    /// The label `k` of the modality — the position at which the step fired.
    pub position: String,
    pub rate: f64,
}

#[derive(Clone, Debug, Default)]
pub struct LabelledTransitionGraph {
    /// Human-readable marking per node.
    pub node_labels: Vec<String>,
    /// Full configuration key per node (marking and weight fingerprint).
    pub node_keys: Vec<String>,
    pub edges: Vec<TransitionEdge>,
}

impl LabelledTransitionGraph {
    pub fn node_count(&self) -> usize {
        self.node_labels.len()
    }
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Project onto the term-marginal: collapse nodes that differ only in their
    /// weight map. Useful for reading a plastic model's structure, and honest
    /// only if the reader knows the projection lost the map.
    pub fn term_projection(&self) -> LabelledTransitionGraph {
        let mut index: BTreeMap<String, usize> = BTreeMap::new();
        let mut labels = Vec::new();
        let mut remap = vec![0usize; self.node_count()];
        for (i, l) in self.node_labels.iter().enumerate() {
            let j = *index.entry(l.clone()).or_insert_with(|| {
                labels.push(l.clone());
                labels.len() - 1
            });
            remap[i] = j;
        }
        let mut agg: BTreeMap<(usize, usize, usize), f64> = BTreeMap::new();
        for e in &self.edges {
            *agg.entry((remap[e.from], remap[e.to], e.class)).or_insert(0.0) += e.rate;
        }
        LabelledTransitionGraph {
            node_keys: labels.clone(),
            node_labels: labels,
            edges: agg
                .into_iter()
                .map(|((f, t, c), rate)| TransitionEdge {
                    from: f,
                    to: t,
                    rule: crate::redex::COMM,
                    class: c,
                    position: String::new(),
                    rate,
                })
                .collect(),
        }
    }

    pub fn to_dot(&self, title: &str) -> String {
        let mut s = String::new();
        let _ = writeln!(s, "digraph \"{}\" {{", escape(title));
        let _ = writeln!(s, "  rankdir=LR;");
        let _ = writeln!(
            s,
            "  node [shape=box, style=rounded, fontname=\"Helvetica\", fontsize=9];"
        );
        let _ = writeln!(s, "  edge [fontname=\"Helvetica\", fontsize=8];");
        for (i, l) in self.node_labels.iter().enumerate() {
            let _ = writeln!(s, "  n{} [label=\"{}\"];", i, escape(l));
        }
        for e in &self.edges {
            let _ = writeln!(
                s,
                "  n{} -> n{} [label=\"{}  {:.4}\"];",
                e.from,
                e.to,
                e.class,
                e.rate
            );
        }
        let _ = writeln!(s, "}}");
        s
    }

    pub fn to_json(&self) -> String {
        let mut s = String::from("{\"nodes\":[");
        for (i, l) in self.node_labels.iter().enumerate() {
            if i > 0 {
                s.push(',');
            }
            let _ = write!(
                s,
                "{{\"id\":{},\"marking\":\"{}\",\"key\":\"{}\"}}",
                i,
                escape(l),
                escape(self.node_keys.get(i).map(|x| x.as_str()).unwrap_or(""))
            );
        }
        s.push_str("],\"edges\":[");
        for (i, e) in self.edges.iter().enumerate() {
            if i > 0 {
                s.push(',');
            }
            let _ = write!(
                s,
                "{{\"from\":{},\"to\":{},\"rule\":{},\"class\":{},\"position\":\"{}\",\"rate\":{}}}",
                e.from,
                e.to,
                e.rule.0,
                e.class,
                escape(&e.position),
                fmt_f64(e.rate)
            );
        }
        s.push_str("]}");
        s
    }
}

/// The CTMC generator `Q`: off-diagonal entries are summed transition rates,
/// diagonal entries are `-Σ_{j≠i} Q(i,j)`.
///
/// # Self-transitions, and why the diagonal is not `-a₀`
///
/// Nothing forbids a rule whose firing returns a configuration equal to the one
/// it fired in — a persistent receipt whose body restores what it consumed, for
/// instance. Such a redex contributes to `a₀` but to no off-diagonal entry, so
/// setting the diagonal to `-a₀` gives a generator whose rows do not sum to
/// zero. The formula above is the correct one, and it is what this assembles.
///
/// The sampler does not agree, and deliberately. [`crate::propensity`] leaves
/// self-loop redexes in `a₀`, so [`crate::explore::ssa`] fires them and advances
/// the clock. A self-loop of rate `λ` is a *fictitious jump* in the sense of
/// uniformisation, so the two conventions induce the same distribution of the
/// state at every time and differ only in the number of recorded events, hence
/// in sojourn statistics. Neither is wrong; mixing them silently would be. The
/// crate reports `Q` by the formula above and samples with self-transitions
/// retained, and `law_self_transitions_are_fictitious_jumps` pins it.
///
/// Summing over *derivations* rather than targets is the classical reading. In
/// the complex case the same aggregation happens under a square root — see
/// [`crate::quantum`] — which is what makes the two readings agree.
#[derive(Clone, Debug, Default)]
pub struct Generator {
    pub n: usize,
    pub q: Vec<Vec<f64>>,
}

impl Generator {
    pub fn from_graph(g: &LabelledTransitionGraph) -> Generator {
        let n = g.node_count();
        let mut q = vec![vec![0.0; n]; n];
        for e in &g.edges {
            if e.from != e.to {
                q[e.from][e.to] += e.rate;
            }
        }
        for (i, row) in q.iter_mut().enumerate() {
            let off: f64 = row.iter().enumerate().filter(|(j, _)| *j != i).map(|(_, v)| *v).sum();
            row[i] = -off;
        }
        Generator { n, q }
    }

    pub fn total_exit_rate(&self, i: usize) -> f64 {
        -self.q[i][i]
    }

    /// The stationary distribution, by the power method on the uniformised
    /// chain `P = I + Q/λ`. Returns `None` when the chain has no unique
    /// stationary distribution reachable this way (several absorbing classes,
    /// or no transitions at all).
    pub fn stationary(&self, iters: usize, tol: f64) -> Option<Vec<f64>> {
        if self.n == 0 {
            return None;
        }
        let lambda = (0..self.n)
            .map(|i| self.total_exit_rate(i))
            .fold(0.0f64, f64::max);
        if lambda <= 0.0 {
            return None;
        }
        let lambda = lambda * 1.01;
        let mut pi = vec![1.0 / self.n as f64; self.n];
        for _ in 0..iters {
            let mut next = vec![0.0; self.n];
            for i in 0..self.n {
                for j in 0..self.n {
                    let p = if i == j {
                        1.0 + self.q[i][j] / lambda
                    } else {
                        self.q[i][j] / lambda
                    };
                    next[j] += pi[i] * p;
                }
            }
            let sum: f64 = next.iter().sum();
            if sum <= 0.0 {
                return None;
            }
            for v in next.iter_mut() {
                *v /= sum;
            }
            let delta: f64 = pi
                .iter()
                .zip(next.iter())
                .map(|(a, b)| (a - b).abs())
                .sum();
            pi = next;
            if delta < tol {
                break;
            }
        }
        Some(pi)
    }

    pub fn to_csv(&self, labels: &[String]) -> String {
        let mut s = String::from("from\\to");
        for l in labels {
            let _ = write!(s, ",{}", escape(l));
        }
        s.push('\n');
        for (i, row) in self.q.iter().enumerate() {
            let _ = write!(s, "{}", escape(labels.get(i).map(|x| x.as_str()).unwrap_or("")));
            for v in row {
                let _ = write!(s, ",{}", fmt_f64(*v));
            }
            s.push('\n');
        }
        s
    }
}

fn escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', " ")
}

fn fmt_f64(v: f64) -> String {
    if v.is_finite() {
        format!("{v:.10}")
    } else {
        "0".into()
    }
}
