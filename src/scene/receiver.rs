use crate::bxdf::BSDF;
use crate::geometry::{Aabb, Geometry, Intersection, Ray};
use crate::Vec3;
use serde::{Deserialize, Serialize};

/// A receiver consists of a geometry and a BSDF.
#[derive(Serialize, Deserialize)]
pub struct Receiver {
    geometry: Box<dyn Geometry>,
    #[serde(default)]
    pub bsdf: BSDF,
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
