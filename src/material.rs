use crate::color::Color;
use crate::float_eq::FloatEq;
use crate::pattern::Pattern;

#[derive(Copy, Clone, Debug)]
pub struct Material {
    pub pattern: Pattern,
    pub ambient: f64,   // usually 0..1
    pub diffuse: f64,   // usually 0..1
    pub specular: f64,  // usually 0..1
    pub shininess: f64, // usually 10..200
}

impl Material {
    fn new(pattern: Pattern, ambient: f64, diffuse: f64, specular: f64, shininess: f64) -> Self {
        assert!(ambient >= 0.0, "Ambient must be nonnegative");
        assert!(diffuse >= 0.0, "Diffuse must be nonnegative");
        assert!(specular >= 0.0, "Specular must be nonnegative");
        assert!(shininess >= 0.0, "Shininess must be nonnegative");

        Self {
            pattern,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            pattern: Pattern::solid(Color::white()),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.pattern == other.pattern
            && self.ambient.float_eq(other.ambient)
            && self.diffuse.float_eq(other.diffuse)
            && self.specular.float_eq(other.specular)
            && self.shininess.float_eq(other.shininess)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_default_material() {
        let m = Material::default();

        assert_eq!(m.pattern, Pattern::solid(Color::new(1.0, 1.0, 1.0)));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }
}
