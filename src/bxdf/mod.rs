pub mod bsdf;
pub mod fresnel;
pub mod lambertian;
pub mod oren_nayar;
pub mod refraction;
pub mod specular;

pub use bsdf::*;
pub use fresnel::*;
pub use lambertian::*;
pub use oren_nayar::*;
pub use specular::*;

use crate::util::mc::sample_unit_hemisphere;
use crate::{Float, Rot3, Spectrum, Vec2, Vec3, PACKET_SIZE};
use cgmath::{InnerSpace, Rotation as cgRot};
use core::ops::Mul;
#[cfg(not(feature = "f64"))]
use std::f32::consts::FRAC_1_PI;
#[cfg(feature = "f64")]
use std::f64::consts::FRAC_1_PI;

/// A rotation is either
/// - not happening
/// - flipping direction
/// - some rotation
#[derive(Copy, Clone, Debug)]
pub enum Rotation {
    None,
    Flip,
    Some(Rot3),
}

impl core::ops::Neg for Rotation {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        match self {
            Rotation::Some(r) => Self::Some(r.invert()),
            _ => self,
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
pub const fn bxdf_normal() -> Vec3 {
    Vec3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    }
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
pub const fn cos_theta(v: Vec3) -> Float {
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
    Rot3::between_vectors(bxdf_normal(), v)
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
    /// Returns whether this flag is `reflective`.
    #[inline]
    pub fn reflective(&self) -> bool {
        self.contains(Self::REFLECTION)
    }
    /// Returns whether this flag is `transmissive`.
    #[inline]
    pub fn transmissive(&self) -> bool {
        self.contains(Self::TRANSMISSION)
    }
    /// Returns whether this flag is `diffuse`.
    #[inline]
    pub fn diffuse(&self) -> bool {
        self.contains(Self::DIFFUSE)
    }
    /// Returns whether this flag is `glossy`.
    #[inline]
    pub fn glossy(&self) -> bool {
        self.contains(Self::GLOSSY)
    }
    /// Returns whether this flag is `specular`.
    #[inline]
    pub fn specular(&self) -> bool {
        self.contains(Self::SPECULAR)
    }
    /// Returns whether this flag is `non-specular`.
    #[inline]
    pub fn non_specular(&self) -> bool {
        !self.specular()
    }
}

#[derive(Clone, Debug)]
pub enum BxDFSamplePacket {
    Bundle(Option<BxDFSample<[Float; PACKET_SIZE]>>),
    Split([Option<BxDFSample<Float>>; PACKET_SIZE]),
}

#[derive(Copy, Clone, Debug)]
pub struct BxDFSample<T> {
    pub spectrum: T,
    pub incident: Vec3,
    pub pdf: Float,
    pub flag: BxDFFlag,
}
impl<T> BxDFSample<T> {
    pub const fn new(spectrum: T, incident: Vec3, pdf: Float, flag: BxDFFlag) -> Self {
        Self {
            spectrum,
            incident,
            pdf,
            flag,
        }
    }
}

#[typetag::serde]
pub trait BxDF {
    /// Returns the type of this bxdf.
    fn flag(&self) -> BxDFFlag;

    /// Matches the flag to be a subset of [Self::flag].
    #[inline]
    fn match_flag(&self, f: BxDFFlag) -> bool {
        let sf = self.flag();
        sf.contains(f)
    }

    /// Evaluates the BxDF.
    ///
    /// # Constraints
    /// * `incident`: All values should be finite (neither infinite nor `NaN`).
    ///                Should be normalized.
    /// * `outgoing`: All values should be finite.
    ///                Should be normalized.
    ///
    /// # Arguments
    /// * `incident`: The incident direction onto the intersection we evaluate
    /// * `outgoing`: The outgoing light direction
    fn evaluate(&self, incident: Vec3, outgoing: Vec3) -> Spectrum;

