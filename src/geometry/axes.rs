//! Coordinate axes.

use crate::{
    Vector,
};

#[pyclass]
pub struct Axes {

}

#[pymethods]
impl Axes {
    #[new]
    /// Create a set of coordinate axes with a specified origin.
    pub fn new(origin: Vector) -> Self {
        
    }
}