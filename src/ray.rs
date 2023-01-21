use crate::point::Point;
use crate::sphere::Sphere;
use crate::vector::Vector;

#[derive(Copy, Clone)]
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
    pub fn intersect_sphere(self, sphere: Sphere) -> Option<[f32; 2]> {
        let sphere_center = Point::new(0.0, 0.0, 0.0);
        let sphere_center_to_ray = self.origin - sphere_center;

        let a = self.direction.dot(self.direction);
        let b = 2.0 * self.direction.dot(sphere_center_to_ray);
        let c = sphere_center_to_ray.dot(sphere_center_to_ray) - 1.0;

        let discriminant = b.powf(2.0) - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        };

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        if t1 < t2 {
            Some([t1, t2])
        } else {
            Some([t2, t1])
        }
    }
}

#[cfg(test)]
mod tests {
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
        let sphere = Sphere::new();

        // t values at which the ray intersects the sphere
        let intersect_ts = ray.intersect_sphere(sphere);

        assert_eq!(Some([4.0, 6.0]), intersect_ts)
    }

    #[test]
    fn a_ray_can_miss_a_sphere() {
        let ray = Ray::new(Point::new(0.0, 2.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new();

        let intersect_ts = ray.intersect_sphere(sphere);

        assert_eq!(None, intersect_ts)
    }

    #[test]
    fn a_ray_originating_inside_the_sphere_intersects_the_sphere_in_two_points() {
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new();

        let intersect_ts = ray.intersect_sphere(sphere);

        assert_eq!(Some([-1.0, 1.0]), intersect_ts)
    }

    #[test]
    fn a_ray_can_intersect_a_sphere_behind_it_two_times() {
        let ray = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new();

        let intersect_ts = ray.intersect_sphere(sphere);

        assert_eq!(Some([-6.0, -4.0]), intersect_ts)
    }
}
