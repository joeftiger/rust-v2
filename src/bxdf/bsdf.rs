use crate::bxdf::{same_hemisphere, world_to_bxdf, BxDF, BxDFFlag, BxDFSample, BxDFSamplePacket};
use crate::sampler::Sample;
use crate::{Float, Spectrum, Vec3, PACKET_SIZE};
use cgmath::Rotation;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct BSDF {
    #[serde(default)]
    bxdfs: Vec<Box<dyn BxDF>>,
}

impl BSDF {
    pub fn new(bxdfs: Vec<Box<dyn BxDF>>) -> Self {
        Self { bxdfs }
    }

    pub fn empty() -> Self {
        Self::new(Vec::new())
    }

    pub fn size(&self) -> usize {
        self.bxdfs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bxdfs.is_empty()
    }

    fn num_types(&self, f: BxDFFlag) -> usize {
        self.bxdfs.iter().filter(|bxdf| bxdf.match_flag(f)).count()
    }

    fn random_matching_bxdf(&self, flags: BxDFFlag, sample: Float) -> Option<&dyn BxDF> {
        let count = self.num_types(flags);
        if count == 0 {
            return None;
        }

        let index = (sample * count as Float) as usize;
        self.bxdfs
            .iter()
            .filter_map(|bxdf| {
                if bxdf.match_flag(flags) {
                    Some(bxdf.as_ref())
                } else {
                    None
                }
            })
            .nth(index)
    }

    /// Evaluates a random BxDF.
    ///
    /// # Arguments
    /// * `normal` - The surface normal. Used to rotate into the local BxDF space.
    /// * `incident_world` - The incoming incident vector in world space.
    /// * `outgoing_world` - The outgoing incident vector in world space.
    /// * `sample`: The random sample
    /// * `flags` - The flags to match a BxDF randomly.
    pub fn evaluate(
        &self,
        normal: Vec3,
        incident_world: Vec3,
        outgoing_world: Vec3,
        sample: Float,
        mut flags: BxDFFlag,
    ) -> Spectrum {
        let rotation = world_to_bxdf(normal);

        let incident = rotation.rotate_vector(incident_world);
        let outgoing = rotation.rotate_vector(outgoing_world);

        // transmission or reflection
        if same_hemisphere(incident, outgoing) {
            flags.remove(BxDFFlag::TRANSMISSION);
        } else {
            flags.remove(BxDFFlag::REFLECTION);
        }

        self.random_matching_bxdf(flags, sample)
            .map_or(Spectrum::splat(0.0), |bxdf| {
                bxdf.evaluate(incident, outgoing)
            })
    }

    /// Evaluates a random BxDF.
    ///
    /// # Arguments
    /// * `normal` - The surface normal. Used to rotate into the local BxDF space.
    /// * `incident_world` - The incoming incident vector in world space.
    /// * `outgoing_world` - The outgoing incident vector in world space.
    /// * `sample`: The random sample
    /// * `flags` - The flags to match a BxDF randomly.
    /// * `indices` - The spectral indices
    pub fn evaluate_packet(
        &self,
        normal: Vec3,
        incident_world: Vec3,
        outgoing_world: Vec3,
        sample: Float,
        mut flags: BxDFFlag,
        indices: &[usize; PACKET_SIZE],
    ) -> [Float; PACKET_SIZE] {
        let rotation = world_to_bxdf(normal);
        let incident = rotation.rotate_vector(incident_world);
        let outgoing = rotation.rotate_vector(outgoing_world);

        // transmission or reflection
        if same_hemisphere(incident, outgoing) {
            flags.remove(BxDFFlag::TRANSMISSION);
        } else {
            flags.remove(BxDFFlag::REFLECTION);
        }

        if let Some(bxdf) = self.random_matching_bxdf(flags, sample) {
            bxdf.evaluate_packet(incident, outgoing, indices)
        } else {
            [0.0; PACKET_SIZE]
        }
    }

