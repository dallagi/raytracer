use crate::canvas::Canvas;
use crate::matrix::Matrix;
use crate::point::Point;
use crate::ray::Ray;
use crate::world::World;

// TODO: rename fields to better clarify between camera/rendered canvas

pub struct Camera {
    /// Horizontal size (in pixels) of the rendered canvas
    hsize: usize,
    /// Vertical size (in pixels) of the rendered canvas
    vsize: usize,
    /// Angle describing how much the camera can see
    /// A narrow angle means a more zoomed-in image, while
    /// a wide angle means a zoomed-out one.
    field_of_view: f64,
    /// Transformation describing how the world should be oriented
    /// relative to the camera
    transform: Matrix<4, 4>,
    /// The size (in world space units) of a pixel on the canvas
    pixel_size: f64,
    /// Half the width of the camera's canvas
    half_width: f64,
    /// Half the height of the camera's canvas
    half_height: f64,
}

impl Camera {
    const CANVAS_WORLD_Z: f64 = -1.0;

    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Self {
        let mut camera = Self {
            hsize,
            vsize,
            field_of_view,
            transform: Matrix::identity(),
            // fields below are initialized in `set_canvas_properties`
            pixel_size: 0.0,
            half_width: 0.0,
            half_height: 0.0,
        };

        camera.set_canvas_properties();

        camera
    }

    pub fn with_transform(self, transform_matrix: Matrix<4, 4>) -> Self {
        Self {
            transform: self.transform >> transform_matrix,
            ..self
        }
    }

    pub fn render(&self, world: World) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);

        for y in 0..self.vsize {
            self.print_progress(y);

            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at_intersection_with(ray);
                image.write_pixel(x, y, color);
            }
        }

        println!("\nDone.");
        image
    }

    fn print_progress(&self, y: usize) {
        let pixels_count = self.hsize * self.vsize;
        let progress_percentage =
            (((self.hsize * y) as f64 / pixels_count as f64) * 100_f64).round();
        print!("\rRendering... {progress_percentage}%");
    }

    /// Builds a ray that starts from the camera and passes through pixel (x, y) on the canvas
    fn ray_for_pixel(&self, pixel_x: usize, pixel_y: usize) -> Ray {
        // offsets from the edges of canvas to the pixel's center
        let x_offset = (pixel_x as f64 + 0.5) * self.pixel_size;
        let y_offset = (pixel_y as f64 + 0.5) * self.pixel_size;

        // untransformed coordinates of the pixel in world space
        // (camera looks toward -z, so +x is on the left)
        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;
        let world_pixel_point = Point::new(world_x, world_y, Self::CANVAS_WORLD_Z);

        let transformed_pixel = self.transform.inverse() * world_pixel_point;
        let transformed_origin = self.transform.inverse() * Point::origin();
        let direction = (transformed_pixel - transformed_origin).normalize();

        return Ray::new(transformed_origin, direction);
    }

    fn set_canvas_properties(&mut self) {
        // width of half the camera's canvas
        // (note that this canvas is one unit in front of the camera)
        let half_view = (self.field_of_view / 2.0).tan();

        let aspect_ratio = (self.hsize as f64) / (self.vsize as f64);

        if aspect_ratio >= 1.0 {
            self.half_width = half_view;
            self.half_height = half_view / aspect_ratio;
        } else {
            self.half_width = half_view * aspect_ratio;
            self.half_height = half_view;
        }

        self.pixel_size = (self.half_width * 2.0) / (self.hsize as f64)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::color::Color;
    use crate::float_eq::FloatEq;
    use crate::matrix::transformations;
    use crate::point::Point;
    use crate::vector::Vector;
    use crate::view_transform;

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn default_transform_for_camera_is_identity_matrix() {
        let camera = Camera::new(160, 120, PI / 2.0);

        assert_eq!(160, camera.hsize);
        assert_eq!(120, camera.vsize);
        assert_eq!(PI / 2.0, camera.field_of_view);
    }

    #[test]
    fn pixel_size_is_calculated_correctly_for_horizontal_canvas() {
        let camera = Camera::new(200, 125, PI / 2.0);

        assert!(0.01_f64.float_eq(camera.pixel_size))
    }

    #[test]
    fn pixel_size_is_calculated_correctly_for_vertical_canvas() {
        let camera = Camera::new(125, 200, PI / 2.0);

        assert!(0.01_f64.float_eq(camera.pixel_size))
    }

    #[test]
    fn can_create_ray_through_the_center_of_the_canvas() {
        let camera = Camera::new(201, 101, PI / 2.0);

        let ray = camera.ray_for_pixel(100, 50);

        assert_eq!(Point::origin(), ray.origin);
        assert_eq!(Vector::new(0.0, 0.0, -1.0), ray.direction)
    }

    #[test]
    fn can_create_ray_through_a_corner_of_the_canvas() {
        let camera = Camera::new(201, 101, PI / 2.0);

        let ray = camera.ray_for_pixel(0, 0);

        assert_eq!(Point::origin(), ray.origin);
        assert_eq!(Vector::new(0.66519, 0.33259, -0.66851), ray.direction)
    }

    #[test]
    fn can_create_ray_from_transformed_camera() {
        let camera = Camera::new(201, 101, PI / 2.0)
            .with_transform(transformations::translation(0.0, -2.0, 5.0))
            .with_transform(transformations::rotation_y(PI / 4.0));

        let ray = camera.ray_for_pixel(100, 50);

        assert_eq!(Point::new(0.0, 2.0, -5.0), ray.origin);
        assert_eq!(
            Vector::new((2.0_f64).sqrt() / 2.0, 0.0, -(2.0_f64).sqrt() / 2.0),
            ray.direction
        )
    }

    #[test]
    fn renders_world() {
        let world = World::default();
        let eye_from = Point::new(0.0, 0.0, -5.0);
        let eye_to = Point::origin();
        let up = Vector::new(0.0, 1.0, 0.0);
        let camera = Camera::new(11, 11, PI / 2.0)
            .with_transform(view_transform::view_transform(eye_from, eye_to, up));

        let image = camera.render(world);

        assert_eq!(Color::new(0.38066, 0.47583, 0.2855), image.pixel_at(5, 5))
    }
}
