//! Coordinate axes.

use crate::{
    Bezier,
    Shape,
    Vector,
};

use pyo3::prelude::*;

/// Color for major gridlines.
const MAJOR_COLOR: [u8; 3] = [255, 255, 255];

/// Color for minor gridlines.
const MINOR_COLOR: [u8; 3] = [160, 160, 160];

/// Thickness for major gridlines.
const MAJOR_THICKNESS: i32 = 4;

/// Thickness for minor gridlines.
const MINOR_THICKNESS: i32 = 2;

#[pyclass]
#[derive(Clone)]
/// Linear-linear coordinate axes.
pub struct LinearAxes (Shape);

#[pymethods]
impl LinearAxes {
    #[new]
    /// Create a set of coordinate axes with a specified origin, line spacing, and line count.
    pub fn new(origin: Vector, spacing: f64, x_count: (usize, usize), y_count: (usize, usize)) -> Self {
        // Minimum/maximum X and Y values
        let mut x_min = origin.x;
        let mut x_max = origin.x;
        let mut y_min = origin.y;
        let mut y_max = origin.y;

        // Current X value
        let mut x = origin.x;

        // X values to draw minor gridlines at
        let mut x_vals = Vec::new();

        // Draw positive X lines
        for _ in 0..x_count.1 {
            x += spacing;
            x_vals.push(x);
            x_max = x_max.max(x);
        }

        // Draw negative X lines
        x = origin.x;
        for _ in 0..x_count.0 {
            x -= spacing;
            x_vals.push(x);
            x_min = x_min.min(x);
        }

        // Current Y value
        let mut y = origin.y;

        // Y values to draw minor gridlines at
        let mut y_vals = Vec::new();

        // Draw positive Y lines
        for _ in 0..y_count.1 {
            y += spacing;
            y_vals.push(y);
            y_max = y_max.max(y);
        }

        // Draw negative Y lines
        y = origin.y;
        for _ in 0..y_count.0 {
            y -= spacing;
            y_vals.push(y);
            y_min = y_min.min(y);
        }

        // Resultant Bezier curves
        let mut curves = Vec::new();

        // Construct Bezier curves for each X minor gridline
        for x_val in x_vals {
            let gridline = Bezier::new(
                vec![Vector::new(x_val, y_min), Vector::new(x_val, y_max)],
                Vector::zero(),
                MINOR_COLOR,
                MINOR_THICKNESS,
            );

            curves.push(gridline);
        }

        // Construct Bezier curves for each Y minor gridline
        for y_val in y_vals {
            let gridline = Bezier::new(
                vec![Vector::new(x_min, y_val), Vector::new(x_max, y_val)],
                Vector::zero(),
                MINOR_COLOR,
                MINOR_THICKNESS,
            );

            curves.push(gridline);
        }

        // Construct major gridlines
        let x_major = Bezier::new(
            vec![Vector::new(origin.x, y_min), Vector::new(origin.x, y_max)],
            Vector::zero(),
            MAJOR_COLOR,
            MAJOR_THICKNESS,
        );
        let y_major = Bezier::new(
            vec![Vector::new(x_min, origin.y), Vector::new(x_max, origin.y)],
            Vector::zero(),
            MAJOR_COLOR,
            MAJOR_THICKNESS,
        );
        curves.push(x_major);
        curves.push(y_major);

        Self (Shape::new(curves, Vector::zero()))
    }

    #[getter]
    /// Extract the chain of Bezier curves.
    pub fn get_shape(&self) -> Shape {
        self.0.clone()
    }
}