use crate::geometry::{Aabb, Geometry, Intersection, Ray};
use crate::Vec3;
use serde::{Deserialize, Deserializer, Serialize};

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

#[derive(Default, Serialize)]
pub struct Scene {
    #[serde(skip_serializing)]
    emitters: Vec<u32>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    objects: Vec<SceneObject>,
    #[serde(skip_serializing)]
    bvh: Tree,
}

impl Scene {
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }

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

    pub fn add_object(&mut self, o: SceneObject) {
        if o.emitter() {
            self.emitters.push(self.objects.len() as u32);
        }

        self.objects.push(o);
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
            .into_iter()
            .any(|i| self.objects[i as usize].intersects(ray))
    }
}

impl<'de> Deserialize<'de> for Scene {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let objects = {
            #[derive(Deserialize)]
            struct Scene {
                #[serde(default)]
                objects: Vec<SceneObject>,
            }
            let s = Scene::deserialize(deserializer)?;
            s.objects
        };

        let emitters = objects
            .iter()
            .enumerate()
            .filter_map(|(i, o)| if o.emitter() { Some(i as u32) } else { None })
            .collect();

        let mut scene = Scene {
            emitters,
            objects,
            bvh: Default::default(),
        };
        scene.build_tree();

        Ok(scene)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cgmath::Zero;

    #[test]
    fn serde_empty() {
        let empty = Scene::default();

        let ser = ron::to_string(&empty).unwrap();
        eprintln!("serde_empty():\t\t{:?}", &ser);
        let de = ron::from_str::<Scene>(&ser).unwrap();

        assert!(de.emitters.is_empty());
        assert!(de.objects.is_empty());
        assert!(de.bvh.is_empty());
    }

    #[test]
    fn serde_receiver() {
        let mut scene = Scene::default();
        scene.add_object(SceneObject::dummy_receiver());

        let ser = ron::to_string(&scene).unwrap();
        eprintln!("serde_objects():\t\t{:?}", &ser);
        let de = ron::from_str::<Scene>(&ser).unwrap();

        assert!(de.emitters.is_empty());
        assert!(!de.bvh.is_empty());
        assert_eq!(1, de.objects.len());
        let o = &de.objects[0];
        assert!(o.receiver());
        assert_eq!(Aabb::unit(), o.bounds());
        assert_eq!("dummy", o.tag());
    }

    #[test]
    fn serde_emitter() {
        let mut scene = Scene::default();
        scene.add_object(SceneObject::dummy_emitter());

        let ser = ron::to_string(&scene).unwrap();
        eprintln!("serde_objects():\t\t{:?}", &ser);
        let de = ron::from_str::<Scene>(&ser).unwrap();

        assert_eq!(1, de.emitters.len());
        assert!(!de.bvh.is_empty());
        assert_eq!(1, de.objects.len());
        let o = &de.objects[0];
        assert!(o.emitter());
        assert_eq!(Aabb::new(Vec3::zero(), Vec3::zero()), o.bounds());
        assert_eq!("dummy", o.tag());
    }
}
