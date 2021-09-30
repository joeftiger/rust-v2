use crate::bxdf::{BxDF, BxDFFlag};
use crate::{Float, Spectrum, Vec3};
use serde::{Deserialize, Serialize};
#[cfg(not(feature = "f64"))]
use std::f32::consts::FRAC_1_PI;
#[cfg(feature = "f64")]
use std::f64::consts::FRAC_1_PI;

/// The lambertian reflection reflects equally into all directions of the hemisphere.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LambertianReflection {
    r: Spectrum,
}

impl LambertianReflection {
    /// Creates a new lambertian reflection.
    ///
    /// # Arguments
    /// * `r` - The reflective filter spectrum
    pub const fn new(r: Spectrum) -> Self {
        Self { r }
    }
}

#[typetag::serde]
impl BxDF for LambertianReflection {
    #[inline(always)]
    fn flag(&self) -> BxDFFlag {
        BxDFFlag::REFLECTION | BxDFFlag::DIFFUSE
    }

    fn evaluate(&self, _: Vec3, _: Vec3) -> Spectrum {
        self.r * FRAC_1_PI
    }

    #[inline]
    fn evaluate_lambda(&self, _: Vec3, _: Vec3, index: usize) -> Float {
        self.r[index] * FRAC_1_PI
    }
}

/// The lambertian transmission transmits equally into all directions of the hemisphere.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LambertianTransmission {
    t: Spectrum,
}

impl LambertianTransmission {
    /// Creates a new lambertian transmission.
    ///
    /// # Arguments
    /// * `t` - The transmissive filter spectrum
    pub const fn new(t: Spectrum) -> Self {
        Self { t }
    }
}

#[typetag::serde]
impl BxDF for LambertianTransmission {
    #[inline(always)]
    fn flag(&self) -> BxDFFlag {
        BxDFFlag::DIFFUSE | BxDFFlag::TRANSMISSION
    }

    fn evaluate(&self, _: Vec3, _: Vec3) -> Spectrum {
        self.t * FRAC_1_PI
    }

    #[inline]
    fn evaluate_lambda(&self, _: Vec3, _: Vec3, index: usize) -> Float {
        self.t[index] * FRAC_1_PI
    }
}
