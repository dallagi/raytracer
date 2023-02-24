use crate::color::Color;
use crate::light::Light;
use crate::material::Material;
use crate::point::Point;
use crate::vector::Vector;

pub fn lighting(
    material: Material,
    light: Light,
    position: Point,
    eye_vector: Vector,
    normal_vector: Vector,
) -> Color {
    // combine the surface color with the light's color/intensity
    let effective_color = material.color * light.intensity;

    // direction to the light source
    let light_vector = (light.position - position).normalize();

    //ambient contribution
    let ambient = effective_color * material.ambient;

    // light_dot_normal represents the cosine of the angle between the
    // light vector and the normal vector. A negative number means the
    // light is on the other side of the surface.
    let light_dot_normal = light_vector.dot(normal_vector);

    let (diffuse, specular) = if light_dot_normal < 0.0 {
        (Color::black(), Color::black())
    } else {
        let diffuse = effective_color * material.diffuse * light_dot_normal;

        // reflect_dot_eye represents the cosine of the angle between the
        // reflection vector and the eye vector. A negative number means the
        // light reflects away from the eye.
        let reflect_vector = (-light_vector).reflect(normal_vector);
        let reflect_dot_eye = reflect_vector.dot(eye_vector);

        let specular = if reflect_dot_eye <= 0.0 {
            Color::black()
        } else {
            let factor = reflect_dot_eye.powf(material.shininess);
            light.intensity * material.specular * factor
        };

        (diffuse, specular)
    };
    ambient + diffuse + specular
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_lighting_with_eye_between_light_and_surface() {
        // ambient, diffuse and specular components at full strength
        let material = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);
        let eye_v = Vector::new(0.0, 0.0, -1.0);
        let normal_v = Vector::new(0.0, 0.0, -1.0);
        let light = Light::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let result = lighting(material, light, position, eye_v, normal_v);

        assert_eq!(Color::new(1.9, 1.9, 1.9), result);
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_with_eye_offset_45_degrees() {
        // specular component at roughly zero, ambient and diffuse at full strength
        let material = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);
        let eye_v = Vector::new(0.0, (2.0_f64).sqrt() / 2.0, -(2.0_f64).sqrt() / 2.0);
        let normal_v = Vector::new(0.0, 0.0, -1.0);
        let light = Light::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let result = lighting(material, light, position, eye_v, normal_v);

        assert_eq!(Color::new(1.0, 1.0, 1.0), result);
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45() {
        // specular component at roughly zero
        let material = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);
        let eye_v = Vector::new(0.0, 0.0, -1.0);
        let normal_v = Vector::new(0.0, 0.0, -1.0);
        let light = Light::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let result = lighting(material, light, position, eye_v, normal_v);

        assert_eq!(Color::new(0.7364, 0.7364, 0.7364), result);
    }

    #[test]
    fn lighting_with_eye_in_path_of_reflection_vector() {
        // specular component at full strength, ambient and diffuse same as previous test
        let material = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, -(2.0_f64).sqrt() / 2.0, -(2.0_f64).sqrt() / 2.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = Light::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let result = lighting(material, light, position, eyev, normalv);

        assert_eq!(Color::new(1.63638, 1.63638, 1.63638), result);
    }

    #[test]
    fn lighting_with_light_behind_surface() {
        // in this case only the ambient lighting will be considered
        let material = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);
        let eye_v = Vector::new(0.0, 0.0, -1.0);
        let normal_v = Vector::new(0.0, 0.0, -1.0);
        let light = Light::new(Point::new(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));

        let result = lighting(material, light, position, eye_v, normal_v);

        assert_eq!(Color::new(0.1, 0.1, 0.1), result);
    }
}
