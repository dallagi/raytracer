use crate::color::Color;
use crate::float_eq::FloatEq;

#[derive(Copy, Clone, Debug)]
pub struct Material {
    pub color: Color,
    pub ambient: f32,   // usually 0..1
    pub diffuse: f32,   // usually 0..1
    pub specular: f32,  // usually 0..1
    pub shininess: f32, // usually 10..200
}

impl Material {
    fn new(color: Color, ambient: f32, diffuse: f32, specular: f32, shininess: f32) -> Self {
        assert!(ambient >= 0.0, "Ambient must be nonnegative");
        assert!(diffuse >= 0.0, "Diffuse must be nonnegative");
        assert!(specular >= 0.0, "Specular must be nonnegative");
        assert!(shininess >= 0.0, "Shininess must be nonnegative");

        Self {
            color,
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
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
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

        assert_eq!(m.color, Color::new(1.0, 1.0, 1.0));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }
}