use std::{f32::consts::PI, fs::File};

use raytracer::{
    canvas::Canvas, color::Color, matrix::transformations::rotation_z, point::Point,
    ppm_writer::PpmWriter,
};

const HOURS_COUNT: usize = 12;
const CANVAS_SIZE: usize = 100;

fn main() {
    let mut canvas = Canvas::new(CANVAS_SIZE, CANVAS_SIZE);

    let twelve_o_clock = Point::new(0.0, 40.0, 0.0);

    let radiant_5mins = (PI * 2.0) / HOURS_COUNT as f32;
    for hour in 0..HOURS_COUNT {
        let hour_point = rotation_z(hour as f32 * radiant_5mins) * twelve_o_clock;
        println!("{hour}: {:?}", hour_point);
        const HALF_CANVAS: i32 = CANVAS_SIZE as i32 / 2;

        canvas.write_pixel(
            (HALF_CANVAS + hour_point.x as i32) as usize,
            (HALF_CANVAS + hour_point.y as i32) as usize,
            Color::new(1.0, 0.0, 0.0),
        );
    }

    let out_path = "examples/out/clock_hours.ppm";
    let file = File::create(out_path).expect("Failed to create file");
    let mut ppm_writer = PpmWriter::from_writer(file);

    println!("Writing to {out_path}");
    ppm_writer
        .write_canvas(&canvas)
        .expect("Failed to write to file");
    println!("Done");
}
