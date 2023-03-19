use crate::point::Point;
use crate::vector::Vector;

/// Normal of sphere at the given point in the object space.
pub fn object_normal_at(object_point: Point) -> Vector {
    (object_point - Point::origin()).normalize()
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
}
