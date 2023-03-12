use crate::matrix::transformations;
use crate::matrix::Matrix;
use crate::point::Point;
use crate::vector::Vector;

/// Transformation matrix to move the eye from the default orientation to a desired one.
///
/// * `from` - Desired eye position.
/// * `to` - Point in the scene the eye should be looking at.
/// * `up` - Which direction is up. No need for this vector to be normalized or perpendicular,
///          it should just roughly point to the correct direction.
pub fn view_transform(from: Point, to: Point, up: Vector) -> Matrix<4, 4> {
    let forward = (to - from).normalize();
    let left = forward.cross(up.normalize());

    // Make `up` normalized and perpendicular
    let true_up = left.cross(forward);

    // First translate the scene away from the eye
    let translation = transformations::translation(-from.x, -from.y, -from.z);

    // Then transform the orientation of the scene
    let orientation = Matrix::new([
        [left.x, left.y, left.z, 0.0],
        [true_up.x, true_up.y, true_up.z, 0.0],
        [-forward.x, -forward.y, -forward.z, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);

    translation >> orientation
}

#[cfg(test)]
mod tests {
    use crate::matrix::transformations;

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn transformation_matrix_for_default_orientation_is_identity_matrix() {
        let transformation_matrix = view_transform(
            Point::origin(),
            Point::new(0.0, 0.0, -1.0),
            Vector::new(0.0, 1.0, 0.0),
        );

        assert_eq!(Matrix::<4, 4>::identity(), transformation_matrix)
    }

    #[test]
    fn looking_at_positive_z_mirrors_the_image() {
        // ie. it swaps front/back and left/right

        let transformation_matrix = view_transform(
            Point::origin(),
            Point::new(0.0, 0.0, 1.0),
            Vector::new(0.0, 1.0, 0.0),
        );

        // remember: reflection is the same as scaling by a negative value
        assert_eq!(
            transformations::scaling(-1.0, 1.0, -1.0),
            transformation_matrix
        )
    }

    #[test]
    fn view_transformation_moves_the_world_to_simulate_the_eye_being_in_the_desired_position() {
        let transformation_matrix = view_transform(
            Point::new(0.0, 0.0, 8.0), // we set the eye at 8 units of depth
            Point::new(0.0, 0.0, 0.0), // eye is looking at origin
            Vector::new(0.0, 1.0, 0.0),
        );

        assert_eq!(
            // the world gets moved 8 units of depth away from the eye
            transformations::translation(0.0, 0.0, -8.0),
            transformation_matrix
        )
    }

    #[test]
    fn view_transformation_works_appropriately_for_arbitrarely_inputs() {
        let transformation_matrix = view_transform(
            Point::new(1.0, 3.0, 2.0),
            Point::new(4.0, -2.0, 8.0),
            Vector::new(1.0, 1.0, 0.0),
        );

        let expected = Matrix::new([
            [-0.50709, 0.50709, 0.67612, -2.36643],
            [0.76772, 0.60609, 0.12122, -2.82843],
            [-0.35857, 0.59761, -0.71714, 0.00000],
            [0.00000, 0.00000, 0.00000, 1.00000],
        ]);

        assert_eq!(expected, transformation_matrix)
    }
}
