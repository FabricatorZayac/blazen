use micromath::vector::F32x2;

pub const TEXTURE_WIDTH: u32 = 60;
pub const TEXTURE_HEIGHT: u32 = 80;

#[derive(Clone, Copy)]
pub struct Texture<'a> {
    pub buf: &'a [u8],
    pub uv: [F32x2; 3],
}
