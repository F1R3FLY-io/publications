//! A small, seeded, dependency-free PRNG.
//!
//! Studies are scientific artifacts: a result that cannot be reproduced from
//! `(theory hash, seed, budget)` is not a result. Vendoring the generator
//! rather than depending on one also means the numbers do not move when a
//! dependency bumps its algorithm.
//!
//! SplitMix64 — a well-characterised, equidistributed 64-bit generator.

#[derive(Clone, Debug)]
pub struct Rng {
    state: u64,
}

impl Rng {
    pub fn seeded(seed: u64) -> Rng {
        Rng {
            state: seed.wrapping_add(0x9E3779B97F4A7C15),
        }
    }

    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }

    /// Uniform on the open interval (0, 1); never returns 0, so `ln(1/u)` is
    /// always finite.
    pub fn unit(&mut self) -> f64 {
        loop {
            let x = (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64;
            if x > 0.0 && x < 1.0 {
                return x;
            }
        }
    }
}
