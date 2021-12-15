use crate::geometry::{Aabb, Geometry, Intersection, Ray};
use crate::Vec3;
use serde::{Deserialize, Serialize, Serializer};

pub mod emitter;
pub mod object;
pub mod receiver;
pub mod sampleable;

use crate::geometry::bvh::Tree;
pub use emitter::*;
pub use object::*;
pub use receiver::*;
pub use sampleable::*;

/// A scene intersection is a more detailed `Intersection`, also containing a reference to the
/// intersected object.
#[derive(Clone)]
pub struct SceneIntersection<'a> {
    pub i: Intersection,
    pub object: &'a SceneObject,
}

impl<'a> SceneIntersection<'a> {
    pub const fn new(i: Intersection, object: &'a SceneObject) -> Self {
        Self { i, object }
    }
}

#[derive(Deserialize, Default)]
#[serde(from = "SceneData")]
pub struct Scene {
    emitters: Vec<u32>,
    objects: Vec<SceneObject>,
    #[serde(skip)]
    bvh: Tree,
}

impl Scene {
    pub fn num_emitters(&self) -> usize {
        self.emitters.len()
    }

    pub fn num_objects(&self) -> usize {
        self.objects.len()
    }

    /// Returns all emitter indices in the scene
    pub fn emitters(&self) -> &[u32] {
        &self.emitters
    }

    pub fn get_emitter(&self, index: usize) -> Option<&Emitter> {
        match self.get_object(index) {
            SceneObject::Emitter(e) => Some(e),
            _ => None,
        }
    }

    pub fn get_object(&self, index: usize) -> &SceneObject {
        &self.objects[index]
    }

    fn build_tree(&mut self) {
        let indices: Vec<u32> = (0..self.objects.len() as u32).collect();
        self.bvh = Tree::new(&indices, |i| self.objects[i as usize].bounds());
    }

    pub fn intersect(&self, mut ray: Ray) -> Option<SceneIntersection> {
        let mut intersection = None;

        for hit_index in self.bvh.intersect(ray) {
            let object = self.get_object(hit_index as usize);

            if let Some(i) = object.intersect(ray) {
                ray.t_end = i.t;
                intersection = Some(SceneIntersection::new(i, object));
            }
        }

        intersection
    }
}

#[derive(Default)]
pub struct SceneBuilder {
    objects: Vec<SceneObject>,
}

impl SceneBuilder {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn append(mut self, obj: SceneObject) -> Self {
        self.objects.push(obj);
        self
    }

    pub fn append_all(mut self, mut objs: Vec<SceneObject>) -> Self {
        self.objects.append(&mut objs);
        self
    }

    pub fn build(self) -> Scene {
        let emitters = self
            .objects
            .iter()
            .enumerate()
            .filter_map(|(i, o)| match o {
                SceneObject::Emitter(_) => Some(i as u32),
                SceneObject::Receiver(_) => None,
            })
            .collect();

        let mut scene = Scene {
            emitters,
            objects: self.objects,
            bvh: Default::default(),
        };
        scene.build_tree();

        scene
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

#[derive(Serialize)]
pub struct SceneDataRef<'a> {
    objects: &'a [SceneObject],
}

impl<'a> From<&'a Scene> for SceneDataRef<'a> {
    fn from(s: &'a Scene) -> Self {
        Self {
            objects: &s.objects,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct SceneData {
    objects: Vec<SceneObject>,
}

impl From<Scene> for SceneData {
    fn from(s: Scene) -> Self {
        Self { objects: s.objects }
    }
}

impl From<SceneData> for Scene {
    fn from(s: SceneData) -> Self {
        SceneBuilder::new().append_all(s.objects).build()
    }
}

impl Serialize for Scene {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        SceneDataRef {
            objects: &self.objects,
        }
        .serialize(serializer)
    }
}
