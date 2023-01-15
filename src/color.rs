use crate::float_eq::FloatEq;
use std::ops;

#[derive(Debug)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Color {
    fn new(red: f32, green: f32, blue: f32) -> Self {
        Self { red, green, blue }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.red.float_eq(other.red)
            && self.green.float_eq(other.green)
            && self.blue.float_eq(other.blue)
    }
}

impl ops::Add for Color {
    type Output = Color;

    fn add(self, other: Self) -> Self::Output {
        Self::new(
            self.red + other.red,
            self.green + other.green,
            self.blue + other.blue,
        )
    }
}

impl ops::Sub for Color {
    type Output = Color;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(
            self.red - other.red,
            self.green - other.green,
            self.blue - other.blue,
        )
    }
}

impl ops::Mul<f32> for Color {
    type Output = Color;

    fn mul(self, factor: f32) -> Self::Output {
        Color::new(self.red * factor, self.green * factor, self.blue * factor)
    }
}

/// Blends two colors together.
/// Aka Hadamar product, or Schur product
impl ops::Mul for Color {
    type Output = Color;

    fn mul(self, other: Self) -> Self::Output {
        Self::new(
            self.red * other.red,
            self.green * other.green,
            self.blue * other.blue,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colors_are_red_green_blue_tuples() {
        let color = Color::new(-0.5, 0.4, 1.7);

        assert_eq!(-0.5, color.red);
        assert_eq!(0.4, color.green);
        assert_eq!(1.7, color.blue);
    }

    #[test]
    fn colors_can_be_added() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert_eq!(Color::new(1.6, 0.7, 1.0), c1 + c2)
    }

    #[test]
    fn colors_can_be_substracted() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert_eq!(Color::new(0.2, 0.5, 0.5), c1 - c2)
    }

    #[test]
    fn colors_can_be_multiplied_by_a_scalar() {
        let color = Color::new(0.2, 0.3, 0.4);

        assert_eq!(Color::new(0.4, 0.6, 0.8), color * 2.0)
    }

    #[test]
    fn colors_can_be_multiplied_between_each_other() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);

        assert_eq!(Color::new(0.9, 0.2, 0.04), c1 * c2)
    }
}
