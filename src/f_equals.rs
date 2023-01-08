pub trait FEquals {
    fn f_equals(&self, rhs: Self) -> bool;
}

const EPSILON: f32 = 0.000001;

impl FEquals for f32 {
    fn f_equals(&self, rhs: Self) -> bool {
        let error_margin = EPSILON;

        if (self - rhs).abs() < error_margin {
            true
        } else {
            false
        }
    }
}

impl FEquals for f64 {
    fn f_equals(&self, rhs: Self) -> bool {
        let error_margin = EPSILON as f64;

        if (self - rhs).abs() < error_margin {
            true
        } else {
            false
        }
    }
}
