use micromath::vector::I32x2;
use wasm4::draw::Framebuffer;

use crate::cards::card::Card;

use super::{texture::Texture, Render, Triangle};

pub struct CardModel<'a> {
    pub origin: I32x2,
    pub card: &'a Card,
    pub texture: [Texture<'a>; 2],
}

impl Render for CardModel<'_> {
    fn render(self, fb: &Framebuffer) {

        let vertices = [
            I32x2 {
                x: self.origin.x - 15,
                y: self.origin.y - 20,
            },
            I32x2 {
                x: self.origin.x + 15,
                y: self.origin.y - 20,
            },
            I32x2 {
                x: self.origin.x + 15,
                y: self.origin.y + 20,
            },
            I32x2 {
                x: self.origin.x - 15,
                y: self.origin.y + 20,
            },
        ];

        let t1 = Triangle {
            vertices: [
                vertices[0],
                vertices[1],
                vertices[3],
            ],
            texture: Some(self.texture[0]),
        };
        let t2 = Triangle {
            vertices: [
                vertices[1],
                vertices[2],
                vertices[3],
            ],
            texture: Some(self.texture[1]),
        };

        t1.render(fb);
        t2.render(fb);
    }
}

// spades_ace
// NOTE: This will be replaced by prodcedural textures later
pub const ACE: [u8; 600] = [ 0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xf0,0x7f,0xff,0xff,0xff,0xff,0xff,0xfe,0x03,0xff,0xff,0xff,0xff,0xff,0xff,0xc7,0x1f,0xff,0xff,0xff,0xff,0xff,0xfc,0xf9,0xff,0xff,0xff,0xff,0xff,0xff,0xcf,0x9f,0xff,0xff,0xff,0xff,0xff,0xfc,0x01,0xff,0xff,0xff,0xff,0xff,0xff,0xc0,0x1f,0xff,0xff,0xff,0xff,0xff,0xfc,0xf9,0xff,0xff,0xff,0xff,0xff,0xff,0xcf,0x9f,0xff,0xff,0xff,0xff,0xff,0xfc,0xf9,0xff,0xff,0xff,0xff,0xff,0xff,0xcf,0x9f,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xfc,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0x87,0xff,0xff,0xff,0xff,0xff,0xff,0xf0,0x3f,0xff,0xf0,0xff,0xff,0xff,0xfe,0x01,0xff,0xff,0x0f,0xff,0xff,0xff,0xc0,0x0f,0xff,0xf0,0xff,0xff,0xff,0xfc,0x00,0xff,0xff,0x0f,0xff,0xff,0xff,0xc4,0x8f,0xff,0xc0,0x3f,0xff,0xff,0xfe,0xcd,0xff,0xfc,0x03,0xff,0xff,0xff,0xf8,0x7f,0xff,0xc0,0x3f,0xff,0xff,0xff,0x03,0xff,0xfc,0x03,0xff,0xff,0xff,0xff,0xff,0xff,0x00,0x0f,0xff,0xff,0xff,0xff,0xff,0xf0,0x00,0xff,0xff,0xff,0xff,0xff,0xfc,0x00,0x03,0xff,0xff,0xff,0xff,0xff,0xc0,0x00,0x3f,0xff,0xff,0xff,0xff,0xf0,0x00,0x00,0xff,0xff,0xff,0xff,0xff,0x00,0x00,0x0f,0xff,0xff,0xff,0xff,0xc0,0x00,0x00,0x3f,0xff,0xff,0xff,0xfc,0x00,0x00,0x03,0xff,0xff,0xff,0xfc,0x00,0x00,0x00,0x03,0xff,0xff,0xff,0xc0,0x00,0x00,0x00,0x3f,0xff,0xff,0xf0,0x00,0x00,0x00,0x00,0xff,0xff,0xff,0x00,0x00,0x00,0x00,0x0f,0xff,0xff,0xc0,0x00,0x00,0x00,0x00,0x3f,0xff,0xfc,0x00,0x00,0x00,0x00,0x03,0xff,0xff,0xc0,0x00,0x00,0x00,0x00,0x3f,0xff,0xfc,0x00,0x00,0x00,0x00,0x03,0xff,0xff,0xc0,0x00,0x00,0x00,0x00,0x3f,0xff,0xfc,0x00,0x00,0x00,0x00,0x03,0xff,0xff,0xc0,0x00,0x00,0x00,0x00,0x3f,0xff,0xfc,0x00,0x00,0x00,0x00,0x03,0xff,0xff,0xc0,0x00,0x30,0xc0,0x00,0x3f,0xff,0xfc,0x00,0x03,0x0c,0x00,0x03,0xff,0xff,0xf0,0x03,0xf0,0xfc,0x00,0xff,0xff,0xff,0x00,0x3f,0x0f,0xc0,0x0f,0xff,0xff,0xfc,0x3f,0xf0,0xff,0xc3,0xff,0xff,0xff,0xc3,0xff,0x0f,0xfc,0x3f,0xff,0xff,0xff,0xff,0xf0,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0x0f,0xff,0xff,0xff,0xff,0xff,0xff,0xc0,0x3f,0xff,0xff,0xff,0xff,0xff,0xfc,0x03,0xff,0xff,0xff,0xff,0xff,0xff,0x00,0x0f,0xff,0xff,0xff,0xff,0xff,0xf0,0x00,0xff,0xff,0xff,0xff,0xff,0xfc,0x00,0x03,0xff,0xc0,0xff,0xff,0xff,0xc0,0x00,0x3f,0xfe,0x1f,0xff,0xff,0xff,0x00,0x0f,0xff,0xb3,0x7f,0xff,0xff,0xf0,0x00,0xff,0xf1,0x23,0xff,0xff,0xff,0xff,0xff,0xff,0x00,0x3f,0xff,0xff,0xff,0xff,0xff,0xf0,0x03,0xff,0xff,0xff,0xff,0xff,0xff,0x80,0x7f,0xff,0xff,0xff,0xff,0xff,0xfc,0x0f,0xff,0xff,0xff,0xff,0xff,0xff,0xe1,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0x3f,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xf9,0xf3,0xff,0xff,0xff,0xff,0xff,0xff,0x9f,0x3f,0xff,0xff,0xff,0xff,0xff,0xf9,0xf3,0xff,0xff,0xff,0xff,0xff,0xff,0x9f,0x3f,0xff,0xff,0xff,0xff,0xff,0xf8,0x03,0xff,0xff,0xff,0xff,0xff,0xff,0x80,0x3f,0xff,0xff,0xff,0xff,0xff,0xf9,0xf3,0xff,0xff,0xff,0xff,0xff,0xff,0x9f,0x3f,0xff,0xff,0xff,0xff,0xff,0xf8,0xe3,0xff,0xff,0xff,0xff,0xff,0xff,0xc0,0x7f,0xff,0xff,0xff,0xff,0xff,0xfe,0x0f,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff ];
