use crate::camera::Camera;
use crate::geometry::Ray;
use crate::sampler::CameraSampler;
use crate::{Float, Mat4, UVec2, Vec2, Vec3};
use cgmath::{ElementWise, EuclideanSpace, InnerSpace, Point3, Transform, Zero};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct CameraConfig {
    sampler: CameraSampler,
    eye: Vec3,
    target: Vec3,
    up: Vec3,
    fov: Float,
    resolution: UVec2,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(from = "PerspectiveConfig")]
#[serde(into = "PerspectiveConfig")]
pub struct PerspectiveCamera {
    sampler: CameraSampler,
    look_at: Mat4,
    bot_left: Vec2,
    top_right: Vec2,
    res: UVec2,
    inv_res: Vec2,
}

impl PerspectiveCamera {
    pub fn new(
        sampler: CameraSampler,
        eye: Vec3,
        target: Vec3,
        up: Vec3,
        fov: Float,
        resolution: UVec2,
    ) -> Self {
        let res = resolution.cast().unwrap();
        let look_at = Mat4::look_at_lh(Point3::from_vec(eye), Point3::from_vec(target), up);
        let inv_res = Vec2::new(1.0, 1.0).div_element_wise(res);

        let y = (0.5 * fov).to_radians();
        let top_right = Vec2::new(y * res.x * inv_res.y, y);
        let bottom_left = -top_right;

        Self {
            sampler,
            look_at,
            bot_left: bottom_left,
            top_right,
            res: resolution,
            inv_res,
        }
    }
}

#[typetag::serde]
impl Camera for PerspectiveCamera {
    fn resolution(&self) -> UVec2 {
        self.res
    }

    fn primary_ray(&self, pixel: UVec2) -> Ray {
        let sample = self.sampler.sample();
        let direction = self.bot_left
            + (self.top_right - self.bot_left)
                .mul_element_wise(pixel.cast().unwrap() + sample)
                .mul_element_wise(self.inv_res);
        let direction = direction.extend(1.0).normalize();

        let origin = self.look_at.transform_point(Point3::from_vec(Vec3::zero()));
        let direction = self.look_at.transform_vector(direction);

        Ray::new(origin.to_vec(), direction)
    }
}

#[derive(Deserialize, Serialize)]
enum PerspectiveConfig {
    Full(PerspectiveCamera),
    Config(CameraConfig),
}
impl From<PerspectiveConfig> for PerspectiveCamera {
    fn from(conf: PerspectiveConfig) -> Self {
        match conf {
            PerspectiveConfig::Full(c) => c,
            PerspectiveConfig::Config(c) => {
                Self::new(c.sampler, c.eye, c.target, c.up, c.fov, c.resolution)
            }
        }
    }
}
impl From<PerspectiveCamera> for PerspectiveConfig {
    fn from(cam: PerspectiveCamera) -> Self {
        Self::Full(cam)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(from = "NaiveConfig")]
#[serde(into = "NaiveConfig")]
pub struct NaiveCamera {
    sampler: CameraSampler,
    eye: Vec3,
    x_dir: Vec3,
    y_dir: Vec3,
    lower_left: Vec3,
    res: UVec2,
}

impl NaiveCamera {
    pub fn new(
        sampler: CameraSampler,
        eye: Vec3,
        target: Vec3,
        up: Vec3,
        fov: Float,
        resolution: UVec2,
    ) -> Self {
        // compute orientation and distance of eye to scene center
        let view = (target - eye).normalize();
        let axis_right = view.cross(up).normalize();
        let axis_up = axis_right.cross(view); // normalized by definition
        let distance = (target - eye).magnitude();

        let w = resolution.x as Float;
        let h = resolution.y as Float;
        let image_height = 2.0 * distance * (0.5 * fov).to_radians().tan();
        let image_width = w / h * image_height;

        // compute width & height of the image plane
        // based on the opening angle of the camera (fovy) and the distance
        // of the eye to the near plane (distance)
        let x_dir = axis_right * image_width / w;
        let y_dir = -axis_up * image_height / h;

        let lower_left = target - 0.5 * w * x_dir - 0.5 * h * y_dir;

        Self {
            sampler,
            eye,
            x_dir,
            y_dir,
            lower_left,
            res: resolution,
        }
    }
}

#[typetag::serde]
impl Camera for NaiveCamera {
    fn resolution(&self) -> UVec2 {
        self.res
    }

    fn primary_ray(&self, pixel: UVec2) -> Ray {
        let sample = self.sampler.sample();

        let direction = self.lower_left
            + (pixel.x as Float + sample.x) * self.x_dir
            + (pixel.y as Float + sample.y) * self.y_dir
            - self.eye;
        let direction = direction.normalize();

        Ray::new(self.eye, direction)
    }
}

#[derive(Deserialize, Serialize)]
enum NaiveConfig {
    Full(Naive),
    Config(CameraConfig),
}
impl From<NaiveConfig> for NaiveCamera {
    fn from(conf: NaiveConfig) -> Self {
        match conf {
            NaiveConfig::Full(n) => Self {
                sampler: n.sampler,
                eye: n.eye,
                x_dir: n.x_dir,
                y_dir: n.y_dir,
                lower_left: n.lower_left,
                res: n.res
            },
            NaiveConfig::Config(c) => {
                Self::new(c.sampler, c.eye, c.target, c.up, c.fov, c.resolution)
            }
        }
    }
}
impl From<NaiveCamera> for NaiveConfig {
    fn from(c: NaiveCamera) -> Self {
        Self::Full(Naive {
            sampler: c.sampler,
            eye: c.eye,
            x_dir: c.x_dir,
            y_dir: c.y_dir,
            lower_left: c.lower_left,
            res: c.res
        })
    }
}

#[derive(Deserialize, Serialize)]
struct Naive {
    sampler: CameraSampler,
    eye: Vec3,
    x_dir: Vec3,
    y_dir: Vec3,
    lower_left: Vec3,
    res: UVec2,
}