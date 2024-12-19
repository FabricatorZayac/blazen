use core::intrinsics::{cosf32, sinf32};

use crate::animator::animator::Animate;
use crate::gfx::Vec2;
use crate::gfx::{matrix::Mat2, texture::Texture};

use super::{card::Card, view::CardView};

#[derive(Debug)]
pub struct CardState {
    card: Card,
    origin: Vec2,
    diff_vecs: [Vec2; 4],

    current_rotation: f32,
}

impl CardState {
    pub fn new(card: Card, origin: Vec2) -> Self {
        Self { card, origin, diff_vecs: [
            (-15.0, -20.0).into(),
            (15.0, -20.0).into(),
            (15.0, 20.0).into(),
            (-15.0, 20.0).into(),
        ], current_rotation: 0.0}
    }
    pub fn texture(&self) -> [Texture; 2] {
        self.card.texture()
    }
    pub fn vertices(&self) -> [Vec2; 4] {
        self.diff_vecs.map(|diff| { self.origin + diff })
    }
    pub fn view(&self) -> CardView { self.into() }

    pub fn rotate(&mut self, rad: f32) {
        let matrix: Mat2 = unsafe { (
            (cosf32(rad), -sinf32(rad)),
            (sinf32(rad), cosf32(rad)),
        ) }.into();

        self.diff_vecs = self.diff_vecs.map(|vec| vec * matrix);
    }
}
impl Animate for CardState {
    fn set_rotation(&mut self, rotation: f32) {
        let rotation_rad = rotation/180.0* core::f32::consts::PI;
        self.rotate(self.current_rotation - rotation_rad);
        self.current_rotation = rotation_rad;
    }
}
