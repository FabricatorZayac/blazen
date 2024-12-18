use core::ops::{Add, Index, Mul, Sub};

use micromath::vector::{Component, Vector2d};

#[derive(Clone, Copy, Debug)]
pub struct Mat2d<C: Component>(
    pub Vector2d<C>,
    pub Vector2d<C>,
);

impl<C> Add for Mat2d<C>
where
    C: Component,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(
            self.0 + rhs.0,
            self.1 + rhs.1,
        )
    }
}

impl<C> Sub for Mat2d<C>
where
    C: Component,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(
            self.0 - rhs.0,
            self.1 - rhs.1,
        )
    }
}

impl<C> Mul<Mat2d<C>> for Vector2d<C>
where 
    C: Component
{
    type Output = Vector2d<C>;

    fn mul(self, rhs: Mat2d<C>) -> Self::Output {
        Self {
            x: self.x * rhs[0][0] + self.y * rhs[1][0],
            y: self.x * rhs[0][1] + self.y * rhs[1][1],
        }
    }
}

impl<C> Mul<C> for Mat2d<C>
where 
    C: Component
{
    type Output = Mat2d<C>;

    fn mul(self, rhs: C) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl<C> Index<usize> for Mat2d<C>
where 
    C: Component,
{
    type Output = Vector2d<C>;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            _ => panic!("index out of range"),
        }
    }
}

impl<C> From<((C, C), (C, C))> for Mat2d<C>
where
    C: Component
{
    fn from(value: ((C, C), (C, C))) -> Self {
        Self(value.0.into(), value.1.into())
    }
}
