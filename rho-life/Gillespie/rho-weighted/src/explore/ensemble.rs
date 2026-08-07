//! Trajectory ensembles.
//!
//! A single stochastic trajectory says almost nothing. A study runs `n` of
//! them from the same configuration with distinct seeds and reports the
//! distribution — which is what makes the output a graph with bands rather than
//! a line with false confidence.

use std::collections::BTreeMap;

use crate::explore::ssa::{Sim, Trace};
use crate::explore::Stop;
use crate::matching::Matching;
use crate::theory::{Configuration, WeightedTheory};

pub struct Ensemble {
    pub traces: Vec<Trace>,
    pub seeds: Vec<u64>,
}

impl Ensemble {
    pub fn run<M: Matching>(
        theory: &WeightedTheory,
        matcher: &M,
        init: &Configuration,
        base_seed: u64,
        n: usize,
        max_steps: usize,
        horizon: f64,
    ) -> Ensemble {
        let mut traces = Vec::with_capacity(n);
        let mut seeds = Vec::with_capacity(n);
        for i in 0..n {
            let seed = base_seed.wrapping_mul(0x100000001B3).wrapping_add(i as u64);
            let mut sim = Sim::new(theory, matcher, init.clone(), seed);
            traces.push(sim.run(max_steps, horizon));
            seeds.push(seed);
        }
        Ensemble { traces, seeds }
    }

    /// Time-weighted occupancy averaged over the ensemble.
    pub fn mean_occupancy(&self) -> BTreeMap<String, f64> {
        let mut acc: BTreeMap<String, f64> = BTreeMap::new();
        let mut total = 0.0;
        for t in &self.traces {
            for (k, v) in t.occupancy() {
                *acc.entry(k).or_insert(0.0) += v;
                total += v;
            }
        }
        if total > 0.0 {
            for v in acc.values_mut() {
                *v /= total;
            }
        }
        acc
    }

    pub fn summary(&self) -> EnsembleSummary {
        let lens: Vec<f64> = self.traces.iter().map(|t| t.len() as f64).collect();
        let times: Vec<f64> = self.traces.iter().map(|t| t.end_time).collect();
        EnsembleSummary {
            runs: self.traces.len(),
            mean_steps: mean(&lens),
            mean_end_time: mean(&times),
            sd_end_time: sd(&times),
            truncated: self
                .traces
                .iter()
                .filter(|t| t.stop.is_truncation())
                .count(),
            absorbing: self
                .traces
                .iter()
                .filter(|t| t.stop == Stop::Absorbing)
                .count(),
        }
    }

    /// All sampled waiting times, pooled — the input to the exponentiality
    /// check.
    pub fn taus(&self) -> Vec<f64> {
        self.traces
            .iter()
            .flat_map(|t| t.steps.iter().skip(1).map(|s| s.tau))
            .collect()
    }
}

#[derive(Clone, Debug)]
pub struct EnsembleSummary {
    pub runs: usize,
    pub mean_steps: f64,
    pub mean_end_time: f64,
    pub sd_end_time: f64,
    pub truncated: usize,
    pub absorbing: usize,
}

pub fn mean(xs: &[f64]) -> f64 {
    if xs.is_empty() {
        return 0.0;
    }
    xs.iter().sum::<f64>() / xs.len() as f64
}

pub fn sd(xs: &[f64]) -> f64 {
    if xs.len() < 2 {
        return 0.0;
    }
    let m = mean(xs);
    (xs.iter().map(|x| (x - m).powi(2)).sum::<f64>() / (xs.len() - 1) as f64).sqrt()
}
