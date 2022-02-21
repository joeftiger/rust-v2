#![allow(dead_code)]

use crate::bxdf::refraction::RefractiveType;
use crate::bxdf::{
    bxdf_incident_to, bxdf_normal, cos_theta, fresnel_dielectric, refract, BxDF, BxDFFlag,
    BxDFSample, BxDFSamplePacket, Fresnel, FresnelDielectric, FresnelType,
};
use crate::util::PacketOps;
use crate::{Float, Spectrum, Vec2, Vec3, PACKET_SIZE};
use serde::{Deserialize, Serialize};

#[inline]
fn etas(
    eta_i: RefractiveType,
    eta_t: RefractiveType,
    outgoing: Vec3,
) -> (RefractiveType, RefractiveType, Vec3) {
    let entering = cos_theta(outgoing) > 0.0;
    if entering {
        (eta_i, eta_t, bxdf_normal())
    } else {
        (eta_t, eta_i, -bxdf_normal())
    }
}

/// Describes a specular reflection
#[derive(Serialize, Deserialize)]
pub struct SpecularReflection {
    r: Spectrum,
    fresnel: FresnelType,
}

impl SpecularReflection {
    /// Creates a new specular reflection.
    ///
    /// # Arguments
    /// * `r` - The reflection
    /// * `fresnel` - The fresnel
    pub fn new(r: Spectrum, fresnel: FresnelType) -> Self {
        Self { r, fresnel }
    }
}

#[typetag::serde]
impl BxDF for SpecularReflection {
    #[inline(always)]
    fn flag(&self) -> BxDFFlag {
        BxDFFlag::REFLECTION | BxDFFlag::SPECULAR
    }

    #[inline]
    fn evaluate(&self, _: Vec3, _: Vec3) -> Spectrum {
        Spectrum::splat(0.0)
    }

    #[inline]
    fn evaluate_packet(&self, _: Vec3, _: Vec3, _: &[usize; PACKET_SIZE]) -> [Float; PACKET_SIZE] {
        [0.0; PACKET_SIZE]
    }

    #[inline(always)]
    fn evaluate_lambda(&self, _: Vec3, _: Vec3, _: usize) -> Float {
        0.0
    }

    fn sample(&self, outgoing: Vec3, _: Vec2) -> Option<BxDFSample<Spectrum>> {
        let incident = bxdf_incident_to(outgoing);

        let cos_i = cos_theta(incident);
        let spectrum = self.fresnel.evaluate(cos_i) * self.r;

        Some(BxDFSample::new(spectrum, incident, 1.0, self.flag()))
    }

    fn sample_packet(
        &self,
        outgoing: Vec3,
        _: Vec2,
        indices: &[usize; PACKET_SIZE],
    ) -> BxDFSamplePacket {
        let incident = bxdf_incident_to(outgoing);

        let cos_i = cos_theta(incident);

        let lambdas = indices.map(Spectrum::lambda);
        let fresnel = self.fresnel.evaluate_packet(cos_i, &lambdas);
        let packet = indices.map(|i| self.r[i]).mul(fresnel);

        let bundle = Some(BxDFSample::new(packet, incident, 1.0, self.flag()));

        BxDFSamplePacket::Bundle(bundle)
    }

    fn sample_lambda(
        &self,
        outgoing: Vec3,
        _: Vec2,
        light_wave_index: usize,
    ) -> Option<BxDFSample<Float>> {
        let incident = bxdf_incident_to(outgoing);

        let cos_i = cos_theta(incident);
        let light_wave = self.r.as_light_wave(light_wave_index);
        let spectrum =
            self.fresnel.evaluate_lambda(light_wave.lambda, cos_i) * light_wave.intensity;

        Some(BxDFSample::new(spectrum, incident, 1.0, self.flag()))
    }

