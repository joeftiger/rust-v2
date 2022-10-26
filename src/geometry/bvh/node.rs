use super::{Candidate, Plane, Side};
use crate::geometry::{Aabb, Geometry, Ray};
use crate::Float;

const K_T: Float = 4.;
const K_I: Float = 1.;

// #[derive(Clone)]
pub struct InternalNode {
    left_space: Aabb,
    left_node: Node,
    right_space: Aabb,
    right_node: Node,
}

pub enum Node {
    Leaf(Vec<u32>),
    Node(Box<InternalNode>),
}

impl Default for Node {
    fn default() -> Self {
        Self::empty()
    }
}

impl Node {
    pub fn new(space: Aabb, candidates: Vec<Candidate>, n: usize, sides: &mut Vec<Side>) -> Self {
        let (cost, best_index, n_l, n_r) = Self::partition(n, space, &candidates);

        // Check that the cost of the splitting is not higher than the cost of the leaf.
        if cost > K_I * n as Float {
            // Create indices values vector
            let values = candidates
                .iter()
                .filter_map(|e| {
                    if e.is_left && e.dim() == 0 {
                        Some(e.index)
                    } else {
                        None
                    }
                })
                .collect();
            return Self::Leaf(values);
        }

        // Compute the new spaces divided by `plane`
        let (left_space, right_space) = Self::split_space(space, candidates[best_index].plane);

        // Compute which candidates are part of the left and right space
        let (left_candidates, right_candidates) = Self::classify(candidates, best_index, sides);

        Self::Node(Box::new(InternalNode {
            left_node: Self::new(left_space, left_candidates, n_l, sides),
            right_node: Self::new(right_space, right_candidates, n_r, sides),
            left_space,
            right_space,
        }))
    }

    #[inline]
    pub const fn empty() -> Self {
        Self::Leaf(Vec::new())
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Node::Leaf(v) => v.is_empty(),
            Node::Node(_) => false,
        }
    }

    /// Compute the best splitting candidate
    /// Return:
    /// * Cost of the split
    /// * Index of the best candidate
    /// * Number of items in the left partition
    /// * Number of items in the right partition
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
            let dim = candidate.dim();

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

    pub fn intersect(&self, ray: Ray, intersected_values: &mut Vec<u32>) {
        match self {
            Self::Leaf(values) => intersected_values.extend(values.iter().copied()),
            Self::Node(node) => {
                if Some(true) == node.right_space.contains(ray.at(ray.t_start))
                    || node.right_space.intersects(ray)
                {
                    node.right_node.intersect(ray, intersected_values);
                }
                if Some(true) == node.left_space.contains(ray.at(ray.t_start))
                    || node.left_space.intersects(ray)
                {
                    node.left_node.intersect(ray, intersected_values);
                }
            }
        }
    }

    fn split_space(space: Aabb, plane: Plane) -> (Aabb, Aabb) {
        let mut left = space;
        let mut right = space;
        match plane {
            Plane::X(x) => {
                left.max.x = x.clamp(space.min.x, space.max.x);
                right.min.x = left.max.x;
            }
            Plane::Y(y) => {
                left.max.y = y.clamp(space.min.y, space.max.y);
                right.min.y = left.max.y;
            }
            Plane::Z(z) => {
                left.max.z = z.clamp(space.min.z, space.max.z);
                right.min.z = left.max.z;
            }
        }
        (left, right)
    }

    fn classify(
        candidates: Vec<Candidate>,
        best_index: usize,
        sides: &mut [Side],
    ) -> (Vec<Candidate>, Vec<Candidate>) {
        // Step 1: Udate sides to classify items
        Self::classify_items(&candidates, best_index, sides);

        // Step 2: Splicing candidates left and right subspace
        Self::splicing_candidates(candidates, sides)
    }

    /// Step 1 of classify.
    /// Given a candidate list and a splitting candidate identify which items are part of the
    /// left, right and both subspaces.
    fn classify_items(candidates: &[Candidate], best_index: usize, sides: &mut [Side]) {
        let best_dimension = candidates[best_index].dim();
        for i in 0..(best_index + 1) {
            if candidates[i].dim() == best_dimension {
                if candidates[i].is_left {
                    sides[candidates[i].index as usize] = Side::Both;
                } else {
                    sides[candidates[i].index as usize] = Side::Left;
                }
            }
        }
        for i in best_index..candidates.len() {
            if candidates[i].dim() == best_dimension && candidates[i].is_left {
                sides[candidates[i].index as usize] = Side::Right;
            }
        }
    }

    // Step 2: Splicing candidates left and right subspace given items sides
    fn splicing_candidates(
        candidates: Vec<Candidate>,
        sides: &[Side],
    ) -> (Vec<Candidate>, Vec<Candidate>) {
        let mut left = Vec::with_capacity(candidates.len() / 2);
        let mut right = Vec::with_capacity(candidates.len() / 2);

        for c in candidates {
            match sides[c.index as usize] {
                Side::Left => left.push(c),
                Side::Right => right.push(c),
                Side::Both => {
                    right.push(c);
                    left.push(c);
                }
            }
        }
        (left, right)
    }

    /// Compute surface area volume of a space (AABB).
    #[inline]
    fn surface_area(v: Aabb) -> Float {
        v.volume()
    }

    /// Surface Area Heuristic (SAH)
    fn cost(p: Plane, v: Aabb, n_l: usize, n_r: usize) -> Float {
        // Split space
        let (v_l, v_r) = Self::split_space(v, p);

        // Compute the surface area of both subspace
        let (vol_l, vol_r) = (Self::surface_area(v_l), Self::surface_area(v_r));

        // Compute the surface area of the whole space
        let vol_v = vol_l + vol_r;

        // If one of the subspace is empty then the split can't be worth
        if vol_v == 0. || vol_l == 0. || vol_r == 0. {
            return Float::INFINITY;
        }

        // Decrease cost if it cuts empty space
        let factor = if n_l == 0 || n_r == 0 { 0.8 } else { 1. };

        factor * (K_T + K_I * (n_l as Float * vol_l / vol_v + n_r as Float * vol_r / vol_v))
    }
}
