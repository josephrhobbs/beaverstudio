//! A shape make from Bezier curves.

use image::{
    Rgb,
    RgbImage,
};

use pyo3::prelude::*;

use crate::{
    Animate,
    Animation,
    Artist,
    Bezier,
    Interpolate,
    Trace,
    Vector,
};

#[pyclass]
#[derive(Clone)]
/// A shape made from Bezier curves.
pub struct Shape {
    /// Curves contained in this shape.
    curves: Vec<Bezier>,

    /// Thickness of this shape.
    pub thickness: i32,
    
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
            thickness: curves[0].thickness,
            origin,
            curves,
        }
    }

    #[getter]
    /// Construct a (static) animation from this shape.
    pub fn get_display(&self) -> Animation {
        Animate::animate(self)
    }

    /// Interpolate this shape with another.
    pub fn into(&self, other: Shape) -> Animation {
        Interpolate::new(self.clone(), other).animate()
    }

    #[getter]
    /// Construct a tracing animation from this shape.
    pub fn get_trace(&self) -> Animation {
        Trace::new(self.clone()).animate()
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

        self.origin + self.curves[idx].trace(t_curve)
    }
}

impl Artist for Shape {
    fn draw(&self, location: Vector, image: &mut RgbImage) {
        for curve in &self.curves {
            curve.draw(self.origin + location, image);
        }
    }
}

impl Animate for Shape {
    fn play(&self, _: f64) -> Box<dyn Artist> {
        Box::new(self.clone())
    }

    fn clone_box(&self) -> Box<dyn Animate> {
        Box::new(self.clone())
    }
}