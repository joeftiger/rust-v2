use crate::Float;

#[derive(Copy, Clone, PartialEq)]
pub enum Plane {
    X(Float),
    Y(Float),
    Z(Float),
}

impl Plane {
    #[inline]
    pub fn val(self) -> Float {
        match self {
            Plane::X(v) => v,
            Plane::Y(v) => v,
            Plane::Z(v) => v,
        }
    }
}
