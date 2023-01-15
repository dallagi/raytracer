use std::ops::Deref;

use ndarray::iter::AxisIter;
use ndarray::prelude::*;
use ndarray::Array2;
use ndarray::Dim;

use crate::color::Color;

pub struct Canvas(pub Array2<Color>);

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let array = Array2::from_elem((width, height).f(), Color::black());
        Self(array)
    }

    pub fn width(&self) -> usize {
        let (width, _height) = self.0.dim();
        width
    }

    pub fn height(&self) -> usize {
        let (_width, height) = self.0.dim();
        height
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.0[[x, y]]
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) -> &Self {
        self.0[[x, y]] = color;
        self
    }

    pub fn iter_rows(&self) -> AxisIter<Color, Dim<[usize; 1]>> {
        self.0.axis_iter(ndarray::Axis(1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_canvas_filled_with_black_pixels() {
        let canvas = Canvas::new(10, 20);

        assert_eq!(10, canvas.width());
        assert_eq!(20, canvas.height());

        for x in 0..10 {
            for y in 0..20 {
                assert_eq!(Color::black(), canvas.pixel_at(x, y));
            }
        }
    }

    #[test]
    fn writing_pixel_to_canvas() {
        let mut canvas = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);

        canvas.write_pixel(2, 3, red);

        assert_eq!(red, canvas.pixel_at(2, 3));
    }
}
