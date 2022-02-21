use crate::geometry::Disk;
use crate::scene::{Sampleable, SurfaceSample};
use crate::{Float, Vec2, Vec3};

use crate::bxdf::bxdf_to_world;
use crate::util::mc::sample_unit_disk_concentric;
#[cfg(not(feature = "f64"))]
use std::f32::consts::PI;
#[cfg(feature = "f64")]
use std::f64::consts::PI;

#[typetag::serde]
impl Sampleable for Disk {
    #[inline]
    fn surface_area(&self) -> Float {
        self.radius.powi(2) * PI
    }

    fn sample_surface(&self, _point: Vec3, sample: Vec2) -> SurfaceSample {
        let uv = sample_unit_disk_concentric(sample) * self.radius;
        let point = bxdf_to_world(self.normal).rotate_vector(uv.extend(0.0));

        SurfaceSample::new(point, self.normal)
    }
}
