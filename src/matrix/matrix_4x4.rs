use crate::{float_eq::FloatEq, matrix::Matrix};

const SIZE: usize = 4;

impl Matrix<SIZE, SIZE> {
    pub fn minor(self, row: usize, column: usize) -> f32 {
        self.submatrix(row, column).determinant()
    }

    pub fn cofactor(self, row: usize, column: usize) -> f32 {
        let res = self.minor(row, column);

        if (row + column) % 2 == 0 {
            res
        } else {
            -res
        }
    }

    pub fn determinant(self) -> f32 {
        (0..SIZE)
            .map(|row| self[(row, 0)] * self.cofactor(row, 0))
            .sum()
    }

    pub fn inverse(self) -> Self {
        let determinant = self.determinant();
        if determinant.float_eq(0.0) {
            panic!("Cannot invert matrix with determinant 0");
        }
        let mut res = self.cofactor_matrix().transpose();
        for r in 0..SIZE {
            for c in 0..SIZE {
                res[(r, c)] = res[(r, c)] / determinant;
            }
        }
        res
    }

    fn cofactor_matrix(self) -> Self {
        let mut res = Self::zeros();
        for r in 0..SIZE {
            for c in 0..SIZE {
                res[(r, c)] = self.cofactor(r, c);
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn determinant_of_4x4_matrix() {
        let matrix = Matrix::new([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ]);

        assert_eq!(690.0, matrix.cofactor(0, 0));
        assert_eq!(447.0, matrix.cofactor(0, 1));
        assert_eq!(210.0, matrix.cofactor(0, 2));
        assert_eq!(51.0, matrix.cofactor(0, 3));
        assert_eq!(-4071.0, matrix.determinant());
    }

    #[test]
    fn calculates_inverse_of_matrix() {
        let matrix = Matrix::new([
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        ]);

        let expected_inverse = Matrix::new([
            [0.21805, 0.45113, 0.24060, -0.04511],
            [-0.80827, -1.45677, -0.44361, 0.52068],
            [-0.07895, -0.22368, -0.05263, 0.19737],
            [-0.52256, -0.81391, -0.30075, 0.30639],
        ]);
        assert_eq!(532.0, matrix.determinant());
        assert_eq!(-160.0, matrix.cofactor(2, 3));
        assert!((-160.0 / 532.0).float_eq(expected_inverse[(3, 2)]));
        assert_eq!(expected_inverse, matrix.inverse());
    }

    #[test]
    fn inverse_of_another_matrix() {
        let matrix = Matrix::new([
            [8.0, -5.0, 9.0, 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0],
        ]);

        let expected_inverse = Matrix::new([
            [-0.15385, -0.15385, -0.28205, -0.53846],
            [-0.07692, 0.12308, 0.02564, 0.03077],
            [0.35897, 0.35897, 0.43590, 0.92308],
            [-0.69231, -0.69231, -0.76923, -1.92308],
        ]);
        assert_eq!(expected_inverse, matrix.inverse())
    }

    #[test]
    fn inverse_of_a_third_matrix() {
        let matrix = Matrix::new([
            [9.0, 3.0, 0.0, 9.0],
            [-5.0, -2.0, -6.0, -3.0],
            [-4.0, 9.0, 6.0, 4.0],
            [-7.0, 6.0, 6.0, 2.0],
        ]);

        let expected_inverse = Matrix::new([
            [-0.04074, -0.07778, 0.14444, -0.22222],
            [-0.07778, 0.03333, 0.36667, -0.33333],
            [-0.02901, -0.14630, -0.10926, 0.12963],
            [0.17778, 0.06667, -0.26667, 0.33333],
        ]);
        assert_eq!(expected_inverse, matrix.inverse())
    }

    #[test]
    #[should_panic]
    fn inversion_panics_when_matrix_is_not_invertible() {
        let matrix = Matrix::new([
            [-4.0, 2.0, -2.0, 3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);

        matrix.inverse();
    }

    #[test]
    fn multiplying_a_product_by_its_inverse() {
        let matrix_1 = Matrix::new([
            [9.0, 3.0, 0.0, 9.0],
            [-5.0, -2.0, -6.0, -3.0],
            [-4.0, 9.0, 6.0, 4.0],
            [-7.0, 6.0, 6.0, 2.0],
        ]);
        let matrix_2 = Matrix::new([
            [8.0, -5.0, 9.0, 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0],
        ]);

        assert_eq!(matrix_1, matrix_1 * matrix_2 * matrix_2.inverse())
    }
}
