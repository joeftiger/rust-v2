use core::hash::{Hash, Hasher};

#[derive(Copy, Clone)]
pub struct Item {
    pub value: u32,
    pub id: u32,
}

impl Item {
    pub const fn new(value: u32, id: u32) -> Self {
        Self { value, id }
    }
}

impl Hash for Item {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl PartialEq for Item {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Item {}
