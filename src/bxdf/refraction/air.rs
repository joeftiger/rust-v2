//! Air coefficients.
//!
//! # Resources
//! * Data taken from [here](https://refractiveindex.info/?shelf=other&book=air&page=Borzsonyi) on
//! 2021-02-21.
//! * Sellmeier data taken from the paper [Linear refractive index and absorption measurements of nonlinear optical liquids in the visible and near-infrared spectral region](https://d-nb.info/102770462X/34) on 2021-02-21.

use crate::Float;

/// Computes the refractive index of **air** according to the Sellmeier equation.
///
/// # Performance
/// Benchmarked to be the fastest algorithm to compute the refractive index for a specific wavelength.
///
/// # Constraints
/// * `lambda` - Should be finite (neither infinite nor `NaN`).
///
/// # Arguments
/// * `lambda` - The wavelength in **µm**
///
/// # Returns
/// * The refractive index
#[inline(always)]
pub fn sellmeier_n(lambda: Float) -> Float {
    debug_assert!(lambda.is_finite());

    let l2 = lambda * lambda;

    let one = 0.75831 * l2 / (l2 - 0.01007);
    let two = 0.08495 * l2 / (l2 - 8.91377);

    Float::sqrt(1.0 + one + two)
}
