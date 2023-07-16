use crate::color::Color;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::point::Point;
use crate::shape::Shape;
use crate::vector::Vector;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Object {
    pub shape: Shape,
    pub transformation: Matrix<4, 4>,
    pub material: Material,
}

impl Object {
    pub fn sphere(transformation: Matrix<4, 4>, material: Material) -> Self {
        Self {
            transformation,
            material,
            shape: Shape::Sphere,
        }
    }

    pub fn plane(transformation: Matrix<4, 4>, material: Material) -> Self {
        Self {
            transformation,
            material,
            shape: Shape::Plane,
        }
    }

    pub fn normal_at(self, world_point: Point) -> Vector {
        let inverse_transformation = self.transformation.inverse();
        let object_point = inverse_transformation * world_point;

        let object_normal = self.shape.object_normal_at(object_point);

        let world_normal = inverse_transformation.transpose() * object_normal;
        world_normal.normalize()
    }

    pub fn object_color_at(self, point: Point) -> Color {
        self.material.pattern.object_color_at(self, point)
    }
}

impl Default for Object {
    fn default() -> Self {
        Self::sphere(Matrix::identity(), Material::default())
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::color::Color;
    use crate::matrix::transformations;
    use crate::pattern::Pattern;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn an_object_default_transformation_is_identity_matrix() {
        let object = Object::default();

        assert_eq!(Matrix::identity(), object.transformation);
    }

    #[test]
    fn an_object_transformation_can_be_changed() {
        let mut object = Object::default();
        let transformation = transformations::translation(1.0, 2.0, 3.0);
        object.transformation = transformation;

        assert_eq!(transformation, object.transformation);
    }

    #[test]
    fn normal_is_a_normalized_vector() {
        let object = Object::default();
        let normal = object.normal_at(Point::new(0.0, 0.0, 1.0));
        assert_eq!(normal, normal.normalize());
    }

    #[test]
    fn computing_normal_on_translated_object() {
        let mut object = Object::default();
        object.transformation = transformations::translation(0.0, 1.0, 0.0);

        let normal = object.normal_at(Point::new(0.0, 1.70711, -0.70711));

        assert_eq!(normal, Vector::new(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn computing_normal_on_transformed_object() {
        let mut object = Object::default();
        let transformation =
            transformations::scaling(1.0, 0.5, 1.0) * transformations::rotation_z(PI / 5.0);
        object.transformation = transformation;

        let normal = object.normal_at(Point::new(0.0, f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0));

        assert_eq!(normal, Vector::new(0.0, 0.97014, -0.24254));
    }

    #[test]
    fn object_has_default_material() {
        let object = Object::default();

        assert_eq!(Material::default(), object.material);
    }

    #[test]
    fn object_may_be_assigned_a_material() {
        let mut object = Object::default();
        let material = Material {
            ambient: 1.0,
            ..Default::default()
        };

        object.material = material;

        assert_eq!(material, object.material);
    }

    #[test]
    fn material_pattern_is_transformed_together_with_object() {
        let transformation = transformations::scaling(2.0, 2.0, 2.0);
        let material = Material {
            pattern: Pattern::stripe(Color::white(), Color::black(), Matrix::identity()),
            ..Material::default()
        };

        let object = Object::sphere(transformation, material);

        let color = object.object_color_at(Point::new(1.5, 0.0, 0.0));
    }
}
