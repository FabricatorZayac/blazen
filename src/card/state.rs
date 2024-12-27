use constgebra::CMatrix;

use crate::animator::transform::{Rotate, Scale, Translate};
use crate::MouseSemaphore;
use crate::{animator::animation_state::AnimationState, util::Duration};
use crate::gfx::{Render, Triangle, TriangleFill, Vectorize};

use super::card::Card;

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
    fn is_hovered(&self, mouse: &MouseSemaphore) -> bool {
        match mouse.state() {
            Some(m) => {
                let left   = (m.x as i32) > self.origin[0] - 16;
                let right  = (m.x as i32) < self.origin[0] + 16;
                let bottom = (m.y as i32) > self.origin[1] - 21;
                let top    = (m.y as i32) < self.origin[1] + 21;
                left && right && bottom && top
            }
            None => false,
        }
    }
}

impl CardState {
    pub fn set_animation(&mut self, animation: AnimationState) {
        self.animation = Some(animation);
    }
    pub fn update(&mut self) {
        // default card animation
        if self.animation.is_none() {
            self.set_animation(idle1());
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
    pub fn handle_input(&mut self, mouse: &MouseSemaphore) {
        if mouse.state().is_none() {
            return;
        }
        if self.is_hovered(mouse) {
            // let m = mouse.state().unwrap();
            mouse.lock();
            self.set_animation(hover_anim());

            // if m.buttons.left {
            //     self.origin = [m.x as i32, m.y as i32];
            // }
        }
    }
}

impl Render for CardState {
    fn render(&self, fb: &wasm4::draw::Framebuffer) {
        let transform = self.animate();
        let origin = self.origin.map(|i| i as f64).vectorize();
        let texture = self.card.texture();
        let vertices = self.apply_transform(transform)
            .map(Vectorize::vectorize)
            .map(|vertex| vertex.add(origin))
            .map(Vectorize::devectorize)
            .map(|vertex: [f64; 2]| vertex.map(|f| f as i32));

        let t1 = Triangle {
            vertices: [vertices[0], vertices[1], vertices[3]],
            fill: TriangleFill::Texture(texture[0]),
        };
        let t2 = Triangle {
            vertices: [vertices[1], vertices[2], vertices[3]],
            fill: TriangleFill::Texture(texture[1]),
        };

        t1.render(fb);
        t2.render(fb);
    }
}

fn hover_anim() -> AnimationState {
    AnimationState::new(
        &[Scale::new([1.3, 1.3], [1.0, 1.0]).into()],
        Duration::from_secs(0.1),
        Some(idle1),
    )
}

pub fn idle1() -> AnimationState {
    AnimationState::new(&[
        Rotate::new(5.0, 0.0).into(),
        // Shear::new([-0.2, 0.0], [0.2, 0.0]).into(),
        Translate::new([-1.0, 1.0], [1.0, 1.0]).into(),
    ], Duration::from_secs(1.0), Some(idle2))
}

fn idle2() -> AnimationState {
    AnimationState::new(&[
        Rotate::new(0.0, -5.0).into(),
        // Shear::new([0.2, 0.0], [-0.2, 0.0]).into(),
        Translate::new([1.0, 1.0], [1.0, -1.0]).into(),
    ], Duration::from_secs(1.0), Some(idle3))
}

fn idle3() -> AnimationState {
    AnimationState::new(&[
        Rotate::new(-5.0, 0.0).into(),
        // Shear::new([0.2, 0.0], [-0.2, 0.0]).into(),
        Translate::new([1.0, -1.0], [-1.0, -1.0]).into(),
    ], Duration::from_secs(1.0), Some(idle4))
}

fn idle4() -> AnimationState {
    AnimationState::new(&[
        Rotate::new(0.0, 5.0).into(),
        // Shear::new([0.2, 0.0], [-0.2, 0.0]).into(),
        Translate::new([-1.0, -1.0], [-1.0, 1.0]).into(),
    ], Duration::from_secs(1.0), Some(idle1))
}
