use super::{Item, Plane};
use crate::geometry::Aabb;
use core::cmp::Ordering;
use std::sync::Arc;

pub type Candidates<T> = Vec<Candidate<T>>;

#[derive(Clone)]
pub struct Candidate<T>
where
    T: Clone,
{
    pub plane: Plane,
    pub is_left: bool,
    pub item: Arc<Item<T>>,
}

impl<T> Candidate<T>
where
    T: Clone,
{
    pub fn new(plane: Plane, is_left: bool, item: Arc<Item<T>>) -> Self {
        Self {
            plane,
            is_left,
            item,
        }
    }

    pub fn gen_candidates(item: Arc<Item<T>>, aabb: &Aabb) -> Candidates<T> {
        vec![
            Self::new(Plane::X(aabb.min.x), true, item.clone()),
            Self::new(Plane::X(aabb.max.x), false, item.clone()),
            Self::new(Plane::Y(aabb.min.y), true, item.clone()),
            Self::new(Plane::Y(aabb.max.y), false, item.clone()),
            Self::new(Plane::Z(aabb.min.z), true, item.clone()),
            Self::new(Plane::Z(aabb.max.z), false, item),
        ]
    }

    pub fn dimension(&self) -> usize {
        match self.plane {
            Plane::X(_) => 0,
            Plane::Y(_) => 1,
            Plane::Z(_) => 2,
        }
    }
}

impl<T> Ord for Candidate<T>
where
    T: Clone,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.plane.val().total_cmp(&other.plane.val())
    }
}

impl<T> PartialOrd for Candidate<T>
where
    T: Clone,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> PartialEq for Candidate<T>
where
    T: Clone,
{
    fn eq(&self, other: &Self) -> bool {
        self.plane == other.plane
    }
}

impl<T> Eq for Candidate<T> where T: Clone {}
