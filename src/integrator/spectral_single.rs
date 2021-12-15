use crate::bxdf::BxDFFlag;
use crate::camera::sensor::Pixel;
use crate::geometry::{offset_ray_towards, Ray};
use crate::integrator::{DirectIllumination, Integrator};
use crate::sampler::{FloatSampler, SpectralSampler};
use crate::scene::{Scene, SceneObject};
use cgmath::InnerSpace;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct SpectralSingle {
    max_depth: u32,
    sampler: FloatSampler,
    spectral_sampler: SpectralSampler,
    direct_illum: DirectIllumination,
}

impl SpectralSingle {
    pub fn new(max_depth: u32, sampler: FloatSampler, spectral_sampler: SpectralSampler, direct_illum: DirectIllumination) -> Self {
        Self { max_depth, sampler, spectral_sampler, direct_illum }
    }
}

#[typetag::serde]
impl Integrator for SpectralSingle {
    fn integrate(&self, scene: &Scene, primary_ray: Ray, pixel: &mut Pixel) {
        if let Some(mut hit) = scene.intersect(primary_ray) {
            for index in self.spectral_sampler.create() {
                let mut illumination = 0.0;
                let mut throughput = 1.0;

                for _ in 0..self.max_depth {
                    let outgoing = -hit.i.incoming;
                    let point = hit.i.point;
                    let normal = hit.i.normal;
                    let bsdf = hit.object.bsdf();

                    if let SceneObject::Emitter(e) = hit.object {
                        illumination += throughput * e.radiance_lambda(outgoing, normal, index);
                    }

                    illumination += throughput
                        * self
                            .direct_illum
                            .sample_lambda(scene, &hit, &self.sampler, index);

                    if let Some(bxdf_sample) = bsdf.sample_lambda(
                        normal,
                        outgoing,
                        self.sampler.sample(),
                        BxDFFlag::empty(),
                        index,
                    ) {
                        if bxdf_sample.pdf == 0.0 || bxdf_sample.spectrum == 0.0 {
                            break;
                        }

                        let cos_abs = if bxdf_sample.flag.specular() {
                            // division of cosine omitted in specular bxdfs
                            1.0
                        } else {
                            bxdf_sample.incident.dot(normal).abs()
                        };

                        throughput *= bxdf_sample.spectrum * (cos_abs / bxdf_sample.pdf);

                        let ray = offset_ray_towards(point, normal, bxdf_sample.incident);
                        match scene.intersect(ray) {
                            Some(i) => hit = i,
                            None => break,
                        }
                    }
                }

                pixel.add_lambda(illumination, index);
            }
        } else {
            pixel.add_none();
        }
    }
}
