#![allow(clippy::excessive_precision)]

//! Borosilicate crown glass (known as BK7) coefficients.
//!
//! # Resources
//! * Data taken from [here](https://refractiveindex.info/?shelf=3d&book=glass&page=BK7) on
//! 2021-10-11.

use crate::Float;

/// Computes the refractive index of **glass** according to the Sellmeier equation.
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
    let one = 1.03961212 * l2 / (l2 - 0.00600069867);
    let two = 0.231792344 * l2 / (l2 - 0.0200179144);
    let three = 1.01046945 * l2 / (l2 - 103.560653);

    Float::sqrt(1.0 + one + two + three)
}

pub static INDEX_K: [Float; 25] = {
    [
        0.3, 0.31, 0.32, 0.334, 0.35, 0.365, 0.37, 0.38, 0.39, 0.4, 0.405, 0.42, 0.436, 0.46, 0.5,
        0.546, 0.58, 0.62, 0.66, 0.7, 1.06, 1.53, 1.97, 2.325, 2.5,
    ]
};
pub static K: [Float; 25] = {
    [
        0.0000028607,
        0.0000013679,
        6.6608e-7,
        2.6415e-7,
        9.2894e-8,
        3.4191e-8,
        2.7405e-8,
        2.074e-8,
        1.3731e-8,
        1.0227e-8,
        9.0558e-9,
        9.3912e-9,
        1.1147e-8,
        1.0286e-8,
        9.5781e-9,
        6.9658e-9,
        9.2541e-9,
        1.1877e-8,
        1.2643e-8,
        8.9305e-9,
        1.0137e-8,
        9.839e-8,
        0.0000010933,
        0.0000042911,
        0.00000813,
    ]
};
