use crate::Float;

#[derive(Copy, Clone, Debug)]
pub enum Plane {
    X(Float),
    Y(Float),
    Z(Float),
}

impl Plane {
    #[inline]
    pub const fn dim(self) -> usize {
        match self {
            Plane::X(_) => 0,
            Plane::Y(_) => 1,
            Plane::Z(_) => 2,
        }
    }

    #[inline]
    pub const fn val(self) -> Float {
        match self {
            Plane::X(v) => v,
            Plane::Y(v) => v,
            Plane::Z(v) => v,
        }
    }
}
