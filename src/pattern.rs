use crate::color::Color;
use crate::matrix::Matrix;
use crate::object::Object;
use crate::pattern::stripe::StripeProperties;
use crate::point::Point;

mod stripe;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Pattern {
    Solid(Color),
    Stripe(StripeProperties),
}

impl Pattern {
    pub fn solid(color: Color) -> Self {
        Pattern::Solid(color)
    }

    pub fn stripe(first_color: Color, second_color: Color, transformation: Matrix<4, 4>) -> Self {
        Pattern::Stripe(StripeProperties::new(
            first_color,
            second_color,
            transformation,
        ))
    }

    /// Color of pattern at point in object space
    pub fn object_color_at(&self, object: Object, point: Point) -> Color {
        match self {
            Pattern::Solid(color) => *color,
            Pattern::Stripe(stripe_properties) => stripe_properties.object_color_at(object, point),
        }
    }
}
