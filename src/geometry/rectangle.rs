//! A rectangle.

use pyo3::prelude::*;

use crate::{
    Bezier,
    Shape,
    Vector,
};

#[pyclass(extends=Shape)]
#[pyo3(name = "Rect")]
#[derive(Clone)]
/// A rectangle.
pub struct Rectangle;

#[pymethods]
impl Rectangle {
    #[new]
    /// Construct a new rectangle.
    pub fn pynew(center: Vector, width: f64, height: f64, color: [u8; 3], thickness: i32) -> (Self, Shape) {
        // Half-sides
        let xside = Vector::new(0.5*width, 0.0);
        let yside = Vector::new(0.0, 0.5*height);

        // New shape
        let shape = Shape::new(vec![
            Bezier::new(vec![xside + yside, -xside + yside], Vector::zero(), color, thickness),
            Bezier::new(vec![-xside + yside, -xside - yside], Vector::zero(), color, thickness),
            Bezier::new(vec![-xside - yside, xside - yside], Vector::zero(), color, thickness),
            Bezier::new(vec![xside - yside, xside + yside], Vector::zero(), color, thickness),
        ], center);

        (Self {}, shape)
    }
}