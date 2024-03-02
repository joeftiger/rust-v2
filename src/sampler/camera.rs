use crate::{Float, Vec2};
use serde::{Deserialize, Serialize};

#[cfg(not(feature = "f64"))]
fn rand() -> f32 {
    fastrand::f32()
}
#[cfg(feature = "f64")]
fn rand() -> f64 {
    fastrand::f64()
}

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
