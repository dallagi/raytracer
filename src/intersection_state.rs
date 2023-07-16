use crate::intersection::Intersection;
use crate::object::Object;
use crate::point::Point;
use crate::ray::Ray;
use crate::vector::Vector;

const EPSILON: f64 = 0.000000001;

/// Precomputed state for an intersection
#[derive(Clone, Debug)]
pub struct IntersectionState {
    pub t: f64,
    pub object: Object,
    pub point: Point,
    /// point slightly moved towards the direction of the normal
    /// this will be used when testing for shadows, in order to bump
    /// the point over the surface and avoid self-shadowing due to
    /// unreliable floating point computations.
    pub over_point: Point,
    pub eye_v: Vector,
    pub normal_v: Vector,
    /// whether the hit occurred inside the object
    pub inside: bool,
}

impl IntersectionState {
    pub fn prepare(intersection: Intersection, ray: Ray) -> Self {
        let t = intersection.t;
        let object = intersection.object;
        let point = ray.position(t);

        let eye_v = -ray.direction;
        let mut normal_v = object.normal_at(point);

        let mut inside = false;

        if Self::inside_object(normal_v, eye_v) {
            inside = true;
            // reverse the normal, since we're inside the object
            normal_v = -normal_v;
        }

        let over_point = point + normal_v * EPSILON;

        Self {
            t,
            object,
            point,
            over_point,
            eye_v,
            normal_v,
            inside,
        }
    }

    fn inside_object(normal_v: Vector, eye_v: Vector) -> bool {
        // if normal vector (roughly) points away from the eye vector
        // then we're probably inside the object.
        // Remember that the eye vector points *towards* the eye, ie.
        // opposite to the ray directioin.
        normal_v.dot(eye_v) < 0.0
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::transformations;

    use super::*;

    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let object = Object::default();
        let intersection = Intersection::new(4.0, object);

        let intersection_state = IntersectionState::prepare(intersection, ray);

        assert_eq!(intersection.t, intersection_state.t);
        assert_eq!(intersection.object, intersection_state.object);
        assert_eq!(Point::new(0.0, 0.0, -1.0), intersection_state.point);
        assert_eq!(Vector::new(0.0, 0.0, -1.0), intersection_state.eye_v);
        assert_eq!(Vector::new(0.0, 0.0, -1.0), intersection_state.normal_v);
    }

    #[test]
    fn if_hit_happened_outside_the_object_inside_is_false() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let object = Object::default();
        let intersection = Intersection::new(4.0, object);

        let intersection_state = IntersectionState::prepare(intersection, ray);

        assert_eq!(false, intersection_state.inside);
    }

    #[test]
    fn if_hit_happened_inside_the_object_inside_is_true_and_normal_is_inverted() {
        let ray = Ray::new(Point::origin(), Vector::new(0.0, 0.0, 1.0));
        let object = Object::default();
        let intersection = Intersection::new(1.0, object);

        let intersection_state = IntersectionState::prepare(intersection, ray);

        assert_eq!(true, intersection_state.inside);
        assert_eq!(Point::new(0.0, 0.0, 1.0), intersection_state.point);
        assert_eq!(Vector::new(0.0, 0.0, -1.0), intersection_state.eye_v);
        assert_eq!(Vector::new(0.0, 0.0, -1.0), intersection_state.normal_v);
    }

    #[test]
    fn hit_should_offset_the_point() {
        // over_point should be slightly bumped in the direction of the normal
        // compared to the point, to move it above the surface and thus avoid
        // bad results when testing for shadows due to unreliability of float
        // point operations.

        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Object {
            transformation: transformations::translation(0.0, 0.0, 1.0),
            ..Object::default()
        };
        let intersection = Intersection::new(5.0, shape);

        let intersection_state = IntersectionState::prepare(intersection, ray);

        assert!(intersection_state.over_point.z < -EPSILON / 2.0);
        assert!(intersection_state.point.z > intersection_state.over_point.z)
    }
}
