//! Air coefficients.
//!
//! # Resources
//! * Data taken from [here](https://refractiveindex.info/?shelf=other&book=air&page=Borzsonyi) on
//! 2021-10-11.
use crate::Float;

#[inline(always)]
pub fn sellmeier_n(lambda: Float) -> Float {
    debug_assert!(lambda.is_finite());

    let l2 = lambda * lambda;

    let one = 14926.44e-8 * l2 / (l2 - 19.36e-6);
    let two = 41807.57e-8 * l2 / (l2 - 7.434e-3);

    Float::sqrt(1.0 + one + two)
}
