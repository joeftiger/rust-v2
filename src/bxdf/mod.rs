use crate::{Vec3, Rot3, Float};
use core::ops::Mul;
use cgmath::{Rotation as cgRot, InnerSpace, Rotation3, One, Rad, Angle};

// A rotation is either
/// - not happening
/// - flipping direction
/// - some rotation
#[derive(Copy, Clone, Debug)]
pub enum Rotation {
    None,
    Flip,
    Some(Rot3),
}

impl Rotation {
    /// Reversed the rotation (if any).
    #[inline]
    pub fn reversed(&self) -> Self {
        match self {
            Rotation::Some(r) => Self::Some(r.invert()),
            _ => *self,
        }
    }
}

impl Mul<Vec3> for Rotation {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        match self {
            Rotation::None => rhs,
            Rotation::Flip => flip(rhs),
            Rotation::Some(r) => r.rotate_vector(rhs),
        }
    }
}

/// Allows indicating whether an intersection was found along a path starting from a camera or one
/// starting from a light source.
///
/// This has implications on the calculations of `BSDF`.
#[derive(PartialEq)]
pub enum TransportMode {
    Radiance,
    Importance,
}

/// The BxDF normal is defined in the y-axis of the world space.
///
/// # Returns
/// * The global BxDF normal
#[inline]
pub fn bxdf_normal() -> Vec3 {
    Vec3::unit_y()
}

#[inline]
pub fn bxdf_incident_to(v: Vec3) -> Vec3 {
    Vec3::new(-v.x, v.y, -v.z)
}

#[inline]
pub fn is_neg(v: Vec3) -> bool {
    v.y < 0.0
}

#[inline]
pub fn flip_if_neg(mut v: Vec3) -> Vec3 {
    if is_neg(v) {
        v.y = -v.y;
    }
    v
}

#[inline]
pub fn flip(mut v: Vec3) -> Vec3 {
    v.y = -v.y;
    v
}

#[inline]
pub fn bxdf_is_parallel(v: Vec3) -> bool {
    v.y == 0.0
}

#[inline]
pub fn cos_theta(v: Vec3) -> Float {
    v.y
}

#[inline]
pub fn cos2_theta(v: Vec3) -> Float {
    cos_theta(v) * cos_theta(v)
}

#[inline]
pub fn sin2_theta(v: Vec3) -> Float {
    (1.0 - cos2_theta(v)).max(0.0)
}

#[inline]
pub fn sin_theta(v: Vec3) -> Float {
    sin2_theta(v).sqrt()
}

#[inline]
pub fn tan_theta(v: Vec3) -> Float {
    sin_theta(v) / cos_theta(v)
}

#[inline]
pub fn tan2_theta(v: Vec3) -> Float {
    sin2_theta(v) / cos2_theta(v)
}

#[inline]
pub fn cos_phi(v: Vec3) -> Float {
    let sin_theta = sin_theta(v);
    if sin_theta == 0.0 {
        0.0
    } else {
        (v.x / sin_theta).clamp(-1.0, 1.0)
    }
}

#[inline]
pub fn sin_phi(v: Vec3) -> Float {
    let sin_theta = sin_theta(v);
    if sin_theta == 0.0 {
        0.0
    } else {
        (v.z / sin_theta).clamp(-1.0, 1.0)
    }
}

#[inline]
pub fn cos2_phi(v: Vec3) -> Float {
    let cos_phi = cos_phi(v);
    cos_phi * cos_phi
}

#[inline]
pub fn sin2_phi(v: Vec3) -> Float {
    let sin_phi = sin_phi(v);
    sin_phi * sin_phi
}

#[inline]
pub fn cos_d_phi(a: Vec3, b: Vec3) -> Float {
    let abxz = a.x * b.x + a.z * b.z;
    let axz = a.x * a.x + a.z * a.z;
    let bxz = b.x * b.x + b.z * b.z;

    (abxz / Float::sqrt(axz * bxz)).clamp(-1.0, 1.0)
}

#[inline]
pub fn refract(v: Vec3, n: Vec3, eta: Float) -> Option<Vec3> {
    let cos_i = n.dot(v);
    let sin_t2 = eta * eta * cos_i.mul_add(-cos_i, 1.0).max(0.0);

    if sin_t2 > 1.0 {
        None
    } else {
        let cos_t = Float::sqrt(1.0 - sin_t2);
        let right = eta.mul_add(cos_i, -cos_t);
        let r = eta * -v + right * n;

        Some(r)
    }
}

#[inline]
pub fn face_forward(v: Vec3, n: Vec3) -> Vec3 {
    if n.dot(v) > 0.0 {
        v
    } else {
        -v
    }
}

#[inline]
pub fn same_hemisphere(a: Vec3, b: Vec3) -> bool {
    a.y * b.y > 0.0
}

#[inline]
pub fn world_to_bxdf(v: Vec3) -> Rotation {
    if v == Vec3::unit_y() {
        Rotation::None
    } else if v == -Vec3::unit_y() {
        Rotation::Flip
    } else {
        Rotation::Some(Rot3::between_vectors(v, bxdf_normal()))
    }
}

#[inline]
pub fn bxdf_to_world(v: Vec3) -> Rot3 {
    if v == Vec3::unit_y() {
        Rot3::one()
    } else if v == -Vec3::unit_y() {
        Rot3::from_angle_z(Rad::turn_div_2())
    } else {
        Rot3::between_vectors(bxdf_normal(), v)
    }
}

bitflags::bitflags! {
    pub struct BxDFFlag: u8 {
        const NONE = 0;
        const REFLECTION = 1 << 0;
        const TRANSMISSION = 1 << 1;
        const DIFFUSE = 1 << 2;
        const GLOSSY = 1 << 3;
        const SPECULAR = 1 << 4;
        const ALL = Self::REFLECTION.bits | Self::TRANSMISSION.bits | Self::DIFFUSE.bits | Self::GLOSSY.bits | Self::SPECULAR.bits;
    }
}

impl BxDFFlag {
    #[inline]
    pub fn reflective(&self) -> bool {
        self.contains(Self::REFLECTION)
    }
    #[inline]
    pub fn transmissive(&self) -> bool {
        self.contains(Self::TRANSMISSION)
    }
    #[inline]
    pub fn diffuse(&self) -> bool {
        self.contains(Self::DIFFUSE)
    }
    #[inline]
    pub fn glossy(&self) -> bool {
        self.contains(Self::GLOSSY)
    }
    #[inline]
    pub fn specular(&self) -> bool {
        self.contains(Self::SPECULAR)
    }
    #[inline]
    pub fn non_specular(&self) -> bool {
        !self.specular()
    }
}
