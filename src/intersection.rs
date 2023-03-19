use crate::float_eq::FloatEq;
use crate::object::Object;

#[derive(Copy, Clone, Debug)]
pub struct Intersection {
    pub t: f64,
    pub object: Object,
}

impl Intersection {
    pub fn new(t: f64, object: Object) -> Self {
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
        let sphere = Object::default();
        let intersection = Intersection::new(3.5, sphere);

        assert_eq!(3.5, intersection.t);
        assert_eq!(sphere, intersection.object);
    }
}
