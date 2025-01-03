use core::ops::{Add, Index, IndexMut, Mul};

use super::matrix::Matrix;

pub type Vec3 = Vector<3>;

pub struct Vector<const N: usize>([f32; N]);

impl<const N: usize> IndexMut<usize> for Vector<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl<const N: usize> Index<usize> for Vector<N> {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl<const N: usize> Default for Vector<N> {
    fn default() -> Self {
        Vector([Default::default(); N])
    }
}

impl<const N: usize> Add for Vector<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut out = Self::default();
        for i in 0..N {
            out[i] = self[i] + rhs[i];
        }
        out
    }
}
impl<const N: usize> Add for &Vector<N> {
    type Output = Vector<N>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut out = Vector::default();
        for i in 0..N {
            out[i] = self[i] + rhs[i];
        }
        out
    }
}

impl<const N: usize> Mul for Vector<N> {
    type Output = f32;

    fn mul(self, rhs: Self) -> Self::Output {
        (0..N).into_iter().fold(0.0, |acc, i| acc + self[i] * rhs[i])
    }
}

impl<const N: usize> Mul for &Vector<N> {
    type Output = f32;

    fn mul(self, rhs: Self) -> Self::Output {
        (0..N).into_iter().fold(0.0, |acc, i| acc + self[i] * rhs[i])
    }
}

impl<const N: usize> Mul<&Matrix<N>> for Vector<N> {
    type Output = Self;

    fn mul(self, rhs: &Matrix<N>) -> Self::Output {
        let mut out = Vector::default();
        for i in 0..N {
            out[i] = &self * &Into::<Self>::into(rhs[i]);
        }

        out
    }
}

impl<const N: usize> Mul<Matrix<N>> for Vector<N> {
    type Output = Self;

    fn mul(self, rhs: Matrix<N>) -> Self::Output {
        let mut out = Vector::default();
        for i in 0..N {
            out[i] = &self * &Into::<Self>::into(rhs[i]);
        }

        out
    }
}

impl<const N: usize> From<[f32; N]> for Vector<N> {
    fn from(value: [f32; N]) -> Self {
        Self(value)
    }
}

impl<const N: usize> IntoIterator for Vector<N> {
    type Item = f32;

    type IntoIter = <[f32; N] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
