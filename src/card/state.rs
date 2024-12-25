use constgebra::CMatrix;
use wasm4::control::MouseState;

use crate::animator::transform::{Rotate, Scale, Shear, Translate};
use crate::MouseSemaphore;
use crate::{animator::animation_state::AnimationState, util::Duration};
use crate::gfx::Vectorize;

use super::{card::Card, view::CardView};

pub struct CardState {
    card: Card,
    origin: [i32; 2],
    diff_vecs: [[f64; 2]; 4],

    animation: Option<AnimationState>,
}

impl CardState {
    pub fn new(card: Card, origin: [i32; 2], animation: Option<AnimationState>) -> Self {
        Self {
            card,
            origin,
            diff_vecs: [
                [-16.0, -21.0],
                [ 16.0, -21.0],
                [ 16.0,  21.0],
                [-16.0,  21.0],
            ],
            animation,
        }
    }
    pub fn view(&self) -> CardView {
        let transform = self.animate();
        let vertices = self.apply_transform(transform).map(Vectorize::vectorize);
        let origin = self.origin.map(|i| i as f64).vectorize();
        CardView::new(
            self.card.texture(),
            vertices
                .map(|vertex| vertex.add(origin))
                .map(Vectorize::devectorize)
                .map(|vertex: [f64; 2]| vertex.map(|f| f as i32))
        )
    }
    fn animate(&self) -> CMatrix<3, 3> {
        self.animation
            .as_ref()
            .map_or(Some(CMatrix::identity()), AnimationState::update)
            .unwrap_or(CMatrix::identity())
    }
    fn apply_transform(&self, matrix: CMatrix<3, 3>) -> [[f64; 2]; 4] {
        self.diff_vecs
            .map(Vectorize::vectorize)
            .map(|vec| vec.mul(matrix))
            .map(Vectorize::devectorize)
    }
    fn is_hovered(&self, mouse: &mut MouseSemaphore) -> bool {
        if mouse.lock {
            return false;
        }

        let m = mouse.state().unwrap();

        let left   = (m.x as i32) > self.origin[0] - 15;
        let right  = (m.x as i32) < self.origin[0] + 15;
        let bottom = (m.y as i32) > self.origin[1] - 20;
        let top    = (m.y as i32) < self.origin[1] + 20;
        left && right && bottom && top
    }
}

impl CardState {
    pub fn set_animation(&mut self, animation: AnimationState) {
        self.animation = Some(animation);
    }
    pub fn update(&mut self) {
        // default card animation
        if self.animation.is_none() {
            self.set_animation(anim1());
        }

        // replace animation when it ends
        let mut next: Option<AnimationState> = None;
        if let Some(anim) = &self.animation {
            if anim.finished() {
                next = anim.get_next()
            }
        }
        if let Some(anim) = next {
            self.set_animation(anim);
        }
    }
    pub fn handle_input(&mut self, mouse: &mut MouseSemaphore) {
        if mouse.state().is_none() {
            return;
        }
        if self.is_hovered(mouse) {
            let m = mouse.state().unwrap();
            mouse.lock();
            self.set_animation(hover_anim());

            if m.buttons.left {
                self.origin = [m.x as i32, m.y as i32];
            }
        }
    }
}

fn hover_anim() -> AnimationState {
    AnimationState::new(
        &[Scale::new([1.8, 1.8], [1.0, 1.0]).into()],
        Duration::from_secs(0.1),
        Some(anim1),
    )
}

fn anim1() -> AnimationState {
    AnimationState::new(&[
        Rotate::new(-10.0, 10.0).into(),
        Shear::new([-0.2, 0.0], [0.2, 0.0]).into(),
        Translate::new([-2.0, -2.0], [2.0, 2.0]).into(),
    ], Duration::from_secs(2.0), Some(anim2))
}

fn anim2() -> AnimationState {
    AnimationState::new(&[
        Rotate::new(10.0, -10.0).into(),
        Shear::new([0.2, 0.0], [-0.2, 0.0]).into(),
        Translate::new([2.0, 2.0], [-2.0, -2.0]).into(),
    ], Duration::from_secs(2.0), Some(anim1))
}
