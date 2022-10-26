use crate::camera::Camera;
use crate::geometry::Ray;
use crate::sampler::CameraSampler;
use crate::{Float, Mat4, UVec2, Vec2, Vec3};
use cgmath::{ElementWise, EuclideanSpace, InnerSpace, Point3, Transform, Zero};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
struct CameraConfig {
    sampler: CameraSampler,
    eye: Vec3,
    target: Vec3,
    up: Vec3,
    fov: Float,
    resolution: UVec2,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(from = "CameraConfig")]
#[serde(into = "CameraConfig")]
pub struct PerspectiveCamera {
    conf: CameraConfig,
    sampler: CameraSampler,
    look_at: Mat4,
    bot_left: Vec2,
    top_right: Vec2,
    res: UVec2,
    inv_res: Vec2,
}

#[typetag::serde]
impl Camera for PerspectiveCamera {
    fn resolution(&self) -> UVec2 {
        self.res
    }

    fn primary_ray(&self, pixel: UVec2) -> Ray {
        let sample = self.sampler.sample();
        let direction = (self.bot_left
            + (self.top_right - self.bot_left)
                .mul_element_wise(pixel.cast().unwrap() + sample)
                .mul_element_wise(self.inv_res))
        .extend(-1.0)
        .normalize();

        let origin = self.look_at.transform_vector(Vec3::zero());
        let direction = self.look_at.transform_vector(direction);

        Ray::new(origin, direction)
    }
}
impl From<CameraConfig> for PerspectiveCamera {
    fn from(conf: CameraConfig) -> Self {
        let res = conf.resolution.cast().unwrap();
        let look_at = Mat4::look_at_rh(
            Point3::from_vec(conf.eye),
            Point3::from_vec(conf.target),
            conf.up,
        );
        let inv_res = Vec2::new(1.0, 1.0).div_element_wise(res);

        let y = (0.5 * conf.fov).to_radians();
        let top_right = Vec2::new(y * res.x * inv_res.y, y);
        let bottom_left = -top_right;

        Self {
            conf,
            sampler: conf.sampler,
            look_at,
            bot_left: bottom_left,
            top_right,
            res: conf.resolution,
            inv_res,
        }
    }
}
impl From<PerspectiveCamera> for CameraConfig {
    fn from(p: PerspectiveCamera) -> Self {
        p.conf
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(from = "CameraConfig")]
#[serde(into = "CameraConfig")]
pub struct NaiveCamera {
    conf: CameraConfig,
    sampler: CameraSampler,
    eye: Vec3,
    x_dir: Vec3,
    y_dir: Vec3,
    lower_left: Vec3,
    res: UVec2,
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
impl From<CameraConfig> for NaiveCamera {
    fn from(conf: CameraConfig) -> Self {
        // compute orientation and distance of eye to scene center
        let view = (conf.target - conf.eye).normalize();
        let axis_right = view.cross(conf.up).normalize();
        let axis_up = axis_right.cross(view); // normalized by definition
        let distance = (conf.target - conf.eye).magnitude();

        let w = conf.resolution.x as Float;
        let h = conf.resolution.y as Float;
        let image_height = 2.0 * distance * (0.5 * conf.fov).to_radians().tan();
        let image_width = w / h * image_height;

        // compute width & height of the image plane
        // based on the opening angle of the camera (fovy) and the distance
        // of the eye to the near plane (distance)
        let x_dir = axis_right * image_width / w;
        let y_dir = -axis_up * image_height / h;

        let lower_left = conf.target - 0.5 * w * x_dir - 0.5 * h * y_dir;

        Self {
            conf,
            sampler: conf.sampler,
            eye: conf.eye,
            x_dir,
            y_dir,
            lower_left,
            res: conf.resolution,
        }
    }
}
impl From<NaiveCamera> for CameraConfig {
    fn from(n: NaiveCamera) -> Self {
        n.conf
    }
}
