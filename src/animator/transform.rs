use core::{f32::consts::PI, fmt::Debug, ops::{Add, Mul, Sub}};
use derive_more::derive::From;
use derive_new::new;

use crate::linalg::matrix::Mat3;

pub trait Transform: Debug {
    fn apply(&self, progress: f32) -> Mat3;
}

#[derive(Debug, From, Clone, Copy)]
pub enum Transformation {
    Rotate(Rotate),
    Translate(Translate),
    Scale(Scale),
    Shear(Shear)
}

#[derive(Debug, new, Clone, Copy)]
pub struct Rotate {
    start_angle: f32,
    end_angle: f32,
}
#[derive(Debug, new, Clone, Copy)]
pub struct Translate {
    start: [f32; 2],
    end: [f32; 2],
}
#[derive(Debug, new, Clone, Copy)]
pub struct Scale {
    start: [f32; 2],
    end: [f32; 2],
}
#[derive(Debug, new, Clone, Copy)]
pub struct Shear {
    start: [f32; 2],
    end: [f32; 2],
}

fn lerp<T>(start: T, end: T, progress: T) -> T
    where T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy
{
    start + (end - start) * progress
}

impl Transform for Rotate {
    fn apply(&self, progress: f32) -> Mat3 {
        let angle = lerp(self.start_angle, self.end_angle, progress as f32).to_radians();
        [[cos(angle), -sin(angle), 0.0],
         [sin(angle),  cos(angle), 0.0],
         [       0.0,         0.0, 1.0]].into()
    }
}
impl Transform for Translate {
    fn apply(&self, progress: f32) -> Mat3 {
        let x = lerp(self.start[0], self.end[0], progress);
        let y = lerp(self.start[1], self.end[1], progress);
        [[ 1.0, 0.0,   x],
         [ 0.0, 1.0,   y],
         [ 0.0, 0.0, 1.0]].into()
    }
}
impl Transform for Scale {
    fn apply(&self, progress: f32) -> Mat3 {
        let x = lerp(self.start[0], self.end[0], progress);
        let y = lerp(self.start[1], self.end[1], progress);
        [[ x,   0.0, 0.0],
         [ 0.0,   y, 0.0],
         [ 0.0, 0.0, 1.0]].into()
    }
}
// this seems to be broken for now
impl Transform for Shear {
    fn apply(&self, progress: f32) -> Mat3 {
        let x = lerp(self.start[0], self.end[0], progress);
        let y = lerp(self.start[1], self.end[1], progress);
        [[ 1.0,   y, 0.0],
         [   x, 1.0, 0.0],
         [ 0.0, 0.0, 1.0]].into()
    }
}

impl Transform for [Transformation] {
    fn apply(&self, progress: f32) -> Mat3 {
        self.iter().fold(Mat3::identity(), |acc, transform| acc.mul(transform.apply(progress)))
    }
}

impl Transform for Transformation {
    fn apply(&self, progress: f32) -> Mat3 {
        match self {
            // this feels like it should be a macro, but whatever
            Transformation::Rotate(rotate) => rotate.apply(progress),
            Transformation::Translate(translate) => translate.apply(progress),
            Transformation::Scale(scale) => scale.apply(progress),
            Transformation::Shear(shear) => shear.apply(progress),
        }
    }
}

fn sin(x: f32) -> f32 {
    const B: f32 = 4.0 / PI;
    const C: f32 = -4.0 / (PI * PI);
    let y = B * x + C * x * x.abs();

    y
}

fn cos(x: f32) -> f32 {
    sin(x + PI / 2.0)
}
