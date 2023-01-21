use crate::{float_eq::FloatEq, sphere::Object};

#[derive(Copy, Clone, Debug)]
pub struct Intersection {
    pub t: f32,
    pub object: Object,
}

impl Intersection {
    pub fn new(t: f32, object: Object) -> Self {
        Self { t, object }
    }
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        self.t.float_eq(other.t) && self.object == other.object
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn intersectin_encapsulates_t_and_object() {
        let sphere = Object::sphere();
        let intersection = Intersection::new(3.5, sphere);

        assert_eq!(3.5, intersection.t);
        assert_eq!(sphere, intersection.object);
    }
}