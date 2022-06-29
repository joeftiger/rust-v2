use crate::bxdf::BSDF;
use crate::geometry::{Aabb, Geometry, Intersection, Ray};
use crate::Vec3;
use serde::{Deserialize, Serialize};

/// A receiver consists of a geometry and a BSDF.
#[derive(Serialize, Deserialize)]
pub struct Receiver {
    pub geometry: Box<dyn Geometry>,
    #[serde(default)]
    pub bsdf: BSDF,
    #[serde(default)]
    pub tag: String,
}

impl Receiver {
    pub const fn new(geometry: Box<dyn Geometry>, bsdf: BSDF, tag: String) -> Self {
        Self { geometry, bsdf, tag }
    }

    #[cold]
    #[inline(never)]
    pub fn dummy() -> Self {
        Self {
            geometry: Box::new(Aabb::unit()),
            bsdf: Default::default(),
            tag: "dummy".into(),
        }
    }
}

#[typetag::serde]
impl Geometry for Receiver {
    #[inline]
    fn contains(&self, point: Vec3) -> Option<bool> {
        self.geometry.contains(point)
    }

    #[inline]
    fn bounds(&self) -> Aabb {
        self.geometry.bounds()
    }

    #[inline]
    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        self.geometry.intersect(ray)
    }

    #[inline]
    fn intersects(&self, ray: Ray) -> bool {
        self.geometry.intersects(ray)
    }
}
