//! Interpolation between two Bezier curves.

use image::{
    Rgb,
    RgbImage,
};

use crate::{
    Animation,
    Bezier,
};

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

impl Artist for Interpolate {
    fn draw(&self, progress: f64) -> 
}