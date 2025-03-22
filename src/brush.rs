//! A brush of a given thickness.

#[derive(Clone)]
/// A brush that draws a line or curve.
pub struct Brush {
    pub points: Vec<(i32, i32)>,
}

impl Brush {
    pub fn new(thickness: i32) -> Self {
        // Curve "brush"
        let mut points = Vec::new();

        // See if points are in brush
        for i in -thickness..thickness {
            for j in -thickness..thickness {
                if 4 * i.pow(2) + 4 * j.pow(2) <= thickness.pow(2) {
                    points.push((i, j));
                }
            }
        }

        Self {
            points,
        }
    }
}