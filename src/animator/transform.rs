use core::{fmt::Debug, intrinsics::{cosf64, sinf64}};
use constgebra::CMatrix;
use derive_more::derive::From;
use derive_new::new;

pub trait Transform: Debug {
    fn apply(&self, progress: f64) -> CMatrix<3, 3>;
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
    start_angle: f64,
    end_angle: f64,
}
#[derive(Debug, new, Clone, Copy)]
pub struct Translate {
    start: [f64; 2],
    end: [f64; 2],
}
#[derive(Debug, new, Clone, Copy)]
pub struct Scale {
    start: [f64; 2],
    end: [f64; 2],
}
#[derive(Debug, new, Clone, Copy)]
pub struct Shear {
    start: [f64; 2],
    end: [f64; 2],
}

fn lerp(start: f64, end: f64, progress: f64) -> f64 {
    start + (end - start) * progress
}

impl Transform for Rotate {
    fn apply(&self, progress: f64) -> CMatrix<3, 3> {
        let angle = lerp(self.start_angle, self.end_angle, progress).to_radians();
        CMatrix::new(unsafe { [
            [ cosf64(angle), sinf64(angle), 0.0],
            [-sinf64(angle), cosf64(angle), 0.0],
            [           0.0,           0.0, 1.0],
        ] })
    }
}
impl Transform for Translate {
    fn apply(&self, progress: f64) -> CMatrix<3, 3> {
        let x = lerp(self.start[0], self.end[0], progress);
        let y = lerp(self.start[1], self.end[1], progress);
        CMatrix::new([
            [ 1.0, 0.0, 0.0],
            [ 0.0, 1.0, 0.0],
            [   x,   y, 1.0],
        ])
    }
}
impl Transform for Scale {
    fn apply(&self, progress: f64) -> CMatrix<3, 3> {
        let x = lerp(self.start[0], self.end[0], progress);
        let y = lerp(self.start[1], self.end[1], progress);
        CMatrix::new([
            [ x,   0.0, 0.0],
            [ 0.0,   y, 0.0],
            [ 0.0, 0.0, 1.0],
        ])
    }
}
// this seems to be broken for now
impl Transform for Shear {
    fn apply(&self, progress: f64) -> CMatrix<3, 3> {
        let x = lerp(self.start[0], self.end[0], progress);
        let y = lerp(self.start[1], self.end[1], progress);
        CMatrix::new([
            [ 1.0,   y, 0.0],
            [   x, 1.0, 0.0],
            [ 0.0, 0.0, 1.0],
        ])
    }
}

impl Transform for [Transformation] {
    fn apply(&self, progress: f64) -> CMatrix<3, 3> {
        self.iter().fold(CMatrix::identity(), |acc, transform| acc.mul(transform.apply(progress)))
    }
}

impl Transform for Transformation {
    fn apply(&self, progress: f64) -> CMatrix<3, 3> {
        match self {
            // this feels like it should be a macro, but whatever
            Transformation::Rotate(rotate) => rotate.apply(progress),
            Transformation::Translate(translate) => translate.apply(progress),
            Transformation::Scale(scale) => scale.apply(progress),
            Transformation::Shear(shear) => shear.apply(progress),
        }
    }
}

/*
// Combine transformations in a fixed array
    let transformations = [
        TransformationType::Rotate(rotate),
        TransformationType::Translate(translate),
        TransformationType::Scale(scale),
    ];

    // Create a transformation chain
    let chain = TransformationChain::new(&transformations);

    // Apply the entire chain
    let progress = 0.3;
    let result = chain.apply(progress);
 */
