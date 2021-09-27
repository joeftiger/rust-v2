use crate::{Float, Vec3};

/// A ray consists of an
/// - `origin` and a
/// - `direction`.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ray {
    /// The origin of the ray.
    pub origin: Vec3,
    /// The direction of the ray
    pub direction: Vec3,
    pub t_start: Float,
    pub t_end: Float,
}

impl Ray {
    #[inline]
    pub const fn new(origin: Vec3, direction: Vec3) -> Self {
        Self::new2(origin, direction, 0.0, Float::INFINITY)
    }

    #[inline]
    pub const fn new2(origin: Vec3, direction: Vec3, t_start: Float, t_end: Float) -> Self {
        Self {
            origin,
            direction,
            t_start,
            t_end,
        }
    }

    #[inline]
    pub fn at(&self, t: Float) -> Vec3 {
        debug_assert!(t.is_finite());

        self.origin + t * self.direction
    }

    #[inline]
    pub fn contains(&self, t: Float) -> bool {
        self.t_start <= t && t <= self.t_end
    }
}
