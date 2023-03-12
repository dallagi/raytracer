use crate::color::Color;
use crate::intersection_state::IntersectionState;
use crate::lighting::lighting;
use crate::world::World;

fn shade_hit(world: World, intersection_state: IntersectionState) -> Color {
    lighting(
        intersection_state.object.material,
        world.light,
        intersection_state.point,
        intersection_state.eye_v,
        intersection_state.normal_v,
    )
}

#[cfg(test)]
mod tests {
    use crate::intersection::Intersection;
    use crate::light::Light;
    use crate::point::Point;
    use crate::ray::Ray;
    use crate::vector::Vector;

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn shading_an_intersection_from_the_outside() {
        let world = World::default();
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = world.objects.first().unwrap().clone();
        let intersection = Intersection::new(4.0, shape);

        let intersection_state = IntersectionState::prepare(intersection, ray);
        let color = shade_hit(world, intersection_state);

        assert_eq!(Color::new(0.38066, 0.47583, 0.2855), color);
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut world = World::default();
        world.light = Light::new(Point::new(0.0, 0.25, 0.0), Color::new(1.0, 1.0, 1.0));

        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let shape = world.objects[1].clone();
        let intersection = Intersection::new(0.5, shape);

        let intersection_state = IntersectionState::prepare(intersection, ray);
        let color = shade_hit(world, intersection_state);

        assert_eq!(Color::new(0.90498, 0.90498, 0.90498), color);
    }
}
