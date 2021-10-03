use crate::{Float, Vec2};
use serde::{Deserialize, Serialize};

#[cfg(not(feature = "f64"))]
use fastrand::f32 as rand;
#[cfg(feature = "f64")]
use fastrand::f64 as rand;

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum CameraSampler {
    Const(Float, Float),
    Random,
}

impl CameraSampler {
    #[inline]
    pub fn sample(self) -> Vec2 {
        match self {
            CameraSampler::Const(x, y) => Vec2::new(x, y),
            CameraSampler::Random => Vec2::new(rand(), rand()),
        }
    }
}
