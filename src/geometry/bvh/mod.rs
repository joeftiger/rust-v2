use crate::geometry::{Aabb, Geometry, Ray};

mod candidate;
mod item;
mod node;
mod plane;
mod side;

use candidate::*;
use item::*;
use node::*;
use plane::*;
use side::*;

pub struct Tree {
    root: Node,
    space: Aabb,
}

impl Tree {
    pub fn new<F: Fn(u32) -> Aabb>(values: &[u32], f: F) -> Self {
        let mut space = Aabb::empty();
        let mut candidates = Candidates::with_capacity(values.len() as usize * 6);

        values.iter().enumerate().for_each(|(id, &value)| {
            let bounds = f(value);
            let item = Item::new(value, id as u32);
            candidates.append(&mut Candidate::gen_candidates(item, &bounds));

            space = space.join(bounds);
        });

        candidates.sort();

        let mut sides = vec![Side::Both; values.len()];
        let root = Node::new(space, candidates, values.len(), &mut sides);

        Self { root, space }
    }

    #[inline]
    pub fn bounds(&self) -> Aabb {
        self.space
    }

    pub fn intersect(&self, ray: Ray) -> Vec<u32> {
        if self.space.intersects(ray) {
            let mut items = Vec::new();
            self.root.intersect(ray, &mut items);

            items
        } else {
            Vec::new()
        }
    }
}

impl Default for Tree {
    fn default() -> Self {
        Self {
            root: Node::Leaf { items: Vec::new() },
            space: Aabb::empty(),
        }
    }
}
