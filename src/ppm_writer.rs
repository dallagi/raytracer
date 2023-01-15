use crate::canvas::Canvas;
use std::io;
use std::io::Write;

const PPM_FLAVOR_IDENTIFIER: &[u8] = b"P3";
const MIN_PIXEL_VALUE: f32 = 0.0;
const MAX_PIXEL_VALUE: f32 = 255.0;
const MAX_PPM_BODY_ROW_LENGTH: usize = 70;

pub struct PpmWriter<W: io::Write> {
    writer: io::BufWriter<W>,
}

impl<W: io::Write> PpmWriter<W> {
    pub fn from_writer(writer: W) -> Self {
        Self {
            writer: io::BufWriter::new(writer),
        }
    }

    pub fn write_canvas(&mut self, canvas: &Canvas) -> io::Result<()> {
        self.write_header(canvas)?;
        self.write_body(canvas)?;
        self.writer.flush()?;
        Ok(())
    }

    fn write_header(&mut self, canvas: &Canvas) -> Result<(), io::Error> {
        self.writer.write(PPM_FLAVOR_IDENTIFIER)?;
        self.writer.write(b"\n")?;
        self.write_i32_as_str(canvas.width() as i32)?;
        self.writer.write(b" ")?;
        self.write_i32_as_str(canvas.height() as i32)?;
        self.writer.write(b"\n")?;
        self.write_i32_as_str(MAX_PIXEL_VALUE as i32)?;
        self.writer.write(b"\n")?;
        Ok(())
    }

    fn write_body(&mut self, canvas: &Canvas) -> Result<(), io::Error> {
        for row in canvas.iter_rows() {
            let mut row_length = 0;

            for pixel in row.iter() {
                let pixel_scaled = pixel.scale(MIN_PIXEL_VALUE, MAX_PIXEL_VALUE);

                for component in [pixel_scaled.red, pixel_scaled.green, pixel_scaled.blue] {
                    let component_formatted = (component.round() as i32).to_string();
                    let whitespace_size = 1;

                    if row_length + whitespace_size + component_formatted.len()
                        > MAX_PPM_BODY_ROW_LENGTH
                    {
                        self.writer.write(b"\n")?;
                        row_length = 0;
                    }
                    if row_length != 0 {
                        row_length += self.writer.write(b" ")?;
                    }

                    row_length += self.writer.write(component_formatted.as_bytes())?;
                }
            }

            self.writer.write(b"\n")?;
        }
        Ok(())
    }

    fn write_i32_as_str(&mut self, num: i32) -> Result<usize, io::Error> {
        self.writer.write(num.to_string().as_bytes())
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

        {
            let mut ppm_writer = PpmWriter::from_writer(&mut ppm_buffer);
            ppm_writer.write_canvas(&canvas).expect("Failed to write");
        }

        assert_eq!(&["P3", "5 3", "255"], &str_lines(&ppm_buffer)[0..3])
    }

    #[test]
    fn constructs_ppm_pixel_data() {
        let mut canvas = Canvas::new(5, 3);
        canvas.write_pixel(0, 0, Color::new(1.5, 0.0, 0.0));
        canvas.write_pixel(2, 1, Color::new(0.0, 0.5, 0.0));
        canvas.write_pixel(4, 2, Color::new(-0.5, 0.0, 1.0));
        let mut ppm_buffer: Vec<u8> = vec![];

        {
            let mut ppm_writer = PpmWriter::from_writer(&mut ppm_buffer);
            ppm_writer.write_canvas(&canvas).unwrap();
        }

        assert_eq!(
            &[
                "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0",
                "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0",
                "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"
            ],
            &str_lines(&ppm_buffer)[3..6]
        )
    }

    #[test]
    fn ppm_body_rows_larger_than_70_bytes_are_split() {
        let mut canvas = Canvas::new(10, 2);
        for x in 0..10 {
            for y in 0..2 {
                canvas.write_pixel(x, y, Color::new(1.0, 0.8, 0.6));
            }
        }

        let mut ppm_buffer: Vec<u8> = vec![];

        {
            let mut ppm_writer = PpmWriter::from_writer(&mut ppm_buffer);
            ppm_writer.write_canvas(&canvas).unwrap();
        }

        assert_eq!(
            &[
                "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
                "153 255 204 153 255 204 153 255 204 153 255 204 153",
                "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
                "153 255 204 153 255 204 153 255 204 153 255 204 153"
            ],
            &str_lines(&ppm_buffer)[3..7]
        )
    }

    #[test]
    fn ppm_files_are_terminated_by_newline() {
        let canvas = Canvas::new(5, 3);
        let mut ppm_buffer: Vec<u8> = vec![];

        {
            let mut ppm_writer = PpmWriter::from_writer(&mut ppm_buffer);
            ppm_writer.write_canvas(&canvas).unwrap();
        }

        assert_eq!(ppm_buffer.last(), Some(&('\n' as u8)))
    }

    fn str_lines(buffer: &[u8]) -> Vec<&str> {
        let str_buffer = std::str::from_utf8(buffer).expect("Failed to parse string as utf-8");
        str_buffer.lines().collect()
    }
}
