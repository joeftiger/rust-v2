use crate::geometry::{Aabb, Geometry, Ray};
use std::collections::HashSet;
use std::sync::Arc;

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

pub struct Tree<T> {
    root: Node<T>,
    space: Aabb,
}

impl<T> Tree<T>
where
    T: Clone,
{
    pub fn new<F: Fn(&T) -> Aabb>(values: Vec<T>, f: F) -> Self {
        let mut space = Aabb::empty();
        let n = values.len();
        let mut candidates = Candidates::with_capacity(n * 6);

        values.iter().enumerate().for_each(|(id, v)| {
            let bounds = f(v);
            let item = Arc::new(Item::new(v.clone(), id as u32));
            candidates.append(&mut Candidate::gen_candidates(item, &bounds));

            space = space.join(bounds);
        });

        candidates.sort();

        let mut sides = vec![Side::Both; n];
        let root = Node::new(space, candidates, n, &mut sides);

        Self { root, space }
    }

    #[inline]
    pub fn bounds(&self) -> Aabb {
        self.space
    }

    pub fn intersect(&self, ray: Ray) -> Vec<Arc<T>> {
        if self.space.intersects(ray) {
            let mut items = HashSet::new();
            self.root.intersect(ray, &mut items);

            items.iter().map(|i| i.value.clone()).collect()
        } else {
            vec![]
        }
    }
}

impl<T> Default for Tree<T> {
    fn default() -> Self {
        Self {
            root: Node::Leaf {
                items: HashSet::new(),
            },
            space: Aabb::empty(),
        }
    }
}
