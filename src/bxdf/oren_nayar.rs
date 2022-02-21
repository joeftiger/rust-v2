use crate::bxdf::{cos_phi, cos_theta, sin_phi, sin_theta, BxDF, BxDFFlag};
use crate::util::floats::EPSILON;
use crate::*;
use serde::{Deserialize, Serialize};
#[cfg(not(feature = "f64"))]
use std::f32::consts::FRAC_1_PI;
#[cfg(feature = "f64")]
use std::f64::consts::FRAC_1_PI;

/// The Oren-Nayar reflectance model describes rough opaque diffuse surfaces where each facet is lambertian (diffuse).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrenNayar {
    r: Spectrum,
    a: Float,
    b: Float,
}

impl OrenNayar {
    /// Creates a new Oren-Nayar reflection.
    ///
    /// # Constraints
    /// * `sigma` - Should be in range `[0, inf)`.
    ///
    /// # Arguments
    /// * `r` - The reflection
    /// * `sigma` - The roughness (gradient of the surface elevation) in degrees
    pub fn new(r: Spectrum, sigma: Float) -> Self {
        let sigma = sigma.to_radians();
        let sigma2 = sigma * sigma;
        let a = 1.0 - (sigma2 / (2.0 * (sigma2 + 0.33)));
        let b = 0.45 * sigma2 / (sigma2 + 0.09);

        Self { r, a, b }
    }

    /// Calculates the Oren Nayar scaling parameter.
    ///
    /// # Constraints
    /// * `incident` - All values should be finite (neither infinite nor `NaN`).
    ///                Should be normalized.
    /// * `outgoing` - All values should be finite.
    ///                Should be normalized.
    ///
    /// # Arguments
    /// * `incident` - The incident direction onto the intersection we evaluate
    /// * `outgoing` - The outgoing light direction
    ///
    /// # Returns
    /// `(A + B * max_cos * sin_alpha * tan_beta / PI)`
    fn calc_param(&self, incident: Vec3, outgoing: Vec3) -> Float {
        let sin_theta_i = sin_theta(incident);
        let sin_theta_o = sin_theta(outgoing);

        let max_cos = if sin_theta_i > EPSILON && sin_theta_o > EPSILON {
            let sin_phi_i = sin_phi(incident);
            let sin_phi_o = sin_phi(outgoing);
            let cos_phi_i = cos_phi(incident);
            let cos_phi_o = cos_phi(outgoing);

            let d_cos = cos_phi_i * cos_phi_o + sin_phi_i * sin_phi_o;
            d_cos.max(0.0)
        } else {
            return FRAC_1_PI * self.a;
        };

        let cos_theta_i_abs = cos_theta(incident).abs();
        let cos_theta_o_abs = cos_theta(outgoing).abs();

        let sin_alpha;
        let tan_beta;
        if cos_theta_i_abs > cos_theta_o_abs {
            sin_alpha = sin_theta_o;
            tan_beta = sin_theta_i / cos_theta_i_abs;
        } else {
            sin_alpha = sin_theta_i;
            tan_beta = sin_theta_o / cos_theta_o_abs;
        }

        FRAC_1_PI * (self.a + self.b * max_cos * sin_alpha * tan_beta)
    }
}

#[typetag::serde]
impl BxDF for OrenNayar {
    #[inline(always)]
    fn flag(&self) -> BxDFFlag {
        BxDFFlag::DIFFUSE | BxDFFlag::REFLECTION
    }

    fn evaluate(&self, incident: Vec3, outgoing: Vec3) -> Spectrum {
        let oren_nayar = self.calc_param(incident, outgoing);
        debug_assert!(oren_nayar >= 0.0, "Oren Nayar parameter is >= 0");

        self.r * oren_nayar
    }

    fn evaluate_packet(
        &self,
        incident: Vec3,
        outgoing: Vec3,
        indices: &[usize; PACKET_SIZE],
    ) -> [Float; PACKET_SIZE] {
        let oren_nayar = self.calc_param(incident, outgoing);

        indices.map(|i| self.r[i] * oren_nayar)
    }

    fn evaluate_lambda(&self, incident: Vec3, outgoing: Vec3, index: usize) -> Float {
        let oren_nayar = self.calc_param(incident, outgoing);

        self.r[index] * oren_nayar
    }
}
