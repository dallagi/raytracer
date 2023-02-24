use std::ops;

use crate::float_eq::FloatEq;

#[derive(Copy, Clone, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub const W: f64 = 0.0;

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Length of the vector
    pub fn magnitude(self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt() as f64
    }

    /// Convert into unit vector
    /// ie. into vector with magnitude 1
    pub fn normalize(self) -> Self {
        let magnitude = self.magnitude();

        Self::new(self.x / magnitude, self.y / magnitude, self.z / magnitude)
    }

    /// Dot product
    /// The smaller the dot product, the larger the angle between the two vectors
    /// If two unit vectors have same dot product, they are the same
    /// If dot product is -1, vectors point in opposite directions
    /// Dot product of two unit vectors is the cosine of the angle between them
    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Cross product
    /// Returns a vector that is perpendicular to the two input vectors
    pub fn cross(self, other: Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn reflect(self, normal: Vector) -> Vector {
        self - normal * 2.0 * self.dot(normal)
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.x.float_eq(other.x) && self.y.float_eq(other.y) && self.z.float_eq(other.z)
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

impl ops::Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, factor: f64) -> Self::Output {
        Vector::new(self.x * factor, self.y * factor, self.z * factor)
    }
}

impl ops::Div<f64> for Vector {
    type Output = Vector;

    fn div(self, factor: f64) -> Self::Output {
        Vector::new(self.x / factor, self.y / factor, self.z / factor)
    }
}

#[cfg(test)]
mod tests {
    use crate::float_eq::FloatEq;

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
        assert!(1.0.float_eq(Vector::new(1.0, 0.0, 0.0).magnitude()));
        assert!(1.0.float_eq(Vector::new(0.0, 1.0, 0.0).magnitude()));
        assert!(1.0.float_eq(Vector::new(0.0, 0.0, 1.0).magnitude()));
        assert!((14.0 as f64)
            .sqrt()
            .float_eq(Vector::new(1.0, 2.0, 3.0).magnitude() as f64));
        assert!((14.0 as f64)
            .sqrt()
            .float_eq(Vector::new(-1.0, -2.0, -3.0).magnitude() as f64));
    }

    #[test]
    fn normalizing_converts_vector_into_unit_vector() {
        let vector = Vector::new(0.0, 0.0, 4.0);
        assert_eq!(Vector::new(0.0, 0.0, 1.0), vector.normalize());
        assert!(vector.normalize().magnitude().float_eq(1.0));

        let vector = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(
            Vector::new(0.26726124, 0.5345225, 0.8017837),
            vector.normalize()
        );
        assert!(vector.normalize().magnitude().float_eq(1.0));
    }

    #[test]
    fn dot_product_is_sum_of_products_of_components_of_vector() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);

        assert_eq!(20.0, v1.dot(v2))
    }

    #[test]
    fn cross_product_returns_vector_perpendicular_to_input_vectors() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);

        assert_eq!(Vector::new(-1.0, 2.0, -1.0), v1.cross(v2));
        assert_eq!(Vector::new(1.0, -2.0, 1.0), v2.cross(v1));
    }

    #[test]
    fn reflecting_vector_approaching_at_45_degrees() {
        let vector = Vector::new(1.0, -1.0, 0.0);
        let normal = Vector::new(0.0, 1.0, 0.0);

        let reflected = vector.reflect(normal);

        assert_eq!(reflected, Vector::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn reflecting_vector_off_slanted_surface() {
        let vector = Vector::new(0.0, -1.0, 0.0);
        let normal = Vector::new((2.0_f64).sqrt() / 2.0, (2.0_f64).sqrt() / 2.0, 0.0);

        let reflected = vector.reflect(normal);

        assert_eq!(reflected, Vector::new(1.0, 0.0, 0.0));
    }
}
