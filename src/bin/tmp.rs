#![feature(exclusive_range_pattern)]

use ron::ser::PrettyConfig;
use rust_v2::bxdf::refraction::RefractiveType;

use rust_v2::bxdf::{
    BxDF, FresnelDielectric, FresnelSpecular, FresnelType, LambertianReflection, OrenNayar,
    SpecularReflection, BSDF,
};

use rust_v2::camera::{Camera, CameraConfig, PerspectiveCamera};
use rust_v2::color::Color;

use rust_v2::config::Config;
use rust_v2::geometry::{Point, Sphere};
use rust_v2::integrator::{DirectIllumination, Integrator, SpectralPath};
use rust_v2::renderer::RendererData;

use rust_v2::sampler::{CameraSampler, FloatSampler, SpectralSampler};
use rust_v2::scene::{Emitter, Receiver, SceneBuilder, SceneData, SceneObject};
use rust_v2::util::mc::sample_unit_disk_concentric;
use rust_v2::{Float, UVec2, Vec3};

const PASSES: usize = 60000;
const FOV: Float = 70.0;
const RESOLUTION: UVec2 = UVec2::new(512, 512);

const SPHERES: usize = 32;
const SPREAD: Float = 16.0;
const RADIUS: Float = 1.0;
const SIGMA: Float = 0.4;
const P_EMITTER: Float = 0.25;

fn main() {
    let config = config();
    let camera = camera();
    let integrator = integrator();
    let scene = scene();

    let data = RendererData::new(config, camera, None, integrator, scene);

    let ron = ron::ser::to_string_pretty(&data, PrettyConfig::default()).unwrap();
    std::fs::write("./spheres.ron", ron).unwrap();
}

fn config() -> Config {
    Config {
        output: "spheres".into(),
        passes: PASSES,
        threads: None,
    }
}

fn camera() -> Box<dyn Camera> {
    let sampler = CameraSampler::Random;
    let eye = Vec3::new(0.0, 5.0, 10.0);
    let target = Vec3::new(0.0, 0.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);

    let config = CameraConfig {
        sampler,
        eye,
        target,
        up,
        fov: FOV,
        resolution: RESOLUTION,
    };

    Box::new(PerspectiveCamera::new(config))
}

fn integrator() -> Box<dyn Integrator> {
    let max_depth = 8;
    let sampler = FloatSampler::Random;
    let spectral_sampler = SpectralSampler::Hero;
    let direct_illum = DirectIllumination::All;

    Box::new(SpectralPath::new(
        max_depth,
        sampler,
        spectral_sampler,
        direct_illum,
    ))
}

fn scene() -> SceneData {
    let mut scene = SceneBuilder::new();
    let sampler = FloatSampler::Random;

    let mut num_emitters = -1;
    for i in 0..SPHERES {
        let pos = sample_unit_disk_concentric(sampler.vec2()) * SPREAD;
        let center = Vec3::new(pos.x, RADIUS, pos.y);

        let geometry = Box::new(Sphere::new(center, RADIUS));
        let obj = if sampler.float() < P_EMITTER {
            SceneObject::Emitter(Emitter::new(
                geometry,
                BSDF::empty(),
                Color::White.into(),
                format!("Receiver {}", i),
            ))
        } else {
            num_emitters += 1;
            let bsdf = BSDF::new(vec![bxdf(sampler)]);
            SceneObject::Receiver(Receiver::new(
                geometry,
                bsdf,
                format!("Emitter {}", num_emitters),
            ))
        };

        scene = scene.append(obj);
    }

    scene.append(sky()).append(sun()).build_data()
}

fn bxdf(sampler: FloatSampler) -> Box<dyn BxDF> {
    let variants = Color::variants();
    const LEN: usize = Color::num();
    const OTHERS: usize = LEN + 2;

    let random = sampler.float() * OTHERS as Float;
    match random as usize {
        i @ 0..LEN => Box::new(OrenNayar::new(variants[i].into(), SIGMA)),
        i @ LEN..OTHERS => {
            let dielectric = FresnelDielectric::new(RefractiveType::Air, RefractiveType::Glass);
            let fresnel = FresnelType::Dielectric(dielectric);

            match i - LEN {
                0 => Box::new(SpecularReflection::new(Color::White.into(), fresnel)),
                1 => Box::new(FresnelSpecular::new(
                    Color::White.into(),
                    Color::White.into(),
                    dielectric.eta_i,
                    dielectric.eta_t,
                )),
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

fn sky() -> SceneObject {
    let geometry = Box::new(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1000.0));
    let bxdfs: Vec<Box<dyn BxDF>> =
        vec![Box::new(LambertianReflection::new(Color::BlueSky.into()))];
    let bsdf = BSDF::new(bxdfs);
    SceneObject::Receiver(Receiver::new(geometry, bsdf, "sky".into()))
}

fn sun() -> SceneObject {
    let geometry = Box::new(Point(Vec3::new(100.0, 100.0, -30.0)));
    SceneObject::Emitter(Emitter::new(
        geometry,
        BSDF::empty(),
        Color::White.into(),
        "sun".into(),
    ))
}
