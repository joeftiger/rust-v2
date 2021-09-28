use crate::geometry::spherical_to_cartesian_trig;
use crate::{Float, Vec2, Vec3};
use cgmath::Zero;
#[cfg(not(feature = "f64"))]
use core::f32::consts::{FRAC_PI_2, FRAC_PI_4, TAU};
#[cfg(feature = "f64")]
use core::f64::consts::{FRAC_PI_2, FRAC_PI_4, TAU};

/// Samples a non-concentric point on the unit disk.
///
/// # Constraints
/// * `sample`: All values should be within `[0, 1]`.
///
/// # Arguments
/// * `sample`: A random sample
#[inline]
pub fn sample_unit_disk(sample: Vec2) -> Vec2 {
    let (sin, cos) = Float::sin_cos(sample.x * TAU);

    sample.y * Vec2::new(cos, sin)
}

/// Samples a concentric point on the unit disk.
///
/// # Constraints
/// * `sample`: All values should be within `[0, 1]`.
///
/// # Arguments
/// * `sample`: A random sample
#[inline]
pub fn sample_unit_disk_concentric(sample: Vec2) -> Vec2 {
    // Map uniform random numbers to [-1,1]^2
    let offset = 2.0 * sample - Vec2::new(1.0, 1.0);

    // Handle degeneracy at the origin
    if offset.x == 0.0 || offset.y == 0.0 {
        return Vec2::zero();
    }

    // Apply concentric mapping to point
    let (r, theta) = if offset.x.abs() > offset.y.abs() {
        (offset.x, FRAC_PI_4 * offset.y / offset.x)
    } else {
        (offset.y, -FRAC_PI_4 * offset.x / offset.y + FRAC_PI_2)
    };

    let (sin, cos) = theta.sin_cos();

    Vec2::new(r * cos, r * sin)
}

/// Samples a point on the unit hemisphere with a cosine distribution described by the sample.
///
/// # Constraints
/// * `sample`: All values should be within `[0, 1]`.
///
/// # Arguments
/// * `sample`: A random sample
#[inline]
pub fn sample_unit_hemisphere(sample: Vec2) -> Vec3 {
    let d = sample_unit_disk_concentric(sample);

    let b = 1.0 - d.y * d.y;
    let right = b - d.x * d.x;
    let y = right.max(0.0).sqrt();

    Vec3::new(d.x, y, d.y)
}

/// Samples a point on the unit sphere with a uniform distribution described by the sample.
///
/// # Constraints
/// * `sample`: All values should be within `[0, 1]`.
///
/// # Arguments
/// * `sample`: A random sample
#[inline]
pub fn sample_unit_sphere(sample: Vec2) -> Vec3 {
    let z = 1.0 - 2.0 * sample.x;

    let r = (1.0 - z * z).max(0.0).sqrt();
    let (sin_phi, cos_phi) = Float::sin_cos(sample.y * TAU);

    let x = r * cos_phi;
    let y = r * sin_phi;

    Vec3::new(x, y, z)
}
/// Samples a cone around the `(0, 1, 0)` axis with a uniform distribution described by the sample.
///
/// # Constraints
/// * `sample`: All values should be within `[0, 1]`.
///
/// # Arguments
/// * `sample`: A random sample in `[0, 1]`
/// * `cos_theta_max`: The max angle
///
/// # Results
/// * `Vector3`: A direction in the cone around `(0, 1, 0)`
#[inline]
pub fn sample_unit_cone(sample: Vec2, cos_theta_max: Float) -> Vec3 {
    let cos_theta = cos_theta_max.lerp(1.0, sample.x);
    let sin_theta = Float::sqrt(1.0 - cos_theta * cos_theta);
    let (sin_phi, cos_phi) = Float::sin_cos(sample.y * TAU);

    spherical_to_cartesian_trig(sin_theta, cos_theta, sin_phi, cos_phi)
}

/// Computes the pdf a uniformly sampled unit cone.
///
/// # Arguments
/// * `cos_theta`: The cone angle
#[inline]
pub fn uniform_cone_pdf(cos_theta: Float) -> Float {
    1.0 / (TAU * (1.0 - cos_theta))
}
