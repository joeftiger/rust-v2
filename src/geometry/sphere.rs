use crate::geometry::{Aabb, Geometry, Intersection, Ray};
use crate::util::math;
use crate::{Float, Vec3};
use cgmath::InnerSpace;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sphere {
    center: Vec3,
    radius: Float,
}

impl Sphere {
    #[inline]
    pub const fn new(center: Vec3, radius: Float) -> Self {
        Self { center, radius }
    }

    #[inline]
    pub fn radius2(&self) -> Float {
        self.radius * self.radius
    }
}

impl Geometry for Sphere {
    fn contains(&self, point: Vec3) -> Option<bool> {
        Some((point - self.center).magnitude2() <= self.radius2())
    }

    fn bounds(&self) -> Aabb {
        let diff = Vec3::new(self.radius, self.radius, self.radius);
        let min = self.center - diff;
        let max = self.center + diff;

        Aabb::new(min, max)
    }

    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        let dir = ray.direction;
        let oc = ray.origin - self.center;

        if !ray.contains(oc.magnitude() - self.radius) {
            return None;
        }

        let a = dir.dot(dir);
        let b = 2.0 * dir.dot(oc);
        let c = oc.dot(oc) - self.radius2();

        let (t_min, t_max) = math::solve_quadratic(a, b, c)?;

        let t = if ray.contains(t_min) {
            t_min
        } else if ray.contains(t_max) {
            t_max
        } else {
            return None;
        };

        let point = ray.at(t);
        let normal = (point - self.center).normalize();

        Some(Intersection::new(point, normal, t))
    }

    fn intersects(&self, ray: Ray) -> bool {
        let dir = ray.direction;
        let oc = ray.origin - self.center;

        if !ray.contains(oc.magnitude() - self.radius) {
            return false;
        }

        let a = dir.dot(dir);
        let b = 2.0 * dir.dot(oc);
        let c = oc.dot(oc) - self.radius2();

        if let Some((t_min, t_max)) = math::solve_quadratic(a, b, c) {
            ray.contains(t_min) || ray.contains(t_max)
        } else {
            false
        }
    }
}
