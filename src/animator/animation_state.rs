// use core::intrinsics::{cosf64, sinf64};

use constgebra::CMatrix;

use crate::{util::Duration, FrameCounter};

use super::transform::{Transform, Transformation};

#[derive(Debug)]
pub struct AnimationState {
    transformation: heapless::Vec<Transformation, 4>,
    deadline: u32,
    duration: Duration,
    next: Option<fn() -> AnimationState>,
}

impl AnimationState {
    pub fn new(
        transformation: &[Transformation],
        duration: Duration,
        next: Option<fn () -> AnimationState>,
    ) -> Self {
        Self {
            transformation: heapless::Vec::from_slice(transformation).unwrap(),
            deadline: FrameCounter::get() + duration.as_frames(),
            duration,
            next,
        }
    }
    pub fn update(&self) -> Option<CMatrix<3, 3>> {
        if self.finished() {
            None
        } else {
            let elapsed = Duration::from_frames(self.duration.as_frames() - (self.deadline - FrameCounter::get()));
            let progress = (elapsed.as_secs() / self.duration.as_secs()).min(1.0) as f64;

            Some(self.transformation.apply(progress))
        }
    }
    pub fn finished(&self) -> bool {
        FrameCounter::get() >= self.deadline
    }
    pub fn get_next(&self) -> Option<AnimationState> {
        self.next.map(|f| f())
    }
}
