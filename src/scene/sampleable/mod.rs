mod point;
mod sphere;

use crate::geometry::Geometry;
use crate::{Float, Vec2, Vec3};

#[derive(Copy, Clone)]
pub struct SurfaceSample {
    pub point: Vec3,
    pub normal: Vec3,
}
impl SurfaceSample {
    pub fn new(point: Vec3, normal: Vec3) -> Self {
        Self { point, normal }
    }
}

#[typetag::serde]
pub trait Sampleable: Geometry {
    fn surface_area(&self) -> Float;

    fn sample_surface(&self, origin: Vec3, sample: Vec2) -> SurfaceSample;
}
