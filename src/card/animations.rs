use crate::{
    animator::{
        animation_state::AnimationState,
        transform::{Rotate, Scale, Translate},
    },
    util::Duration,
};

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

pub fn hover_anim() -> AnimationState {
    AnimationState::new(
        &[Scale::new([1.3, 1.3], [1.0, 1.0]).into()],
        Duration::from_secs(0.1),
        Some(idle1),
    )
}
