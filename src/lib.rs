//! Main library for the Beaver Studio.

#![deny(warnings)]
#![deny(missing_docs)]

mod animation;
mod artist;
mod bezier;
mod interpolate;
mod vector;
mod video;

use pyo3::prelude::*;

use animation::Animation;
use artist::Artist;
use bezier::Bezier;
use interpolate::Interpolate;
use vector::Vector;
use video::Video;

/// Python interface for Beaver Studio.
#[pymodule]
fn beaverstudio(_m: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}

#[test]
fn render_video() {
    // Create an empty video
    let mut video = Video::new(
        (1920, 1080),
        [0, 0, 0],
        60.0,
        2.0,
    );

    // First Bezier curve
    let v1 = Vector::new(-100.0, 100.0);
    let v2 = Vector::new(-100.0, -100.0);
    let v3 = Vector::new(100.0, -100.0);
    let v4 = Vector::new(100.0, 100.0);
    let curve1 = Bezier::new(vec![v1, v2, v3, v4], Vector::zero());

    // Second Bezier curve
    let v1 = Vector::new(-100.0, -100.0);
    let v2 = Vector::new(-100.0, 100.0);
    let v3 = Vector::new(100.0, 100.0);
    let v4 = Vector::new(100.0, -100.0);
    let curve2 = Bezier::new(vec![v1, v2, v3, v4], Vector::new(100.0, 0.0));

    let interp = Interpolate::new(curve1, curve2);

    video.add(Box::new(interp), Vector::zero(), 0.0, 2.0);

    // Render the video
    video.render();
}