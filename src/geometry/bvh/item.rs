use core::hash::{Hash, Hasher};
use std::sync::Arc;

#[derive(Clone)]
pub struct Item<T> {
    pub value: Arc<T>,
    pub id: u32,
}

impl<T> Item<T> {
    pub fn new(value: T, id: u32) -> Self {
        Self {
            value: Arc::new(value),
            id,
        }
    }
}

impl<T> Hash for Item<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl<T> PartialEq for Item<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> Eq for Item<T> {}
