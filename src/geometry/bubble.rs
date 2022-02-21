use crate::geometry::{Aabb, Geometry, Intersection, Ray, Sphere};
use crate::{Float, Vec3};
use cgmath::InnerSpace;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bubble {
    inner: Sphere,
    outer: Sphere,
}

impl Bubble {
    #[inline]
    pub const fn new(center: Vec3, inner_radius: Float, outer_radius: Float) -> Self {
        Self {
            inner: Sphere::new(center, inner_radius),
            outer: Sphere::new(center, outer_radius),
        }
    }
}

#[typetag::serde]
impl Geometry for Bubble {
    #[inline]
    fn contains(&self, point: Vec3) -> Option<bool> {
        let m = (point - self.outer.center).magnitude2();

        Some(self.inner.radius2() <= m && m <= self.outer.radius2())
    }

    #[inline]
    fn bounds(&self) -> Aabb {
        self.outer.bounds()
    }

    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        let outer = self.outer.intersect(ray);
        let inner = self.inner.intersect(ray);

        if let Some(o) = outer {
            if let Some(mut i) = inner {
                if o.t < i.t {
                    Some(o)
                } else {
                    // invert the "outer normal", as we hit the bubble from the inside
                    i.normal = -i.normal;
                    Some(i)
                }
            } else {
                Some(o)
            }
        } else {
            inner
        }
    }

    fn intersects(&self, ray: Ray) -> bool {
        self.outer.intersects(ray) || self.inner.intersects(ray)
    }
}
