use rand::{rngs::SmallRng, RngCore as _};

use crate::{
    animator::{
        animation_state::AnimationState,
        transform::{Rotate, Translate},
    },
    util::Duration,
};

pub fn random_idle(rng: &mut SmallRng) -> fn() -> AnimationState {
    match rng.next_u32() % 4 {
        0 => idle1,
        1 => idle2,
        2 => idle3,
        3 => idle4,
        _ => unreachable!(),
    }
}

pub fn idle1() -> AnimationState {
    AnimationState::new(&[
        Rotate::new(5.0, 0.0).into(),
        // Shear::new([-0.2, 0.0], [0.2, 0.0]).into(),
        Translate::new([-1.0, 1.0], [1.0, 1.0]).into(),
    ], Duration::from_secs(1.0), Some(idle2))
}
pub fn idle2() -> AnimationState {
    AnimationState::new(&[
        Rotate::new(0.0, -5.0).into(),
        // Shear::new([0.2, 0.0], [-0.2, 0.0]).into(),
        Translate::new([1.0, 1.0], [1.0, -1.0]).into(),
    ], Duration::from_secs(1.0), Some(idle3))
}
pub fn idle3() -> AnimationState {
    AnimationState::new(&[
        Rotate::new(-5.0, 0.0).into(),
        // Shear::new([0.2, 0.0], [-0.2, 0.0]).into(),
        Translate::new([1.0, -1.0], [-1.0, -1.0]).into(),
    ], Duration::from_secs(1.0), Some(idle4))
}
pub fn idle4() -> AnimationState {
    AnimationState::new(&[
        Rotate::new(0.0, 5.0).into(),
        // Shear::new([0.2, 0.0], [-0.2, 0.0]).into(),
        Translate::new([-1.0, -1.0], [-1.0, 1.0]).into(),
    ], Duration::from_secs(1.0), Some(idle1))
}
