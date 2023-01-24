use crate::intersection::Intersection;
use crate::intersections::Intersections;
use crate::matrix::Matrix;
use crate::point::Point;
use crate::sphere::Sphere;
use crate::vector::Vector;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Self { origin, direction }
    }

    pub fn position(self, t: f32) -> Point {
        self.origin + self.direction * t
    }

    /// Returns the values of t at which the ray will intersect a sphere.
    /// Returns None if no intersection exists.
    /// For single intersections (ie. tangent lines), it will return the same t two times.
    ///
    /// See https://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection
    pub fn intersect(self, object: Sphere) -> Intersections {
        let transformed_ray = self.transform(object.transformation.inverse());

        let sphere_center = Point::new(0.0, 0.0, 0.0);
        let sphere_center_to_ray = transformed_ray.origin - sphere_center;

        let a = transformed_ray.direction.dot(transformed_ray.direction);
        let b = 2.0 * transformed_ray.direction.dot(sphere_center_to_ray);
        let c = sphere_center_to_ray.dot(sphere_center_to_ray) - 1.0;

        let discriminant = b.powf(2.0) - 4.0 * a * c;

        if discriminant < 0.0 {
            return Intersections::empty();
        };

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        if t1 < t2 {
            Intersections::of(&[Intersection::new(t1, object), Intersection::new(t2, object)])
        } else {
            Intersections::of(&[Intersection::new(t2, object), Intersection::new(t1, object)])
        }
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
    use crate::{matrix::transformations, sphere::Sphere};

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
    fn a_ray_can_intersect_a_sphere_at_two_points() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::default();

        // t values at which the ray intersects the sphere
        let intersections = ray.intersect(sphere);

        assert_eq!(2, intersections.count());
        assert_eq!(4.0, intersections[0].t);
        assert_eq!(6.0, intersections[1].t);
    }

    #[test]
    fn a_ray_can_miss_a_sphere() {
        let ray = Ray::new(Point::new(0.0, 2.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::default();

        let intersect_ts = ray.intersect(sphere);

        assert_eq!(Intersections::empty(), intersect_ts)
    }

    #[test]
    fn a_ray_originating_inside_the_sphere_intersects_the_sphere_in_two_points() {
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::default();

        let intersections = ray.intersect(sphere);

        assert_eq!(2, intersections.count());
        assert_eq!(-1.0, intersections[0].t);
        assert_eq!(1.0, intersections[1].t);
    }

    #[test]
    fn a_ray_can_intersect_a_sphere_behind_it_two_times() {
        let ray = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::default();

        let intersections = ray.intersect(sphere);

        assert_eq!(2, intersections.count());
        assert_eq!(-6.0, intersections[0].t);
        assert_eq!(-4.0, intersections[1].t);
    }

    #[test]
    fn intersect_sets_the_object_of_the_intersection() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::default();

        let intersections = ray.intersect(sphere);

        assert_eq!(sphere, intersections[0].object);
        assert_eq!(sphere, intersections[1].object);
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
        let mut sphere = Sphere::default();
        sphere.transformation = transformations::scaling(2.0, 2.0, 2.0);

        let intersections = ray.intersect(sphere);

        assert_eq!(2, intersections.count());
        assert_eq!(3.0, intersections[0].t);
        assert_eq!(7.0, intersections[1].t);
    }
    #[test]
    fn intersect_can_translate_ray_before_calculation() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut sphere = Sphere::default();
        sphere.transformation = transformations::translation(5.0, 0.0, 0.0);

        let intersections = ray.intersect(sphere);

        assert_eq!(0, intersections.count());
    }
}
