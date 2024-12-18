use micromath::{vector::{F32x2, Vector, Vector2d}, F32};
use crate::gfx::{matrix::Mat2d, texture::Texture};

use super::{card::Card, view::CardView};

pub struct CardState {
    card: Card,
    origin: F32x2,
    diff_vecs: [F32x2; 4],
}

impl CardState {
    pub fn new(card: Card, origin: F32x2) -> Self {
        Self { card, origin, diff_vecs: [
            (-15.0, -20.0).into(),
            (15.0, -20.0).into(),
            (15.0, 20.0).into(),
            (-15.0, 20.0).into(),
        ] }
    }
    pub fn texture(&self) -> [Texture; 2] {
        self.card.texture()
    }
    pub fn vertices(&self) -> [F32x2; 4] {
        self.diff_vecs.map(|diff| { self.origin + diff })
    }
    pub fn origin(&self) -> F32x2 {
        self.origin
    }
    pub fn view(&self) -> CardView { self.into() }

    pub fn rotate(&mut self, rad: F32) {
        let matrix: Mat2d<F32> = (
            (rad.cos(), -rad.sin()),
            (rad.sin(), rad.cos()),
        ).into();

        self.diff_vecs = self.diff_vecs
            .map(|vec| vec.iter().map(Into::<F32>::into).collect::<Vector2d<F32>>())
            .map(|vec| vec * matrix)
            .map(|vec| vec.iter().map(Into::<f32>::into).collect());
        // for vec in self.diff_vecs {
        //     // vec 
        // }
    }
}
