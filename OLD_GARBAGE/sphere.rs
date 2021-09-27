use crate::geometry::differential::{Atlas, Plane};
use crate::geometry::vec::{Vec2, Vec3};
use core::f64::consts::PI;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub const fn new(center: Vec3, radius: f64) -> Self {
        Self { center, radius }
    }
}

/// The unit sphere [Atlas].
/// The chart coordinates of a [Vec2] are interpreted like the following:
///
/// # Chart coordinates
/// [`uv = (x, y)`](Vec2) is the spherical chart coordinate where
/// - `θ = x` with `θ ∈ [0, π]`
/// - `φ = y` with `φ ∈ [-π, π]`
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SphereAtlas;

impl SphereAtlas {
    #[inline]
    pub const fn new() -> Self {
        Self
    }
}

impl Atlas for SphereAtlas {
    /// Let `θ = uv.x` and `φ = uv.y`.
    ///
    /// # Returns
    /// - `true`: `θ ∈ [0, π]` and `φ ∈ [-π, π]`
    /// - `false`: otherwise
    #[inline]
    fn is_valid_uv(&self, uv: Vec2) -> bool {
        (0.0 <= uv.x && uv.x <= PI) && (0.0 < uv.y && uv.y <= PI)
    }

    /// Let `θ = uv.x` and `φ = uv.y`.
    ///
    /// # Returns
    /// The tangential plane of the unit sphere at `uv`
    fn tangential_plane(&self, uv: Vec2) -> Plane {
        let origin = self.f(uv);
        Plane::new(origin, origin.normalized())
    }

    /// Let `θ = uv.x` and `φ = uv.y`.
    fn normal(&self, uv: Vec2) -> Vec3 {
        self.f(uv).normalized()
    }

    /// Let `θ = uv.x` and `φ = uv.y`.
    ///
    /// Calculates the point on the unit sphere of a chart coordinate.
    ///
    /// # Returns
    /// The position on the unit sphere
    fn f(&self, uv: Vec2) -> Vec3 {
        let (sin_theta, cos_theta) = uv.x.sin_cos();
        let (sin_phi, cos_phi) = uv.y.sin_cos();

        Vec3::new(sin_theta * cos_phi, sin_theta * sin_phi, cos_theta)
    }

    /// Calculates the chart coordinate of a point on the unit sphere.
    ///
    /// # Returns
    /// A vector of `x = θ` and `y = φ`.
    fn f_inv(&self, p: Vec3) -> Vec2 {
        let theta = f64::acos(p.z);
        let phi = f64::atan2(p.y, p.x);

        Vec2::new(theta, phi)
    }
}
