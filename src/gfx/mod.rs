use bitvec::{order::Msb0, view::AsBits};
use micromath::vector::I32x2;
use texture::{TEXTURE_HEIGHT, TEXTURE_WIDTH, Texture};
use wasm4::draw::{DrawIndex, Framebuffer};

pub mod model;
pub mod texture;

pub trait Render {
    fn render(self, fb: &Framebuffer);
}

pub struct Triangle<'a> {
    pub vertices: [I32x2; 3],
    pub texture: Option<Texture<'a>>,
}

impl Render for Triangle<'_> {
    fn render(self, fb: &Framebuffer) {
        let normal = self.normalize();
        for y in normal[0].y..normal[1].y {
            for x in normal[0].x..normal[1].x {
                if let Some(bary) = self.barycentric(I32x2 { x, y }) {
                    let texture = self.texture.unwrap();

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

                    let buf = texture.buf;
                    let bits = buf.as_bits::<Msb0>();

                    let tx = (uv.x / z * TEXTURE_WIDTH as f32) as usize;
                    let ty = (uv.y / z * TEXTURE_HEIGHT as f32) as usize;

                    let tx = if tx > TEXTURE_WIDTH as usize {
                        TEXTURE_WIDTH as usize
                    } else { tx };

                    let ty = if ty > TEXTURE_HEIGHT as usize {
                        TEXTURE_HEIGHT as usize
                    } else { ty };

                    if bits[tx + ty * TEXTURE_WIDTH as usize] {
                        pixel(x, y, DrawIndex::Fourth, fb);
                    } else {
                        pixel(x, y, DrawIndex::Second, fb);
                    }
                }
            }
        }
    }
}

impl Triangle<'_> {
    fn normalize(&self) -> [I32x2; 2] {
        let lx = self.vertices.iter().map(|v|{v.x}).min().unwrap();
        let ly = self.vertices.iter().map(|v|{v.y}).min().unwrap();

        let hx = self.vertices.iter().map(|v|{v.x}).max().unwrap();
        let hy = self.vertices.iter().map(|v|{v.y}).max().unwrap();

        [I32x2 { x: lx, y: ly }, I32x2 { x: hx, y: hy }]
    }
    fn barycentric(&self, pt: I32x2) -> Option<Barycentric2D> {
        let pts = self.vertices;
        let det = (pts[0].x - pts[2].x) * (pts[1].y - pts[2].y)
                - (pts[1].x - pts[2].x) * (pts[0].y - pts[2].y);

        let u1 = (pt.x - pts[2].x) * (pts[1].y - pts[2].y)
               + (pts[2].x - pts[1].x) * (pt.y - pts[2].y);

        let u2 = (pt.x - pts[2].x) * (pts[2].y - pts[0].y)
               + (pts[0].x - pts[2].x) * (pt.y - pts[2].y);

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

        Some(Barycentric2D {
            u1,
            u2,
            det,
        })
    }
}

struct Barycentric2D {
    u1: i32,
    u2: i32,
    det: i32,
}

impl Barycentric2D {
    pub fn alpha(&self) -> f32 { self.u1 as f32 }
    pub fn beta(&self) -> f32 { self.u2 as f32 }
    pub fn gamma(&self) -> f32 { (self.det - self.u1 - self.u2) as f32 }
    pub fn det(&self) -> f32 { self.det as f32 }
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
    // The byte index into the framebuffer that contains (x, y)
    let idx = (y as usize * 160 + x as usize) >> 2;

    // Calculate the bits within the byte that corresponds to our position
    let shift = (x as u8 & 0b11) << 1;
    let mask = 0b11 << shift;

    let palette_color: u8 = (color_idx as u16 & 0xf) as u8;
    let color: u8 = (palette_color - 1) & 0b11;

    fb.as_cells()[idx].replace((color << shift) | fb.as_cells()[idx].get() & !mask);
}