    /// No scattering for specular reflection leads to no pdf.
    ///
    /// # Arguments
    /// Ignored
    ///
    /// # Returns
    /// * `0.0`
    fn pdf(&self, _: Vec3, _: Vec3) -> Float {
        0.0
    }
}

/// Describes a specular transmission.
#[derive(Serialize, Deserialize)]
pub struct SpecularTransmission {
    t: Spectrum,
    fresnel: FresnelDielectric,
}

impl SpecularTransmission {
    /// Creates a new specular transmission.
    ///
    /// # Arguments
    /// * `t` - The transmission
    /// * `eta_a` - The index of refraction above the surface
    /// * `eta_b` - The index of refraction below the surface
    /// * `mode` - The transport mode parameter
    ///
    /// # Returns
    /// * Self
    pub fn new(t: Spectrum, eta_i: RefractiveType, eta_t: RefractiveType) -> Self {
        let fresnel = FresnelDielectric::new(eta_i, eta_t);
        Self { t, fresnel }
    }
}

#[typetag::serde]
impl BxDF for SpecularTransmission {
    #[inline(always)]
    fn flag(&self) -> BxDFFlag {
        BxDFFlag::SPECULAR | BxDFFlag::TRANSMISSION
    }

    /// No scattering for specular transmission.
    ///
    /// # Arguments
    /// Ignored
    ///
    /// # Returns
    /// * `0.0` spectrum
    #[inline]
    fn evaluate(&self, _: Vec3, _: Vec3) -> Spectrum {
        Spectrum::splat(0.0)
    }

    #[inline]
    fn evaluate_packet(&self, _: Vec3, _: Vec3, _: &[usize; PACKET_SIZE]) -> [Float; PACKET_SIZE] {
        [0.0; PACKET_SIZE]
    }

    #[inline(always)]
    fn evaluate_lambda(&self, _: Vec3, _: Vec3, _: usize) -> Float {
        0.0
    }

    fn sample(&self, outgoing: Vec3, _: Vec2) -> Option<BxDFSample<Spectrum>> {
        let (eta_i, eta_t, normal) = etas(self.fresnel.eta_i, self.fresnel.eta_t, outgoing);
        let incident = refract(outgoing, normal, eta_i.n_uniform() / eta_t.n_uniform())?;

        let cos_i = cos_theta(incident);
        let spectrum = self.t * (Spectrum::splat(1.0) - self.fresnel.evaluate(cos_i));

        Some(BxDFSample::new(spectrum, incident, 1.0, self.flag()))
    }

    fn sample_packet(
        &self,
        outgoing: Vec3,
        _: Vec2,
        indices: &[usize; PACKET_SIZE],
    ) -> BxDFSamplePacket {
        let (eta_i, eta_t, normal) = etas(self.fresnel.eta_i, self.fresnel.eta_t, outgoing);

        let mut split = [None; PACKET_SIZE];

        for i in 0..PACKET_SIZE {
            let index = indices[i];
            let lambda = Spectrum::lambda(index);
            let eta = eta_i.n(lambda) / eta_t.n(lambda);

            let incident = match refract(outgoing, normal, eta) {
                Some(v) => v,
                None => continue,
            };

            let cos_i = cos_theta(incident);
            let spectrum = self.t[index] * (1.0 - self.fresnel.evaluate_lambda(cos_i, lambda));

            split[i] = Some(BxDFSample::new(spectrum, incident, 1.0, self.flag()));
        }

        BxDFSamplePacket::Split(split)
    }

    fn sample_lambda(&self, outgoing: Vec3, _: Vec2, index: usize) -> Option<BxDFSample<Float>> {
        let (eta_i, eta_t, normal) = etas(self.fresnel.eta_i, self.fresnel.eta_t, outgoing);

        let incident = refract(outgoing, normal, eta_i.n_uniform() / eta_t.n_uniform())?;

        let cos_i = cos_theta(incident);
        let lambda = Spectrum::lambda(index);
        let spectrum = self.t[index] * (1.0 - self.fresnel.evaluate_lambda(cos_i, lambda));

        Some(BxDFSample::new(spectrum, incident, 1.0, self.flag()))
    }

