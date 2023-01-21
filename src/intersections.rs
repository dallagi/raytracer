use std::ops;

use crate::intersection::Intersection;

#[derive(Clone, Debug, PartialEq)]
pub struct Intersections(Vec<Intersection>);

impl Intersections {
    pub fn of(intersections: &[Intersection]) -> Self {
        Self(intersections.to_vec())
    }

    pub fn empty() -> Self {
        Self(vec![])
    }

    pub fn count(&self) -> usize {
        self.0.len()
    }

    pub fn hit(&self) -> Option<Intersection> {
        self.0
            .iter()
            .filter(|intersection| intersection.t >= 0.0)
            .min_by(|i1, i2| i1.t.total_cmp(&i2.t))
            .copied()
    }
}

impl ops::Index<usize> for Intersections {
    type Output = Intersection;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

#[cfg(test)]
mod tests {
    use crate::object::Object;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn aggregates_intersections() {
        let sphere = Object::sphere();
        let intersection_1 = Intersection::new(1.0, sphere);
        let intersection_2 = Intersection::new(2.0, sphere);

        let intersections = Intersections::of(&[intersection_1, intersection_2]);
        assert_eq!(2, intersections.count());
        assert_eq!(1.0, intersections[0].t);
        assert_eq!(2.0, intersections[1].t);
    }

    #[test]
    fn when_all_ts_are_positive_hit_is_intersection_with_lowest_t() {
        let sphere = Object::sphere();
        let intersection_1 = Intersection::new(1.0, sphere);
        let intersection_2 = Intersection::new(2.0, sphere);

        let intersections = Intersections::of(&[intersection_1, intersection_2]);

        assert_eq!(Some(intersection_1), intersections.hit())
    }

    #[test]
    fn when_some_ts_are_negative_hit_is_intersection_with_lowest_nonnegative_t() {
        let sphere = Object::sphere();
        let intersection_1 = Intersection::new(-1.0, sphere);
        let intersection_2 = Intersection::new(1.0, sphere);

        let intersections = Intersections::of(&[intersection_1, intersection_2]);

        assert_eq!(Some(intersection_2), intersections.hit())
    }

    #[test]
    fn when_all_ts_are_negative_intersection_is_nothing() {
        let sphere = Object::sphere();
        let intersection_1 = Intersection::new(-2.0, sphere);
        let intersection_2 = Intersection::new(-1.0, sphere);

        let intersections = Intersections::of(&[intersection_1, intersection_2]);

        assert_eq!(None, intersections.hit())
    }

    #[test]
    fn hit_is_always_intersection_with_lowest_nonnegative_t() {
        let sphere = Object::sphere();
        let intersection_1 = Intersection::new(5.0, sphere);
        let intersection_2 = Intersection::new(7.0, sphere);
        let intersection_3 = Intersection::new(-3.0, sphere);
        let intersection_4 = Intersection::new(2.0, sphere);

        let intersections = Intersections::of(&[
            intersection_1,
            intersection_2,
            intersection_3,
            intersection_4,
        ]);

        assert_eq!(Some(intersection_4), intersections.hit())
    }
}
