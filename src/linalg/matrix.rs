use core::ops::{Index, IndexMut, Mul};

pub type Mat3 = Matrix<3>;

pub struct Matrix<const N: usize>([[f32; N]; N]);

impl<const N: usize> Matrix<N> {
    pub fn identity() -> Self {
        let mut matrix = Matrix::default();

        for i in 0..N {
            matrix[i][i] = 1.0;
        }

        matrix
    }
}

impl<const N: usize> Mul for Matrix<N> {
    type Output = Matrix<N>;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut matrix = Matrix::default();
        for i in 0..N {
            for j in 0..N {
                matrix[i][j] = (0..N)
                    .into_iter()
                    .fold(0.0, |acc, k| acc + self[i][k] * rhs[k][j]);
            }
        }
        matrix
    }
}

impl<const N: usize> Index<usize> for Matrix<N> {
    type Output = [f32; N];

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}
impl<const N: usize> IndexMut<usize> for Matrix<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl<const N: usize> Default for Matrix<N> {
    fn default() -> Self {
        Self([[0.0; N]; N])
    }
}

impl<const N: usize> From<[[f32; N]; N]> for Matrix<N> {
    fn from(value: [[f32; N]; N]) -> Self {
        Self(value)
    }
}
