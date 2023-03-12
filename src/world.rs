use crate::color::Color;
use crate::intersection_state::IntersectionState;
use crate::light::Light;
use crate::lighting::lighting;
use crate::material::Material;
use crate::matrix::transformations;
use crate::point::Point;
use crate::ray::Ray;
use crate::sphere::Sphere;

pub struct World {
    pub lights: Vec<Light>,
    pub objects: Vec<Sphere>,
}

impl Default for World {
    fn default() -> Self {
        let lights = vec![Light::new(
            Point::new(-10.0, 10.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        )];
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
            lights,
            objects: vec![sphere_1, sphere_2],
        }
    }
}

impl World {
    fn color_at_intersection_with(&self, ray: Ray) -> Color {
        let intersections = ray.intersect_world(self);
        let hit = match intersections.hit() {
            None => return Color::black(),
            Some(value) => value,
        };

        let intersection_state = IntersectionState::prepare(hit, ray);
        self.shade_hit(intersection_state)
    }

    fn shade_hit(&self, intersection_state: IntersectionState) -> Color {
        let mut color = Color::black();

        for light in self.lights.iter() {
            color += lighting(
                intersection_state.object.material,
                *light,
                intersection_state.point,
                intersection_state.eye_v,
                intersection_state.normal_v,
            );
        }

        color
    }
}

#[cfg(test)]
mod tests {

    use crate::intersection::Intersection;
    use crate::vector::Vector;

    use pretty_assertions::assert_eq;

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

        assert_eq!(vec![expected_light], default_world.lights);
        assert!(default_world.objects.contains(&expected_sphere_1));
        assert!(default_world.objects.contains(&expected_sphere_2));
    }

    #[test]
    fn shading_an_intersection_from_the_outside() {
        let world = World::default();
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = world.objects.first().unwrap().clone();
        let intersection = Intersection::new(4.0, shape);

        let intersection_state = IntersectionState::prepare(intersection, ray);
        let color = world.shade_hit(intersection_state);

        assert_eq!(Color::new(0.38066, 0.47583, 0.2855), color);
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut world = World::default();
        world.lights = vec![Light::new(
            Point::new(0.0, 0.25, 0.0),
            Color::new(1.0, 1.0, 1.0),
        )];

        let ray = Ray::new(Point::origin(), Vector::new(0.0, 0.0, 1.0));
        let shape = world.objects[1].clone();
        let intersection = Intersection::new(0.5, shape);

        let intersection_state = IntersectionState::prepare(intersection, ray);
        let color = world.shade_hit(intersection_state);

        assert_eq!(Color::new(0.90498, 0.90498, 0.90498), color);
    }

    #[test]
    fn shading_with_multiple_lights() {
        let mut world = World::default();
        world.lights.push(Light::new(
            Point::new(-7.0, 7.0, -7.0),
            Color::new(1.0, 1.0, 1.0),
        ));
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = world.objects.first().unwrap().clone();
        let intersection = Intersection::new(4.0, shape);

        let intersection_state = IntersectionState::prepare(intersection, ray);
        let color = world.shade_hit(intersection_state);

        assert_eq!(Color::new(0.75092, 0.93865, 0.56319), color);
    }

    #[test]
    fn color_is_black_when_ray_misses() {
        let world = World::default();
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0));

        let color = world.color_at_intersection_with(ray);

        assert_eq!(Color::black(), color)
    }

    #[test]
    fn color_is_computed_appropriately_on_ray_hit() {
        let world = World::default();
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));

        let color = world.color_at_intersection_with(ray);

        assert_eq!(Color::new(0.38066, 0.47583, 0.2855), color)
    }

    #[test]
    fn color_is_computed_appropriately_when_ray_originating_from_within_an_outer_object_hits_the_outside_of_an_inner_object(
    ) {
        let mut world = World::default();
        {
            let outer = world.objects.get_mut(0).unwrap();
            outer.material.ambient = 1.0;
            let mut inner = world.objects.get_mut(1).unwrap();
            inner.material.ambient = 1.0;
        }
        let inner = world.objects[1];
        let ray = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0.0, 0.0, -1.0));

        let color = world.color_at_intersection_with(ray);

        assert_eq!(inner.material.color, color)
    }
}
