use super::{Item, Plane};
use crate::geometry::Aabb;
use core::cmp::Ordering;

pub type Candidates = Vec<Candidate>;

#[derive(Copy, Clone)]
pub struct Candidate {
    pub plane: Plane,
    pub is_left: bool,
    pub item: Item,
}

impl Candidate {
    pub const fn new(plane: Plane, is_left: bool, item: Item) -> Self {
        Self {
            plane,
            is_left,
            item,
        }
    }

    #[inline]
    pub fn gen_candidates(item: Item, aabb: &Aabb) -> Candidates {
        vec![
            Self::new(Plane::X(aabb.min.x), true, item),
            Self::new(Plane::X(aabb.max.x), false, item),
            Self::new(Plane::Y(aabb.min.y), true, item),
            Self::new(Plane::Y(aabb.max.y), false, item),
            Self::new(Plane::Z(aabb.min.z), true, item),
            Self::new(Plane::Z(aabb.max.z), false, item),
        ]
    }

    #[inline]
    pub fn dimension(&self) -> usize {
        self.plane.dim()
    }
}

impl Ord for Candidate {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.plane.val().total_cmp(&other.plane.val())
    }
}

impl PartialOrd for Candidate {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Candidate {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.plane == other.plane
    }
}

impl Eq for Candidate {}
