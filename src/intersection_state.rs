use crate::intersection::Intersection;
use crate::point::Point;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vector::Vector;

/// Precomputed state for an intersection
#[derive(Clone)]
pub struct IntersectionState {
    pub t: f64,
    pub object: Sphere,
    pub point: Point,
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

        Self {
            t,
            object,
            point,
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
    use super::*;

    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let object = Sphere::default();
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
        let object = Sphere::default();
        let intersection = Intersection::new(4.0, object);

        let intersection_state = IntersectionState::prepare(intersection, ray);

        assert_eq!(false, intersection_state.inside);
    }

    #[test]
    fn if_hit_happened_inside_the_object_inside_is_true_and_normal_is_inverted() {
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let object = Sphere::default();
        let intersection = Intersection::new(1.0, object);

        let intersection_state = IntersectionState::prepare(intersection, ray);

        assert_eq!(true, intersection_state.inside);
        assert_eq!(Point::new(0.0, 0.0, 1.0), intersection_state.point);
        assert_eq!(Vector::new(0.0, 0.0, -1.0), intersection_state.eye_v);
        assert_eq!(Vector::new(0.0, 0.0, -1.0), intersection_state.normal_v);
    }
}
