//! A video animation.

use crate::Artist;

/// A video animation.
/// 
/// Video animations are controlled by their `progress` variable.  Given a progress value,
/// they return an image artist that is capable of modifying a provided frame.
pub trait Animation {
    fn play(&self, progress: f64) -> Box<dyn Artist>;
}