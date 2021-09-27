//-------------------------------------------------------------
// Data taken from https://en.wikipedia.org/wiki/CIE_1931_color_space#Analytical_approximation

use crate::color::Xyz;
use crate::Float;

pub const CIE_Y_INTEGRAL: Float = 0.106856895;

#[inline]
pub fn lambda_to_xyz_approx(lambda: Float) -> Xyz {
    let lambda = micrometer_to_angstrom(lambda);
    Xyz::new([x_bar(lambda), y_bar(lambda), z_bar(lambda)])
}

/// A piecewise-Gaussian function.
#[inline]
pub fn gaussian(lambda: Float, alpha: Float, mu: Float, sigma1: Float, sigma2: Float) -> Float {
    let sigma = if lambda < mu { sigma1 } else { sigma2 };
    let t = (lambda - mu) / sigma;
    alpha * Float::exp(-(t * t) / 2.0)
}

#[inline]
fn x_bar(lambda: Float) -> Float {
    gaussian(lambda, 1.056, 5998.0, 379.0, 310.0)
        + gaussian(lambda, 0.362, 4420.0, 160.0, 267.0)
        + gaussian(lambda, -0.065, 5011.0, 204.0, 262.0)
}

#[inline]
fn y_bar(lambda: Float) -> Float {
    gaussian(lambda, 0.821, 5688.0, 469.0, 405.0) + gaussian(lambda, 0.286, 5309.0, 163.0, 311.0)
}

#[inline]
fn z_bar(lambda: Float) -> Float {
    gaussian(lambda, 1.217, 4370.0, 118.0, 360.0) + gaussian(lambda, 0.681, 4590.0, 260.0, 138.0)
}

#[inline]
fn micrometer_to_angstrom(lambda: Float) -> Float {
    10_000.0 * lambda
}
