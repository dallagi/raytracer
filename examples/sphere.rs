//// Example from Chapter 6.

use std::fs::File;

use raytracer::canvas::Canvas;
use raytracer::color::Color;
use raytracer::light::Light;
use raytracer::lighting::{self, lighting};
use raytracer::material::Material;
use raytracer::point::Point;
use raytracer::ppm_writer::PpmWriter;
use raytracer::ray::Ray;
use raytracer::sphere::Sphere;

const CANVAS_SIZE: usize = 500;

fn main() {
    let mut canvas = Canvas::new(CANVAS_SIZE, CANVAS_SIZE);
    let mut sphere = Sphere::default();
    sphere.material = Material {
        color: Color::new(1.0, 0.2, 1.0),
        ..Default::default()
    };

    let light_position = Point::new(-10.0, 10.0, -10.0);
    let light_color = Color::new(1.0, 1.0, 1.0);
    let light = Light::new(light_position, light_color);

    // keep in mind that the sphere is at point (0, 0, 0).
    // we'll cast a ray from z = -5 to a wall with z = 10
    // since the sphere has a radius of 1, the shade will reach a
    // radius of 3 (due to distance of ray origin and wall)
    let ray_origin = Point::new(0.0, 0.0, -5.0);
    let wall_z = 10.0;

    let wall_size = 7.0; // sphere should take 6

    let pixel_size = wall_size / CANVAS_SIZE as f32;
    // we'll need `half` since the the wall will be centered around the origin
    // (hence it will includ points with negative coordinates)
    // but the canvas coordinates are all nonnegative, so we need to shift them accordingly
    let half = wall_size / 2.0;

    for y in 0..CANVAS_SIZE {
        // here we substract the value from half to flip the y value, since y on the canvas
        // goes from top to down, while in the world it goes from the bottom up
        let world_y = half - pixel_size * y as f32;

        for x in 0..CANVAS_SIZE {
            // x doesn't need to get flipped, so we just shift by substracting half to center
            // it around the origin
            let world_x = -half + pixel_size * x as f32;

            let position = Point::new(world_y, world_x, wall_z);
            let ray = Ray::new(ray_origin, (position - ray_origin).normalize());
            let intersections = ray.intersect(sphere);

            if let Some(hit) = intersections.hit() {
                let point = ray.position(hit.t);
                let normal = hit.object.normal_at(point);
                let eye = -ray.direction;

                let color = lighting(hit.object.material, light, point, eye, normal);

                canvas.write_pixel(x, y, color);
            }
        }
    }

    let out_path = "examples/out/sphere.ppm";
    let file = File::create(out_path).expect("Failed to create file");
    let mut ppm_writer = PpmWriter::from_writer(file);

    println!("Writing to {out_path}");
    ppm_writer
        .write_canvas(&canvas)
        .expect("Failed to write to file");
    println!("Done");
}
