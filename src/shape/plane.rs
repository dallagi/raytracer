use crate::float_eq::FloatEq;
use crate::intersection::Intersection;
use crate::intersections::Intersections;
use crate::object::Object;
use crate::point::Point;
use crate::ray::Ray;
use crate::vector::Vector;

pub fn object_normal_at(_object_point: Point) -> Vector {
    // The plane will extend in both x and z dimentions
    // so the normal will always be x=0, y=1, z=0
    Vector::new(0.0, 1.0, 0.0)
}

pub fn object_intersect_at(object: Object, ray: Ray) -> Intersections {
    if ray.direction.y.float_eq(0.0) {
        return Intersections::empty();
    }

    let t = (-ray.origin.y) / (ray.direction.y);

    Intersections::of(&[Intersection::new(t, object)])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_of_plane_is_constant_everywhere() {
        assert_eq!(
            Vector::new(0.0, 1.0, 0.0),
            object_normal_at(Point::new(0.0, 0.0, 0.0))
        );
        assert_eq!(
            Vector::new(0.0, 1.0, 0.0),
            object_normal_at(Point::new(10.0, 0.0, -10.0))
        );
        assert_eq!(
            Vector::new(0.0, 1.0, 0.0),
            object_normal_at(Point::new(-5.0, 0.0, 150.0))
        );
    }

    #[test]
    fn ray_parallel_to_plane_wont_intersect_it() {
        let ray = Ray::new(Point::new(0.0, 10.0, 0.0), Vector::new(0.0, 0.0, 1.0));

        assert_eq!(0, object_intersect_at(Object::default(), ray).count())
    }

    #[test]
    fn ray_coplanar_to_plane_wont_intersect_it() {
        // a ray is coplanar when it originates within the plane and it is parallel to it.
        // note that mathematically a coplanar ray would intersect the plane infinite times.
        // however from the raytracer perspective it won't intersect it at all since the
        // plane is infinitely thin and wouldn't be visible when viewed like this.

        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));

        assert_eq!(0, object_intersect_at(Object::default(), ray).count())
    }

    #[test]
    fn ray_intersecting_plane_from_above() {
        let ray = Ray::new(Point::new(0.0, 1.0, 0.0), Vector::new(0.0, -1.0, 0.0));

        let intersections = object_intersect_at(Object::default(), ray);

        assert_eq!(1, intersections.count());
        assert_eq!(1.0, intersections[0].t);
        assert_eq!(Object::default(), intersections[0].object)
    }

    #[test]
    fn ray_intersecting_plane_from_below() {
        let ray = Ray::new(Point::new(0.0, -1.0, 0.0), Vector::new(0.0, 1.0, 0.0));

        let intersections = object_intersect_at(Object::default(), ray);

        assert_eq!(1, intersections.count());
        assert_eq!(1.0, intersections[0].t);
        assert_eq!(Object::default(), intersections[0].object)
    }
}
