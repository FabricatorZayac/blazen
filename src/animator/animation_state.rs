use core::intrinsics::{cosf64, sinf64};

use constgebra::CMatrix;

use crate::util::{Angle, Duration};

#[derive(Debug)]
pub enum Transform {
    Rotate(Angle, Angle),
    Translate([f64; 2]),
    Scale(f64),
}

#[derive(Debug)]
pub enum AnimationState {
    InProgress {
        transformation: Transform,
        duration: Duration,
        elapsed: Duration,
    },
    Done,
}

impl AnimationState {
    pub fn new(transformation: Transform, duration: Duration) -> Self {
        Self::InProgress {
            transformation,
            duration,
            elapsed: Duration::from_frames(0),
        }
    }
    pub fn update(&mut self) -> CMatrix<3, 3> {
        match self {
            AnimationState::InProgress { transformation, duration, elapsed } => {
                elapsed.step();
                if elapsed >= duration {
                    *self = AnimationState::Done;
                    return CMatrix::identity();
                }

                let progress = (elapsed.as_secs() / duration.as_secs()).min(1.0) as f64;

                match transformation {
                    Transform::Rotate(start, end) => {
                        let current_rotation = lerp(start.as_rad(), end.as_rad(), progress);
                        CMatrix::new(unsafe { [
                            [ cosf64(current_rotation), sinf64(current_rotation), 0.0],
                            [-sinf64(current_rotation), cosf64(current_rotation), 0.0],
                            [                      0.0,                      0.0, 1.0],
                        ] })
                    },
                    Transform::Translate(vec) => {
                        let per_frame = vec.map(|c| lerp(0.0, c, progress));
                        CMatrix::new([
                            [         1.0,          0.0, 0.0],
                            [         0.0,          1.0, 0.0],
                            [per_frame[0], per_frame[1], 1.0],
                        ])
                    }
                    Transform::Scale(scale) => {
                        let scale_this_frame = lerp(1.0, *scale, progress);
                        CMatrix::new([
                            [scale_this_frame, 0.0, 0.0],
                            [0.0, scale_this_frame, 0.0],
                            [0.0,              0.0, 1.0],
                        ])
                    }
                }


            },
            AnimationState::Done => {
                CMatrix::identity()
            },
        }
    }
}
fn lerp(start: f64, end: f64, t: f64) -> f64 {
    start + (end - start) * t
}
