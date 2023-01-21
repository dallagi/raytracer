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

pub fn rotation_x(radians: f32) -> Matrix<4, 4> {
    let mut result: Matrix<4, 4> = Matrix::identity();

    result[(1, 1)] = radians.cos();
    result[(1, 2)] = -radians.sin();
    result[(2, 1)] = radians.sin();
    result[(2, 2)] = radians.cos();

    result
}

pub fn rotation_y(radians: f32) -> Matrix<4, 4> {
    let mut result: Matrix<4, 4> = Matrix::identity();

    result[(0, 0)] = radians.cos();
    result[(0, 2)] = radians.sin();
    result[(2, 0)] = -radians.sin();
    result[(2, 2)] = radians.cos();

    result
}
pub fn rotation_z(radians: f32) -> Matrix<4, 4> {
    let mut result: Matrix<4, 4> = Matrix::identity();

    result[(0, 0)] = radians.cos();
    result[(0, 1)] = -radians.sin();
    result[(1, 0)] = radians.sin();
    result[(1, 1)] = radians.cos();

    result
}

pub fn shearing(
    x_to_y: f32,
    x_to_z: f32,
    y_to_x: f32,
    y_to_z: f32,
    z_to_x: f32,
    z_to_y: f32,
) -> Matrix<4, 4> {
    let mut result: Matrix<4, 4> = Matrix::identity();

    result[(0, 1)] = x_to_y;
    result[(0, 2)] = x_to_z;
    result[(1, 0)] = y_to_x;
    result[(1, 2)] = y_to_z;
    result[(2, 0)] = z_to_x;
    result[(2, 1)] = z_to_y;

    result
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use crate::{point::Point, vector::Vector};
    use pretty_assertions::assert_eq;
    use test_case::test_case;

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

    #[test]
    fn reflection_is_scaling_by_negative_value() {
        let transform = scaling(-1.0, 1.0, 1.0);
        let point = Point::new(2.0, 3.0, 4.0);

        // moves a point on the other side of an axis
        assert_eq!(Point::new(-2.0, 3.0, 4.0), transform * point)
    }

    #[test]
    fn rotation_x_rotates_a_point_around_x_axis() {
        let point = Point::new(0.0, 1.0, 0.0);

        let half_quarter = rotation_x(PI / 4.0);
        let full_quarter = rotation_x(PI / 2.0);

        assert_eq!(
            Point::new(0.0, (2.0_f32).sqrt() / 2.0, (2.0_f32).sqrt() / 2.0),
            half_quarter * point
        );
        assert_eq!(Point::new(0.0, 0.0, 1.0), full_quarter * point);
    }

    #[test]
    fn inverse_of_rotation_x_rotates_a_point_in_the_opposite_direction() {
        let point = Point::new(0.0, 1.0, 0.0);

        let half_quarter = rotation_x(PI / 4.0).inverse();

        assert_eq!(
            Point::new(0.0, (2.0_f32).sqrt() / 2.0, -(2.0_f32).sqrt() / 2.0),
            half_quarter * point
        );
    }

    #[test]
    fn rotation_y_rotates_a_point_around_y_axis() {
        let point = Point::new(0.0, 0.0, 1.0);

        let half_quarter = rotation_y(PI / 4.0);
        let full_quarter = rotation_y(PI / 2.0);

        assert_eq!(
            Point::new((2.0_f32).sqrt() / 2.0, 0.0, (2.0_f32).sqrt() / 2.0),
            half_quarter * point
        );
        assert_eq!(Point::new(1.0, 0.0, 0.0), full_quarter * point);
    }

    #[test]
    fn rotation_z_rotates_a_point_around_z_axis() {
        let point = Point::new(0.0, 1.0, 0.0);

        let half_quarter = rotation_z(PI / 4.0);
        let full_quarter = rotation_z(PI / 2.0);

        assert_eq!(
            Point::new(-(2.0_f32).sqrt() / 2.0, (2.0_f32).sqrt() / 2.0, 0.0),
            half_quarter * point
        );
        assert_eq!(Point::new(-1.0, 0.0, 0.0), full_quarter * point);
    }

    #[test_case(
        shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        Point::new(5.0, 3.0, 4.0);
        "x to y"
    )]
    #[test_case(
        shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0),
        Point::new(6.0, 3.0, 4.0);
        "x to z"
    )]
    #[test_case(
        shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0),
        Point::new(2.0, 5.0, 4.0);
        "y to x"
    )]
    #[test_case(
        shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0),
        Point::new(2.0, 7.0, 4.0);
        "y to z"
    )]
    #[test_case(
        shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0),
        Point::new(2.0, 3.0, 6.0);
        "z to x"
    )]
    #[test_case(
        shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0),
        Point::new(2.0, 3.0, 7.0);
        "z to y"
    )]
    fn shearing_moves_an_axis_in_proportion_to_another_axis(
        transform: Matrix<4, 4>,
        expected_result: Point,
    ) {
        let point = Point::new(2.0, 3.0, 4.0);

        assert_eq!(expected_result, transform * point);
    }
}
