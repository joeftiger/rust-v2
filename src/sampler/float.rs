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

#[derive(Copy, Clone, Debug)]
pub struct Sample {
    pub float: Float,
    pub vec2: Vec2,
}

impl Sample {
    pub const fn new(float: Float, vec2: Vec2) -> Self {
        Self { float, vec2 }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum FloatSampler {
    Constant(Float),
    Random,
}

impl FloatSampler {
    /// Generates a new random [Float] inside `[0, 1)`.
    #[inline]
    pub fn float(self) -> Float {
        match self {
            FloatSampler::Constant(c) => c,
            FloatSampler::Random => rand(),
        }
    }

    /// Generates a new random [Vec2] inside `[0, 1)`.
    #[inline]
    pub fn vec2(self) -> Vec2 {
        match self {
            FloatSampler::Constant(c) => Vec2::new(c, c),
            FloatSampler::Random => Vec2::new(rand(), rand()),
        }
    }

    /// Generates a new random [Sample] inside `[0, 1)`.
    #[inline]
    pub fn sample(self) -> Sample {
        match self {
            FloatSampler::Constant(c) => Sample::new(c, Vec2::new(c, c)),
            FloatSampler::Random => Sample::new(rand(), Vec2::new(rand(), rand())),
        }
    }
}
