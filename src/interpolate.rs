//! Interpolation between two Bezier curves.

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
    Vector,
};

#[pyclass]
#[derive(Clone)]
/// An interpolation animation, where one Bezier curve smoothly becomes another.
pub struct Interpolate {
    one: Bezier,
    two: Bezier,
}

#[pymethods]
impl Interpolate {
    #[new]
    /// Construct a new interpolation.
    pub fn new(one: Bezier, two: Bezier) -> Self {
        Self {
            one,
            two,
        }
    }

    /// Construct an animation from this interpolation.
    pub fn animate(&self) -> Animation {
        Animate::animate(self)
    }
}

impl Animate for Interpolate {
    fn play(&self, progress: f64) -> Box<dyn Artist> {
        Box::new(InterpolatedCurve::new(self.one.clone(), self.two.clone(), progress))
    }

    fn clone_box(&self) -> Box<dyn Animate> {
        Box::new(self.clone())
    }
}

/// A linear interpolation between two Bezier curves.
pub struct InterpolatedCurve {
    one: Bezier,
    two: Bezier,
    progress: f64,
}

impl InterpolatedCurve {
    /// Construct a new interpolated curve.
    pub fn new(one: Bezier, two: Bezier, progress: f64) -> Self {
        Self {
            one,
            two,
            progress,
        }
    }
}

impl Artist for InterpolatedCurve {
    fn draw(&self, location: Vector, image: &mut RgbImage) {
        let mut t = 0.0;

        while t <= 1.0 {
            // Calculate offset from location
            let trace1 = self.one.trace(t) + location;
            let trace2 = self.two.trace(t) + location;

            // Compute weighted average
            let trace = trace1 * (1.0 - self.progress) + trace2 * self.progress;

            // Interpolate colors
            // TODO this is a sloppy interpolation, can it be better?
            let (r, g, b): (f64, f64, f64) = (
                (self.one.color.0[0] as f64) * (1.0 - self.progress) + (self.two.color.0[0] as f64) * self.progress,
                (self.one.color.0[1] as f64) * (1.0 - self.progress) + (self.two.color.0[1] as f64) * self.progress,
                (self.one.color.0[2] as f64) * (1.0 - self.progress) + (self.two.color.0[2] as f64) * self.progress,
            );

            // Convert to pixels
            let (x, y) = trace.to_pixels(
                image.width(),
                image.height(),
            );
            image.put_pixel(x, y, Rgb([r as u8, g as u8, b as u8]));

            // Step along the curve
            t += 0.0001;
        }
    }
}