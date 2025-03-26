//! An arbitrary polygon.

use pyo3::prelude::*;

use crate::{
    Bezier,
    Shape,
    Vector,
};

#[pyclass(extends=Shape)]
#[derive(Clone)]
/// A polygon.
pub struct Polygon;

#[pymethods]
impl Polygon {
    #[new]
    /// Construct a new polygon.
    pub fn new(points: Vec<Vector>, center: Vector, color: [u8; 3], thickness: i32) -> (Self, Shape) {
        // Bezier curves
        let mut curves = Vec::new();

        for i in 0..(points.len() - 1) {
            // Construct one side
            let curve = Bezier::new(
                vec![points[i], points[i + 1]],
                Vector::zero(),
                color,
                thickness,
            );
            curves.push(curve);
        }

        // Close polygon
        let curve = Bezier::new(
            vec![points[points.len() - 1], points[0]],
            Vector::zero(),
            color,
            thickness,
        );
        curves.push(curve);

        let shape = Shape::new(curves, center);

        (Self {}, shape)
    }
}