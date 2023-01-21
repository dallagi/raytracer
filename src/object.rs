use crate::matrix::Matrix;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Kind {
    Sphere,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Object {
    pub kind: Kind,
    pub transformation: Matrix<4, 4>,
}

impl Object {
    pub fn sphere() -> Self {
        Self {
            kind: Kind::Sphere,
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
        let sphere = Object::sphere();

        assert_eq!(Matrix::identity(), sphere.transformation);
    }

    #[test]
    fn an_object_transformation_can_be_changed() {
        let mut sphere = Object::sphere();
        let transformation = transformations::translation(1.0, 2.0, 3.0);
        sphere.set_transformation(transformation);

        assert_eq!(transformation, sphere.transformation);
    }
}
