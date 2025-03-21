//! Main library for the Beaver Studio.

mod animation;
mod artist;
mod bezier;
mod vector;
mod video;

use pyo3::prelude::*;

use animation::Animation;
use artist::Artist;
use bezier::Bezier;
use vector::Vector;
use video::Video;

/// Python interface for Beaver Studio.
#[pymodule]
fn beaverstudio(m: &Bound<'_, PyModule>) -> PyResult<()> {
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

    let v1 = Vector::new(-100.0, 100.0);
    let v2 = Vector::new(-100.0, -100.0);
    let v3 = Vector::new(100.0, -100.0);
    let v4 = Vector::new(100.0, 100.0);

    let curve = Bezier::new(vec![v1, v2, v3, v4]);

    // video.add(Box::new(curve));

    // Render the video
    video.render();
}