    /// No scattering for specular transmission leads to no pdf.
    ///
    /// # Arguments
    /// Ignored
    ///
    /// # Returns
    /// * `0.0`
    #[inline(always)]
    fn pdf(&self, _: Vec3, _: Vec3) -> Float {
        0.0
    }
}

/// Combines specular reflection and transmission for better efficiency.
#[derive(Serialize, Deserialize)]
pub struct FresnelSpecular {
    r: Spectrum,
    t: Spectrum,
    fresnel: FresnelDielectric,
}

impl FresnelSpecular {
    /// Creates a new fresnel specular.
    ///
    /// # Arguments
    /// * `r` - The reflection
    /// * `t` - The transmission
    /// * `eta_a` - The index of refraction above the surface
    /// * `eta_b` - The index of refraction below the surface
    /// * `mode` - The transport mode parameter
    pub fn new(r: Spectrum, t: Spectrum, eta_i: RefractiveType, eta_t: RefractiveType) -> Self {
        let fresnel = FresnelDielectric::new(eta_i, eta_t);
        Self { r, t, fresnel }
    }

    fn fresnel_incident(
        outgoing: Vec3,
        sample: Vec2,
        eta_i_orig: Float,
        eta_t_orig: Float,
    ) -> Option<Vec3> {
        let cos_outgoing = cos_theta(outgoing);
        let f = fresnel_dielectric(cos_outgoing, eta_i_orig, eta_t_orig);

        if f < sample.x {
            // if entering
            let (eta_i, eta_t, normal) = if cos_outgoing > 0.0 {
                (eta_i_orig, eta_t_orig, bxdf_normal())
            } else {
                (eta_t_orig, eta_i_orig, -bxdf_normal())
            };

            refract(outgoing, normal, eta_i / eta_t)
        } else {
            Some(bxdf_incident_to(outgoing))
        }
    }
}

#[typetag::serde]
impl BxDF for FresnelSpecular {
    #[inline(always)]
    fn flag(&self) -> BxDFFlag {
        BxDFFlag::REFLECTION | BxDFFlag::SPECULAR | BxDFFlag::TRANSMISSION
    }

    /// No scattering for specular reflection/transmission.
    ///
    /// # Arguments
    /// Ignored
    ///
    /// # Returns
    /// * `0.0` spectrum
    fn evaluate(&self, _: Vec3, _: Vec3) -> Spectrum {
        Spectrum::splat(0.0)
    }

    #[inline]
    fn evaluate_packet(&self, _: Vec3, _: Vec3, _: &[usize; PACKET_SIZE]) -> [Float; PACKET_SIZE] {
        [0.0; PACKET_SIZE]
    }

    #[inline(always)]
    fn evaluate_lambda(&self, _: Vec3, _: Vec3, _: usize) -> Float {
        0.0
    }

    fn sample(&self, outgoing: Vec3, sample: Vec2) -> Option<BxDFSample<Spectrum>> {
        let cos_outgoing = cos_theta(outgoing);

        let eta_i_orig = self.fresnel.eta_i.n_uniform();
        let eta_t_orig = self.fresnel.eta_t.n_uniform();
        let f = fresnel_dielectric(cos_outgoing, eta_i_orig, eta_t_orig);

        if sample.x < f {
            // specular reflection

            let incident = bxdf_incident_to(outgoing);
            let flag = BxDFFlag::SPECULAR | BxDFFlag::REFLECTION;
            let spectrum = self.r * f;
            let pdf = f;

            Some(BxDFSample::new(spectrum, incident, pdf, flag))
        } else {
            // specular transmission

            let entering = cos_outgoing > 0.0;
            let (eta_i, eta_t, normal) = if entering {
                (eta_i_orig, eta_t_orig, bxdf_normal())
            } else {
                (eta_t_orig, eta_i_orig, -bxdf_normal())
            };

            let incident = refract(outgoing, normal, eta_i / eta_t)?;
            let pdf = 1.0 - f;
            let spectrum = self.t * pdf;
            let flag = BxDFFlag::SPECULAR | BxDFFlag::TRANSMISSION;

            Some(BxDFSample::new(spectrum, incident, pdf, flag))
        }
    }

