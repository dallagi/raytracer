use crate::matrix::Matrix;

// TODO: rename fields to better clarify between camera/rendered canvas

struct Camera {
    /// Horizontal size (in pixels) of the rendered canvas
    hsize: f64,
    /// Vertical size (in pixels) of the rendered canvas
    vsize: f64,
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
    fn new(hsize: f64, vsize: f64, field_of_view: f64) -> Self {
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

    fn set_canvas_properties(&mut self) {
        // width of half the camera's canvas
        // (note that this canvas is one unit in front of the camera)
        let half_view = (self.field_of_view / 2.0).tan();

        let aspect_ratio = self.hsize / self.vsize;

        if aspect_ratio >= 1.0 {
            self.half_width = half_view;
            self.half_height = half_view / aspect_ratio;
        } else {
            self.half_width = half_view * aspect_ratio;
            self.half_height = half_view;
        }

        self.pixel_size = (self.half_width * 2.0) / self.hsize
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::float_eq::FloatEq;

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn default_transform_for_camera_is_identity_matrix() {
        let camera = Camera::new(160.0, 120.0, PI / 2.0);

        assert_eq!(160.0, camera.hsize);
        assert_eq!(120.0, camera.vsize);
        assert_eq!(PI / 2.0, camera.field_of_view);
    }

    #[test]
    fn pixel_size_is_calculated_correctly_for_horizontal_canvas() {
        let camera = Camera::new(200.0, 125.0, PI / 2.0);

        assert!(0.01_f64.float_eq(camera.pixel_size))
    }

    #[test]
    fn pixel_size_is_calculated_correctly_for_vertical_canvas() {
        let camera = Camera::new(125.0, 200.0, PI / 2.0);

        assert!(0.01_f64.float_eq(camera.pixel_size))
    }
}
