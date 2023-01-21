use crate::matrix::Matrix;

pub fn translation(x: f32, y: f32, z: f32) -> Matrix<4, 4> {
    let mut result: Matrix<4, 4> = Matrix::identity();

    result[(0, 3)] = x;
    result[(1, 3)] = y;
    result[(2, 3)] = z;

    result
}

pub fn scaling(x: f32, y: f32, z: f32) -> Matrix<4, 4> {
    let mut result: Matrix<4, 4> = Matrix::identity();

    result[(0, 0)] = x;
    result[(1, 1)] = y;
    result[(2, 2)] = z;

    result
}

#[cfg(test)]
mod tests {
    use crate::{point::Point, vector::Vector};

    use super::*;

    #[test]
    fn translation_moves_a_point() {
        let transform = translation(5.0, -3.0, 2.0);
        let point = Point::new(-3.0, 4.0, 5.0);

        assert_eq!(Point::new(2.0, 1.0, 7.0), transform * point);
    }

    #[test]
    fn inverse_of_translation_moves_point_in_reverse() {
        let transform = translation(5.0, -3.0, 2.0).inverse();
        let point = Point::new(-3.0, 4.0, 5.0);

        assert_eq!(Point::new(-8.0, 7.0, 3.0), transform * point);
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let transform = translation(5.0, -3.0, 2.0).inverse();
        let vector = Vector::new(-3.0, 4.0, 5.0);

        assert_eq!(vector, transform * vector);
    }

    #[test]
    fn scaling_moves_a_point_by_multiplication() {
        let transform = scaling(2.0, 3.0, 4.0);
        let point = Point::new(-4.0, 6.0, 8.0);

        assert_eq!(Point::new(-8.0, 18.0, 32.0), transform * point);
    }

    #[test]
    fn scaling_changes_length_of_vector() {
        let transform = scaling(2.0, 3.0, 4.0);
        let vector = Vector::new(-4.0, 6.0, 8.0);

        assert_eq!(Vector::new(-8.0, 18.0, 32.0), transform * vector);
    }

    #[test]
    fn inverse_of_scaling_shrinks_a_vector() {
        let transform = scaling(2.0, 3.0, 4.0).inverse();
        let vector = Vector::new(-4.0, 6.0, 8.0);

        assert_eq!(Vector::new(-2.0, 2.0, 2.0), transform * vector);
    }
}