    fn sample_packet(
        &self,
        outgoing: Vec3,
        sample: Vec2,
        indices: &[usize; PACKET_SIZE],
    ) -> BxDFSamplePacket {
        let cos_outgoing = cos_theta(outgoing);

        let split = indices.map(|i| {
            let lambda = Spectrum::lambda(i);
            let eta_i_orig = self.fresnel.eta_i.n(lambda);
            let eta_t_orig = self.fresnel.eta_t.n(lambda);
            let f = fresnel_dielectric(cos_outgoing, eta_i_orig, eta_t_orig);

            if sample.x < f {
                // specular reflection

                let spectrum = self.r[i] * f;
                let incident = bxdf_incident_to(outgoing);
                let pdf = f;
                let flag = BxDFFlag::SPECULAR | BxDFFlag::REFLECTION;

                Some(BxDFSample::new(spectrum, incident, pdf, flag))
            } else {
                // specular transmission

                let entering = cos_outgoing > 0.0;
                let (eta_i, eta_t, normal) = if entering {
                    (eta_i_orig, eta_t_orig, bxdf_normal())
                } else {
                    (eta_t_orig, eta_i_orig, -bxdf_normal())
                };

                let incident = refract(outgoing, normal, eta_i / eta_t)?;
                let pdf = 1.0 - f;
                let spectrum = self.t[i] * pdf;
                let flag = BxDFFlag::SPECULAR | BxDFFlag::TRANSMISSION;

                Some(BxDFSample::new(spectrum, incident, pdf, flag))
            }
        });

        BxDFSamplePacket::Split(split)
    }

    fn sample_lambda(
        &self,
        outgoing: Vec3,
        sample: Vec2,
        index: usize,
    ) -> Option<BxDFSample<Float>> {
        let cos_outgoing = cos_theta(outgoing);

        let lambda = Spectrum::lambda(index);
        let eta_i_orig = self.fresnel.eta_i.n(lambda);
        let eta_t_orig = self.fresnel.eta_t.n(lambda);
        let f = fresnel_dielectric(cos_outgoing, eta_i_orig, eta_t_orig);

        if sample.x < f {
            // specular reflection

            let spectrum = self.r[index] * f;
            let incident = bxdf_incident_to(outgoing);
            let pdf = f;
            let flag = BxDFFlag::SPECULAR | BxDFFlag::REFLECTION;

            Some(BxDFSample::new(spectrum, incident, pdf, flag))
        } else {
            // specular transmission

            let entering = cos_outgoing > 0.0;
            let (eta_i, eta_t, normal) = if entering {
                (eta_i_orig, eta_t_orig, bxdf_normal())
            } else {
                (eta_t_orig, eta_i_orig, -bxdf_normal())
            };

            let incident = refract(outgoing, normal, eta_i / eta_t)?;
            let pdf = 1.0 - f;
            let spectrum = self.t[index] * pdf;
            let flag = BxDFFlag::SPECULAR | BxDFFlag::TRANSMISSION;

            Some(BxDFSample::new(spectrum, incident, pdf, flag))
        }
    }

    /// No scattering for specular reflection/transmission leads to no pdf.
    ///
    /// # Arguments
    /// Ignored
    ///
    /// # Returns
    /// * `0.0`
    #[inline(always)]
    fn pdf(&self, _: Vec3, _: Vec3) -> Float {
        0.0
    }
}
