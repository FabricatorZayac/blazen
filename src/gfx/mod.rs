use core::ops::Add as _;

use bitvec::{order::Msb0, view::AsBits};
use texture::{Texture, TextureColors, TEXTURE_HEIGHT, TEXTURE_WIDTH};
use wasm4::draw::{DrawIndex, Framebuffer};

use crate::linalg::vector::Vec3;

pub mod texture;

pub trait Vectorize {
    fn vectorize(self) -> Vec3;
    fn devectorize(vector: Vec3) -> Self;
}

impl Vectorize for [f32; 2] {
    fn vectorize(self) -> Vec3 {
        [self[0], self[1], 1.0].into()
    }

    fn devectorize(vector: Vec3) -> Self {
        [vector[0], vector[1]]
    }
}

pub trait Render {
    fn render(&self, fb: &Framebuffer);
}

pub struct Triangle {
    pub vertices: [[i32; 2]; 3],
    pub fill: TriangleFill,
}

pub enum TriangleFill {
    Texture(Texture),
    Color(DrawIndex),
}

impl Render for Triangle {
    fn render(&self, fb: &Framebuffer) {
        let normal = self.normalize();
        for y in normal[0][1]..normal[1][1] {
            for x in normal[0][0]..normal[1][0] {
                if let Some(bary) = self.barycentric([x, y]) {
                    match self.fill {
                        TriangleFill::Texture(texture) => {
                            let uv0 = texture.uv[0];
                            let uv1 = texture.uv[1];
                            let uv2 = texture.uv[2];

                            #[rustfmt::skip]
                            let uv = uv0.map(|a| a * bary.alpha()).vectorize().add(
                                     uv1.map(|a| a * bary.beta()).vectorize()).add(
                                     uv2.map(|a| a * bary.gamma()).vectorize());
                            let tx = (uv[0] * TEXTURE_WIDTH as f32 / bary.det()) as usize;
                            let ty = (uv[1] * TEXTURE_HEIGHT as f32 / bary.det()) as usize;

                            let tx = if tx >= TEXTURE_WIDTH {
                                TEXTURE_WIDTH - 1
                            } else {
                                tx
                            };

                            let ty = if ty >= TEXTURE_HEIGHT {
                                TEXTURE_HEIGHT - 1
                            } else {
                                ty
                            };

                            match texture.colors {
                                TextureColors::OneBpp(idxs) => {
                                    // might replace with picking out the byte + bit offset
                                    let buf = &texture.buf[..600];
                                    let bits = buf.as_bits::<Msb0>();
                                    pixel(
                                        x,
                                        y,
                                        idxs[bits[tx + ty * TEXTURE_WIDTH as usize] as usize],
                                        fb,
                                    );
                                }
                                TextureColors::TwoBpp(idxs) => {
                                    let buf = texture.buf;
                                    let bits = buf.as_bits::<Msb0>();

                                    // This is lowkey scuffed
                                    let pos = (tx + ty * TEXTURE_WIDTH) * 2;
                                    pixel(
                                        x,
                                        y,
                                        idxs[bits[pos] as usize + bits[pos+1] as usize * 2],
                                        fb,
                                    );
                                }
                            }
                        }
                        TriangleFill::Color(idx) => {
                            pixel(x, y, idx, fb);
                        }
                    }
                }
            }
        }
    }
}

impl Triangle {
    fn normalize(&self) -> [[i32; 2]; 2] {
        let lx = self.vertices.iter().map(|v| v[0]).min().unwrap();
        let ly = self.vertices.iter().map(|v| v[1]).min().unwrap();

        let hx = self.vertices.iter().map(|v| v[0]).max().unwrap();
        let hy = self.vertices.iter().map(|v| v[1]).max().unwrap();

        [(lx, ly).into(), (hx, hy).into()]
    }
    fn barycentric(&self, pt: [i32; 2]) -> Option<Barycentric2D> {
        let pts = self.vertices;

        #[rustfmt::skip]
        let det = (pts[0][0] - pts[2][0]) * (pts[1][1] - pts[2][1])
                - (pts[1][0] - pts[2][0]) * (pts[0][1] - pts[2][1]);

        #[rustfmt::skip]
        let u1 = (pt[0] - pts[2][0]) * (pts[1][1] - pts[2][1])
               + (pts[2][0] - pts[1][0]) * (pt[1] - pts[2][1]);

        #[rustfmt::skip]
        let u2 = (pt[0] - pts[2][0]) * (pts[2][1] - pts[0][1])
               + (pts[0][0] - pts[2][0]) * (pt[1] - pts[2][1]);

        let u3 = det - u1 - u2;

        if u1.signum() != det.signum() && u1 != 0 {
            return None;
        }
        if u2.signum() != det.signum() && u2 != 0 {
            return None;
        }
        if u3.signum() != det.signum() && u3 != 0 {
            return None;
        }

        Some(Barycentric2D { u1, u2, det })
    }
}

struct Barycentric2D {
    u1: i32,
    u2: i32,
    det: i32,
}

impl Barycentric2D {
    pub fn alpha(&self) -> f32 {
        self.u1 as f32
    }
    pub fn beta(&self) -> f32 {
        self.u2 as f32
    }
    pub fn gamma(&self) -> f32 {
        (self.det - self.u1 - self.u2) as f32
    }
    pub fn det(&self) -> f32 {
        self.det as f32
    }
}

impl From<[i32; 3]> for Barycentric2D {
    fn from(value: [i32; 3]) -> Self {
        Self {
            u1: value[0],
            u2: value[1],
            det: value[2],
        }
    }
}

fn pixel(x: i32, y: i32, color_idx: DrawIndex, fb: &Framebuffer) {
    if x < 0 || x >= wasm4::sys::SCREEN_SIZE as i32 || y < 0 || y >= wasm4::sys::SCREEN_SIZE as i32 {
        return;
    }
    // The byte index into the framebuffer that contains (x, y)
    let idx = (y as usize * 160 + x as usize) >> 2;

    // Calculate the bits within the byte that corresponds to our position
    let shift = (x as u8 & 0b11) << 1;
    let mask = 0b11 << shift;

    let palette_color: u8 = (color_idx as u16 & 0xf) as u8;
    let color: u8 = (palette_color - 1) & 0b11;

    fb.as_cells()[idx].replace((color << shift) | fb.as_cells()[idx].get() & !mask);
}
