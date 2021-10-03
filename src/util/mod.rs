pub mod floats;
pub mod math;
pub mod mc;
mod packet_impl;
pub mod threadpool;

pub use packet_impl::PacketOps;

pub struct Index(usize);

#[allow(dead_code)]
impl Index {
    #[inline]
    pub const fn new() -> Self {
        Self(0)
    }

    #[inline]
    pub fn inc(&mut self) {
        self.0 += 1;
    }

    #[inline]
    pub fn inc_by(&mut self, inc: usize) {
        self.0 += inc;
    }

    #[inline]
    pub fn get(&self) -> usize {
        self.0
    }

    #[inline]
    pub fn inc_and_get(&mut self) -> usize {
        self.0 += 1;
        self.0
    }

    #[inline]
    //noinspection RsSelfConvention
    pub fn get_and_inc(&mut self) -> usize {
        let tmp = self.0;
        self.0 += 1;
        tmp
    }
}
