#![allow(clippy::excessive_precision)]
//-------------------------------------------------------------
// Data taken from https://en.wikipedia.org/wiki/CIE_1931_color_space#Analytical_approximation

use crate::color::Xyz;
use crate::Float;

pub const CIE_Y_INTEGRAL: Float = 0.106856895;

#[inline]
pub fn lambda_to_xyz_approx(lambda: Float) -> Xyz {
    let lambda = micrometer_to_nanometer(lambda);
    Xyz::new([x_bar(lambda), y_bar(lambda), z_bar(lambda)])
}

/// A piecewise-Gaussian function.
#[inline]
pub fn gaussian(lambda: Float, alpha: Float, mu: Float, sigma1: Float, sigma2: Float) -> Float {
    let sigma = if lambda < mu { sigma1 } else { sigma2 };
    let t = (lambda - mu) / sigma;
    alpha * Float::exp(-0.5 * t * t)
}

#[inline]
fn x_bar(lambda: Float) -> Float {
    gaussian(lambda, 1.056, 599.8, 37.9, 31.0)
        + gaussian(lambda, 0.362, 442.0, 16.0, 26.7)
        + gaussian(lambda, -0.065, 501.1, 20.4, 26.2)
}

#[inline]
fn y_bar(lambda: Float) -> Float {
    gaussian(lambda, 0.821, 568.8, 46.9, 40.5) + gaussian(lambda, 0.286, 530.9, 16.3, 31.1)
}

#[inline]
fn z_bar(lambda: Float) -> Float {
    gaussian(lambda, 1.217, 437.0, 11.8, 36.0) + gaussian(lambda, 0.681, 459.0, 26.0, 13.8)
}

#[inline]
fn micrometer_to_nanometer(lambda: Float) -> Float {
    1_000.0 * lambda
}
