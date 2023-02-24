pub trait FloatEq {
    fn float_eq(&self, rhs: Self) -> bool;
}

const ERROR_MARGIN: f64 = 0.0001;

impl FloatEq for f64 {
    fn float_eq(&self, other: Self) -> bool {
        (self - other).abs() < ERROR_MARGIN
    }
}
