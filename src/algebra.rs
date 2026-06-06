//! Algebra on the balanced ternary ring Z₃ = {-1, 0, +1}.
//!
//! Core operations: polynomial evaluation, linear algebra, and
//! algebraic invariants over the ternary field.

use ternary_types::Ternary::{self, Negative, Neutral, Positive};
use ternary_types::TritVector;

/// Evaluate a polynomial with ternary coefficients at a ternary point.
///
/// P(x) = c₀ + c₁·x + c₂·x² + …
pub fn eval_poly(coeffs: &[Ternary], x: Ternary) -> Ternary {
    // Horner's method in Z₃
    let mut result = Neutral;
    for &c in coeffs.iter().rev() {
        result = result * x + c;
    }
    result
}

/// The trace of a 3×3 ternary matrix.
pub fn trace_3x3(m: &[[Ternary; 3]; 3]) -> Ternary {
    m[0][0] + m[1][1] + m[2][2]
}

/// Determinant of a 3×3 ternary matrix.
pub fn det_3x3(m: &[[Ternary; 3]; 3]) -> Ternary {
    let a = m[0][0]; let b = m[0][1]; let c = m[0][2];
    let d = m[1][0]; let e = m[1][1]; let f = m[1][2];
    let g = m[2][0]; let h = m[2][1]; let i_ = m[2][2];

    // det = a(ei - fh) - b(di - fg) + c(dh - eg)
    a * (e * i_ - f * h) - b * (d * i_ - f * g) + c * (d * h - e * g)
}

/// Compute the Hamming weight (number of non-zero trits) of a vector.
pub fn hamming_weight(v: &[Ternary]) -> usize {
    v.iter().filter(|t| t.is_nonzero()).count()
}

/// Ternary inner product (dot product) of two vectors.
pub fn dot(a: &[Ternary], b: &[Ternary]) -> Ternary {
    TritVector::new(a).dot(&TritVector::new(b))
}

/// Compute the sign (ternary) of a floating-point value.
pub fn ternary_sign(x: f64) -> Ternary {
    if x < -0.5 {
        Negative
    } else if x > 0.5 {
        Positive
    } else {
        Neutral
    }
}

/// Convolution of two ternary sequences (mod 3).
pub fn convolve(a: &[Ternary], b: &[Ternary]) -> Vec<Ternary> {
    let n = a.len();
    let m = b.len();
    let mut result = vec![Neutral; n + m - 1];
    for i in 0..n {
        for j in 0..m {
            result[i + j] = result[i + j] + a[i] * b[j];
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_poly() {
        // P(x) = +1 + 0·x + (-1)·x²
        let coeffs = [Positive, Neutral, Negative];
        assert_eq!(eval_poly(&coeffs, Positive), Positive + -Positive * Positive + Neutral);
    }

    #[test]
    fn test_det_3x3_identity() {
        let eye: [[Ternary; 3]; 3] = [
            [Positive, Neutral, Neutral],
            [Neutral, Positive, Neutral],
            [Neutral, Neutral, Positive],
        ];
        assert_eq!(det_3x3(&eye), Positive);
    }

    #[test]
    fn test_det_3x3_zero_row() {
        let m: [[Ternary; 3]; 3] = [
            [Positive, Neutral, Neutral],
            [Neutral, Positive, Neutral],
            [Neutral, Neutral, Neutral],
        ];
        assert_eq!(det_3x3(&m), Neutral);
    }

    #[test]
    fn test_hamming_weight() {
        let v = [Positive, Neutral, Negative, Neutral];
        assert_eq!(hamming_weight(&v), 2);
    }

    #[test]
    fn test_ternary_sign() {
        assert_eq!(ternary_sign(1.5), Positive);
        assert_eq!(ternary_sign(0.0), Neutral);
        assert_eq!(ternary_sign(-2.0), Negative);
    }

    #[test]
    fn test_convolve() {
        let a = [Positive, Positive];
        let b = [Negative, Positive];
        // (1 + 1x)(-1 + 1x) = -1 + 0x + 1x²
        let c = convolve(&a, &b);
        assert_eq!(c.len(), 3);
        assert_eq!(c[0], Negative);
        assert_eq!(c[1], Neutral);
        assert_eq!(c[2], Positive);
    }
}
