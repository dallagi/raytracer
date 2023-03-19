use crate::intersection::Intersection;
use crate::intersections::Intersections;
use crate::object::Object;
use crate::point::Point;
use crate::ray::Ray;
use crate::vector::Vector;

/// Normal of sphere at the given point in the object-space.
pub fn object_normal_at(object_point: Point) -> Vector {
    (object_point - Point::origin()).normalize()
}

/// Intersections of object-space ray with sphere.
///
/// Returns the values of t at which the ray will intersect a sphere.
/// Returns None if no intersection exists.
/// For single intersections (ie. tangent lines), it will return the same t two times.
///
/// See https://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection
pub fn object_intersect_at(sphere: Object, object_ray: Ray) -> Intersections {
    let sphere_center = Point::origin();
    let sphere_center_to_ray = object_ray.origin - sphere_center;

    let a = object_ray.direction.dot(object_ray.direction);
    let b = 2.0 * object_ray.direction.dot(sphere_center_to_ray);
    let c = sphere_center_to_ray.dot(sphere_center_to_ray) - 1.0;

    let discriminant = b.powf(2.0) - 4.0 * a * c;

    if discriminant < 0.0 {
        return Intersections::empty();
    };

    let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
    let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

    Intersections::of(&[Intersection::new(t1, sphere), Intersection::new(t2, sphere)])
}

#[cfg(test)]
mod tests {
    use crate::object::Object;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn normal_on_sphere_at_x_axis() {
        let point = Point::new(1.0, 0.0, 0.0);
        let normal = object_normal_at(point);
        assert_eq!(normal, Vector::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn normal_on_sphere_at_y_axis() {
        let sphere = Object::default();
        let normal = object_normal_at(Point::new(0.0, 1.0, 0.0));
        assert_eq!(normal, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn normal_on_sphere_at_z_axis() {
        let sphere = Object::default();
        let normal = object_normal_at(Point::new(0.0, 0.0, 1.0));
        assert_eq!(normal, Vector::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn normal_on_sphere_at_nonaxial_point() {
        let sphere = Object::default();
        let point = Point::new(3f64.sqrt() / 3.0, 3f64.sqrt() / 3.0, 3f64.sqrt() / 3.0);
        let normal = object_normal_at(point);
        assert_eq!(
            normal,
            Vector::new(3f64.sqrt() / 3.0, 3f64.sqrt() / 3.0, 3f64.sqrt() / 3.0)
        );
    }

    #[test]
    fn a_ray_can_intersect_a_sphere_at_two_points() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Object::default();

        // t values at which the ray intersects the sphere
        let intersections = object_intersect_at(sphere, ray);

        assert_eq!(2, intersections.count());
        assert_eq!(4.0, intersections[0].t);
        assert_eq!(6.0, intersections[1].t);
    }

    #[test]
    fn a_ray_can_miss_a_sphere() {
        let ray = Ray::new(Point::new(0.0, 2.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Object::default();

        let intersect_ts = object_intersect_at(sphere, ray);

        assert_eq!(Intersections::empty(), intersect_ts)
    }

    #[test]
    fn a_ray_originating_inside_the_sphere_intersects_the_sphere_in_two_points() {
        let ray = Ray::new(Point::origin(), Vector::new(0.0, 0.0, 1.0));
        let sphere = Object::default();

        let intersections = ray.intersect(sphere);

        assert_eq!(2, intersections.count());
        assert_eq!(-1.0, intersections[0].t);
        assert_eq!(1.0, intersections[1].t);
    }

    #[test]
    fn a_ray_can_intersect_a_sphere_behind_it_two_times() {
        let ray = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Object::default();

        let intersections = object_intersect_at(sphere, ray);

        assert_eq!(2, intersections.count());
        assert_eq!(-6.0, intersections[0].t);
        assert_eq!(-4.0, intersections[1].t);
    }

    #[test]
    fn intersect_sets_the_object_of_the_intersection() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Object::default();

        let intersections = object_intersect_at(sphere, ray);

        assert_eq!(sphere, intersections[0].object);
        assert_eq!(sphere, intersections[1].object);
    }
}
