// use serde::{Deserialize, Serialize};
// use crate::bxdf::BxDFFlag;
// use crate::camera::sensor::Pixel;
// use crate::geometry::Ray;
// use crate::integrator::{DirectIllumination, Integrator};
// use crate::sampler::FloatSampler;
// use crate::scene::{Scene, SceneIntersection, SceneObject};
// use crate::Spectrum;
//
// #[derive(Copy, Clone, Debug, Serialize, Deserialize)]
// pub struct Whitted {
//     max_depth: u32,
//     sampler: FloatSampler,
//     direct_illum: DirectIllumination,
// }
// /*
// impl Whitted {
//     pub fn new(max_depth: u32, sampler: FloatSampler, direct_illum: DirectIllumination) -> Self {
//         Self { max_depth, sampler, direct_illum }
//     }
//
//     fn illum(&self, scene: &Scene, hit: SceneIntersection, depth: u32) -> Spectrum {
//         let mut spectrum = Spectrum::splat(0.0);
//
//         let outgoing_world = -hit.i.incoming;
//
//         if let SceneObject::Emitter(e) = hit.object {
//             spectrum += e.radiance(outgoing_world, hit.i.normal);
//         }
//
//         spectrum += self.direct_illum.sample(scene, &hit, &self.sampler);
//
//         // offset by 1 because we start at 0.
//         let new_depth = depth + 1;
//         if new_depth <= self.max_depth {
//             let sample = hit.object.bsdf().sample(hit.i.normal, outgoing_world, self.sampler.sample(), BxDFFlag::empty());
//
//             if let Some(sample) = sample {
//                 sample.fla
//             }
//         }
//
//         spectrum
//     }
// }*/
//
// #[typetag::serde]
// impl Integrator for Whitted {
//     fn integrate(&self, scene: &Scene, primary_ray: Ray, pixel: &mut Pixel) {
//         todo!()
//     }
// }
