use crate::color::Color;

#[derive(Debug)]
pub struct Canvas {
    pixels: Vec<Vec<Color>>,
    width: usize,
    height: usize,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let pixels = vec![vec![Color::black(); width]; height];
        Self {
            pixels,
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.pixels[y][x]
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) -> &Self {
        self.pixels[y][x] = color;
        self
    }

    pub fn iter_rows(&self) -> std::slice::Iter<Vec<Color>> {
        self.pixels.iter()
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
