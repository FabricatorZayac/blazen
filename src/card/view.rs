use wasm4::draw::Framebuffer;

use crate::gfx::{
    Render, Triangle, TriangleFill,
    texture::Texture,
};

pub struct CardView<'a> {
    texture: [Texture<'a>; 2],
    vertices: [[i32; 2]; 4],
}

impl<'a> CardView<'a> {
    pub fn new(texture: [Texture<'a>; 2], vertices: [[i32;2]; 4]) -> Self {
        Self { texture, vertices }
    }
}

impl Render for CardView<'_> {
    fn render(self, fb: &Framebuffer) {
        let t1 = Triangle {
            vertices: [self.vertices[0], self.vertices[1], self.vertices[3]],
            fill: TriangleFill::Texture(self.texture[0]),
        };
        let t2 = Triangle {
            vertices: [self.vertices[1], self.vertices[2], self.vertices[3]],
            fill: TriangleFill::Texture(self.texture[1]),
        };

        t1.render(fb);
        t2.render(fb);
    }
}
