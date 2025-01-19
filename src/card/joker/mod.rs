use joker_effects::Effect;
use textures::JIMBO;
use wasm4::draw::DrawIndex;

use crate::gfx::texture::{Texture, TextureColors, CARD_UV0, CARD_UV1};

use super::Card;

mod textures;
mod joker_effects;

#[derive(Debug)]
pub enum JokerType {
    Jimbo,
}

#[derive(Debug)]
pub enum ProcType {
    Passive,
    Blind,
    Normal,
    Card(fn (Card) -> Option<Effect>), //
}

#[derive(Debug)]
pub struct Joker {
    kind: JokerType,
    proc: ProcType,
}

impl Joker {
    pub fn new(kind: JokerType) -> Self {
        // let effect = match kind {
        //     JokerType::Jimbo => Effect { mult: 4, ..Default::default() },
        // };
        // Self { kind, effect }
        Self { kind, proc: ProcType::Normal }
    }
    pub fn texture(&self) -> [Texture; 2] {
        // TODO: use zip for textures
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
