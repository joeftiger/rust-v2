use crate::bxdf::BSDF;
use crate::geometry::{Aabb, Geometry, Intersection, Ray};
use crate::scene::{Emitter, Receiver};
use crate::Vec3;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum SceneObject {
    Emitter(Emitter),
    Receiver(Receiver),
}

impl SceneObject {
    #[inline]
    pub fn bsdf(&self) -> &BSDF {
        match self {
            SceneObject::Emitter(e) => &e.bsdf,
            SceneObject::Receiver(r) => &r.bsdf,
        }
    }

    #[inline]
    pub fn tag(&self) -> &str {
        match self {
            SceneObject::Emitter(e) => &e.tag,
            SceneObject::Receiver(r) => &r.tag,
        }
    }

    #[inline]
    pub fn emitter(&self) -> bool {
        match self {
            SceneObject::Emitter(_) => true,
            SceneObject::Receiver(_) => false,
        }
    }

    #[inline]
    pub fn receiver(&self) -> bool {
        !self.emitter()
    }
}

#[typetag::serde]
impl Geometry for SceneObject {
    #[inline]
    fn contains(&self, point: Vec3) -> Option<bool> {
        match self {
            SceneObject::Emitter(e) => e.contains(point),
            SceneObject::Receiver(r) => r.contains(point),
        }
    }

    #[inline]
    fn bounds(&self) -> Aabb {
        match self {
            SceneObject::Emitter(e) => e.bounds(),
            SceneObject::Receiver(r) => r.bounds(),
        }
    }

    #[inline]
    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        match self {
            SceneObject::Emitter(e) => e.intersect(ray),
            SceneObject::Receiver(r) => r.intersect(ray),
        }
    }

    #[inline]
    fn intersects(&self, ray: Ray) -> bool {
        match self {
            SceneObject::Emitter(e) => e.intersects(ray),
            SceneObject::Receiver(r) => r.intersects(ray),
        }
    }
}
