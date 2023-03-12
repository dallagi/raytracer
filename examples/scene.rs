//// Example from Chapter 7.

use std::f64::consts::PI;
use std::fs::File;

use raytracer::camera::Camera;
use raytracer::canvas::Canvas;
use raytracer::color::Color;
use raytracer::light::Light;
use raytracer::lighting::lighting;
use raytracer::material::Material;
use raytracer::matrix::transformations;
use raytracer::point::Point;
use raytracer::ppm_writer::PpmWriter;
use raytracer::ray::Ray;
use raytracer::sphere::Sphere;
use raytracer::vector::Vector;
use raytracer::view_transform;
use raytracer::world::World;

const CANVAS_WIDTH: usize = 1000;
const CANVAS_HEIGHT: usize = 500;

fn main() {
    let light_source = Light::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let world = World::new(
        vec![light_source],
        vec![
            large_sphere(),
            small_sphere(),
            smallest_sphere(),
            floor(),
            left_wall(),
            right_wall(),
        ],
    );

    let camera = Camera::new(CANVAS_WIDTH, CANVAS_HEIGHT, PI / 3.0).with_transform(
        view_transform::view_transform(
            Point::new(0.0, 1.5, -5.0),
            Point::new(0.0, 1.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
        ),
    );

    let canvas = camera.render(world);

    let out_path = "examples/out/scene.ppm";
    let file = File::create(out_path).expect("Failed to create file");
    let mut ppm_writer = PpmWriter::from_writer(file);

    println!("Writing to {out_path}");
    ppm_writer
        .write_canvas(&canvas)
        .expect("Failed to write to file");
    println!("Done");
}

fn large_sphere() -> Sphere {
    Sphere {
        transformation: transformations::translation(-0.5, 1.0, 0.5),
        material: Material {
            color: Color::new(0.1, 1.0, 0.5),
            diffuse: 0.7,
            specular: 0.3,
            ..Material::default()
        },
    }
}

fn small_sphere() -> Sphere {
    Sphere {
        transformation: transformations::scaling(0.5, 0.5, 0.5)
            >> transformations::translation(1.5, 0.5, -0.5),
        material: Material {
            color: Color::new(0.5, 1.0, 0.1),
            diffuse: 0.7,
            specular: 0.3,
            ..Material::default()
        },
    }
}

fn smallest_sphere() -> Sphere {
    Sphere {
        transformation: transformations::scaling(0.33, 0.33, 0.33)
            >> transformations::translation(-1.5, 0.33, -0.75),
        material: Material {
            color: Color::new(0.5, 1.0, 0.1),
            diffuse: 0.7,
            specular: 0.3,
            ..Material::default()
        },
    }
}

fn floor() -> Sphere {
    Sphere {
        transformation: transformations::scaling(10.0, 0.01, 10.0),
        material: wall_material(),
    }
}

fn left_wall() -> Sphere {
    Sphere {
        transformation: transformations::scaling(10.0, 0.01, 10.0)
            >> transformations::rotation_x(PI / 2.0)
            >> transformations::rotation_y(-PI / 4.0)
            >> transformations::translation(0.0, 0.0, 5.0),
        material: wall_material(),
    }
}

fn right_wall() -> Sphere {
    Sphere {
        transformation: transformations::scaling(10.0, 0.01, 10.0)
            >> transformations::rotation_x(PI / 2.0)
            >> transformations::rotation_y(PI / 4.0)
            >> transformations::translation(0.0, 0.0, 5.0),
        material: wall_material(),
    }
}

fn wall_material() -> Material {
    Material {
        color: Color::new(1.0, 0.9, 0.9),
        specular: 0.0,
        ..Material::default()
    }
}
