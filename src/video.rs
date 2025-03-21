//! A video.

use image::{
    Rgb,
    RgbImage,
};

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
        }
    }

    /// Render this video to a series of still frames.
    pub fn render(&self) {
        let frame_count = (self.duration * self.fps) as u32;

        for k in 0..frame_count {
            // New, empty frame
            let mut frame = RgbImage::new(self.width, self.height);

            for i in 0..self.width {
                for j in 0..self.height {
                    frame.put_pixel(i, j, self.background);
                }
            }

            frame.save(format!("frames/frame_{:04}.png", k));
        }
    }
}

#[test]
fn render_empty_video() {
    // Create an empty video
    let video = Video::new(
        (1920, 1080),
        [0, 0, 0],
        60.0,
        2.0,
    );

    // Render the video
    video.render();
}