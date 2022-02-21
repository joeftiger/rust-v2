use crate::camera::Camera;
use crate::geometry::Ray;
use crate::sampler::CameraSampler;
use crate::{Float, UVec2, Vec2, Vec3};
use cgmath::InnerSpace;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
#[serde(from = "OrthographicConfig")]
#[serde(into = "OrthographicConfig")]
pub struct OrthographicCamera {
    top_left: Vec3,
    x_dir: Vec3,
    y_dir: Vec3,
    z_dir: Vec3,
    resolution: UVec2,
    sampler: CameraSampler,
}

#[typetag::serde]
impl Camera for OrthographicCamera {
    #[inline(always)]
    fn resolution(&self) -> UVec2 {
        self.resolution
    }

    #[inline]
    fn primary_ray(&self, pixel: UVec2) -> Ray {
        let sample = self.sampler.sample();
        let right = self.x_dir * (sample.x + pixel.x as Float);
        let down = self.y_dir * (sample.y + pixel.y as Float);
        let origin = self.top_left + right + down;
        Ray::new(origin, self.z_dir)
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
enum OrthographicConfig {
    Full(Full),
    Config(Config),
}

impl From<OrthographicConfig> for OrthographicCamera {
    fn from(oc: OrthographicConfig) -> Self {
        match oc {
            OrthographicConfig::Full(f) => Self::from(f),
            OrthographicConfig::Config(c) => Self::from(c),
        }
    }
}

impl From<OrthographicCamera> for OrthographicConfig {
    fn from(o: OrthographicCamera) -> Self {
        Self::Full(Full::from(o))
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
struct Full {
    top_left: Vec3,
    x_dir: Vec3,
    y_dir: Vec3,
    z_dir: Vec3,
    resolution: UVec2,
    sampler: CameraSampler,
}

impl From<Full> for OrthographicCamera {
    fn from(f: Full) -> Self {
        Self {
            top_left: f.top_left,
            x_dir: f.x_dir,
            y_dir: f.y_dir,
            z_dir: f.z_dir,
            resolution: f.resolution,
            sampler: f.sampler,
        }
    }
}
impl From<OrthographicCamera> for Full {
    fn from(o: OrthographicCamera) -> Self {
        Self {
            top_left: o.top_left,
            x_dir: o.x_dir,
            y_dir: o.y_dir,
            z_dir: o.z_dir,
            resolution: o.resolution,
            sampler: o.sampler,
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
struct Config {
    position: Vec3,
    target: Vec3,
    up: Vec3,
    fov: Vec2,
    resolution: UVec2,
    sampler: CameraSampler,
}

impl From<Config> for OrthographicCamera {
    fn from(c: Config) -> Self {
        let z_dir = (c.target - c.position).normalize();
        let x_unit = z_dir.cross(c.up).normalize();
        let y_unit = z_dir.cross(x_unit).normalize();
        let top_left = c.position - 0.5 * (c.fov.x * x_unit + c.fov.y * y_unit);

        let x_dir = c.fov.x / c.resolution.x as Float * x_unit;
        let y_dir = c.fov.y / c.resolution.y as Float * y_unit;

        Self {
            top_left,
            x_dir,
            y_dir,
            z_dir,
            resolution: c.resolution,
            sampler: c.sampler,
        }
    }
}
