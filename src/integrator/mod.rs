//!# Summary
//! Integrators are a major component in ray tracing as they compute the color of each pixel.
//! They come in various forms, taking different paths.
//!
//! # Notation
//! * `E` - The eye
//! * `L` - The light
//! * `D` - Diffuse reflection or transmission
//! * `G` - Glossy reflection or transmission
//! * `S` - Specular reflection or refraction
//!
//! # Examles
//! The following set of traced paths are specified using regular expression.
//! * **Appel** ray casting: `E(D|G)L`
//! * **Whitted** recursive ray tracing: `E[S*](D|G)L`
//! * **Kajiya** path tracing: `E[(D|G|S)+(D|G)]L`
//! * **Goral** radiosity: `ED*L`

pub mod dummy;
pub mod hero;
pub mod path;
pub mod spectral;
pub mod spectral_single;
pub mod whitted;

pub use hero::*;
pub use path::*;
pub use spectral::*;
pub use spectral_single::*;
pub use whitted::*;

use crate::camera::sensor::Pixel;
use crate::geometry::Ray;
use crate::sampler::FloatSampler;
use crate::scene::{Scene, SceneIntersection};
use crate::{Float, Spectrum, PACKET_SIZE};
use cgmath::InnerSpace;

use crate::bxdf::BxDFFlag;
use crate::util::PacketOps;
use serde::{Deserialize, Serialize};

#[typetag::serde]
pub trait Integrator: Send + Sync {
    /// Calculates the rendering equation.
    ///
    /// # Arguments
    /// * `scene` - The scene to integrate
    /// * `primary_ray` - The primary ray shot into the scene
    /// * `pixel` - The pixel to integrate into
    ///
    /// # Returns
    /// * The color spectrum of the given ray
    fn integrate(&self, scene: &Scene, primary_ray: Ray, pixel: &mut Pixel);
}

/// The direct illumination strategy.
#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum DirectIllumination {
    /// Get direct illumination for all emitters.
    All,
    /// Get direct illumination for a random emitter.
    Random,
}

impl DirectIllumination {
    pub fn emitter_indices(self, scene: &Scene, sample: Float) -> &[u32] {
        match self {
            DirectIllumination::All => scene.emitters(),
            DirectIllumination::Random => {
                let chosen = (sample * scene.num_emitters() as Float) as usize;
                &scene.emitters()[chosen..chosen]
            }
        }
    }

    pub fn sample(
        self,
        scene: &Scene,
        hit: &SceneIntersection,
        sampler: &FloatSampler,
    ) -> Spectrum {
        let mut illum = Spectrum::splat(0.0);
        let bsdf = hit.object.bsdf();
        if bsdf.is_empty() {
            return illum;
        }

        let outgoing_world = -hit.i.incoming;

        let emitter_indices = self.emitter_indices(scene, sampler.float());
        for emitter in emitter_indices
            .iter()
            .map(|&i| scene.get_emitter(i as usize))
            .flatten()
        {
            let emitter_sample = emitter.sample(hit.i.point, sampler.vec2());

            if !emitter_sample.radiance.is_black() && emitter_sample.occlusion.unoccluded(scene) {
                let spectrum = bsdf.evaluate(
                    hit.i.normal,
                    emitter_sample.incident,
                    outgoing_world,
                    sampler.float(),
                    BxDFFlag::empty(),
                );

                if !spectrum.is_black() {
                    let cos = emitter_sample.incident.dot(hit.i.normal);

                    if cos != 0.0 {
                        illum += spectrum * emitter_sample.radiance * cos.abs()
                    }
                }
            }
        }

        illum
    }

    pub fn sample_packet(
        self,
        scene: &Scene,
        hit: &SceneIntersection,
        sampler: &FloatSampler,
        indices: &[usize; PACKET_SIZE],
    ) -> [Float; PACKET_SIZE] {
        let mut illum = [0.0; PACKET_SIZE];
        let bsdf = hit.object.bsdf();
        if bsdf.is_empty() {
            return illum;
        }

        let outgoing_world = -hit.i.incoming;

        let emitter_indices = self.emitter_indices(scene, sampler.float());
        for emitter in emitter_indices
            .iter()
            .map(|&i| scene.get_emitter(i as usize))
            .flatten()
        {
            let emitter_sample = emitter.sample_packet(hit.i.point, sampler.vec2(), indices);

            if !emitter_sample.radiance.is_black() && emitter_sample.occlusion.unoccluded(scene) {
                let spectrum = bsdf.evaluate_packet(
                    hit.i.normal,
                    emitter_sample.incident,
                    outgoing_world,
                    sampler.float(),
                    BxDFFlag::empty(),
                    indices,
                );

                if !spectrum.is_black() {
                    let cos = emitter_sample.incident.dot(hit.i.normal);

                    if cos != 0.0 {
                        let rhs = spectrum.mul(emitter_sample.radiance).mul_t(cos.abs());
                        illum.add_assign(rhs)
                    }
                }
            }
        }

        illum
    }

    pub fn sample_lambda(
        self,
        scene: &Scene,
        hit: &SceneIntersection,
        sampler: &FloatSampler,
        index: usize,
    ) -> Float {
        let mut illum = 0.0;
        let bsdf = hit.object.bsdf();
        if bsdf.is_empty() {
            return illum;
        }

        let outgoing_world = -hit.i.incoming;

        let emitter_indices = self.emitter_indices(scene, sampler.float());
        for emitter in emitter_indices
            .iter()
            .map(|&i| scene.get_emitter(i as usize))
            .flatten()
        {
            let emitter_sample = emitter.sample_lambda(hit.i.point, sampler.vec2(), index);

            if emitter_sample.radiance != 0.0 && emitter_sample.occlusion.unoccluded(scene) {
                let spectrum = bsdf.evaluate_lambda(
                    hit.i.normal,
                    emitter_sample.incident,
                    outgoing_world,
                    sampler.float(),
                    BxDFFlag::empty(),
                    index,
                );

                // TODO: Ignore checks for single floats because performance?
                if spectrum != 0.0 {
                    let cos = emitter_sample.incident.dot(hit.i.normal);

                    if cos != 0.0 {
                        illum += spectrum * emitter_sample.radiance * cos.abs()
                    }
                }
            }
        }

        illum
    }
}
