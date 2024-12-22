use constgebra::CMatrix;

use crate::animator::animation_state::AnimationState;
use crate::gfx::Vectorize;

use super::{card::Card, view::CardView};

pub struct CardState {
    card: Card,
    origin: [i32; 2],
    diff_vecs: [[f64; 2]; 4],

    animations: heapless::Vec<AnimationState, 10>,
}

impl CardState {
    pub fn new(card: Card, origin: [i32; 2]) -> Self {
        Self {
            card,
            origin,
            diff_vecs: [
                [-16.0, -21.0],
                [ 16.0, -21.0],
                [ 16.0,  21.0],
                [-16.0,  21.0],
            ],
            animations: heapless::Vec::new(),
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
        self.animations
            .iter()
            .map(AnimationState::update)
            .reduce(CMatrix::mul)
            .unwrap_or(CMatrix::identity())
    }
    fn apply_transform(&self, matrix: CMatrix<3, 3>) -> [[f64; 2]; 4] {
        self.diff_vecs
            .map(Vectorize::vectorize)
            .map(|vec| vec.mul(matrix))
            .map(Vectorize::devectorize)
    }
}

impl CardState {
    pub fn add_animation(&mut self, animation: AnimationState) {
        self.animations.push(animation).unwrap();
    }
}
