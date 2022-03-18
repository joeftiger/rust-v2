use crate::geometry::{spherical_to_cartesian_frame_trig, CoordinateSystem, Sphere};
use crate::scene::{Sampleable, SurfaceSample};
use crate::util::mc::sample_unit_sphere;
use crate::{Float, Vec2, Vec3};
use cgmath::InnerSpace;
#[cfg(not(feature = "f64"))]
use std::f32::consts::TAU;
#[cfg(feature = "f64")]
use std::f64::consts::TAU;

fn sample_surface_inside(sphere: &Sphere, sample: Vec2) -> SurfaceSample {
    let mut normal = sample_unit_sphere(sample);
    let point = sphere.center + sphere.radius * normal;
    if sphere.inverse {
        normal = -normal;
    }

    SurfaceSample::new(point, normal)
}

#[typetag::serde]
impl Sampleable for Sphere {
    // Copyright: https://github.com/mmp/pbrt-v3/blob/master/src/shapes/sphere.cpp
    fn sample_surface(&self, origin: Vec3, sample: Vec2) -> SurfaceSample {
        let oc = self.center - origin;
        let dist_sq = oc.magnitude2();
        let r2 = self.radius2();

        if dist_sq < r2 {
            // inside the sphere (may happen)
            sample_surface_inside(self, sample)
        } else {
            let distance = dist_sq.sqrt();
            let axis = oc / -distance;
            let frame = CoordinateSystem::from_y(axis);

            /* PBR code */
            let sin_theta_max = Float::sqrt(r2 / dist_sq);
            let sin_theta_max2 = sin_theta_max * sin_theta_max;
            let inv_sin_theta_max = 1.0 / sin_theta_max;
            let inv_sin_theta_max2 = inv_sin_theta_max * inv_sin_theta_max;
            let cos_theta_max = Float::max(0.0, 1.0 - sin_theta_max2).sqrt();

            let mut cos_theta = (cos_theta_max - 1.0) * sample.x + 1.0;
            let mut sin_theta2 = 1.0 - cos_theta * cos_theta;

            if sin_theta2 < 0.00068523 {
                sin_theta2 = sin_theta_max2 * sample.x;
                cos_theta = (1.0 - sin_theta2).sqrt();
            }

            let cos_alpha = sin_theta2.mul_add(
                inv_sin_theta_max,
                cos_theta * Float::max(0.0, 1.0 - sin_theta2 * inv_sin_theta_max2).sqrt(),
            );
            let sin_alpha = Float::max(0.0, 1.0 - cos_alpha * cos_alpha).sqrt();
            let (sin_phi, cos_phi) = Float::sin_cos(sample.y * TAU);

            let mut normal =
                spherical_to_cartesian_frame_trig(sin_phi, cos_phi, sin_alpha, cos_alpha, &frame);
            /* end PBR code */
            if self.inverse {
                normal = -normal;
            }

            let point = self.center + self.radius * normal;

            SurfaceSample::new(point, normal)
        }
    }
}
