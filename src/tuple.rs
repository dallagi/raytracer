use crate::f_equals::FEquals;
use crate::tuple::Kind::{Point, Undefined, Vector};
use std::ops;

#[derive(Debug, PartialEq)]
enum Kind {
    Point,
    Vector,
    Undefined,
}

#[derive(Debug, PartialEq)]
struct Tuple {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

const W_POINT: f32 = 1.0;
const W_VECTOR: f32 = 0.0;

impl Tuple {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn point(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z,
            w: W_POINT,
        }
    }

    pub fn vector(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z,
            w: W_VECTOR,
        }
    }

    pub fn kind(&self) -> Kind {
        match self.w {
            w if w.f_equals(W_VECTOR) => Vector,
            w if w.f_equals(W_POINT) => Point,
            _ => Undefined,
        }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt() as f32
    }
}

impl ops::Add<Tuple> for Tuple {
    type Output = Tuple;

    fn add(self, rhs: Tuple) -> Self::Output {
        Tuple::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl ops::Sub<Tuple> for Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Tuple) -> Self::Output {
        Tuple::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl ops::Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Self::Output {
        Tuple::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl ops::Mul<f32> for Tuple {
    type Output = Tuple;

    fn mul(self, factor: f32) -> Self::Output {
        Tuple::new(
            self.x * factor,
            self.y * factor,
            self.z * factor,
            self.w * factor,
        )
    }
}

impl ops::Div<f32> for Tuple {
    type Output = Tuple;

    fn div(self, factor: f32) -> Self::Output {
        Tuple::new(
            self.x / factor,
            self.y / factor,
            self.z / factor,
            self.w / factor,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_tuple_with_w_1_is_a_point() {
        let t = Tuple::new(4.3, -4.2, 3.1, 1.0);

        assert_eq!(4.3, t.x);
        assert_eq!(-4.2, t.y);
        assert_eq!(3.1, t.z);
        assert_eq!(1.0, t.w);
        assert_eq!(Point, t.kind());
    }

    #[test]
    fn a_tuple_with_w_0_is_a_vector() {
        let t = Tuple::new(4.3, -4.2, 3.1, 0.0);

        assert_eq!(4.3, t.x);
        assert_eq!(-4.2, t.y);
        assert_eq!(3.1, t.z);
        assert_eq!(0.0, t.w);
        assert_eq!(Vector, t.kind());
    }

    #[test]
    fn point_creates_tuple_with_w_1() {
        let p = Tuple::point(4.0, -4.0, 3.0);

        assert_eq!(Tuple::new(4.0, -4.0, 3.0, 1.0), p);
    }

    #[test]
    fn vector_creates_tuple_with_w_0() {
        let v = Tuple::vector(4.0, -4.0, 3.0);

        assert_eq!(Tuple::new(4.0, -4.0, 3.0, 0.0), v);
    }

    #[test]
    fn adding_two_touples() {
        let t1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let t2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);

        assert_eq!(Tuple::new(1.0, 1.0, 6.0, 1.0), t1 + t2);
    }

    #[test]
    fn substract_two_points() {
        let p1 = Tuple::point(3.0, 2.0, 1.0);
        let p2 = Tuple::point(5.0, 6.0, 7.0);

        assert_eq!(Tuple::vector(-2.0, -4.0, -6.0), p1 - p2);
    }

    #[test]
    fn substract_vector_from_point() {
        let point = Tuple::point(3.0, 2.0, 1.0);
        let vector = Tuple::vector(5.0, 6.0, 7.0);

        assert_eq!(Tuple::point(-2.0, -4.0, -6.0), point - vector);
    }

    #[test]
    fn substract_two_vectors() {
        let point = Tuple::vector(3.0, 2.0, 1.0);
        let vector = Tuple::vector(5.0, 6.0, 7.0);

        assert_eq!(Tuple::vector(-2.0, -4.0, -6.0), point - vector);
    }

    #[test]
    fn negate_tuple() {
        let tuple = Tuple::new(1.0, -2.0, 3.0, -4.0);

        assert_eq!(Tuple::new(-1.0, 2.0, -3.0, 4.0), -tuple);
    }

    #[test]
    fn multiply_tuple_by_scalar() {
        let tuple = Tuple::new(1.0, -2.0, 3.0, -4.0);

        assert_eq!(Tuple::new(3.5, -7.0, 10.5, -14.0), tuple * 3.5);
    }

    #[test]
    fn divide_tuple_by_scalar() {
        let tuple = Tuple::new(1.0, -2.0, 3.0, -4.0);

        assert_eq!(Tuple::new(0.5, -1.0, 1.5, -2.0), tuple / 2.0);
    }

    #[test]
    fn calculate_magnitude_of_vector() {
        assert!(1.0.f_equals(Tuple::vector(1.0, 0.0, 0.0).magnitude()));
        assert!(1.0.f_equals(Tuple::vector(0.0, 1.0, 0.0).magnitude()));
        assert!(1.0.f_equals(Tuple::vector(0.0, 0.0, 1.0).magnitude()));
        assert!((14.0 as f64)
            .sqrt()
            .f_equals(Tuple::vector(1.0, 2.0, 3.0).magnitude() as f64));
        assert!((14.0 as f64)
            .sqrt()
            .f_equals(Tuple::vector(-1.0, -2.0, -3.0).magnitude() as f64));
    }
}
