use crate::geometry::{Aabb, Geometry, Intersection, Plane, Ray, Sphere};
use crate::{Float, Vec3};
use cgmath::InnerSpace;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Disk {
    pub center: Vec3,
    pub normal: Vec3,
    pub radius: Float,
}

impl Disk {
    pub const fn new(center: Vec3, normal: Vec3, radius: Float) -> Self {
        Self {
            center,
            normal,
            radius,
        }
    }
}

#[typetag::serde]
impl Geometry for Disk {
    #[inline(always)]
    fn contains(&self, _point: Vec3) -> Option<bool> {
        None
    }

    #[inline]
    fn bounds(&self) -> Aabb {
        Sphere::new(self.center, self.radius).bounds()
    }

    #[inline]
    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        Plane::new(self.center, self.normal)
            .intersect(ray)
            .filter(|i| (i.point - self.center).magnitude2() <= self.radius.powi(2))
    }
}
