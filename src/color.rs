use crate::float_eq::FloatEq;
use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        Self { red, green, blue }
    }

    pub fn black() -> Self {
        Color::new(0.0, 0.0, 0.0)
    }

    /// Scale color values between `min` and `max`
    pub fn scale(&self, min: f32, max: f32) -> Self {
        Self {
            red: Self::scale_component(self.red, min, max),
            green: Self::scale_component(self.green, min, max),
            blue: Self::scale_component(self.blue, min, max),
        }
    }

    fn scale_component(component: f32, min: f32, max: f32) -> f32 {
        min + (component.clamp(0.0, 1.0) * (max - min))
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

    use pretty_assertions::assert_eq;

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

    #[test]
    fn scales_normalized_color_to_given_scale() {
        let color = Color::new(1.0, 0.0, 0.0);

        assert_eq!(Color::new(255.0, 0.0, 0.0), color.scale(0.0, 255.0));
    }

    #[test]
    fn scale_clamps_color_between_0_and_1_before_scaling() {
        let color = Color::new(1.5, 0.0, -0.5);

        assert_eq!(Color::new(255.0, 0.0, 0.0), color.scale(0.0, 255.0));
    }
}
