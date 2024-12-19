use heapless::Vec; // Fixed-size vector
use crate::animator::animator::{Animate, Animator};
// For specifying vector capacity

const MAX_ANIMATIONS: usize = 8;

pub struct AnimationManager {
    animators: Vec<Animator, MAX_ANIMATIONS>,
}

impl AnimationManager{
    pub fn new() -> Self {
        Self {
            animators: Vec::new(),
        }
    }

    pub fn add(&mut self, animator: Animator) {
        let _ = self.animators.push(animator);
    }

    pub fn update<T: Animate>(&mut self, target: &mut T, delta_time: u32) {
        let mut i = 0;
        while i < self.animators.len() {
            let animator = &mut self.animators[i];
            animator.update(target, delta_time);
            if animator.is_complete() {
                self.animators.swap_remove(i);
            } else {
                i += 1;
            }
        }
    }
}
