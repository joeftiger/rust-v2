use crate::geometry::Disk;
use crate::scene::{Sampleable, SurfaceSample};
use crate::{Vec2, Vec3};

use crate::bxdf::bxdf_to_world;
use crate::util::mc::sample_unit_disk_concentric;

#[typetag::serde]
impl Sampleable for Disk {
    fn sample_surface(&self, _point: Vec3, sample: Vec2) -> SurfaceSample {
        let uv = sample_unit_disk_concentric(sample) * self.radius;
        let point = bxdf_to_world(self.normal).rotate_vector(uv.extend(0.0));

        SurfaceSample::new(point, self.normal)
    }
}
