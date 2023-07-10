use crate::color::Color;
use crate::pattern::stripe::StripeProperties;
use crate::point::Point;

mod stripe;

pub enum Pattern {
    Solid(Color),
    Stripe(StripeProperties),
}

impl Pattern {
    pub fn solid(color: Color) -> Self {
        Pattern::Solid(color)
    }

    pub fn stripe(first_color: Color, second_color: Color) -> Self {
        Pattern::Stripe(StripeProperties::new(first_color, second_color))
    }

    pub fn color_at(&self, point: Point) -> Color {
        match self {
            Pattern::Solid(color) => color.clone(),
            Pattern::Stripe(stripe_properties) => stripe_properties.stripe_at(point),
        }
    }
}
