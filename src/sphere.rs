#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Object {
    Sphere,
}

impl Object {
    pub fn sphere() -> Self {
        Self::Sphere
    }
}
