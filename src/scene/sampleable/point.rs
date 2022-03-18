use crate::geometry::Point;
use crate::scene::{Sampleable, SurfaceSample};
use crate::{Vec2, Vec3};
use cgmath::InnerSpace;

#[typetag::serde]
impl Sampleable for Point {
    #[inline]
    fn sample_surface(&self, point: Vec3, _sample: Vec2) -> SurfaceSample {
        let normal = point - self.0;

        SurfaceSample::new(self.0, normal.normalize())
    }
}
