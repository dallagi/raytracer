use crate::color::Color;
use crate::point::Point;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct StripeProperties {
    first_stripe: Color,
    second_stripe: Color,
}

impl StripeProperties {
    pub fn new(first_stripe: Color, second_stripe: Color) -> Self {
        Self {
            first_stripe,
            second_stripe,
        }
    }

    pub fn object_color_at(&self, point: Point) -> Color {
        let x_is_even = point.x.floor() as i64 % 2 == 0;

        if x_is_even {
            Color::white()
        } else {
            Color::black()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::material::Material;
    use crate::matrix::transformations;
    use crate::object::Object;
    use crate::pattern::Pattern;

    use super::*;

    #[test]
    fn stripe_pattern_is_constant_in_y() {
        let pattern = StripeProperties::new(Color::white(), Color::black());

        assert_eq!(
            Color::white(),
            pattern.object_color_at(Point::new(0.0, 0.0, 0.0))
        );
        assert_eq!(
            Color::white(),
            pattern.object_color_at(Point::new(0.0, 1.0, 0.0))
        );
        assert_eq!(
            Color::white(),
            pattern.object_color_at(Point::new(0.0, 2.0, 0.0))
        )
    }

    #[test]
    fn stripe_pattern_is_constant_in_z() {
        let pattern = StripeProperties::new(Color::white(), Color::black());

        assert_eq!(
            Color::white(),
            pattern.object_color_at(Point::new(0.0, 0.0, 0.0))
        );
        assert_eq!(
            Color::white(),
            pattern.object_color_at(Point::new(0.0, 0.0, 1.0))
        );
        assert_eq!(
            Color::white(),
            pattern.object_color_at(Point::new(0.0, 0.0, 2.0))
        )
    }

    #[test]
    fn stripe_pattern_alternates_in_x() {
        let pattern = StripeProperties::new(Color::white(), Color::black());

        assert_eq!(
            Color::white(),
            pattern.object_color_at(Point::new(0.0, 0.0, 0.0))
        );
        assert_eq!(
            Color::white(),
            pattern.object_color_at(Point::new(0.9, 0.0, 0.0))
        );
        assert_eq!(
            Color::black(),
            pattern.object_color_at(Point::new(1.0, 0.0, 0.0))
        );
        assert_eq!(
            Color::black(),
            pattern.object_color_at(Point::new(-0.1, 0.0, 0.0))
        );
        assert_eq!(
            Color::black(),
            pattern.object_color_at(Point::new(-1.0, 0.0, 0.0))
        );
        assert_eq!(
            Color::white(),
            pattern.object_color_at(Point::new(-1.1, 0.0, 0.0))
        )
    }
}