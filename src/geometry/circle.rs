//! A circle.

use pyo3::prelude::*;

use crate::{
    Bezier,
    Shape,
    Vector,
};

/// Magic number for a circle, relating the positions of Bezier control points.
pub const MAGIC: f64 = 0.552284749831;

#[pyclass(extends=Shape)]
#[derive(Clone)]
/// A circle.
pub struct Circle;

#[pymethods]
impl Circle {
    #[new]
    /// Construct a new circle.
    pub fn pynew(center: Vector, radius: f64, color: [u8; 3], thickness: i32) -> (Self, Shape) {
        // Magic steps
        let xstep = Vector::new(radius*MAGIC, 0.0);
        let ystep = Vector::new(0.0, radius*MAGIC);

        // Radius steps
        let xrad = Vector::new(radius, 0.0);
        let yrad = Vector::new(0.0, radius);

        // Shape
        let shape = Shape::new(vec![
            Bezier::new(vec![xrad, xrad + ystep, yrad + xstep, yrad], Vector::zero(), color, thickness),
            Bezier::new(vec![yrad, yrad - xstep, -xrad + ystep, -xrad], Vector::zero(), color, thickness),
            Bezier::new(vec![-xrad, -xrad - ystep, -yrad - xstep, -yrad], Vector::zero(), color, thickness),
            Bezier::new(vec![-yrad, -yrad + xstep, xrad - ystep, xrad], Vector::zero(), color, thickness),
        ], center);

        (Self {}, shape)
    }
}