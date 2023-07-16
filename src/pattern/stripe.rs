use crate::color::Color;
use crate::matrix::Matrix;
use crate::object::Object;
use crate::point::Point;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct StripeProperties {
    first_stripe: Color,
    second_stripe: Color,
    transformation: Matrix<4, 4>,
}

impl StripeProperties {
    pub fn new(first_stripe: Color, second_stripe: Color, transformation: Matrix<4, 4>) -> Self {
        Self {
            first_stripe,
            second_stripe,
            transformation,
        }
    }

    pub fn object_color_at(&self, object: Object, point: Point) -> Color {
        let combined_transformations =
            object.transformation.inverse() >> self.transformation.inverse();

        let object_space_point = combined_transformations * point;
        let x_is_even = object_space_point.x.floor() as i64 % 2 == 0;

        if x_is_even {
            self.first_stripe
        } else {
            self.second_stripe
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::transformations;
    use crate::object::Object;

    use super::*;

    #[test]
    fn stripe_pattern_is_constant_in_y() {
        let pattern = StripeProperties::new(Color::white(), Color::black(), Matrix::identity());

        assert_eq!(
            Color::white(),
            pattern.object_color_at(Object::default(), Point::new(0.0, 0.0, 0.0))
        );
        assert_eq!(
            Color::white(),
            pattern.object_color_at(Object::default(), Point::new(0.0, 1.0, 0.0))
        );
        assert_eq!(
            Color::white(),
            pattern.object_color_at(Object::default(), Point::new(0.0, 2.0, 0.0))
        )
    }

    #[test]
    fn stripe_pattern_is_constant_in_z() {
        let pattern = StripeProperties::new(Color::white(), Color::black(), Matrix::identity());

        assert_eq!(
            Color::white(),
            pattern.object_color_at(Object::default(), Point::new(0.0, 0.0, 0.0))
        );
        assert_eq!(
            Color::white(),
            pattern.object_color_at(Object::default(), Point::new(0.0, 0.0, 1.0))
        );
        assert_eq!(
            Color::white(),
            pattern.object_color_at(Object::default(), Point::new(0.0, 0.0, 2.0))
        )
    }

    #[test]
    fn stripe_pattern_alternates_in_x() {
        let pattern = StripeProperties::new(Color::white(), Color::black(), Matrix::identity());

        assert_eq!(
            Color::white(),
            pattern.object_color_at(Object::default(), Point::new(0.0, 0.0, 0.0))
        );
        assert_eq!(
            Color::white(),
            pattern.object_color_at(Object::default(), Point::new(0.9, 0.0, 0.0))
        );
        assert_eq!(
            Color::black(),
            pattern.object_color_at(Object::default(), Point::new(1.0, 0.0, 0.0))
        );
        assert_eq!(
            Color::black(),
            pattern.object_color_at(Object::default(), Point::new(-0.1, 0.0, 0.0))
        );
        assert_eq!(
            Color::black(),
            pattern.object_color_at(Object::default(), Point::new(-1.0, 0.0, 0.0))
        );
        assert_eq!(
            Color::white(),
            pattern.object_color_at(Object::default(), Point::new(-1.1, 0.0, 0.0))
        )
    }

    #[test]
    fn stripe_pattern_adapts_to_object_transformation() {
        let pattern = StripeProperties::new(Color::white(), Color::black(), Matrix::identity());
        let object = Object {
            transformation: transformations::scaling(2.0, 2.0, 2.0),
            ..Object::default()
        };

        assert_eq!(
            Color::white(),
            pattern.object_color_at(object, Point::new(1.5, 0.0, 0.0))
        );
    }

    #[test]
    fn stripe_pattern_adapts_to_its_own_transformation() {
        let pattern = StripeProperties::new(
            Color::white(),
            Color::black(),
            transformations::scaling(2.0, 2.0, 2.0),
        );

        assert_eq!(
            Color::white(),
            pattern.object_color_at(Object::default(), Point::new(1.5, 0.0, 0.0))
        );
    }

    #[test]
    fn stripe_pattern_adapts_to_both_its_own_and_object_transformations() {
        let pattern = StripeProperties::new(
            Color::white(),
            Color::black(),
            transformations::translation(0.5, 0.0, 0.0),
        );
        let object = Object {
            transformation: transformations::scaling(2.0, 2.0, 2.0),
            ..Object::default()
        };

        assert_eq!(
            Color::white(),
            pattern.object_color_at(object, Point::new(2.5, 0.0, 0.0))
        );
    }
}
