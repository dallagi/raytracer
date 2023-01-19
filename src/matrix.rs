use std::ops;

use crate::float_eq::FloatEq;

#[derive(Copy, Clone, Debug)]
struct Matrix<const ROWS: usize, const COLS: usize>([[f32; COLS]; ROWS]);

impl<const ROWS: usize, const COLS: usize> Matrix<ROWS, COLS> {
    pub fn new(content: [[f32; COLS]; ROWS]) -> Self {
        Self(content)
    }
}

impl<const ROWS: usize, const COLS: usize> ops::Index<(usize, usize)> for Matrix<ROWS, COLS> {
    type Output = f32;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[index.0][index.1]
    }
}

impl<const ROWS: usize, const COLS: usize> PartialEq for Matrix<ROWS, COLS> {
    fn eq(&self, other: &Self) -> bool {
        for r in 0..ROWS {
            for c in 0..COLS {
                if !self[(r, c)].float_eq(other[(r, c)]) {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn constructing_and_inspecting_a_4x4_matrix() {
        let matrix = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);

        assert_eq!(1.0, matrix[(0, 0)]);
        assert_eq!(4.0, matrix[(0, 3)]);
        assert_eq!(5.5, matrix[(1, 0)]);
        assert_eq!(7.5, matrix[(1, 2)]);
        assert_eq!(11.0, matrix[(2, 2)]);
        assert_eq!(13.5, matrix[(3, 0)]);
        assert_eq!(15.5, matrix[(3, 2)]);
    }

    #[test]
    fn can_represent_2x2_matrix() {
        let matrix = Matrix::new([[-3.0, 5.0], [1.0, -2.0]]);

        assert_eq!(-3.0, matrix[(0, 0)]);
        assert_eq!(5.0, matrix[(0, 1)]);
        assert_eq!(1.0, matrix[(1, 0)]);
        assert_eq!(-2.0, matrix[(1, 1)]);
    }

    #[test]
    fn can_represent_3x3_matrix() {
        let matrix = Matrix::new([[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]]);

        assert_eq!(-3.0, matrix[(0, 0)]);
        assert_eq!(-2.0, matrix[(1, 1)]);
        assert_eq!(1.0, matrix[(2, 2)]);
    }

    #[test]
    fn matrix_are_equal_if_all_elements_are_equal() {
        let matrix_1 = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let matrix_2 = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        assert_eq!(matrix_1, matrix_2);
    }
    #[test]
    fn matrix_are_not_equal_if_some_element_is_different() {
        let matrix_1 = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let matrix_2 = Matrix::new([
            [2.0, 3.0, 4.0, 5.0],
            [6.0, 7.0, 8.0, 9.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        ]);

        assert_ne!(matrix_1, matrix_2);
    }
}
