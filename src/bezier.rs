//! Bezier curve implementation.

use crate::Vector;

/// Compute the binomial coefficient C(n,k).
fn binom(mut n: u32, k: u32) -> u32 {
    let mut output = 1;

    for _ in 0..k {
        output *= n;
        n -= 1;
    }

    for i in 2..=k {
        output /= i;
    }

    output
}

/// A generalized Bezier curve, constructed with a series of control points.
pub struct Bezier {
    /// Control points.
    points: Vec<Vector>,
}

impl Bezier {
    /// Construct a new Bezier curve.
    pub fn new(points: Vec<Vector>) -> Self {
        Self {
            points,
        }
    }

    /// Draw this Bezier curve.
    pub fn draw(&self, t: f64) -> Vector {
        // Zero vector
        let mut result = Vector::zero();

        // No curve if no points :(
        if self.points.len() == 0 {
            return result;
        }

        // Order of this Bezier curve
        let order = self.points.len() - 1;

        // Sum contributions from each control point
        for k in 0..self.points.len() {
            // Binomial coefficient
            let coeff = binom(order as u32, k as u32);

            // Add contribution
            result = result + self.points[k] * coeff * (1.0 - t).powi((order - k) as i32) * t.powi(k as i32);
        }

        result
    }
}