use crate::{Float, Vec2};
use serde::{Deserialize, Serialize};

thread_local! {
    static RNG: fastrand::Rng = fastrand::Rng::with_seed(0);
}

#[cfg(not(feature = "f64"))]
fn rand() -> f32 {
    RNG.with(|rng| rng.f32())
}
#[cfg(feature = "f64")]
fn rand() -> f64 {
    RNG.with(|rng| rng.f64())
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
