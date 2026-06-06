//! Analysis in ternary space.
//!
//! Discrete calculus, dynamics, and statistical operations over Z₃.

use ternary_types::Ternary::{self, Negative, Neutral, Positive};

/// Forward difference: Δf[n] = f[n+1] - f[n] (Z₃ subtraction).
pub fn forward_difference(seq: &[Ternary]) -> Vec<Ternary> {
    seq.windows(2).map(|w| w[1] - w[0]).collect()
}

/// Compute the running sum (cumulative integral) of a ternary sequence.
pub fn cumulative_sum(seq: &[Ternary]) -> Vec<Ternary> {
    let mut sum = Neutral;
    let mut result = Vec::with_capacity(seq.len());
    for &t in seq {
        sum = sum + t;
        result.push(sum);
    }
    result
}

/// Count how many times the sequence changes value between consecutive elements.
pub fn num_transitions(seq: &[Ternary]) -> usize {
    seq.windows(2).filter(|w| w[0] != w[1]).count()
}

/// Find local extrema in a ternary sequence.
/// A position is an extremum if the trit changes direction.
pub fn local_extrema(seq: &[Ternary]) -> Vec<usize> {
    if seq.len() < 3 {
        return Vec::new();
    }
    let diffs = forward_difference(seq);
    let mut extrema = Vec::new();
    for i in 1..diffs.len() {
        if diffs[i] != Neutral && diffs[i-1] != Neutral && diffs[i] != diffs[i-1] {
            extrema.push(i);
        }
    }
    extrema
}

/// Entropy of a ternary sequence (in bits).
pub fn ternary_entropy(seq: &[Ternary]) -> f64 {
    let n = seq.len() as f64;
    if n == 0.0 {
        return 0.0;
    }
    let neg_count = seq.iter().filter(|&&t| t == Negative).count() as f64;
    let zero_count = seq.iter().filter(|&&t| t == Neutral).count() as f64;
    let pos_count = seq.iter().filter(|&&t| t == Positive).count() as f64;

    let mut h = 0.0;
    for &c in &[neg_count, zero_count, pos_count] {
        if c > 0.0 {
            let p = c / n;
            h -= p * p.log2();
        }
    }
    h
}

/// Compute the autocorrelation of a ternary sequence at lag τ.
pub fn autocorrelation(seq: &[Ternary], lag: usize) -> f64 {
    if lag >= seq.len() {
        return 0.0;
    }
    let n = seq.len() - lag;
    let mut sum = 0;
    for i in 0..n {
        let a = i8::from(seq[i]);
        let b = i8::from(seq[i + lag]);
        sum += a * b;
    }
    sum as f64 / n as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forward_difference() {
        let seq = [Positive, Neutral, Positive, Negative];
        let diff = forward_difference(&seq);
        assert_eq!(diff.len(), 3);
        // Z₃: +1→0: -1 (Negative), 0→+1: +1 (Positive), +1→-1: -2 ≡ +1 (Positive)
        assert_eq!(diff[0], Negative);
        assert_eq!(diff[1], Positive);
        assert_eq!(diff[2], Positive);
    }

    #[test]
    fn test_num_transitions() {
        let seq = [Positive, Neutral, Neutral, Positive];
        assert_eq!(num_transitions(&seq), 2);
    }

    #[test]
    fn test_ternary_entropy() {
        // All same → entropy 0
        let seq = [Positive, Positive, Positive];
        assert!((ternary_entropy(&seq) - 0.0).abs() < 1e-10);

        // Uniform distribution over 3 values
        let seq = [Negative, Neutral, Positive];
        let h = ternary_entropy(&seq);
        assert!((h - 1.585).abs() < 0.01); // log2(3) ≈ 1.585
    }

    #[test]
    fn test_cumulative_sum() {
        let seq = [Positive, Negative, Positive];
        let cs = cumulative_sum(&seq);
        assert_eq!(cs[0], Positive);   // 1
        assert_eq!(cs[1], Neutral);    // 1 + -1 = 0
        assert_eq!(cs[2], Positive);   // 0 + +1 = +1
    }

    #[test]
    fn test_autocorrelation_lag0() {
        let seq = [Positive, Negative, Positive];
        let ac = autocorrelation(&seq, 0);
        // (1*1 + -1*-1 + 1*1) / 3 = 3/3 = 1
        assert!((ac - 1.0).abs() < 1e-10);
    }
}
