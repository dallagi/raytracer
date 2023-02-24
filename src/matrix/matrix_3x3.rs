use crate::matrix::Matrix;

const SIZE: usize = 3;

impl Matrix<SIZE, SIZE> {
    pub fn minor(self, row: usize, column: usize) -> f64 {
        self.submatrix(row, column).determinant()
    }

    pub fn cofactor(self, row: usize, column: usize) -> f64 {
        let res = self.minor(row, column);

        if (row + column) % 2 == 0 {
            res
        } else {
            -res
        }
    }

    pub fn determinant(self) -> f64 {
        (0..SIZE)
            .map(|row| self[(row, 0)] * self.cofactor(row, 0))
            .sum()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minor_of_3x3_matrix() {
        let matrix = Matrix::new([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        let sub = matrix.submatrix(1, 0);

        assert_eq!(25.0, sub.determinant());
        assert_eq!(25.0, matrix.minor(1, 0));
    }

    #[test]
    fn cofactor_of_3x3_matrix() {
        let matrix = Matrix::new([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);

        assert_eq!(-12.0, matrix.minor(0, 0));
        assert_eq!(-12.0, matrix.cofactor(0, 0));
        assert_eq!(25.0, matrix.minor(1, 0));
        assert_eq!(-25.0, matrix.cofactor(1, 0));
    }

    #[test]
    fn determinant_of_3x3_matrix() {
        let matrix = Matrix::new([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);

        assert_eq!(56.0, matrix.cofactor(0, 0));
        assert_eq!(12.0, matrix.cofactor(0, 1));
        assert_eq!(-46.0, matrix.cofactor(0, 2));
        assert_eq!(-196.0, matrix.determinant());
    }
}
