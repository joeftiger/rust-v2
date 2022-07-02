#![allow(clippy::excessive_precision)]
//-------------------------------------------------------------
// Data taken from https://en.wikipedia.org/wiki/CIE_1931_color_space#Analytical_approximation

use crate::color::Xyz;
use crate::color::color_data::LAMBDA_NUM;
use crate::Float;

pub const CIE_Y_INTEGRAL: Float = 0.106856895;

pub const CIE_X_BAR: [Float; LAMBDA_NUM] = [0.000159952, 0.0023616, 0.0191097, 0.084736, 0.204492, 0.314679, 0.383734, 0.370702, 0.302273, 0.195618, 0.080507, 0.016172, 0.003816, 0.037465, 0.117749, 0.236491, 0.376772, 0.529826, 0.705224, 0.878655, 1.01416, 1.11852, 1.12399, 1.03048, 0.856297, 0.647467, 0.431567, 0.268329, 0.152568, 0.0812606, 0.0408508, 0.0199413, 0.00957688, 0.00455263, 0.00217496, 0.00104476];
pub const CIE_Y_BAR: [Float; LAMBDA_NUM] = [1.7364e-5, 0.0002534, 0.0020044, 0.008756, 0.021391, 0.038676, 0.062077, 0.089456, 0.128201, 0.18519, 0.253589, 0.339133, 0.460777, 0.606741, 0.761757, 0.875211, 0.961988, 0.991761, 0.99734, 0.955552, 0.868934, 0.777405, 0.658341, 0.527963, 0.398057, 0.283493, 0.179828, 0.107633, 0.060281, 0.0318004, 0.0159051, 0.0077488, 0.00371774, 0.00176847, 0.00084619, 0.00040741];
pub const CIE_Z_BAR: [Float; LAMBDA_NUM] = [0.000704776, 0.0104822, 0.0860109, 0.389366, 0.972542, 1.55348, 1.96728, 1.9948, 1.74537, 1.31756, 0.772125, 0.415254, 0.218502, 0.112044, 0.060709, 0.030451, 0.013676, 0.003988, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];

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
