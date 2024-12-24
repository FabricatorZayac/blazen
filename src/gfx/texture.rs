use core::mem::MaybeUninit;

use wasm4::draw::DrawIndex;

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

static mut TEXTURE_BUFFER: MaybeUninit<[u8; 1200]> = MaybeUninit::uninit();
pub struct TextureBuffer; // Maybe there's a more elegant way to do a singleton,
impl TextureBuffer {      // but I don't care
    pub fn init() {
        unsafe {
            TEXTURE_BUFFER = MaybeUninit::zeroed();
        }
    }
    pub fn get_mut() -> &'static mut [u8] {
        unsafe {
            TEXTURE_BUFFER.assume_init_mut()
        }
    }
    pub fn get() -> &'static [u8] {
        unsafe {
            TEXTURE_BUFFER.assume_init_ref()
        }
    }
}
