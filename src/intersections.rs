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
}

impl ops::Index<usize> for Intersections {
    type Output = Intersection;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

#[cfg(test)]
mod tests {
    use crate::sphere::Object;
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
}
