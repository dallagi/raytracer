use crate::matrix::Matrix;

impl<const SIZE: usize> Matrix<SIZE, SIZE> {
    pub fn identity() -> Self {
        let mut res: Matrix<SIZE, SIZE> = Self::zeros();
        for i in 0..SIZE {
            res[(i, i)] = 1.0;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_identity_matrix() {
        assert_eq!(
            Matrix::new([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]),
            Matrix::<3, 3>::identity()
        )
    }
}
