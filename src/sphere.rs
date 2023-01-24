use crate::{material::Material, matrix::Matrix, point::Point, vector::Vector};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Sphere {
    pub transformation: Matrix<4, 4>,
    pub material: Material,
}

impl Sphere {
    pub fn new(transformation: Matrix<4, 4>, material: Material) -> Self {
        Self {
            transformation,
            material,
        }
    }

    pub fn normal_at(self, world_point: Point) -> Vector {
        let inverse_transformation = self.transformation.inverse();

        let object_point = inverse_transformation * world_point;
        let object_normal = (object_point - self.origin()).normalize();
        let world_normal = inverse_transformation.transpose() * object_normal;

        world_normal.normalize()
    }

    pub fn origin(self) -> Point {
        Point::new(0.0, 0.0, 0.0)
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            transformation: Matrix::identity(),
            material: Default::default(),
        }
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
        let sphere = Sphere::default();

        assert_eq!(Matrix::identity(), sphere.transformation);
    }

    #[test]
    fn an_object_transformation_can_be_changed() {
        let mut sphere = Sphere::default();
        let transformation = transformations::translation(1.0, 2.0, 3.0);
        sphere.transformation = transformation;

        assert_eq!(transformation, sphere.transformation);
    }

    #[test]
    fn normal_on_sphere_at_x_axis() {
        let sphere = Sphere::default();
        let point = Point::new(1.0, 0.0, 0.0);
        let normal = sphere.normal_at(point);
        assert_eq!(normal, Vector::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn normal_on_sphere_at_y_axis() {
        let sphere = Sphere::default();
        let normal = sphere.normal_at(Point::new(0.0, 1.0, 0.0));
        assert_eq!(normal, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn normal_on_sphere_at_z_axis() {
        let sphere = Sphere::default();
        let normal = sphere.normal_at(Point::new(0.0, 0.0, 1.0));
        assert_eq!(normal, Vector::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn normal_on_sphere_at_nonaxial_point() {
        let sphere = Sphere::default();
        let point = Point::new(3f32.sqrt() / 3.0, 3f32.sqrt() / 3.0, 3f32.sqrt() / 3.0);
        let normal = sphere.normal_at(point);
        assert_eq!(
            normal,
            Vector::new(3f32.sqrt() / 3.0, 3f32.sqrt() / 3.0, 3f32.sqrt() / 3.0)
        );
    }

    #[test]
    fn normal_is_a_normalized_vector() {
        let sphere = Sphere::default();
        let normal = sphere.normal_at(Point::new(0.0, 0.0, 1.0));
        assert_eq!(normal, normal.normalize());
    }

    #[test]
    fn computing_normal_on_translated_sphere() {
        let mut sphere = Sphere::default();
        sphere.transformation = transformations::translation(0.0, 1.0, 0.0);

        let normal = sphere.normal_at(Point::new(0.0, 1.70711, -0.70711));

        assert_eq!(normal, Vector::new(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn computing_normal_on_transformed_sphere() {
        let mut sphere = Sphere::default();
        let transformation =
            transformations::scaling(1.0, 0.5, 1.0) * transformations::rotation_z(PI / 5.0);
        sphere.transformation = transformation;

        let normal = sphere.normal_at(Point::new(0.0, f32::sqrt(2.0) / 2.0, -f32::sqrt(2.0) / 2.0));

        assert_eq!(normal, Vector::new(0.0, 0.97014, -0.24254));
    }

    #[test]
    fn sphere_has_default_material() {
        let sphere = Sphere::default();

        assert_eq!(Material::default(), sphere.material);
    }

    #[test]
    fn sphere_may_be_assigned_a_material() {
        let mut sphere = Sphere::default();
        let material = Material {
            ambient: 1.0,
            ..Default::default()
        };

        sphere.material = material;

        assert_eq!(material, sphere.material);
    }
}
