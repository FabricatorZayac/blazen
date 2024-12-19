use bitvec::{order::Msb0, view::AsBits};
use micromath::vector::F32x2;
use texture::{TEXTURE_HEIGHT, TEXTURE_WIDTH, Texture, TextureColors};
use wasm4::draw::{DrawIndex, Framebuffer};

pub mod texture;
pub mod matrix;

pub type Vec2 = F32x2;

pub trait Render {
    fn render(self, fb: &Framebuffer);
}

pub struct Triangle<'a> {
    pub vertices: [[i32; 2]; 3],
    pub fill: TriangleFill<'a>,
}

pub enum TriangleFill<'a> {
    Texture(Texture<'a>),
    Color(DrawIndex),
}

impl Render for Triangle<'_> {
    fn render(self, fb: &Framebuffer) {
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
                            let z = bary.alpha() / bary.det()
                                  + bary.beta() / bary.det()
                                  + bary.gamma() / bary.det();

                            #[rustfmt::skip]
                            let uv = (uv0 * bary.alpha() * (1.0 / bary.det()))
                                   + (uv1 * bary.beta() * (1.0 / bary.det()))
                                   + (uv2 * bary.gamma() * (1.0 / bary.det()));

                            let tx = (uv.x / z * TEXTURE_WIDTH as f32) as usize;
                            let ty = (uv.y / z * TEXTURE_HEIGHT as f32) as usize;

                            let tx = if tx > TEXTURE_WIDTH as usize {
                                TEXTURE_WIDTH as usize
                            } else {
                                tx
                            };

                            let ty = if ty > TEXTURE_HEIGHT as usize {
                                TEXTURE_HEIGHT as usize
                            } else {
                                ty
                            };

                            match texture.colors {
                                TextureColors::OneBpp(idxs) => {
                                    let bits = texture.buf.as_bits::<Msb0>();
                                    pixel(
                                        x,
                                        y,
                                        idxs[bits[tx + ty * TEXTURE_WIDTH as usize] as usize],
                                        fb,
                                    );
                                }
                                TextureColors::TwoBpp(_) => {
                                    unimplemented!()
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

impl Triangle<'_> {
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
