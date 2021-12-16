use crate::bxdf::BxDFFlag;
use crate::camera::sensor::Pixel;
use crate::geometry::{offset_ray_towards, Ray};
use crate::integrator::{DirectIllumination, Integrator};
use crate::sampler::FloatSampler;
use crate::scene::{Scene, SceneObject};
use crate::Spectrum;
use cgmath::InnerSpace;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct Path {
    max_depth: u32,
    sampler: FloatSampler,
    direct_illum: DirectIllumination,
}

impl Path {
    pub fn new(max_depth: u32, sampler: FloatSampler, direct_illum: DirectIllumination) -> Self {
        Self { max_depth, sampler, direct_illum }
    }
}

#[typetag::serde]
impl Integrator for Path {
    fn integrate(&self, scene: &Scene, primary_ray: Ray, pixel: &mut Pixel) {
        if let Some(mut hit) = scene.intersect(primary_ray) {
            let mut illumination = Spectrum::splat(0.0);
            let mut throughput = Spectrum::splat(1.0);

            for _ in 0..self.max_depth {
                let outgoing = -hit.i.incoming;
                let point = hit.i.point;
                let normal = hit.i.normal;
                let bsdf = hit.object.bsdf();

                if let SceneObject::Emitter(e) = &hit.object {
                    illumination += throughput * e.radiance(outgoing, normal);
                }

                illumination += throughput * self.direct_illum.sample(scene, &hit, &self.sampler);

                if let Some(bxdf_sample) =
                    bsdf.sample(normal, outgoing, self.sampler.sample(), BxDFFlag::empty())
                {
                    if bxdf_sample.pdf == 0.0 || bxdf_sample.spectrum.is_black() {
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
                } else {
                    break;
                }
            }

            pixel.add(illumination);
        } else {
            pixel.add_none()
        }
    }
}
