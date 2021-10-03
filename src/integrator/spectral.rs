use crate::bxdf::{BxDFFlag, BxDFSamplePacket};
use crate::camera::sensor::Pixel;
use crate::geometry::{offset_ray_towards, Ray};
use crate::integrator::{DirectIllumination, Integrator};
use crate::sampler::{FloatSampler, SpectralSampler};
use crate::scene::{Scene, SceneIntersection, SceneObject};
use crate::util::PacketOps;
use crate::{Float, PACKET_SIZE};
use cgmath::InnerSpace;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct SpectralPath {
    max_depth: u32,
    sampler: FloatSampler,
    spectral_sampler: SpectralSampler,
    direct_illum: DirectIllumination,
}

impl SpectralPath {
    fn trace_single<'a>(
        &self,
        scene: &'a Scene,
        mut hit: SceneIntersection<'a>,
        index: usize,
        illumination: &mut Float,
        throughput: &mut Float,
        curr_depth: u32,
    ) {
        for _ in curr_depth..self.max_depth {
            let outgoing = -hit.i.incoming;
            let point = hit.i.point;
            let normal = hit.i.normal;
            let bsdf = hit.object.bsdf();

            if let SceneObject::Emitter(e) = hit.object {
                *illumination += *throughput * e.radiance_lambda(outgoing, normal, index);
            }

            *illumination += *throughput
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

                *throughput *= bxdf_sample.spectrum * (cos_abs / bxdf_sample.pdf);

                let ray = offset_ray_towards(point, normal, bxdf_sample.incident);
                match scene.intersect(ray) {
                    Some(i) => hit = i,
                    None => break,
                }
            } else {
                break;
            }
        }
    }

    fn trace_bundle<'a>(
        &self,
        scene: &'a Scene,
        mut hit: SceneIntersection<'a>,
        indices: &[usize; PACKET_SIZE],
        illumination: &mut [Float; PACKET_SIZE],
        throughput: &mut [Float; PACKET_SIZE],
    ) {
        for curr_depth in 0..self.max_depth {
            let outgoing = -hit.i.incoming;
            let point = hit.i.point;
            let normal = hit.i.normal;
            let bsdf = hit.object.bsdf();

            if let SceneObject::Emitter(e) = hit.object {
                illumination
                    .add_assign(throughput.mul(e.radiance_packet(outgoing, normal, indices)));
            }

            illumination.add_assign(throughput.mul(self.direct_illum.sample_packet(
                scene,
                &hit,
                &self.sampler,
                indices,
            )));

            match bsdf.sample_packet(
                normal,
                outgoing,
                self.sampler.sample(),
                BxDFFlag::empty(),
                indices,
            ) {
                BxDFSamplePacket::Bundle(Some(bxdf_sample)) => {
                    if bxdf_sample.pdf == 0.0 || bxdf_sample.spectrum.is_black() {
                        break;
                    }

                    let cos_abs = if bxdf_sample.flag.specular() {
                        // division of cosine omitted in specular bxdfs
                        1.0
                    } else {
                        bxdf_sample.incident.dot(normal).abs()
                    };

                    throughput.mul_assign(bxdf_sample.spectrum.mul_t(cos_abs / bxdf_sample.pdf));

                    let ray = offset_ray_towards(point, normal, bxdf_sample.incident);
                    match scene.intersect(ray) {
                        Some(i) => hit = i,
                        None => break,
                    }
                }
                BxDFSamplePacket::Split(split) => {
                    for i in 0..PACKET_SIZE {
                        if let Some(bxdf_sample) = split[i] {
                            if bxdf_sample.pdf == 0.0 || bxdf_sample.spectrum == 0.0 {
                                continue;
                            }

                            let cos_abs = if bxdf_sample.flag.specular() {
                                // division of cosine omitted in specular bxdfs
                                1.0
                            } else {
                                bxdf_sample.incident.dot(normal).abs()
                            };

                            throughput[i] *= bxdf_sample.spectrum * (cos_abs / bxdf_sample.pdf);

                            let ray = offset_ray_towards(point, normal, bxdf_sample.incident);
                            match scene.intersect(ray) {
                                Some(new_hit) => {
                                    self.trace_single(
                                        scene,
                                        new_hit,
                                        indices[i],
                                        &mut illumination[i],
                                        &mut throughput[i],
                                        curr_depth,
                                    );
                                }
                                None => continue,
                            }
                        }
                    }

                    break;
                }
                _ => break,
            }
        }
    }
}

#[typetag::serde]
impl Integrator for SpectralPath {
    fn integrate(&self, scene: &Scene, primary_ray: Ray, pixel: &mut Pixel) {
        if let Some(hit) = scene.intersect(primary_ray) {
            let indices = self.spectral_sampler.create();
            let mut illumination = [0.0; PACKET_SIZE];
            let mut throughput = [1.0; PACKET_SIZE];

            self.trace_bundle(scene, hit, &indices, &mut illumination, &mut throughput);

            pixel.add_packet(&illumination, &indices);
        } else {
            pixel.add_none();
        }
    }
}
