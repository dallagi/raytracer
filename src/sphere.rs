use crate::matrix::Matrix;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Sphere {
    pub transformation: Matrix<4, 4>,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            transformation: Matrix::identity(),
        }
    }

    pub fn set_transformation(&mut self, transformation_matrix: Matrix<4, 4>) {
        self.transformation = transformation_matrix;
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::transformations;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn a_sphere_default_transformation_is_identity_matrix() {
        let sphere = Sphere::new();

        assert_eq!(Matrix::identity(), sphere.transformation);
    }

    #[test]
    fn an_object_transformation_can_be_changed() {
        let mut sphere = Sphere::new();
        let transformation = transformations::translation(1.0, 2.0, 3.0);
        sphere.set_transformation(transformation);

        assert_eq!(transformation, sphere.transformation);
    }
}
