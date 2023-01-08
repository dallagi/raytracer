pub trait FEquals {
    fn f_equals(&self, rhs: Self) -> bool;
}

impl FEquals for f32 {
    fn f_equals(&self, rhs: Self) -> bool {
        let error_margin = f32::EPSILON;

        if (self - rhs).abs() < error_margin {
            true
        } else {
            false
        }
    }
}
