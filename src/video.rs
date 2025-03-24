//! A video.

use image::{
    Rgb,
    RgbImage,
};

use indicatif::ProgressIterator;

use pyo3::prelude::*;

use crate::{
    Animation,
    Vector,
};

/// Type alias for an animation with its location and start/stop frames.
type Instance = (Animation, Vector, u32, u32);

#[pyclass]
/// A video, represented as a series of still frames.
pub struct Video {
    #[pyo3(get, set)]
    /// Video width (pixels).
    width: u32,

    #[pyo3(get, set)]
    /// Video height (pixels).
    height: u32,

    /// Background color (RGB).
    background: Rgb<u8>,

    #[pyo3(get, set)]
    /// Video frame rate (fps).
    fps: f64,

    #[pyo3(get, set)]
    /// Video duration (seconds).
    duration: f64,

    /// Video animations, combined with their location, start frame, and end frame.
    animations: Vec<Instance>,
}

#[pymethods]
impl Video {
    #[new]
    /// Construct a new video.
    pub fn new(
        size: (u32, u32),
        background: [u8; 3],
        fps: f64,
        duration: f64,
    ) -> Self {
        Self {
            width: size.0,
            height: size.1,
            background: Rgb (background),
            fps,
            duration,
            animations: Vec::new(),
        }
    }

    /// Add an animation to this video.
    /// 
    /// Note that `start` and `end` are given in seconds.  These are converted into
    /// frame numbers based on the FPS of the video.
    pub fn add(&mut self, animation: Animation, location: Vector, start: f64, end: f64) {
        // Frame numbers from timestamps
        let start_frame = (start * self.fps) as u32;
        let end_frame = (end * self.fps) as u32;

        self.animations.push((animation, location, start_frame, end_frame));
    }

    /// Render this video from a series of still frames.
    pub fn render(&self, location: String) {
        // How many frames?
        let frame_count = (self.duration * self.fps) as u32;

        for k in (0..frame_count).progress() {
            // New, empty frame
            let mut frame = RgbImage::new(self.width, self.height);

            // Create background
            for i in 0..self.width {
                for j in 0..self.height {
                    frame.put_pixel(i, j, self.background);
                }
            }

            for (animation, location, start, end) in &self.animations {
                // Determine progress of this animation
                let progress = (k as f64 - *start as f64) / (*end as f64 - *start as f64);

                if 0.0 <= progress && progress <= 1.0 {
                    // Transform the progress variable
                    let progress_transform = 0.5 - 0.5 * (progress * 3.141592653).cos();

                    // Construct visual artist from this animation
                    let artist = animation.0.play(progress_transform);

                    // Draw on this frame
                    artist.draw(*location, &mut frame);
                }
            }

            frame.save(format!("{}/frame_{:04}.png", location, k)).unwrap();
        }
    }
}