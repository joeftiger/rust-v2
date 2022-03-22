use crate::bxdf::face_forward;
use crate::geometry::Plane;
use crate::scene::{Sampleable, SurfaceSample};
use crate::{Vec2, Vec3};
use cgmath::InnerSpace;

#[typetag::serde]
impl Sampleable for Plane {
    fn sample_surface(&self, origin: Vec3, _sample: Vec2) -> SurfaceSample {
        let v = origin - self.point;
        let distance = v.dot(self.normal);

        let point = origin - distance * self.normal;
        let normal = face_forward(v, self.normal);

        SurfaceSample::new(point, normal)
    }
}
