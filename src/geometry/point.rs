use crate::geometry::{Aabb, Geometry, Intersection, Ray};
use crate::Vec3;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Point(pub Vec3);

#[typetag::serde]
impl Geometry for Point {
    #[inline(always)]
    fn contains(&self, _point: Vec3) -> Option<bool> {
        None
    }

    #[inline]
    fn bounds(&self) -> Aabb {
        Aabb::new(self.0, self.0)
    }

    #[inline(always)]
    fn intersect(&self, _ray: Ray) -> Option<Intersection> {
        None
    }

    #[inline(always)]
    fn intersects(&self, _ray: Ray) -> bool {
        false
    }
}
