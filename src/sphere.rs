use crate::{matrix::Matrix, point::Point, vector::Vector};

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

    pub fn normal_at(self, world_point: Point) -> Vector {
        let object_point = self.transformation.inverse() * world_point;

        let object_normal = (object_point - self.origin()).normalize();

        let world_normal = self.transformation.inverse().transpose() * object_normal;
        world_normal.normalize()
    }

    pub fn origin(self) -> Point {
        Point::new(0.0, 0.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

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

    #[test]
    fn normal_on_sphere_at_x_axis() {
        let sphere = Sphere::new();
        let point = Point::new(1.0, 0.0, 0.0);
        let normal = sphere.normal_at(point);
        assert_eq!(normal, Vector::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn normal_on_sphere_at_y_axis() {
        let sphere = Sphere::new();
        let normal = sphere.normal_at(Point::new(0.0, 1.0, 0.0));
        assert_eq!(normal, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn normal_on_sphere_at_z_axis() {
        let sphere = Sphere::new();
        let normal = sphere.normal_at(Point::new(0.0, 0.0, 1.0));
        assert_eq!(normal, Vector::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn normal_on_sphere_at_nonaxial_point() {
        let sphere = Sphere::new();
        let point = Point::new(3f32.sqrt() / 3.0, 3f32.sqrt() / 3.0, 3f32.sqrt() / 3.0);
        let normal = sphere.normal_at(point);
        assert_eq!(
            normal,
            Vector::new(3f32.sqrt() / 3.0, 3f32.sqrt() / 3.0, 3f32.sqrt() / 3.0)
        );
    }

    #[test]
    fn normal_is_a_normalized_vector() {
        let sphere = Sphere::new();
        let normal = sphere.normal_at(Point::new(0.0, 0.0, 1.0));
        assert_eq!(normal, normal.normalize());
    }

    #[test]
    fn computing_normal_on_translated_sphere() {
        let mut sphere = Sphere::new();
        sphere.set_transformation(transformations::translation(0.0, 1.0, 0.0));

        let normal = sphere.normal_at(Point::new(0.0, 1.70711, -0.70711));

        assert_eq!(normal, Vector::new(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn computing_normal_on_transformed_sphere() {
        let mut sphere = Sphere::new();
        let transformation =
            transformations::scaling(1.0, 0.5, 1.0) * transformations::rotation_z(PI / 5.0);
        sphere.set_transformation(transformation);

        let normal = sphere.normal_at(Point::new(0.0, f32::sqrt(2.0) / 2.0, -f32::sqrt(2.0) / 2.0));

        assert_eq!(normal, Vector::new(0.0, 0.97014, -0.24254));
    }
}
