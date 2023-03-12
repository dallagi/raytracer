use crate::{float_eq::FloatEq, vector::Vector};
use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub const W: f64 = 1.0;

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn origin() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x.float_eq(other.x) && self.y.float_eq(other.y) && self.z.float_eq(other.z)
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
