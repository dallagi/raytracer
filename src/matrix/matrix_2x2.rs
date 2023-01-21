use crate::matrix::Matrix;

const SIZE: usize = 2;

impl Matrix<SIZE, SIZE> {
    pub fn determinant(self) -> f32 {
        self[(0, 0)] * self[(1, 1)] - self[(0, 1)] * self[(1, 0)]
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn determinant_of_2x2_matrix() {
        let matrix = Matrix::new([[1.0, 5.0], [-3.0, 2.0]]);

        assert_eq!(17.0, matrix.determinant());
    }
}
