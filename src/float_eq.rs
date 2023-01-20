pub trait FloatEq {
    fn float_eq(&self, rhs: Self) -> bool;
}

const ERROR_MARGIN: f32 = 0.00001;

impl FloatEq for f32 {
    fn float_eq(&self, other: Self) -> bool {
        (self - other).abs() < ERROR_MARGIN
    }
}

impl FloatEq for f64 {
    fn float_eq(&self, other: Self) -> bool {
        (self - other).abs() < (ERROR_MARGIN as f64)
    }
}
