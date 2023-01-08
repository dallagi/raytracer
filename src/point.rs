use crate::{f_equals::FEquals, vector::Vector};
use std::ops;

#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x.f_equals(other.x) && self.y.f_equals(other.y) && self.z.f_equals(other.z)
    }
}

impl ops::Add<Vector> for Point {
    type Output = Point;

    fn add(self, vector: Vector) -> Self::Output {
        Point::new(self.x + vector.x, self.y + vector.y, self.z + vector.z)
    }
}

impl ops::Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, vector: Vector) -> Self::Output {
        Point::new(self.x - vector.x, self.y - vector.y, self.z - vector.z)
    }
}

impl ops::Sub<Point> for Point {
    type Output = Vector;

    fn sub(self, other: Point) -> Self::Output {
        Vector::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding_vector_to_point_results_in_new_point() {
        let point = Point::new(3.0, -2.0, 5.0);
        let vector = Vector::new(-2.0, 3.0, 1.0);

        assert_eq!(Point::new(1.0, 1.0, 6.0), point + vector);
    }

    #[test]
    fn substracting_two_points_results_in_vector() {
        let p1 = Point::new(3.0, 2.0, 1.0);
        let p2 = Point::new(5.0, 6.0, 7.0);

        assert_eq!(Vector::new(-2.0, -4.0, -6.0), p1 - p2);
    }

    #[test]
    fn substracting_vector_from_point_results_in_point() {
        let point = Point::new(3.0, 2.0, 1.0);
        let vector = Vector::new(5.0, 6.0, 7.0);

        assert_eq!(Point::new(-2.0, -4.0, -6.0), point - vector);
    }
}
