use crate::canvas::Canvas;
use std::io;

// see https://users.rust-lang.org/t/generic-writer-file-string/51308/2

const MAGIC_NUMBER: &[u8] = b"P3";
const MIN_PIXEL_VALUE: f32 = 0.0;
const MAX_PIXEL_VALUE: f32 = 255.0;

struct PpmWriter<W: io::Write> {
    writer: W,
}

impl<W: io::Write> PpmWriter<W> {
    fn from_writer(writer: W) -> Self {
        Self { writer }
    }

    fn write_canvas(&mut self, canvas: &Canvas) -> io::Result<()> {
        self.write_header(canvas)?;
        self.write_body(canvas)?;
        Ok(())
    }

    fn write_header(&mut self, canvas: &Canvas) -> Result<(), io::Error> {
        self.writer.write(MAGIC_NUMBER)?;
        self.writer.write(b"\n")?;
        self.write_i32_as_str(canvas.width() as i32)?;
        self.writer.write(b" ")?;
        self.write_i32_as_str(canvas.height() as i32)?;
        self.writer.write(b"\n")?;
        self.write_i32_as_str(MAX_PIXEL_VALUE as i32)?;
        self.writer.write(b"\n")?;
        self.writer.flush()?;
        Ok(())
    }

    fn write_body(&mut self, canvas: &Canvas) -> Result<(), io::Error> {
        for row in canvas.iter_rows() {
            for (idx, pixel) in row.indexed_iter() {
                let pixel_scaled = pixel.scale(MIN_PIXEL_VALUE, MAX_PIXEL_VALUE);

                self.write_i32_as_str(pixel_scaled.red.round() as i32)?;
                self.writer.write(b" ")?;
                self.write_i32_as_str(pixel_scaled.green.round() as i32)?;
                self.writer.write(b" ")?;
                self.write_i32_as_str(pixel_scaled.blue.round() as i32)?;

                if !Self::last_pixel_in_row(idx, canvas) {
                    self.writer.write(b" ")?;
                }
            }
            self.writer.write(b"\n")?;
        }
        Ok(())
    }

    fn write_i32_as_str(&mut self, num: i32) -> Result<(), io::Error> {
        self.writer.write(num.to_string().as_bytes())?;
        Ok(())
    }

    fn last_pixel_in_row(idx: usize, canvas: &Canvas) -> bool {
        idx == (canvas.width() - 1)
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn constructs_ppm_header() {
        let canvas = Canvas::new(5, 3);
        let mut ppm_buffer: Vec<u8> = vec![];
        let mut ppm_writer = PpmWriter::from_writer(&mut ppm_buffer);

        ppm_writer.write_canvas(&canvas).expect("Failed to write");

        assert_eq!(&["P3", "5 3", "255"], &str_lines(&ppm_buffer)[0..3])
    }

    #[test]
    fn constructs_ppm_pixel_data() {
        let mut canvas = Canvas::new(5, 3);
        canvas.write_pixel(0, 0, Color::new(1.5, 0.0, 0.0));
        canvas.write_pixel(2, 1, Color::new(0.0, 0.5, 0.0));
        canvas.write_pixel(4, 2, Color::new(-0.5, 0.0, 1.0));
        let mut ppm_buffer: Vec<u8> = vec![];
        let mut ppm_writer = PpmWriter::from_writer(&mut ppm_buffer);

        ppm_writer.write_canvas(&canvas)?;

        assert_eq!(
            &[
                "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0",
                "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0",
                "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"
            ],
            &str_lines(&ppm_buffer)[3..6]
        )
    }

    fn str_lines(buffer: &[u8]) -> Vec<&str> {
        let str_buffer = std::str::from_utf8(buffer).expect("Failed to parse string as utf-8");
        str_buffer.lines().collect()
    }
}
