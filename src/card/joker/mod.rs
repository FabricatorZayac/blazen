use smart_default::SmartDefault;
use textures::JIMBO;
use wasm4::draw::DrawIndex;

use crate::gfx::texture::{Texture, TextureColors, CARD_UV0, CARD_UV1};

mod textures;

#[derive(Debug, SmartDefault)]
pub struct Effect {
    points: f32,
    mult: f32,
    #[default = 1.0]
    multx: f32,
}

#[derive(Debug)]
pub enum JokerType {
    Jimbo,
}

#[derive(Debug)]
pub struct Joker {
    kind: JokerType,
    effect: Effect,
}

impl Joker {
    pub fn new(kind: JokerType) -> Self {
        let effect = match kind {
            JokerType::Jimbo => Effect { mult: 4.0, ..Default::default() },
        };
        Self { kind, effect }
    }
    pub fn texture(&self) -> [Texture; 2] {
        let buf = &match self.kind {
            JokerType::Jimbo => JIMBO,
        };
        let colors = match self.kind {
            JokerType::Jimbo => TextureColors::TwoBpp([
                DrawIndex::Second,
                DrawIndex::First,
                DrawIndex::Third,
                DrawIndex::Fourth,
            ]),
        };
        [ Texture { buf, uv: CARD_UV0, colors },
          Texture { buf, uv: CARD_UV1, colors } ]
    }
}
