//// A simple example from chapter 1 on using points and vectors
//// Calculates how far a projectile goes, considering gravity and wind
use raytracer::{point::Point, vector::Vector};

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
        velocity: Vector::new(1.0, 1.0, 0.0).normalize(),
    };
    let environment = Environment {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    while projectile.position.y > 0.0 {
        projectile = tick(&environment, &projectile);
    }

    println!("Projectile ended at position {:?}", projectile.position)
}
