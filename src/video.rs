//! A video.

use image::{
    Rgb,
    RgbImage,
};

use crate::Animation;

/// A video, represented as a series of still frames.
pub struct Video {
    /// Video width (pixels).
    width: u32,

    /// Video height (pixels).
    height: u32,

    /// Background color (RGB).
    background: Rgb<u8>,

    /// Video frame rate.
    fps: f64,

    /// Video duration (seconds).
    duration: f64,

    /// Video animations, combined with start and end frames.
    animations: Vec<(Box<dyn Animation>, u32, u32)>,
}

impl Video {
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
    pub fn add(&mut self, animation: Box<dyn Animation>, start: f64, end: f64) {
        // Frame numbers from timestamps
        let start_frame = (start * self.fps) as u32;
        let end_frame = (end * self.fps) as u32;

        self.animations.push((animation, start_frame, end_frame));
    }

    /// Render this video into a series of still frames.
    pub fn render(&self) {
        let frame_count = (self.duration * self.fps) as u32;

        for k in 0..frame_count {
            // New, empty frame
            let mut frame = RgbImage::new(self.width, self.height);

            // Create background
            for i in 0..self.width {
                for j in 0..self.height {
                    frame.put_pixel(i, j, self.background);
                }
            }

            for (anim, start, end) in &self.animations {
                // Determine progress of this animation
                let progress = (k as f64 - *start as f64) / (*end as f64 - *start as f64);

                // Transform the progress variable
                let progress_transform = 0.5 - 0.5 * (progress * 3.141592653).cos();

                // Construct visual artist from this animation
                let artist = anim.play(progress_transform);
            }

            frame.save(format!("frames/frame_{:04}.png", k)).unwrap();
        }
    }
}