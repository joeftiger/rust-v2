use crate::bxdf::BxDFFlag;
use crate::camera::sensor::Pixel;
use crate::geometry::{offset_ray_towards, Ray};
use crate::integrator::{DirectIllumination, Integrator};
use crate::sampler::FloatSampler;
use crate::scene::{Scene, SceneIntersection, SceneObject};
use crate::Spectrum;
use cgmath::InnerSpace;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Whitted {
    max_depth: u32,
    sampler: FloatSampler,
    direct_illum: DirectIllumination,
}

impl Whitted {
    pub const fn new(
        max_depth: u32,
        sampler: FloatSampler,
        direct_illum: DirectIllumination,
    ) -> Self {
        Self {
            max_depth,
            sampler,
            direct_illum,
        }
    }

    fn integrate_flag(
        &self,
        scene: &Scene,
        intersection: &SceneIntersection,
        depth: u32,
        flag: BxDFFlag,
    ) -> Spectrum {
        let outgoing = -intersection.i.incoming;
        let bsdf = intersection.object.bsdf();
        let normal = intersection.i.normal;

        if let Some(bxdf_sample) = bsdf.sample(normal, outgoing, self.sampler.sample(), flag) {
            if bxdf_sample.pdf > 0.0 && !bxdf_sample.spectrum.is_black() {
                let cos_abs = if bxdf_sample.flag.specular() {
                    // division of cosine omitted in specular bxdfs
                    1.0
                } else {
                    bxdf_sample.incident.dot(normal).abs()
                };

                if cos_abs != 0.0 {
                    let refl_ray =
                        offset_ray_towards(intersection.i.point, normal, bxdf_sample.incident);

                    if let Some(si) = scene.intersect(refl_ray) {
                        let illum = self.illumination(scene, &si, depth);
                        return illum * bxdf_sample.spectrum * (cos_abs / bxdf_sample.pdf);
                    }
                }
            }
        }

        Spectrum::splat(0.0)
    }

    fn illumination(
        &self,
        scene: &Scene,
        intersection: &SceneIntersection,
        depth: u32,
    ) -> Spectrum {
        let mut illumination = Spectrum::splat(0.0);

        if let SceneObject::Emitter(e) = intersection.object {
            illumination += e.radiance();
        }

        illumination += self.direct_illum.sample(scene, intersection, &self.sampler);

        let new_depth = depth + 1;
        if new_depth < self.max_depth {
            let reflection = BxDFFlag::SPECULAR | BxDFFlag::REFLECTION;
            let transmission = BxDFFlag::SPECULAR | BxDFFlag::TRANSMISSION;
            let both = reflection | transmission;
            illumination += self.integrate_flag(scene, intersection, new_depth, reflection);
            illumination += self.integrate_flag(scene, intersection, new_depth, transmission);
            illumination += self.integrate_flag(scene, intersection, new_depth, both);
        }

        illumination
    }
}

#[typetag::serde]
impl Integrator for Whitted {
    fn integrate(&self, scene: &Scene, primary_ray: Ray, pixel: &mut Pixel) {
        // if pixel.position == crate::UVec2::new(100, 232) {
        //     println!("debug");
        // }

        if let Some(i) = scene.intersect(primary_ray) {
            let illumination = self.illumination(scene, &i, 0);

            pixel.add(illumination);
        } else {
            pixel.add_none();
        }
    }
}
