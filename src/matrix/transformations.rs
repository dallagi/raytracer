use crate::matrix::Matrix;

pub fn translation(x: f32, y: f32, z: f32) -> Matrix<4, 4> {
    let mut result: Matrix<4, 4> = Matrix::identity();

    result[(0, 3)] = x;
    result[(1, 3)] = y;
    result[(2, 3)] = z;

    result
}

#[cfg(test)]
mod tests {
    use crate::{point::Point, vector::Vector};

    use super::*;

    #[test]
    fn multiplying_by_translation_matrix() {
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
}
