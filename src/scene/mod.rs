use crate::geometry::{Aabb, Geometry, Intersection, Ray};
use crate::Vec3;
use serde::{Deserialize, Serialize};

pub mod emitter;
pub mod object;
pub mod receiver;
pub mod sampleable;

pub use emitter::*;
pub use object::*;
pub use receiver::*;
pub use sampleable::*;

#[derive(Serialize, Deserialize)]
pub struct Scene;

#[typetag::serde]
impl Geometry for Scene {
    fn contains(&self, point: Vec3) -> Option<bool> {
        todo!()
    }

    fn bounds(&self) -> Aabb {
        todo!()
    }

    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        todo!()
    }

    fn intersects(&self, ray: Ray) -> bool {
        todo!()
    }
}
