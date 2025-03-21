//! Main library for the Beaver Studio.

mod video;

use pyo3::prelude::*;

/// Python interface for Beaver Studio.
#[pymodule]
fn beaverstudio(m: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}
