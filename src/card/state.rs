use constgebra::{CMatrix, CVector};

use crate::animator::animation_state::AnimationState;
use crate::gfx::texture::Texture;
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
    pub fn texture(&self) -> [Texture; 2] {
        self.card.texture()
    }
    pub fn vertices(&self) -> [[i32; 2]; 4] {
        self.diff_vecs.map(|diff| CVector::new([self.origin.map(|i| i as f64)]).add(CVector::new([diff])).finish()[0].map(|f| f as i32))
    }
    pub fn origin(&self) -> [i32; 2] {
        self.origin
    }
    pub fn view(&mut self) -> CardView {
        let transform = self.animate();
        let vertices = self.apply_transform(transform);
        CardView::new(self.texture(), vertices.map(|diff| CVector::new([self.origin.map(|i| i as f64)]).add(CVector::new([diff])).finish()[0].map(|f| f as i32)))
    }
}

impl CardState {
    fn animate(&mut self) -> CMatrix<3, 3> {
        self.animations
            .iter_mut()
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

    pub fn add_animation(&mut self, animation: AnimationState) {
        self.animations.push(animation).unwrap();
    }
}
