// use core::intrinsics::{cosf64, sinf64};

use constgebra::CMatrix;

use crate::{util::Duration, FrameCounter};

use super::transform::{Transform, Transformation};

#[derive(Debug)]
pub struct AnimationState {
    transformation: heapless::Vec<Transformation, 10>,
    deadline: u32,
    duration: Duration,
}

impl AnimationState {
    pub fn new(transformation: heapless::Vec<Transformation, 10>, duration: Duration) -> Self {
        Self {
            transformation,
            deadline: FrameCounter::get() + duration.as_frames(),
            duration,
        }
    }
    pub fn update(&self) -> CMatrix<3, 3> {
        if FrameCounter::get() >= self.deadline {
            return CMatrix::identity();
        }

        let elapsed = Duration::from_frames(self.duration.as_frames() - (self.deadline - FrameCounter::get()));
        let progress = (elapsed.as_secs() / self.duration.as_secs()).min(1.0) as f64;

        self.transformation.apply(progress)
    }
}
