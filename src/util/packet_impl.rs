use crate::{Float, PACKET_SIZE};

/// A utility trait allowing implementation of add, sub, mul, div, etc. for `[Float; `[PACKET_SIZE]`]`.
pub trait PacketOps<T> {
    #[must_use]
    fn add(self, rhs: Self) -> Self;
    fn add_assign(&mut self, rhs: Self);
    #[must_use]
    fn sub(self, rhs: Self) -> Self;
    fn sub_assign(&mut self, rhs: Self);
    #[must_use]
    fn mul(self, rhs: Self) -> Self;
    fn mul_assign(&mut self, rhs: Self);
    #[must_use]
    fn mul_t(self, rhs: T) -> Self;
    fn mul_assign_t(&mut self, rhs: T);
    #[must_use]
    fn div(self, rhs: Self) -> Self;
    fn div_assign(&mut self, rhs: Self);
    #[must_use]
    fn div_t(self, rhs: T) -> Self;
    fn div_assign_t(&mut self, rhs: T);
    #[must_use]
    fn neg(self) -> Self;
    fn neg_assign(&mut self);
    #[must_use]
    fn is_black(&self) -> bool;
}

#[allow(clippy::needless_range_loop)]
impl PacketOps<Float> for [Float; PACKET_SIZE] {
    #[inline]
    fn add(mut self, rhs: Self) -> Self {
        self.add_assign(rhs);
        self
    }

    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..PACKET_SIZE {
            self[i] += rhs[i];
        }
    }

    #[inline]
    fn sub(mut self, rhs: Self) -> Self {
        self.sub_assign(rhs);
        self
    }

    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..PACKET_SIZE {
            self[i] -= rhs[i];
        }
    }

    #[inline]
    fn mul(mut self, rhs: Self) -> Self {
        self.mul_assign(rhs);
        self
    }

    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        for i in 0..PACKET_SIZE {
            self[i] *= rhs[i];
        }
    }

    #[inline]
    fn mul_t(mut self, rhs: Float) -> Self {
        self.mul_assign_t(rhs);
        self
    }

    #[inline]
    fn mul_assign_t(&mut self, rhs: Float) {
        for i in 0..PACKET_SIZE {
            self[i] *= rhs;
        }
    }

    #[inline]
    fn div(mut self, rhs: Self) -> Self {
        self.div_assign(rhs);
        self
    }

    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        for i in 0..PACKET_SIZE {
            self[i] /= rhs[i];
        }
    }

    #[inline]
    fn div_t(mut self, rhs: Float) -> Self {
        self.div_assign_t(rhs);
        self
    }

    #[inline]
    fn div_assign_t(&mut self, rhs: Float) {
        for i in 0..PACKET_SIZE {
            self[i] /= rhs;
        }
    }

    #[inline]
    fn neg(mut self) -> Self {
        self.neg_assign();
        self
    }

    #[inline]
    fn neg_assign(&mut self) {
        for i in 0..PACKET_SIZE {
            self[i] = -self[i];
        }
    }

    #[inline]
    fn is_black(&self) -> bool {
        self.iter().all(|&f| f == 0.0)
    }
}
