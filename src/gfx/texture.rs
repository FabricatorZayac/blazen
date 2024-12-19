use wasm4::draw::DrawIndex;

pub const TEXTURE_WIDTH: usize = 60;
pub const TEXTURE_HEIGHT: usize = 80;

#[derive(Clone, Copy)]
pub struct Texture<'a> {
    pub buf: &'a [u8],
    pub uv: [[f64; 2]; 3],
    pub colors: TextureColors,
}

#[derive(Clone, Copy)]
pub enum TextureColors {
    OneBpp([DrawIndex; 2]),
    TwoBpp([DrawIndex; 4]),
}
