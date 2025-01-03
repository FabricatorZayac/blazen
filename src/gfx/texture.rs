use wasm4::draw::DrawIndex;

use crate::__heap_base;

pub const TEXTURE_WIDTH: usize = 60;
pub const TEXTURE_HEIGHT: usize = 80;

#[derive(Clone, Copy)]
pub struct Texture {
    pub buf: &'static [u8],
    pub uv: UV,
    pub colors: TextureColors,
}

#[derive(Clone, Copy)]
pub enum TextureColors {
    OneBpp([DrawIndex; 2]),
    TwoBpp([DrawIndex; 4]),
}

type UV = [[f32; 2]; 3];
pub const CARD_UV0: UV = [ [0.0, 0.0], [1.0, 0.0], [0.0, 1.0] ];
pub const CARD_UV1: UV = [ [1.0, 0.0], [1.0, 1.0], [0.0, 1.0] ];

pub const TEXTURE_BUFFER: *mut [u8; 1200] = &raw mut __heap_base as *mut [u8; 1200];
