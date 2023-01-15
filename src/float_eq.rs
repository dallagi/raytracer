pub trait FloatEq {
    fn float_eq(&self, rhs: Self) -> bool;
}

const EPSILON: f32 = 0.000001;

impl FloatEq for f32 {
    fn float_eq(&self, rhs: Self) -> bool {
        let error_margin = EPSILON;

        if (self - rhs).abs() < error_margin {
            true
        } else {
            false
        }
    }
}

impl FloatEq for f64 {
    fn float_eq(&self, rhs: Self) -> bool {
        let error_margin = EPSILON as f64;

        if (self - rhs).abs() < error_margin {
            true
        } else {
            false
        }
    }
}
