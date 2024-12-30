use wasm4::draw::DrawIndex;

use crate::__heap_base;

pub const TEXTURE_WIDTH: usize = 60;
pub const TEXTURE_HEIGHT: usize = 80;

#[derive(Clone, Copy)]
pub struct Texture {
    pub uv: [[f64; 2]; 3],
    pub colors: TextureColors,
}

#[derive(Clone, Copy)]
pub enum TextureColors {
    OneBpp([DrawIndex; 2]),
    TwoBpp([DrawIndex; 4]),
}

pub const TEXTURE_BUFFER: *mut TextureBuffer = &raw mut __heap_base as *mut TextureBuffer;
pub struct TextureBuffer([u8; 1200]);
impl TextureBuffer {
    pub fn get_mut(&mut self) -> &mut [u8] {
        self.0.as_mut()
    }
    pub fn get(&self) -> &[u8] {
        self.0.as_ref()
    }
}
