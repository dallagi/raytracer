use crate::color::Color;
use crate::light::Light;
use crate::material::Material;
use crate::matrix::transformations;
use crate::point::Point;
use crate::sphere::Sphere;

pub struct World {
    pub light: Light,
    pub objects: Vec<Sphere>,
}

impl Default for World {
    fn default() -> Self {
        let light = Light::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let sphere_1 = Sphere {
            material: Material {
                color: Color::new(0.8, 1.0, 0.6),
                diffuse: 0.7,
                specular: 0.2,
                ..Default::default()
            },
            ..Default::default()
        };

        let sphere_2 = Sphere {
            transformation: transformations::scaling(0.5, 0.5, 0.5),
            ..Default::default()
        };

        Self {
            light,
            objects: vec![sphere_1, sphere_2],
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn default_world_contains_two_spheres() {
        let expected_light = Light::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let expected_sphere_1 = Sphere {
            material: Material {
                color: Color::new(0.8, 1.0, 0.6),
                diffuse: 0.7,
                specular: 0.2,
                ..Default::default()
            },
            ..Default::default()
        };

        let expected_sphere_2 = Sphere {
            transformation: transformations::scaling(0.5, 0.5, 0.5),
            ..Default::default()
        };

        let default_world = World::default();

        assert_eq!(expected_light, default_world.light);
        assert!(default_world.objects.contains(&expected_sphere_1));
        assert!(default_world.objects.contains(&expected_sphere_2));
    }
}
