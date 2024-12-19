use core::ops::{Add, Index, Mul, Sub};

use micromath::{vector::{Component, Vector, Vector2d}, F32};

#[derive(Clone, Copy, Debug)]
pub struct Mat2(
    Vector2d<f32>,
    Vector2d<f32>,
);

impl Add for Mat2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(
            self.0 + rhs.0,
            self.1 + rhs.1,
        )
    }
}

impl Sub for Mat2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(
            self.0 - rhs.0,
            self.1 - rhs.1,
        )
    }
}

impl Mul<Mat2> for Vector2d<f32> {
    type Output = Vector2d<f32>;

    fn mul(self, rhs: Mat2) -> Self::Output {
        Self {
            x: self.x * rhs[0][0] + self.y * rhs[1][0],
            y: self.x * rhs[0][1] + self.y * rhs[1][1],
        }
    }
}

impl Mul<f32> for Mat2 {
    type Output = Mat2;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl Index<usize> for Mat2 {
    type Output = Vector2d<f32>;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            _ => panic!("index out of range"),
        }
    }
}

impl From<((f32, f32), (f32, f32))> for Mat2 {
    fn from(value: ((f32, f32), (f32, f32))) -> Self {
        Self(value.0.into(), value.1.into())
    }
}
