//! A shape make from Bezier curves.

use image::{
    Rgb,
    RgbImage,
};

use pyo3::prelude::*;

use crate::{
    Artist,
    Bezier,
    Vector,
};

#[pyclass]
#[derive(Clone)]
/// A shape made from Bezier curves.
pub struct Shape {
    /// Curves contained in this shape.
    curves: Vec<Bezier>,
    
    /// Origin of this shape.
    origin: Vector,

    /// Color of this shape.
    pub color: Rgb<u8>,
}

#[pymethods]
impl Shape {
    #[new]
    /// Construct a new shape of Bezier curves.
    pub fn new(curves: Vec<Bezier>, origin: Vector) -> Self {
        Self {
            color: curves[0].color,
            origin,
            curves,
        }
    }
}

impl Shape {
    /// Trace this shape of Bezier curves.
    pub fn trace(&self, t: f64) -> Vector {
        // No curve if no points :(
        if self.curves.len() == 0 {
            return Vector::zero();
        }

        // Which curve are we on?
        let idx = (t * self.curves.len() as f64).trunc() as usize;

        // Progress along this curve
        let t_curve = (t * self.curves.len() as f64).fract();

        self.curves[idx].trace(t_curve) + self.origin
    }
}

impl Artist for Shape {
    fn draw(&self, location: Vector, image: &mut RgbImage) {
        for curve in &self.curves {
            curve.draw(location, image);
        }
    }
}