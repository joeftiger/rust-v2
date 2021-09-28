use super::{Candidate, Candidates, Item, Plane, Side};
use crate::geometry::{Aabb, Geometry, Ray};
use crate::Float;

const K_T: Float = 15.0;
const K_I: Float = 20.0;

#[derive(Clone)]
pub struct InternalNode {
    left_space: Aabb,
    left_node: Node,
    right_space: Aabb,
    right_node: Node,
}

impl InternalNode {
    fn new(left_space: Aabb, left_node: Node, right_space: Aabb, right_node: Node) -> Self {
        Self {
            left_space,
            left_node,
            right_space,
            right_node,
        }
    }
}

#[derive(Clone)]
pub enum Node {
    Leaf { items: Vec<Item> },
    Node { node: Box<InternalNode> },
}

impl Node {
    pub fn new(space: Aabb, candidates: Candidates, n: usize, sides: &mut Vec<Side>) -> Self {
        let (cost, best_index, n_l, n_r) = Self::partition(n, space, &candidates);

        // Check that the cost of the splitting is not higher than the cost of the leaf.
        if cost > K_I * n as Float {
            // Create the set of primitives
            let items = candidates
                .iter()
                .filter_map(|c| {
                    if c.is_left && c.dimension() == 0 {
                        Some(c.item)
                    } else {
                        None
                    }
                })
                .collect();

            return Self::Leaf { items };
        }

        let (left_space, right_space) = Self::split_space(space, candidates[best_index].plane);
        let (left_candidates, right_candidates) = Self::classify(candidates, best_index, sides);

        let inner_node = InternalNode::new(
            left_space,
            Self::new(left_space, left_candidates, n_l, sides),
            right_space,
            Self::new(right_space, right_candidates, n_r, sides),
        );

        Self::Node {
            node: Box::new(inner_node),
        }
    }

    fn partition(n: usize, space: Aabb, candidates: &[Candidate]) -> (Float, usize, usize, usize) {
        let mut best_cost = Float::INFINITY;
        let mut best_candidate_index = 0;

        // Variables to keep count the number of items in both subspace for each dimension
        let mut n_l = [0; 3];
        let mut n_r = [n; 3];

        // Keep n_l and n_r for the best splitting candidate
        let mut best_n_l = 0;
        let mut best_n_r = n;

        // Find best candidate
        for (i, candidate) in candidates.iter().enumerate() {
            let dim = candidate.dimension();

            // If the right candidate removes it from the right subspace
            if !candidate.is_left {
                n_r[dim] -= 1;
            }

            // Compute the cost of the split and update the best split
            let cost = Self::cost(candidate.plane, space, n_l[dim], n_r[dim]);
            if cost < best_cost {
                best_cost = cost;
                best_candidate_index = i;
                best_n_l = n_l[dim];
                best_n_r = n_r[dim];
            }

            // If the left candidate add it from the left subspace
            if candidate.is_left {
                n_l[dim] += 1;
            }
        }
        (best_cost, best_candidate_index, best_n_l, best_n_r)
    }

    fn cost(p: Plane, v: Aabb, n_l: usize, n_r: usize) -> Float {
        let (left, right) = Self::split_space(v, p);

        let volume_left = left.volume();
        if volume_left == 0.0 {
            return Float::INFINITY;
        }

        let volume_right = right.volume();
        if volume_right == 0.0 {
            return Float::INFINITY;
        }

        let total_volume = volume_left + volume_right;

        // Decrease cost if it cuts empty space
        let factor = if n_l == 0 || n_r == 0 { 0.8 } else { 1.0 };

        factor
            * (K_T
                + K_I
                    * (n_l as Float * volume_left / total_volume
                        + n_r as Float * volume_right / total_volume))
    }

    fn split_space(space: Aabb, plane: Plane) -> (Aabb, Aabb) {
        let mut left = space;
        let mut right = space;
        match plane {
            Plane::X(x) => {
                let clamp = x.clamp(space.min.x, space.max.x);
                left.max.x = clamp;
                right.min.x = clamp;
            }
            Plane::Y(y) => {
                let clamp = y.clamp(space.min.y, space.max.y);
                left.max.y = clamp;
                right.min.y = clamp;
            }
            Plane::Z(z) => {
                let clamp = z.clamp(space.min.z, space.max.z);
                left.max.z = clamp;
                right.min.z = clamp;
            }
        }
        (left, right)
    }

    fn classify(
        candidates: Candidates,
        best_index: usize,
        sides: &mut Vec<Side>,
    ) -> (Candidates, Candidates) {
        // Step 1: Udate sides to classify items
        Self::classify_items(&candidates, best_index, sides);

        // Step 2: Splicing candidates left and right subspace
        Self::splicing_candidates(candidates, sides)
    }

    /// Step 1 of classify.
    /// Given a candidate list and a splitting candidate identify wich items are part of the
    /// left, right and both subspaces.
    fn classify_items(candidates: &[Candidate], best_index: usize, sides: &mut Vec<Side>) {
        let best_dimension = candidates[best_index].dimension();
        for i in 0..(best_index + 1) {
            if candidates[i].dimension() == best_dimension {
                if !candidates[i].is_left {
                    sides[candidates[i].item.id as usize] = Side::Left;
                } else {
                    sides[candidates[i].item.id as usize] = Side::Both;
                }
            }
        }
        for i in best_index..candidates.len() {
            if candidates[i].dimension() == best_dimension && candidates[i].is_left {
                sides[candidates[i].item.id as usize] = Side::Right;
            }
        }
    }

    /// Step 2: Splicing candidates left and right subspace given items sides
    fn splicing_candidates(mut candidates: Candidates, sides: &[Side]) -> (Candidates, Candidates) {
        let mut left_candidates = Candidates::with_capacity(candidates.len() / 2);
        let mut right_candidates = Candidates::with_capacity(candidates.len() / 2);

        for e in candidates.drain(..) {
            match sides[e.item.id as usize] {
                Side::Left => left_candidates.push(e),
                Side::Right => right_candidates.push(e),
                Side::Both => {
                    right_candidates.push(e.clone());
                    left_candidates.push(e);
                }
            }
        }
        (left_candidates, right_candidates)
    }

    pub fn intersect(&self, ray: Ray, intersect_items: &mut Vec<u32>) {
        match self {
            Self::Leaf { items } => intersect_items.extend(items.iter().map(|i| i.value)),
            Self::Node { node } => {
                if node.left_space.intersects(ray) {
                    node.left_node.intersect(ray, intersect_items);
                }
                if node.right_space.intersects(ray) {
                    node.right_node.intersect(ray, intersect_items);
                }
            }
        }
    }
}
