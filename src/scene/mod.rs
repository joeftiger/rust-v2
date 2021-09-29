use crate::geometry::{Aabb, Geometry, Intersection, Ray};
use crate::Vec3;
use serde::{Deserialize, Serialize};

pub mod emitter;
pub mod object;
pub mod receiver;
pub mod sampleable;

use crate::geometry::bvh::Tree;
use core::convert::{Infallible, TryFrom};
pub use emitter::*;
pub use object::*;
pub use receiver::*;
pub use sampleable::*;

#[derive(Serialize, Deserialize, Default)]
#[serde(try_from = "SceneData")]
pub struct Scene {
    emitters: Vec<u32>,
    objects: Vec<SceneObject>,
    #[serde(skip)]
    bvh: Tree,
}

impl Scene {
    fn with_capacity(emitters: usize, objects: usize) -> Self {
        assert!(emitters <= objects);
        Self {
            emitters: Vec::with_capacity(emitters),
            objects: Vec::with_capacity(objects),
            bvh: Default::default(),
        }
    }

    fn add(&mut self, obj: SceneObject) {
        let index = self.objects.len();
        assert!(index <= u32::MAX as usize);

        if obj.receiver() {
            self.emitters.push(index as u32);
        }
        self.objects.push(obj);
    }

    fn build_tree(&mut self) {
        let indices: Vec<u32> = (0..self.objects.len() as u32).collect();
        self.bvh = Tree::new(&indices, |i| self.objects[i as usize].bounds());
    }
}

#[typetag::serde]
impl Geometry for Scene {
    #[inline(always)]
    fn contains(&self, _point: Vec3) -> Option<bool> {
        None
    }

    #[inline]
    fn bounds(&self) -> Aabb {
        self.bvh.bounds()
    }

    fn intersect(&self, mut ray: Ray) -> Option<Intersection> {
        let mut intersection = None;

        for hit_index in self.bvh.intersect(ray) {
            if let Some(i) = self.objects[hit_index as usize].intersect(ray) {
                ray.t_end = i.t;
                intersection = Some(i);
            }
        }

        intersection
    }

    fn intersects(&self, ray: Ray) -> bool {
        self.bvh
            .intersect(ray)
            .iter()
            .any(|&i| self.objects[i as usize].intersects(ray))
    }
}

impl TryFrom<SceneData> for Scene {
    type Error = Infallible;

    #[cold]
    #[inline(never)]
    fn try_from(raw_scene: SceneData) -> Result<Self, Self::Error> {
        let emitters = raw_scene.objects.iter().filter(|o| o.emitter()).count();

        let mut scene = Scene::with_capacity(emitters, raw_scene.objects.len());
        for obj in raw_scene.objects {
            scene.add(obj);
        }
        scene.build_tree();

        Ok(scene)
    }
}

#[derive(Deserialize)]
struct SceneData {
    objects: Vec<SceneObject>,
}
