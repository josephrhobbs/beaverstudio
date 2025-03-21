//! An image artist.

use image::RgbImage;

/// An image artist.
/// 
/// Image artists create visuals on a given frame.
pub trait Artist {
    /// Draw on the given frame.
    fn draw(&self, image: &mut RgbImage);
}