use micromath::vector::F32x2;
use wasm4::draw::DrawIndex;

pub const TEXTURE_WIDTH: u32 = 60;
pub const TEXTURE_HEIGHT: u32 = 80;

#[derive(Clone, Copy)]
pub struct Texture<'a> {
    pub buf: &'a [u8],
    pub uv: [F32x2; 3],
    pub colors: TextureColors,
}

#[derive(Clone, Copy)]
pub enum TextureColors {
    OneBpp([DrawIndex; 2]),
    TwoBpp([DrawIndex; 4]),
}
