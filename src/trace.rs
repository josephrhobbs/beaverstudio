//! Trace a shape.

use image::RgbImage;

use crate::{
    Artist,
    Animate,
    Shape,
    Vector,
};

#[derive(Clone)]
/// An animation that traces the outline of a shape, drawing it over time.
pub struct Trace {
    /// Shape to be traced.
    shape: Shape,
}

impl Trace {
    /// Construct a new tracing animation.
    pub fn new(shape: Shape) -> Self {
        Self {
            shape,
        }
    }
}

impl Animate for Trace {
    fn play(&self, progress: f64) -> Box<dyn Artist> {
        Box::new(TracedShape::new(self.shape.clone(), progress))
    }

    fn clone_box(&self) -> Box<dyn Animate> {
        Box::new(self.clone())
    }
}

/// A shape that is partially traced.
pub struct TracedShape {
    /// Shape to be traced.
    shape: Shape,

    /// Amount of progress this tracing has made.
    progress: f64,
}

impl TracedShape {
    /// Construct a new shape to be traced.
    pub fn new(shape: Shape, progress: f64) -> Self {
        Self {
            shape,
            progress,
        }
    }
}

impl Artist for TracedShape {
    fn draw(&self, location: Vector, image: &mut RgbImage) {
        let mut t = 0.0;

        while t <= self.progress {
            // Calculate offset from location
            let trace = self.shape.trace(t) + location;

            // Convert to pixels
            let (x, y) = trace.to_pixels(
                image.width(),
                image.height(),
            );
            image.put_pixel(x, y, self.shape.color);

            // Step along the curve
            t += 0.0001;
        }
    }
}