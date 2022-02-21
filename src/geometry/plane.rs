use crate::geometry::{Aabb, Geometry, Intersection, Ray};
use crate::util::floats;
use crate::Vec3;
use cgmath::InnerSpace;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Plane {
    pub point: Vec3,
    pub normal: Vec3,
}

impl Plane {
    pub const fn new(point: Vec3, normal: Vec3) -> Self {
        Self { point, normal }
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
        if floats::approx_eq(denom, 0.0) {
            return None;
        }

        let delta = self.point - ray.origin;
        let t = delta.dot(self.normal) / denom;
        if !ray.contains(t) {
            return None;
        }

        Some(Intersection::new(ray.at(t), self.normal, ray.direction, t))
    }

    #[inline]
    fn intersects(&self, ray: Ray) -> bool {
        let denom = self.normal.dot(ray.direction);
        if floats::approx_eq(denom, 0.0) {
            return false;
        }

        let delta = self.point - ray.origin;
        let t = delta.dot(self.normal) / denom;
        ray.contains(t)
    }
}
