use crate::geometry::vec::Vec3;

/// A ray consists of an
/// - `origin` and a
/// - `direction`.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ray {
    /// The origin of the ray.
    pub origin: Vec3,
    /// The direction of the ray
    pub direction: Vec3,
}

impl Ray {
    #[inline]
    pub const fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    #[inline]
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}
