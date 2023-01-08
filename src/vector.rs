use std::ops;

use crate::f_equals::FEquals;

#[derive(Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Length of the vector
    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt() as f32
    }

    /// Convert into unit vector
    /// ie. into vector with magnitude 1
    pub fn normalize(&self) -> Self {
        let magnitude = self.magnitude();

        Self::new(self.x / magnitude, self.y / magnitude, self.z / magnitude)
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.x.f_equals(other.x) && self.y.f_equals(other.y) && self.z.f_equals(other.z)
    }
}

impl ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Self::Output {
        Vector::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Self::Output {
        Vector::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector::new(-self.x, -self.y, -self.z)
    }
}

impl ops::Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, factor: f32) -> Self::Output {
        Vector::new(self.x * factor, self.y * factor, self.z * factor)
    }
}

impl ops::Div<f32> for Vector {
    type Output = Vector;

    fn div(self, factor: f32) -> Self::Output {
        Vector::new(self.x / factor, self.y / factor, self.z / factor)
    }
}

mod tests {
    use crate::f_equals::FEquals;

    use super::*;

    #[test]
    fn substracting_two_vectors_results_in_vector() {
        let v1 = Vector::new(3.0, 2.0, 1.0);
        let v2 = Vector::new(5.0, 6.0, 7.0);

        assert_eq!(Vector::new(-2.0, -4.0, -6.0), v1 - v2);
    }

    #[test]
    fn negating_vector_negates_its_components() {
        let vector = Vector::new(1.0, -2.0, 3.0);

        assert_eq!(Vector::new(-1.0, 2.0, -3.0), -vector);
    }

    #[test]
    fn multiply_vector_by_scalar() {
        let vector = Vector::new(1.0, -2.0, 3.0);

        assert_eq!(Vector::new(3.5, -7.0, 10.5), vector * 3.5);
    }

    #[test]
    fn divide_vector_by_scalar() {
        let vector = Vector::new(1.0, -2.0, 3.0);

        assert_eq!(Vector::new(0.5, -1.0, 1.5), vector / 2.0);
    }

    #[test]
    fn magnitude_is_sqrt_of_sum_of_squares_of_components() {
        assert!(1.0.f_equals(Vector::new(1.0, 0.0, 0.0).magnitude()));
        assert!(1.0.f_equals(Vector::new(0.0, 1.0, 0.0).magnitude()));
        assert!(1.0.f_equals(Vector::new(0.0, 0.0, 1.0).magnitude()));
        assert!((14.0 as f64)
            .sqrt()
            .f_equals(Vector::new(1.0, 2.0, 3.0).magnitude() as f64));
        assert!((14.0 as f64)
            .sqrt()
            .f_equals(Vector::new(-1.0, -2.0, -3.0).magnitude() as f64));
    }

    #[test]
    fn normalizing_converts_vector_into_unit_vector() {
        let vector = Vector::new(0.0, 0.0, 4.0);
        assert_eq!(Vector::new(0.0, 0.0, 1.0), vector.normalize());
        assert!(vector.normalize().magnitude().f_equals(1.0));

        let vector = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(
            Vector::new(0.26726124, 0.5345225, 0.8017837),
            vector.normalize()
        );
        assert!(vector.normalize().magnitude().f_equals(1.0));
    }
}
