use super::Plane;
use crate::geometry::Aabb;
use std::cmp::Ordering;

#[derive(Copy, Clone, Debug)]
pub struct Candidate {
    pub plane: Plane,
    pub is_left: bool,
    pub index: u32,
}

impl Candidate {
    pub const fn new(plane: Plane, is_left: bool, index: u32) -> Self {
        Self {
            plane,
            is_left,
            index,
        }
    }

    /// Generates split candidates for all dimensions.
    #[inline]
    pub fn gen_candidates(index: u32, aabb: Aabb) -> Vec<Candidate> {
        vec![
            Candidate::new(Plane::X(aabb.min.x), true, index),
            Candidate::new(Plane::X(aabb.max.x), false, index),
            Candidate::new(Plane::Y(aabb.min.y), true, index),
            Candidate::new(Plane::Y(aabb.max.y), false, index),
            Candidate::new(Plane::Z(aabb.min.z), true, index),
            Candidate::new(Plane::Z(aabb.max.z), false, index),
        ]
    }

    pub const fn dim(self) -> usize {
        self.plane.dim()
    }
}

impl PartialOrd for Candidate {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Candidate {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.plane.val().total_cmp(&other.plane.val())
    }
}

impl PartialEq<Self> for Candidate {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.plane.val() == other.plane.val() && self.dim() == other.dim()
    }
}

impl Eq for Candidate {}
