use crate::intersections::Intersections;
use crate::matrix::Matrix;
use crate::object::Object;
use crate::point::Point;
use crate::vector::Vector;
use crate::world::World;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Self { origin, direction }
    }

    pub fn position(self, t: f64) -> Point {
        self.origin + self.direction * t
    }

    pub fn intersect(self, object: Object) -> Intersections {
        let transformed_ray = self.transform(object.transformation.inverse());

        object.shape.object_intersect_at(object, transformed_ray)
    }

    pub fn intersect_world(&self, world: &World) -> Intersections {
        let all_intersections = world
            .objects
            .iter()
            .map(|object| self.intersect(*object))
            .collect();

        Intersections::merge(all_intersections)
    }

    fn transform(self, transformation_matrix: Matrix<4, 4>) -> Self {
        Self {
            origin: transformation_matrix * self.origin,
            direction: transformation_matrix * self.direction,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::material::Material;
    use crate::matrix::transformations;
    use crate::object::Object;
    use crate::world::World;

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn rays_have_an_origin_and_a_direction() {
        let origin = Point::new(1.0, 2.0, 3.0);
        let direction = Vector::new(4.0, 5.0, 6.0);

        let ray = Ray::new(origin, direction);

        assert_eq!(origin, ray.origin);
        assert_eq!(direction, ray.direction);
    }

    #[test]
    fn position_computes_a_point_from_a_distance() {
        let ray = Ray::new(Point::new(2.0, 3.0, 4.0), Vector::new(1.0, 0.0, 0.0));

        assert_eq!(Point::new(2.0, 3.0, 4.0), ray.position(0.0));
        assert_eq!(Point::new(3.0, 3.0, 4.0), ray.position(1.0));
        assert_eq!(Point::new(1.0, 3.0, 4.0), ray.position(-1.0));
        assert_eq!(Point::new(4.5, 3.0, 4.0), ray.position(2.5));
    }

    #[test]
    fn ray_is_transformed_to_object_space_before_calculating_intersection_with_object() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Object::sphere(
            transformations::translation(0.0, 0.0, 5.0),
            Material::default(),
        );

        let intersections = ray.intersect(sphere);

        assert_eq!(2, intersections.count());
        assert_eq!(9.0, intersections[0].t);
        assert_eq!(11.0, intersections[1].t);
    }

    #[test]
    fn can_be_transformed_via_translation() {
        let ray = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
        let transformation = transformations::translation(3.0, 4.0, 5.0);

        assert_eq!(
            Ray::new(Point::new(4.0, 6.0, 8.0), Vector::new(0.0, 1.0, 0.0)),
            ray.transform(transformation)
        )
    }

    #[test]
    fn can_be_transformed_via_scaling() {
        let ray = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
        let transformation = transformations::scaling(2.0, 3.0, 4.0);

        assert_eq!(
            Ray::new(Point::new(2.0, 6.0, 12.0), Vector::new(0.0, 3.0, 0.0)),
            ray.transform(transformation)
        )
    }

    #[test]
    fn intersect_can_scale_ray_before_calculation() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut sphere = Object::default();
        sphere.transformation = transformations::scaling(2.0, 2.0, 2.0);

        let intersections = ray.intersect(sphere);

        assert_eq!(2, intersections.count());
        assert_eq!(3.0, intersections[0].t);
        assert_eq!(7.0, intersections[1].t);
    }

    #[test]
    fn intersect_can_translate_ray_before_calculation() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut sphere = Object::default();
        sphere.transformation = transformations::translation(5.0, 0.0, 0.0);

        let intersections = ray.intersect(sphere);

        assert_eq!(0, intersections.count());
    }

    #[test]
    fn intersect_world_returns_all_intersections_with_objects_in_the_world() {
        let world = World::default();
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));

        let intersections = ray.intersect_world(&world);

        assert_eq!(4, intersections.count());
        assert_eq!(4.0, intersections[0].t);
        assert_eq!(4.5, intersections[1].t);
        assert_eq!(5.5, intersections[2].t);
        assert_eq!(6.0, intersections[3].t);
    }
}
