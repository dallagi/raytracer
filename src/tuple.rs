use crate::tuple::Kind::{Point, Undefined, Vector};

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
            w if w == W_VECTOR => Vector,
            w if w == W_POINT => Point,
            _ => Undefined,
        }
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
}
