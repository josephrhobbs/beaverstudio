//! Interpolation between two Bezier curves.

use image::{
    Rgb,
    RgbImage,
};

use crate::{
    Animation,
    Artist,
    Bezier,
    Vector,
};

/// An interpolation animation, where one Bezier curve smoothly becomes another.
pub struct Interpolate {
    one: Bezier,
    two: Bezier,
}

impl Interpolate {
    /// Construct a new interpolation.
    pub fn new(one: Bezier, two: Bezier) -> Self {
        Self {
            one,
            two,
        }
    }
}

impl Animation for Interpolate {
    fn play(&self, progress: f64) -> Box<dyn Artist> {
        Box::new(InterpolatedCurve::new(self.one.clone(), self.two.clone(), progress))
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

            // Convert to pixels
            let (x, y) = trace.to_pixels(
                image.width(),
                image.height(),
            );
            image.put_pixel(x, y, Rgb([255, 255, 255]));

            // Step along the curve
            t += 0.0001;
        }
    }
}