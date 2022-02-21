mod candidate;
mod node;
mod plane;
mod side;

use crate::geometry::{Aabb, Geometry, Ray};
use candidate::*;
use node::*;
use plane::*;
use side::*;

pub struct Tree {
    root: Node,
    space: Aabb,
}

impl Default for Tree {
    fn default() -> Self {
        Self {
            root: Node::empty(),
            space: Aabb::empty(),
        }
    }
}

impl Tree {
    #[cold]
    pub fn new<F: Fn(u32) -> Aabb>(indices: &[u32], f: F) -> Self {
        let mut space = Aabb::empty();
        let mut candidates = Vec::with_capacity(indices.len() as usize * 6);

        for &index in indices {
            let bounds = f(index);
            candidates.append(&mut Candidate::gen_candidates(index, bounds));

            space = space.join(bounds);
        }

        candidates.sort();

        let mut sides = vec![Side::Both; indices.len()];
        let root = Node::new(space, candidates, indices.len(), &mut sides);

        Self { root, space }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.root.is_empty()
    }

    #[inline]
    pub const fn bounds(&self) -> Aabb {
        self.space
    }

    pub fn intersect(&self, ray: Ray) -> Vec<u32> {
        if Some(true) == self.space.contains(ray.at(ray.t_start)) || self.space.intersects(ray) {
            let mut items = Vec::new();
            self.root.intersect(ray, &mut items);

            items
        } else {
            Vec::new()
        }
    }
}
