//! Main library for the Beaver Studio.

#![deny(warnings)]
#![deny(missing_docs)]

mod animation;
mod artist;
mod bezier;
mod shape;
mod interpolate;
mod rectangle;
mod vector;
mod video;

use pyo3::prelude::*;

use animation::{
    Animation,
    Animate,
};
use artist::Artist;
use bezier::Bezier;
use interpolate::Interpolate;
use rectangle::Rectangle;
use shape::Shape;
use vector::Vector;
use video::Video;

/// Python interface for Beaver Studio.
#[pymodule]
fn beaverstudio(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Add classes
    m.add_class::<Bezier>()?;
    m.add_class::<Interpolate>()?;
    m.add_class::<Rectangle>()?;
    m.add_class::<Shape>()?;
    m.add_class::<Vector>()?;
    m.add_class::<Video>()?;
    
    Ok(())
}