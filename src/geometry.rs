//! Geometry in ternary space.
//!
//! Provides geometric operations over Z₃: distances, transformations,
//! and lattice operations.

use ternary_types::Ternary::{self, Negative, Neutral, Positive};
/// Squared Euclidean distance between two ternary vectors.
///
/// Each trit difference contributes:
/// - 0 if equal
/// - 1 if one is 0 and the other is ±1
/// - 4 if opposite (-1 vs +1)
pub fn squared_distance(a: &[Ternary], b: &[Ternary]) -> usize {
    a.iter().zip(b.iter()).map(|(x, y)| {
        match (x, y) {
            (a, b) if a == b => 0,
            (Negative, Positive) | (Positive, Negative) => 4,
            _ => 1,
        }
    }).sum()
}

/// Manahattan (L1) distance between two ternary vectors.
pub fn manhattan_distance(a: &[Ternary], b: &[Ternary]) -> usize {
    a.iter().zip(b.iter()).map(|(x, y)| {
        let dx = i8::from(*x) - i8::from(*y);
        dx.unsigned_abs() as usize
    }).sum()
}

/// Chebyshev (L∞) distance between two ternary vectors.
pub fn chebyshev_distance(a: &[Ternary], b: &[Ternary]) -> usize {
    a.iter().zip(b.iter()).map(|(x, y)| {
        let dx = i8::from(*x) - i8::from(*y);
        dx.unsigned_abs() as usize
    }).max().unwrap_or(0)
}

/// Reflect a point through the origin: P → -P.
pub fn reflect(v: &[Ternary]) -> Vec<Ternary> {
    v.iter().map(|t| -(*t)).collect()
}

/// Rotate a 2D ternary point by 90° counter-clockwise.
///
/// In Z₃, a 90° rotation sends (x, y) → (-y, x).
pub fn rotate_90(v: (Ternary, Ternary)) -> (Ternary, Ternary) {
    (-v.1, v.0)
}

/// Generate all ternary points on a sphere of given radius (L∞ norm).
pub fn ternary_sphere(radius: usize) -> Vec<Vec<Ternary>> {
    let mut points = Vec::new();
    // For L∞ norm, each coordinate ∈ {-1, 0, +1} scaled by radius
    for &x in &[Negative, Neutral, Positive] {
        for &y in &[Negative, Neutral, Positive] {
            for &z in &[Negative, Neutral, Positive] {
                let coords = vec![x, y, z];
                let norm: usize = coords.iter().map(|t| i8::from(*t).unsigned_abs() as usize).max().unwrap();
                if norm == radius {
                    points.push(coords);
                }
            }
        }
    }
    points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_squared_distance() {
        let a = [Positive, Negative, Neutral];
        let b = [Positive, Positive, Neutral];
        // 0 + 4 + 0 = 4
        assert_eq!(squared_distance(&a, &b), 4);
    }

    #[test]
    fn test_manhattan_distance() {
        let a = [Positive, Negative, Neutral];
        let b = [Neutral, Negative, Positive];
        // |1-0| + |-1-(-1)| + |0-1| = 1 + 0 + 1 = 2
        assert_eq!(manhattan_distance(&a, &b), 2);
    }

    #[test]
    fn test_reflect() {
        let v = [Positive, Negative, Neutral];
        let r = reflect(&v);
        assert_eq!(r, vec![Negative, Positive, Neutral]);
    }

    #[test]
    fn test_rotate_90() {
        let (x, y) = (Positive, Negative);
        let (rx, ry) = rotate_90((x, y));
        assert_eq!(rx, -y); // -(-1) = +1 → Negative?
        assert_eq!(ry, x);  // +1 = Positive
    }

    #[test]
    fn test_ternary_sphere() {
        let sphere = ternary_sphere(1);
        // All 3³ = 27 points minus the origin (0,0,0) = 26
        assert_eq!(sphere.len(), 26);
    }

    #[test]
    fn test_chebyshev_distance() {
        let a = [Positive, Neutral, Neutral];
        let b = [Negative, Neutral, Positive];
        // max(|1-(-1)|, |0-0|, |0-1|) = max(2, 0, 1) = 2
        assert_eq!(chebyshev_distance(&a, &b), 2);
    }
}