    /// Evaluates the BxDF with possible spectral dependencies in a packet size of [PACKET_SIZE].
    ///
    /// # Constraints
    /// * `incident`: All values should be finite (neither infinite nor `NaN`).
    ///               Should be normalized.
    /// * `outgoing` All values should be finite.
    ///              Should be normalized.
    /// * `indices`: All values should be within `[0, `[Spectrum::size]`)`.
    ///
    /// # Arguments
    /// * `incident`: The incident direction onto the intersection we evaluate
    /// * `outgoing`: The outgoing light direction
    /// * `indices`: The indices of the spectrum to evaluate
    // TODO: Use u16 or usize?
    fn evaluate_packet(
        &self,
        incident: Vec3,
        outgoing: Vec3,
        indices: &[usize; PACKET_SIZE],
    ) -> [Float; PACKET_SIZE] {
        let mut packet = [0.0; PACKET_SIZE];
        for i in 0..PACKET_SIZE {
            packet[i] = self.evaluate_lambda(incident, outgoing, indices[i] as usize);
        }
        packet
    }

    /// Evaluates the BxDF with possible spectral dependencies.
    ///
    /// # Constraints
    /// * `incident`: All values should be finite (neither infinite nor `NaN`).
    ///               Should be normalized.
    /// * `outgoing` All values should be finite.
    ///              Should be normalized.
    /// * `index`: Should be inside `[0, `[Spectrum::size]`)`.
    ///
    /// # Arguments
    /// * `incident`: The incident direction onto the intersection we evaluate
    /// * `outgoing`: The outgoing light direction
    /// * `index`: The spectral index
    fn evaluate_lambda(&self, incident: Vec3, outgoing: Vec3, index: usize) -> Float;

    /// Samples the BxDF.
    ///
    /// # Constraints
    /// * `outgoing`: All values should be finite.
    ///                Should be normalized.
    /// * `sample`: All values should be within `[0, 1]`.
    ///
    /// # Arguments
    /// * `outgoing`: The outgoing light direction
    /// * `sample`: The sample space for randomization
    fn sample(&self, outgoing: Vec3, sample: Vec2) -> Option<BxDFSample<Spectrum>> {
        let incident = flip_if_neg(sample_unit_hemisphere(sample));
        let spectrum = self.evaluate(incident, outgoing);
        let pdf = self.pdf(incident, outgoing);

        Some(BxDFSample::new(spectrum, incident, pdf, self.flag()))
    }

    /// Samples the BxDF with possible spectral dependencies in a packet size of [PACKET_SIZE].
    ///
    /// # Constraints
    /// * `outgoing`: All values should be finite.
    ///                Should be normalized.
    /// * `sample`: All values should be within `[0, 1]`.
    ///
    /// # Arguments
    /// * `outgoing`: The outgoing light direction
    /// * `sample`: The sample space for randomization
    fn sample_packet(
        &self,
        outgoing: Vec3,
        sample: Vec2,
        indices: &[usize; PACKET_SIZE],
    ) -> BxDFSamplePacket {
        let incident = flip_if_neg(sample_unit_hemisphere(sample));
        let spectrum = self.evaluate_packet(incident, outgoing, indices);
        let pdf = self.pdf(incident, outgoing);

        let bundle = Some(BxDFSample::new(spectrum, incident, pdf, self.flag()));

        BxDFSamplePacket::Bundle(bundle)
    }

    /// Samples the BxDF with possible spectral dependencies.
    ///
    /// # Constraints
    /// * `outgoing`: All values should be finite.
    ///                Should be normalized.
    /// * `sample`: All values should be within `[0, 1]`.
    ///
    /// # Arguments
    /// * `outgoing`: The outgoing light direction
    /// * `sample`: The sample space for randomization
    fn sample_lambda(
        &self,
        outgoing: Vec3,
        sample: Vec2,
        index: usize,
    ) -> Option<BxDFSample<Float>> {
        let incident = flip_if_neg(sample_unit_hemisphere(sample));
        let lambda = self.evaluate_lambda(incident, outgoing, index);
        let pdf = self.pdf(incident, outgoing);

        Some(BxDFSample::new(lambda, incident, pdf, self.flag()))
    }

    /// Computes the probability density function (`pdf`) for the pair of directions.
    ///
    /// # Constraints
    /// * `incident`: All values should be finite (neither infinite nor `NaN`).
    ///                Should be normalized.
    /// * `outgoing`: All values should be finite.
    ///                Should be normalized.
    ///
    /// # Arguments
    /// * `incident`: The incident direction onto the intersection we evaluate
    /// * `outgoing`: The outgoing light direction
    #[inline]
    fn pdf(&self, incident: Vec3, outgoing: Vec3) -> Float {
        if same_hemisphere(incident, outgoing) {
            cos_theta(incident).abs() * FRAC_1_PI
        } else {
            0.0
        }
    }
}
