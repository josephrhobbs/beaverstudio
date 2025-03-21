//! Bezier curve implementation.

use image::{
    Rgb,
    RgbImage,
};

use pyo3::prelude::*;

use crate::{
    Artist,
    Shape,
    Vector,
};

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

#[pyclass]
#[derive(Clone)]
/// A Bezier curve of arbitrary order, constructed with a series of control points.
pub struct Bezier {
    /// Origin.
    origin: Vector,

    /// Color (RGB).
    pub color: Rgb<u8>,

    /// Control points, relative to the origin.
    points: Vec<Vector>,
}

#[pymethods]
impl Bezier {
    #[new]
    /// Construct a new Bezier curve, given control points and an origin.
    /// 
    /// Note that the control points are *relative* to the given origin.
    pub fn new(points: Vec<Vector>, origin: Vector, color: [u8; 3]) -> Self {
        Self {
            points,
            origin,
            color: Rgb (color),
        }
    }

    #[getter]
    /// Turn this Bezier curve into a shape.
    pub fn get_shape(&self) -> Shape {
        Shape::new(vec![self.clone()], Vector::zero())
    }
}

impl Bezier {
    /// Trace this Bezier curve.
    pub fn trace(&self, t: f64) -> Vector {
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
            result = result + (self.points[k] + self.origin) * coeff * (1.0 - t).powi((order - k) as i32) * t.powi(k as i32);
        }

        result
    }
}

impl Artist for Bezier {
    fn draw(&self, location: Vector, image: &mut RgbImage) {
        let mut t = 0.0;

        while t <= 1.0 {
            // Calculate offset from location
            let trace = self.trace(t) + location;

            // Convert to pixels
            let (x, y) = trace.to_pixels(
                image.width(),
                image.height(),
            );
            image.put_pixel(x, y, self.color);

            // Step along the curve
            t += 0.0001;
        }
    }
}