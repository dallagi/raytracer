use std::ops;

use crate::matrix::Matrix;

impl<const ROWS: usize, const COLS: usize> ops::Index<(usize, usize)> for Matrix<ROWS, COLS> {
    type Output = f32;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[index.0][index.1]
    }
}

impl<const ROWS: usize, const COLS: usize> ops::IndexMut<(usize, usize)> for Matrix<ROWS, COLS> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.0[index.0][index.1]
    }
}
