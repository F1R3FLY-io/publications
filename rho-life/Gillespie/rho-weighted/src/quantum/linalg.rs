//! Minimal complex arithmetic and dense matrices.
//!
//! Vendored rather than depended on, for the same reason as the PRNG: a study's
//! numbers should not move when a dependency bumps its algorithm. The matrices
//! here are small — the basis is the reachable configuration set, which is
//! finite only for a term-finite model — so dense storage and cubic operations
//! are the right trade.

use std::ops::{Add, Mul, Neg, Sub};

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct C {
    pub re: f64,
    pub im: f64,
}

impl C {
    pub const ZERO: C = C { re: 0.0, im: 0.0 };
    pub const ONE: C = C { re: 1.0, im: 0.0 };
    pub const I: C = C { re: 0.0, im: 1.0 };

    pub fn new(re: f64, im: f64) -> C {
        C { re, im }
    }
    pub fn real(re: f64) -> C {
        C { re, im: 0.0 }
    }
    pub fn conj(self) -> C {
        C {
            re: self.re,
            im: -self.im,
        }
    }
    pub fn norm_sqr(self) -> f64 {
        self.re * self.re + self.im * self.im
    }
    pub fn abs(self) -> f64 {
        self.norm_sqr().sqrt()
    }
    pub fn scale(self, k: f64) -> C {
        C {
            re: self.re * k,
            im: self.im * k,
        }
    }
    pub fn is_finite(self) -> bool {
        self.re.is_finite() && self.im.is_finite()
    }
}

impl Add for C {
    type Output = C;
    fn add(self, o: C) -> C {
        C::new(self.re + o.re, self.im + o.im)
    }
}
impl Sub for C {
    type Output = C;
    fn sub(self, o: C) -> C {
        C::new(self.re - o.re, self.im - o.im)
    }
}
impl Mul for C {
    type Output = C;
    fn mul(self, o: C) -> C {
        C::new(
            self.re * o.re - self.im * o.im,
            self.re * o.im + self.im * o.re,
        )
    }
}
impl Neg for C {
    type Output = C;
    fn neg(self) -> C {
        C::new(-self.re, -self.im)
    }
}

/// A dense square complex matrix.
#[derive(Clone, PartialEq, Debug)]
pub struct Matrix {
    pub n: usize,
    pub a: Vec<C>,
}

impl Matrix {
    pub fn zeros(n: usize) -> Matrix {
        Matrix {
            n,
            a: vec![C::ZERO; n * n],
        }
    }

    pub fn identity(n: usize) -> Matrix {
        let mut m = Matrix::zeros(n);
        for i in 0..n {
            m.set(i, i, C::ONE);
        }
        m
    }

    #[inline]
    pub fn get(&self, i: usize, j: usize) -> C {
        self.a[i * self.n + j]
    }
    #[inline]
    pub fn set(&mut self, i: usize, j: usize, v: C) {
        self.a[i * self.n + j] = v;
    }
    #[inline]
    pub fn add_to(&mut self, i: usize, j: usize, v: C) {
        self.a[i * self.n + j] = self.a[i * self.n + j] + v;
    }

    pub fn dagger(&self) -> Matrix {
        let mut out = Matrix::zeros(self.n);
        for i in 0..self.n {
            for j in 0..self.n {
                out.set(j, i, self.get(i, j).conj());
            }
        }
        out
    }

    pub fn mul(&self, o: &Matrix) -> Matrix {
        let n = self.n;
        let mut out = Matrix::zeros(n);
        for i in 0..n {
            for k in 0..n {
                let aik = self.get(i, k);
                if aik == C::ZERO {
                    continue;
                }
                for j in 0..n {
                    out.add_to(i, j, aik * o.get(k, j));
                }
            }
        }
        out
    }

    pub fn add(&self, o: &Matrix) -> Matrix {
        Matrix {
            n: self.n,
            a: self.a.iter().zip(o.a.iter()).map(|(x, y)| *x + *y).collect(),
        }
    }

    pub fn sub(&self, o: &Matrix) -> Matrix {
        Matrix {
            n: self.n,
            a: self.a.iter().zip(o.a.iter()).map(|(x, y)| *x - *y).collect(),
        }
    }

    pub fn scale(&self, k: f64) -> Matrix {
        Matrix {
            n: self.n,
            a: self.a.iter().map(|x| x.scale(k)).collect(),
        }
    }

    pub fn mul_c(&self, z: C) -> Matrix {
        Matrix {
            n: self.n,
            a: self.a.iter().map(|x| *x * z).collect(),
        }
    }

    pub fn trace(&self) -> C {
        (0..self.n).fold(C::ZERO, |acc, i| acc + self.get(i, i))
    }

    /// `M v`.
    pub fn apply(&self, v: &[C]) -> Vec<C> {
        let mut out = vec![C::ZERO; self.n];
        for i in 0..self.n {
            let mut s = C::ZERO;
            for j in 0..self.n {
                s = s + self.get(i, j) * v[j];
            }
            out[i] = s;
        }
        out
    }

    pub fn is_hermitian(&self, tol: f64) -> bool {
        for i in 0..self.n {
            for j in 0..self.n {
                let d = self.get(i, j) - self.get(j, i).conj();
                if d.abs() > tol {
                    return false;
                }
            }
        }
        true
    }
}

pub fn norm_sqr(v: &[C]) -> f64 {
    v.iter().map(|z| z.norm_sqr()).sum()
}

pub fn normalize(v: &mut [C]) {
    let n = norm_sqr(v).sqrt();
    if n > 0.0 {
        for z in v.iter_mut() {
            *z = z.scale(1.0 / n);
        }
    }
}
