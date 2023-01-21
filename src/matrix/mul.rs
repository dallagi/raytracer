use std::ops;

use crate::matrix::Matrix;

impl<const ROWS: usize, const COLS: usize, const OTHER_COLS: usize>
    ops::Mul<Matrix<COLS, OTHER_COLS>> for Matrix<ROWS, COLS>
{
    type Output = Matrix<ROWS, OTHER_COLS>;

    fn mul(self, other: Matrix<COLS, OTHER_COLS>) -> Self::Output {
        let mut result = Matrix::<ROWS, OTHER_COLS>::zeros();

        for r in 0..ROWS {
            for c in 0..OTHER_COLS {
                result[(r, c)] = multiplication_element(&self, &other, (r, c));
            }
        }
        result
    }
}

fn multiplication_element<const A_ROWS: usize, const A_COLS: usize, const B_COLS: usize>(
    m1: &Matrix<A_ROWS, A_COLS>,
    m2: &Matrix<A_COLS, B_COLS>,
    index: (usize, usize),
) -> f32 {
    let (target_row, target_col) = index;

    let mut result = 0.0;
    for i in 0..A_COLS {
        let m1_elem = m1[(target_row, i)];
        let m2_elem = m2[(i, target_col)];

        result += m1_elem * m2_elem
    }

    result
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiplying_two_matrices() {
        let matrix_1 = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let matrix_2 = Matrix::new([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);

        let expected_result = Matrix::new([
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        ]);
        assert_eq!(expected_result, matrix_1 * matrix_2)
    }

    #[test]
    fn multiplying_matrix_by_identity_matrix_returns_original_matrix() {
        let matrix = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        assert_eq!(matrix, matrix * Matrix::<4, 4>::identity());
    }
}
