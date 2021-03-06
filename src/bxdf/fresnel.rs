use crate::bxdf::refraction::RefractiveType;
use crate::{Float, Spectrum, PACKET_SIZE};
use core::mem;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum FresnelType {
    /// A `Fresnel` implementation for dielectric materials.
    Dielectric(FresnelDielectric),
    /// A no-operation `Fresnel` implementation that returns 100% reflection for all incoming directions.
    /// Although this is physically implausible, it is a convenient capability to have available.
    Noop,
}

impl Fresnel for FresnelType {
    #[inline]
    fn evaluate(&self, cos_i: Float) -> Spectrum {
        match self {
            FresnelType::Dielectric(t) => t.evaluate(cos_i),
            FresnelType::Noop => Spectrum::splat(1.0),
        }
    }

    fn evaluate_packet(
        &self,
        cos_i: Float,
        lambdas: &[Float; PACKET_SIZE],
    ) -> [Float; PACKET_SIZE] {
        match self {
            FresnelType::Dielectric(d) => d.evaluate_packet(cos_i, lambdas),
            FresnelType::Noop => [1.0; PACKET_SIZE],
        }
    }

    #[inline]
    fn evaluate_lambda(&self, cos_i: Float, lambda: Float) -> Float {
        match self {
            FresnelType::Dielectric(f) => f.evaluate_lambda(cos_i, lambda),
            FresnelType::Noop => 1.0,
        }
    }
}

/// Computes the fraction of reflected light for parallel polarized light.
///
/// # Arguments
/// * `cos_i` - The cosine of the angle between normal and incident
/// * `cos_t` - The cosine of the angle between normal and transmission
/// * `eta_i` - The index of refraction for the incident medium
/// * `eta_t` - The index of refraction for the transmission medium
#[inline]
pub fn dielectric_parallel(cos_i: Float, cos_t: Float, eta_i: Float, eta_t: Float) -> Float {
    let it = eta_i * cos_t;
    let ti = eta_t * cos_i;

    (ti - it) / (ti + it)
}

/// Computes the fraction of reflected light for perpendicular polarized light.
///
/// # Arguments
/// * `cos_i` - The cosine of the angle between normal and incident
/// * `cos_t` - The cosine of the angle between normal and transmission
/// * `eta_i` - The index of refraction for the incident medium
/// * `eta_t` - The index of refraction for the transmission medium
#[inline]
pub fn dielectric_perpendicular(cos_i: Float, cos_t: Float, eta_i: Float, eta_t: Float) -> Float {
    let tt = eta_t * cos_t;
    let ii = eta_i * cos_i;

    (ii - tt) / (ii + tt)
}

/// Computes the Fresnel reflection for dielectric materials and unpolarized light.
///
/// # Arguments
/// * `cos_i` - The cosine of the angle between normal and incident
/// * `eta_i` - The index of refraction for the incident medium
/// * `eta_t` - The index of refraction for the transmission medium
pub fn fresnel_dielectric(mut cos_i: Float, mut eta_i: Float, mut eta_t: Float) -> Float {
    // potentially swap indices of refraction
    let entering = cos_i > 0.0;
    if !entering {
        mem::swap(&mut eta_i, &mut eta_t);
        cos_i = cos_i.abs();
    }

    // compute cos_t using Snell's law
    let sin_i = Float::max(0.0, 1.0 - cos_i * cos_i).sqrt();
    let sin_t = eta_i * sin_i / eta_t;

    // handle total internal reflection
    if sin_t >= 1.0 {
        return 1.0;
    }

    let cos_t = Float::max(0.0, 1.0 - sin_t * sin_t).sqrt();
    let r_par = dielectric_parallel(cos_i, cos_t, eta_i, eta_t);
    let r_perp = dielectric_perpendicular(cos_i, cos_t, eta_i, eta_t);

    0.5 * (r_par * r_par + r_perp * r_perp)
}

/// Provides an interface for computing Fresnel reflection coefficients.
pub trait Fresnel {
    /// Computes the surface reflectance at an angle.
    ///
    /// # Arguments
    /// * `cos_i` - The cosine of the angle between the normal and the incident
    fn evaluate(&self, cos_i: Float) -> Spectrum;

    /// Computes the surface reflectance at an angle.
    ///
    /// # Arguments
    /// * `cos_i` - The cosine of the angle between the normal and the incident
    /// * `lambdas`: The wavelengths to compute for
    fn evaluate_packet(&self, cos_i: Float, lambdas: &[Float; PACKET_SIZE])
        -> [Float; PACKET_SIZE];

    /// Computes the surface reflectance at an angle.
    ///
    /// # Arguments
    /// * `cos_i` - The cosine of the angle between the normal and the incident
    /// * `lambda` - The wavelength to compute for
    fn evaluate_lambda(&self, cos_i: Float, lambda: Float) -> Float;
}

/// An implementation of `Fresnel` for dielectric materials.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct FresnelDielectric {
    pub eta_i: RefractiveType,
    pub eta_t: RefractiveType,
}

impl FresnelDielectric {
    /// Creates a new dielectric.
    ///
    /// # Arguments
    /// * `eta_i` - The index of refraction for the incident medium
    /// * `eta_t` - The index of refraction for the transmission medium
    ///
    /// # Returns
    /// * Self
    pub const fn new(eta_i: RefractiveType, eta_t: RefractiveType) -> Self {
        Self { eta_i, eta_t }
    }
}

impl Fresnel for FresnelDielectric {
    fn evaluate(&self, cos_i: Float) -> Spectrum {
        let fresnel = fresnel_dielectric(cos_i, self.eta_i.n_uniform(), self.eta_t.n_uniform());

        Spectrum::splat(fresnel)
    }

    fn evaluate_packet(
        &self,
        cos_i: Float,
        lambdas: &[Float; PACKET_SIZE],
    ) -> [Float; PACKET_SIZE] {
        lambdas.map(|l| fresnel_dielectric(cos_i, self.eta_i.n(l), self.eta_t.n(l)))
    }

    #[inline]
    fn evaluate_lambda(&self, cos_i: Float, lambda: Float) -> Float {
        fresnel_dielectric(cos_i, self.eta_i.n(lambda), self.eta_t.n(lambda))
    }
}
