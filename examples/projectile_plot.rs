use std::fs::File;
use std::io::BufWriter;

/// A simple example from chapter 2 on plotting on a canvas.
/// Same as the example in `projectile.rs`, but in this case we plot the
/// trajectory of the projectile on a canvas and save it to file.
///
use raytracer::canvas::Canvas;
use raytracer::color::Color;
use raytracer::point::Point;
use raytracer::ppm_writer::PpmWriter;
use raytracer::vector::Vector;

#[derive(Debug)]
struct Projectile {
    position: Point,
    velocity: Vector,
}

struct Environment {
    gravity: Vector,
    wind: Vector,
}

fn tick(environment: &Environment, projectile: &Projectile) -> Projectile {
    let position = projectile.position + projectile.velocity;
    let velocity = projectile.velocity + environment.gravity + environment.wind;

    Projectile { position, velocity }
}

fn main() {
    let mut projectile = Projectile {
        position: Point::new(0.0, 1.0, 0.0),
        velocity: Vector::new(1.0, 1.8, 0.0).normalize() * 11.25,
    };
    let environment = Environment {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    let mut canvas = Canvas::new(900, 550);
    let red = Color::new(1.0, 0.0, 0.0);

    while projectile.position.y > 0.0 {
        canvas.write_pixel(
            projectile.position.x.round() as usize,
            canvas.height() - projectile.position.y.round() as usize,
            red,
        );
        projectile = tick(&environment, &projectile);
    }

    println!("Projectile ended at position {:?}", projectile.position);
    let file = BufWriter::new(
        File::create("examples/projectile_plot.ppm").expect("Failed to create file"),
    );
    let mut ppm_writer = PpmWriter::from_writer(file);

    println!("Writing to file...");
    ppm_writer
        .write_canvas(&canvas)
        .expect("Failed to write to file");
    println!("Done");
}
