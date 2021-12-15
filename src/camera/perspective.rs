use crate::camera::Camera;
use crate::geometry::Ray;
use crate::sampler::CameraSampler;
use crate::{Float, Mat4, UVec2, Vec2, Vec3};
use cgmath::{ElementWise, EuclideanSpace, InnerSpace, Point3, Transform, Zero};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(from = "CameraConfig")]
#[serde(into = "CameraConfig")]
pub struct PerspectiveCamera {
    sampler: CameraSampler,
    eye: Vec3,
    x_dir: Vec3,
    y_dir: Vec3,
    lower_left: Vec3,
    res: UVec2,
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
            config: Box::new(config),
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
impl Camera for PerspectiveCamera {
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
