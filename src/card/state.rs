use core::intrinsics::{cosf32, cosf64, sinf32, sinf64};

use constgebra::{CMatrix, CVector};

use crate::animator::animator::Animate;
use crate::gfx::texture::Texture;

use super::{card::Card, view::CardView};

pub struct CardState {
    card: Card,
    origin: [i32; 2],
    diff_vecs: [[f64; 2]; 4],

    current_rotation: f64,
}

impl CardState {
    pub fn new(card: Card, origin: [i32; 2]) -> Self {
        Self {
            card,
            origin,
            diff_vecs: [
                [-15.0, -20.0],
                [ 15.0, -20.0],
                [ 15.0,  20.0],
                [-15.0,  20.0],
            ],
            current_rotation: 0.0,
        }
    }
    pub fn texture(&self) -> [Texture; 2] {
        self.card.texture()
    }
    pub fn vertices(&self) -> [[i32; 2]; 4] {
        self.diff_vecs.map(|diff| CVector::new([self.origin.map(|i| i as f64)]).add(CVector::new([diff])).finish()[0].map(|f| f as i32))
    }
    pub fn view(&self) -> CardView { self.into() }

    pub fn rotate(&mut self, rad: f64) {
        let matrix = CMatrix::new(unsafe { [
            [ cosf64(rad), -sinf64(rad)],
            [ sinf64(rad), cosf64(rad)],
        ] });

        self.diff_vecs = self.diff_vecs
            .map(|vec| CVector::new([vec]))
            .map(|vec| vec.mul(matrix).finish()[0]);
    }
}
impl Animate for CardState {
    fn set_rotation(&mut self, rotation: f64) {
        let rotation_rad = rotation/180.0* core::f64::consts::PI;
        self.rotate(self.current_rotation - rotation_rad);
        self.current_rotation = rotation_rad;
    }
}
