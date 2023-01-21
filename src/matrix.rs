mod eq;
mod index;
mod matrix_2x2;
mod matrix_3x3;
mod matrix_4x4;
mod matrix_nxn;
mod mul;

#[derive(Copy, Clone, Debug)]
struct Matrix<const ROWS: usize, const COLS: usize>([[f32; COLS]; ROWS]);

impl<const ROWS: usize, const COLS: usize> Matrix<ROWS, COLS> {
    pub fn new(content: [[f32; COLS]; ROWS]) -> Self {
        Self(content)
    }

    pub fn zeros() -> Self {
        Self([[0.0; COLS]; ROWS])
    }

    pub fn transpose(self) -> Matrix<COLS, ROWS> {
        let mut res: Matrix<COLS, ROWS> = Matrix::zeros();
        for r in 0..ROWS {
            for c in 0..COLS {
                res[(c, r)] = self[(r, c)]
            }
        }
        res
    }

    fn submatrix(
        self,
        row_to_delete: usize,
        column_to_delete: usize,
    ) -> Matrix<{ ROWS - 1 }, { COLS - 1 }> {
        let mut res: Matrix<{ ROWS - 1 }, { COLS - 1 }> = Matrix::zeros();
        for row in 0..(ROWS - 1) {
            for col in 0..(COLS - 1) {
                let source_row = if row < row_to_delete { row } else { row + 1 };
                let source_col = if col < column_to_delete { col } else { col + 1 };

                res[(row, col)] = self[(source_row, source_col)]
            }
        }
        res
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
    fn transpose_swaps_rows_and_columns() {
        let matrix = Matrix::new([
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ]);

        let expected_result = Matrix::new([
            [0.0, 9.0, 1.0, 0.0],
            [9.0, 8.0, 8.0, 0.0],
            [3.0, 0.0, 5.0, 5.0],
            [0.0, 8.0, 3.0, 8.0],
        ]);
        assert_eq!(expected_result, matrix.transpose())
    }

    #[test]
    fn transposing_the_identity_matrix_returns_identity_matrix() {
        assert_eq!(
            Matrix::<4, 4>::identity(),
            Matrix::<4, 4>::identity().transpose()
        )
    }

    #[test]
    fn submatrix_of_3x3_matrix_is_2x2_matrix() {
        let matrix = Matrix::new([[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]]);

        assert_eq!(
            Matrix::new([[-3.0, 2.0], [0.0, 6.0]]),
            matrix.submatrix(0, 2)
        )
    }

    #[test]
    fn submatrix_of_4x4_matrix_is_3x3_matrix() {
        let matrix = Matrix::new([
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0],
        ]);

        let expected_result = Matrix::new([[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0]]);
        assert_eq!(expected_result, matrix.submatrix(2, 1))
    }
}
