//! Trace a shape.

use image::RgbImage;

use crate::{
    Artist,
    Animate,
    Bresenham,
    Brush,
    Shape,
    STEP,
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
        // Build collection of points to interpolate between
        let mut t = 0.0;
        let mut points = Vec::new();

        while t <= self.progress {
            // Step along the curve
            t += STEP;

            // Save this point
            points.push(location + self.shape.trace(t));
        }

        // Brush to draw with
        let brush = Brush::new(self.shape.thickness);

        // Interpolation (Bresenham's line algorithm)
        for i in 0..(points.len() - 1) {
            // Two points to draw between
            let this_point = points[i];
            let next_point = points[i + 1];

            // Convert points to integers
            let (x0, y0) = this_point.to_pixels(image.width(), image.height());
            let (x1, y1) = next_point.to_pixels(image.width(), image.height());

            // Construct Bresenham line
            let line = Bresenham::new(x0, y0, x1, y1).points;

            // Draw first point
            for (x, y) in line {
                for (i, j) in &brush.points {
                    image.put_pixel((x as i32 + i) as u32, (y as i32 + j) as u32, self.shape.color);
                }
            }
        }
    }
}