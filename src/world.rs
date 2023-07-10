use crate::color::Color;
use crate::intersection_state::IntersectionState;
use crate::light::Light;
use crate::lighting::lighting;
use crate::material::Material;
use crate::matrix::transformations;
use crate::object::Object;
use crate::pattern::Pattern;
use crate::point::Point;
use crate::ray::Ray;

pub struct World {
    pub lights: Vec<Light>,
    pub objects: Vec<Object>,
}

impl Default for World {
    fn default() -> Self {
        let lights = vec![Light::new(
            Point::new(-10.0, 10.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        )];
        let sphere_1 = Object {
            material: Material {
                pattern: Pattern::solid(Color::new(0.8, 1.0, 0.6)),
                diffuse: 0.7,
                specular: 0.2,
                ..Default::default()
            },
            ..Default::default()
        };

        let sphere_2 = Object {
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
    pub fn new(lights: Vec<Light>, objects: Vec<Object>) -> Self {
        Self { lights, objects }
    }

    pub fn color_at_intersection_with(&self, ray: Ray) -> Color {
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
                self.is_shadowed(*light, intersection_state.over_point),
            );
        }

        color
    }

    /// Check if a point is shadowed.
    /// Creates a ray from the point to the light source, and checks
    /// if it intersects any object before reaching the light.
    fn is_shadowed(&self, light: Light, point: Point) -> bool {
        let shadow_v = light.position - point;
        let distance = shadow_v.magnitude();
        let direction = shadow_v.normalize();

        let shadow_ray = Ray::new(point, direction);
        let intersections = shadow_ray.intersect_world(self);
        match intersections.hit() {
            // intersection between point and light
            Some(hit) if hit.t < distance => true,
            // the object is on the other side of the light
            Some(_hit) => false,
            // no intersections
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::intersection::Intersection;
    use crate::pattern::Pattern;
    use crate::vector::Vector;

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn default_world_contains_two_spheres() {
        let expected_light = Light::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let expected_sphere_1 = Object {
            material: Material {
                pattern: Pattern::solid(Color::new(0.8, 1.0, 0.6)),
                diffuse: 0.7,
                specular: 0.2,
                ..Default::default()
            },
            ..Default::default()
        };

        let expected_sphere_2 = Object {
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

        assert_eq!(Color::new(0.1, 0.1, 0.1), color);
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
    fn shading_an_intersection_in_the_shadow() {
        let light = Light::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let sphere_1 = Object::default();
        let sphere_2 = Object {
            transformation: transformations::translation(0.0, 0.0, 10.0),
            ..Object::default()
        };
        let world = World::new(vec![light], vec![sphere_1, sphere_2]);
        let ray = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let intersection = Intersection::new(4.0, sphere_2);
        let intersection_state = IntersectionState::prepare(intersection, ray);

        let color = world.shade_hit(intersection_state);

        assert_eq!(Color::new(0.1, 0.1, 0.1), color);
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
        let inner_color = Color::white();
        let mut world = World::default();
        {
            let outer = world.objects.get_mut(0).unwrap();
            outer.material.ambient = 1.0;
            let mut inner = world.objects.get_mut(1).unwrap();
            inner.material.pattern = Pattern::solid(inner_color);
            inner.material.ambient = 1.0;
        }
        let inner = world.objects[1];
        let ray = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0.0, 0.0, -1.0));

        let color = world.color_at_intersection_with(ray);

        assert_eq!(inner_color, color)
    }

    #[test]
    fn point_is_not_in_shadow_when_nothing_is_collinear_between_point_and_light() {
        let world = World::default();
        let point = Point::new(0.0, 10.0, 0.0);

        assert_eq!(false, world.is_shadowed(world.lights[0], point))
    }

    #[test]
    fn point_is_in_shadow_when_an_object_is_between_point_and_light() {
        let world = World::default();
        let point = Point::new(10.0, -10.0, 10.0);

        assert!(world.is_shadowed(world.lights[0], point))
    }

    #[test]
    fn point_is_not_in_shadow_when_it_is_behind_light() {
        let world = World::default();
        let point = Point::new(-20.0, 20.0, -20.0);

        assert_eq!(false, world.is_shadowed(world.lights[0], point))
    }

    #[test]
    fn point_is_not_in_shadow_when_an_object_is_behind_it() {
        let world = World::default();
        let point = Point::new(-2.0, 2.0, -2.0);

        assert_eq!(false, world.is_shadowed(world.lights[0], point))
    }
}
