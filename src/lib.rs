//! Main library for the Beaver Studio.

mod bezier;
mod vector;
mod video;

use pyo3::prelude::*;

use bezier::Bezier;
use vector::Vector;
use video::Video;

/// Python interface for Beaver Studio.
#[pymodule]
fn beaverstudio(m: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}
