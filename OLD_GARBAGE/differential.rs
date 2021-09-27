use crate::geometry::vec::{Vec2, Vec3};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Plane {
    pub origin: Vec3,
    pub normal: Vec3,
}

impl Plane {
    pub const fn new(origin: Vec3, normal: Vec3) -> Self {
        Self { origin, normal }
    }

    #[inline]
    pub fn translate(&mut self, t: Vec3) {
        self.origin += t;
    }
}

pub trait Atlas {
    fn is_valid_uv(&self, uv: Vec2) -> bool;

    fn tangential_plane(&self, uv: Vec2) -> Plane;
    fn normal(&self, uv: Vec2) -> Vec3;
    fn f(&self, uv: Vec2) -> Vec3;
    fn f_inv(&self, xyz: Vec3) -> Vec2;
}