    /// Evaluates a random BxDF.
    ///
    /// # Arguments
    /// * `normal` - The surface normal. Used to rotate into the local BxDF space.
    /// * `incident_world` - The incoming incident vector in world space.
    /// * `outgoing_world` - The outgoing incident vector in world space.
    /// * `sample`: The random sample
    /// * `flags` - The flags to match a BxDF randomly.
    /// * `index` - The spectral index
    pub fn evaluate_lambda(
        &self,
        normal: Vec3,
        incident_world: Vec3,
        outgoing_world: Vec3,
        sample: Float,
        mut flags: BxDFFlag,
        index: usize,
    ) -> Float {
        let rotation = world_to_bxdf(normal);
        let incident = rotation.rotate_vector(incident_world);
        let outgoing = rotation.rotate_vector(outgoing_world);

        // transmission or reflection
        if same_hemisphere(incident, outgoing) {
            flags.remove(BxDFFlag::TRANSMISSION);
        } else {
            flags.remove(BxDFFlag::REFLECTION);
        }

        self.random_matching_bxdf(flags, sample)
            .map_or(0.0, |bxdf| bxdf.evaluate_lambda(incident, outgoing, index))
    }

    /// Samples a random BxDF.
    ///
    /// # Arguments
    /// * `normal` - The surface normal. Used to rotate into the local BxDF space.
    /// * `outgoing_world` - The outgoing incident vector in world space.
    /// * `sample`: The random sample
    /// * `flags` - The flags to match a BxDF randomly.
    /// * `index` - The spectral index
    pub fn sample(
        &self,
        normal: Vec3,
        outgoing_world: Vec3,
        sample: Sample,
        flags: BxDFFlag,
    ) -> Option<BxDFSample<Spectrum>> {
        let rotation = world_to_bxdf(normal);
        let outgoing = rotation.rotate_vector(outgoing_world);

        let bxdf = self.random_matching_bxdf(flags, sample.float)?;

        bxdf.sample(outgoing, sample.vec2).map(|mut s| {
            s.incident = rotation.invert().rotate_vector(s.incident);
            s
        })
    }

    /// Samples a random BxDF.
    ///
    /// # Arguments
    /// * `normal` - The surface normal. Used to rotate into the local BxDF space.
    /// * `outgoing_world` - The outgoing incident vector in world space.
    /// * `sample`: The random sample
    /// * `flags` - The flags to match a BxDF randomly.
    /// * `indices` - The spectral indices
    pub fn sample_packet(
        &self,
        normal: Vec3,
        outgoing_world: Vec3,
        sample: Sample,
        flags: BxDFFlag,
        indices: &[usize; PACKET_SIZE],
    ) -> BxDFSamplePacket {
        let rotation = world_to_bxdf(normal);
        let outgoing = rotation.rotate_vector(outgoing_world);

        let bxdf = if let Some(b) = self.random_matching_bxdf(flags, sample.float) {
            b
        } else {
            return BxDFSamplePacket::Bundle(None);
        };

        let mut packet = bxdf.sample_packet(outgoing, sample.vec2, indices);
        match &mut packet {
            BxDFSamplePacket::Bundle(Some(mut s)) => {
                s.incident = rotation.invert().rotate_vector(s.incident)
            }
            BxDFSamplePacket::Split(mut samples) => {
                for s in samples.iter_mut().flatten() {
                    s.incident = rotation.invert().rotate_vector(s.incident);
                }
            }
            _ => {}
        }

        packet
    }

    /// Samples a random BxDF.
    ///
    /// # Arguments
    /// * `normal` - The surface normal. Used to rotate into the local BxDF space.
    /// * `outgoing_world` - The outgoing incident vector in world space.
    /// * `sample`: The random sample
    /// * `flags` - The flags to match a BxDF randomly.
    /// * `index` - The spectral index
    pub fn sample_lambda(
        &self,
        normal: Vec3,
        outgoing_world: Vec3,
        sample: Sample,
        flags: BxDFFlag,
        index: usize,
    ) -> Option<BxDFSample<Float>> {
        let rotation = world_to_bxdf(normal);
        let outgoing = rotation.rotate_vector(outgoing_world);

        let bxdf = self.random_matching_bxdf(flags, sample.float)?;

        bxdf.sample_lambda(outgoing, sample.vec2, index)
            .map(|mut s| {
                s.incident = rotation.invert().rotate_vector(s.incident);
                s
            })
    }
}
