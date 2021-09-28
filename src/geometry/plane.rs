use crate::geometry::{Aabb, Geometry, Intersection, Ray};
use crate::util::floats;
use crate::{Float, Vec3};
use cgmath::InnerSpace;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Plane {
    normal: Vec3,
    d: Float,
}

impl Plane {
    #[inline]
    pub const fn new(normal: Vec3, d: Float) -> Self {
        Self { normal, d }
    }
}

#[typetag::serde]
impl Geometry for Plane {
    #[inline(always)]
    fn contains(&self, _point: Vec3) -> Option<bool> {
        None
    }

    #[inline]
    fn bounds(&self) -> Aabb {
        Aabb::max()
    }

    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        let denom = self.normal.dot(ray.direction);

        if floats::approx_eq(self.normal.dot(ray.direction), 0.0) {
            return None;
        }

        let p = self.normal * self.d - ray.origin;
        let t = p.dot(self.normal) / denom;
        if !ray.contains(t) {
            return None;
        }

        let point = ray.at(t);

        Some(Intersection::new(point, self.normal, t))
    }

    #[inline]
    fn intersects(&self, ray: Ray) -> bool {
        floats::approx_eq(self.normal.dot(ray.direction), 0.0)
    }
}
