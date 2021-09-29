#![allow(clippy::excessive_precision)]

//! Sapphire coefficients.
//!
//! # Resources
//! * Data taken from [here](https://refractiveindex.info/?shelf=main&book=Al2O3&page=Querry-o) on
//! 2021-02-21.
//! * Sellmeier data taken from [here](https://en.wikipedia.org/wiki/Sellmeier_equation) on
//! 2021-02-21.

use crate::Float;

/// Computes the refractive index of **sapphire** according to the Sellmeier equation.
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
    let l2 = lambda * lambda;
    let one = 1.43134930 * l2 / (l2 - 5.2799261e-3);
    let two = 0.65054713 * l2 / (l2 - 1.42382647e-2);
    let three = 5.3414021 * l2 / (l2 - 325.017834);

    Float::sqrt(1.0 + one + two + three)
}

pub static INDEX_K: [Float; 612] = {
    [
        0.21, 0.22, 0.23, 0.24, 0.25, 0.26, 0.27, 0.28, 0.29, 0.3, 0.31, 0.32, 0.33, 0.34, 0.35,
        0.36, 0.37, 0.38, 0.39, 0.4, 0.41, 0.42, 0.43, 0.44, 0.45, 0.46, 0.47, 0.48, 0.49, 0.5,
        0.51, 0.52, 0.53, 0.54, 0.55, 0.56, 0.57, 0.58, 0.59, 0.6, 0.61, 0.62, 0.63, 0.64, 0.65,
        0.66, 0.67, 0.68, 0.69, 0.7, 0.71, 0.72, 0.73, 0.74, 0.75, 0.76, 0.77, 0.78, 0.79, 0.8,
        0.81, 0.82, 0.83, 0.84, 0.85, 0.86, 0.87, 0.88, 0.89, 0.9, 0.91, 0.92, 0.93, 0.94, 0.95,
        0.96, 0.97, 0.98, 0.99, 1.0, 1.01, 1.02, 1.03, 1.04, 1.05, 1.06, 1.07, 1.08, 1.09, 1.1,
        1.11, 1.12, 1.13, 1.14, 1.15, 1.16, 1.17, 1.18, 1.19, 1.2, 1.21, 1.22, 1.23, 1.24, 1.25,
        1.26, 1.27, 1.28, 1.29, 1.3, 1.31, 1.32, 1.33, 1.34, 1.35, 1.36, 1.37, 1.38, 1.39, 1.4,
        1.41, 1.42, 1.43, 1.44, 1.45, 1.46, 1.47, 1.48, 1.49, 1.5, 1.51, 1.52, 1.53, 1.54, 1.55,
        1.56, 1.57, 1.58, 1.59, 1.6, 1.61, 1.62, 1.63, 1.64, 1.65, 1.66, 1.67, 1.68, 1.69, 1.7,
        1.71, 1.72, 1.73, 1.74, 1.75, 1.76, 1.77, 1.78, 1.79, 1.8, 1.81, 1.82, 1.83, 1.84, 1.85,
        1.86, 1.87, 1.88, 1.89, 1.9, 1.91, 1.92, 1.93, 1.94, 1.95, 1.96, 1.97, 1.98, 1.99, 2.0,
        2.01, 2.02, 2.03, 2.04, 2.05, 2.06, 2.07, 2.08, 2.09, 2.1, 2.11, 2.12, 2.13, 2.14, 2.15,
        2.16, 2.17, 2.18, 2.19, 2.2, 2.21, 2.22, 2.23, 2.24, 2.25, 2.26, 2.27, 2.28, 2.29, 2.3,
        2.31, 2.32, 2.33, 2.34, 2.35, 2.36, 2.37, 2.38, 2.39, 2.4, 2.41, 2.42, 2.43, 2.44, 2.45,
        2.46, 2.47, 2.48, 2.49, 2.5, 2.5063, 2.5126, 2.5189, 2.5253, 2.5316, 2.5381, 2.5445, 2.551,
        2.5575, 2.5641, 2.5707, 2.5773, 2.584, 2.5907, 2.5974, 2.6042, 2.611, 2.6178, 2.6247,
        2.6316, 2.6385, 2.6455, 2.6525, 2.6596, 2.6667, 2.6738, 2.681, 2.6882, 2.6954, 2.7027,
        2.71, 2.7174, 2.7248, 2.7322, 2.7397, 2.7473, 2.7548, 2.7624, 2.7701, 2.7778, 2.7855,
        2.7933, 2.8011, 2.809, 2.8169, 2.8249, 2.8329, 2.8409, 2.849, 2.8571, 2.8653, 2.8736,
        2.8818, 2.8902, 2.8986, 2.907, 2.9155, 2.924, 2.9326, 2.9412, 2.9499, 2.9586, 2.9674,
        2.9762, 2.9851, 2.994, 3.003, 3.012, 3.0211, 3.0303, 3.0395, 3.0488, 3.0581, 3.0675,
        3.0769, 3.0864, 3.096, 3.1056, 3.1153, 3.125, 3.1348, 3.1447, 3.1546, 3.1646, 3.1746,
        3.1847, 3.1949, 3.2051, 3.2154, 3.2258, 3.2362, 3.2468, 3.2573, 3.268, 3.2787, 3.2895,
        3.3003, 3.3113, 3.3223, 3.3333, 3.3445, 3.3557, 3.367, 3.3784, 3.3898, 3.4014, 3.413,
        3.4247, 3.4364, 3.4483, 3.4602, 3.4722, 3.4843, 3.4965, 3.5088, 3.5211, 3.5336, 3.5461,
        3.5587, 3.5714, 3.5842, 3.5971, 3.6101, 3.6232, 3.6364, 3.6496, 3.663, 3.6765, 3.69,
        3.7037, 3.7175, 3.7313, 3.7453, 3.7594, 3.7736, 3.7879, 3.8023, 3.8168, 3.8314, 3.8462,
        3.861, 3.8911, 3.8976, 3.9063, 3.9216, 3.937, 3.9526, 3.9683, 3.9841, 4.0, 4.0161, 4.0323,
        4.0486, 4.065, 4.0816, 4.0984, 4.1152, 4.1322, 4.1494, 4.1667, 4.1841, 4.2017, 4.2194,
        4.2373, 4.2553, 4.2735, 4.2918, 4.3103, 4.329, 4.3478, 4.3668, 4.386, 4.4053, 4.4248,
        4.4444, 4.4643, 4.4843, 4.5045, 4.5249, 4.5455, 4.5662, 4.5872, 4.6083, 4.6296, 4.6512,
        4.6729, 4.6948, 4.717, 4.7393, 4.7619, 4.7847, 4.8077, 4.8309, 4.8544, 4.878, 4.902,
        4.9261, 4.9505, 4.9751, 5.0, 5.0251, 5.0505, 5.0761, 5.102, 5.1282, 5.1546, 5.1813, 5.2083,
        5.2356, 5.2632, 5.291, 5.3191, 5.3476, 5.3763, 5.4054, 5.4348, 5.4645, 5.4945, 5.5249,
        5.5556, 5.5866, 5.618, 5.6497, 5.6818, 5.7143, 5.7471, 5.7803, 5.814, 5.848, 5.8824,
        5.9172, 5.9524, 5.988, 6.0241, 6.0606, 6.0976, 6.135, 6.1728, 6.2112, 6.25, 6.2893, 6.3291,
        6.3694, 6.4103, 6.4516, 6.4935, 6.5359, 6.5789, 6.6225, 6.6667, 6.7114, 6.7568, 6.8027,
        6.8493, 6.8966, 6.9444, 6.993, 7.0423, 7.0922, 7.1429, 7.1942, 7.2464, 7.2993, 7.3529,
        7.4074, 7.4627, 7.5188, 7.5758, 7.6336, 7.6923, 7.7519, 7.8125, 7.874, 7.9365, 8.0, 8.0645,
        8.1301, 8.1967, 8.2645, 8.3333, 8.4034, 8.4746, 8.547, 8.6207, 8.6957, 8.7719, 8.8496,
        8.9286, 9.009, 9.0909, 9.1743, 9.2593, 9.3458, 9.434, 9.5238, 9.6154, 9.7087, 9.8039,
        9.901, 10.0, 10.101, 10.2041, 10.3093, 10.4167, 10.5263, 10.6383, 10.7527, 10.8696, 10.989,
        11.1111, 11.236, 11.3636, 11.4943, 11.6279, 11.7647, 11.9048, 12.0482, 12.1951, 12.3457,
        12.5, 12.6582, 12.8205, 12.987, 13.1579, 13.3333, 13.5135, 13.6986, 13.8889, 14.0845,
        14.2857, 14.4928, 14.7059, 14.9254, 15.1515, 15.3846, 15.625, 15.873, 16.129, 16.3934,
        16.6667, 16.9492, 17.2414, 17.5439, 17.8571, 18.1818, 18.5185, 18.8679, 19.2308, 19.6078,
        20.0, 20.4082, 20.8333, 21.2766, 21.7391, 22.2222, 22.7273, 23.2558, 23.8095, 24.3902,
        25.0, 25.641, 26.3158, 27.027, 27.7778, 28.5714, 29.4118, 30.303, 31.25, 32.2581, 33.3333,
        34.4828, 35.7143, 37.037, 38.4615, 40.0, 41.6667, 43.4783, 45.4545, 47.619, 50.0, 52.6316,
        55.5556,
    ]
};
pub static K: [Float; 612] = {
    [
        -0.052, -0.039, -0.026, -0.019, -0.016, -0.01, -0.004, -0.001, 0.001, 0.007, 0.01, 0.012,
        0.013, 0.014, 0.015, 0.017, 0.018, 0.018, 0.019, 0.019, 0.019, 0.019, 0.02, 0.021, 0.021,
        0.021, 0.021, 0.021, 0.021, 0.021, 0.021, 0.021, 0.02, 0.021, 0.021, 0.02, 0.021, 0.02,
        0.02, 0.02, 0.02, 0.021, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.021, 0.02, 0.02, 0.02, 0.02,
        0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02,
        0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02,
        0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02,
        0.02, 0.021, 0.021, 0.02, 0.021, 0.02, 0.02, 0.02, 0.019, 0.019, 0.019, 0.019, 0.019,
        0.019, 0.019, 0.019, 0.019, 0.02, 0.019, 0.02, 0.02, 0.02, 0.02, 0.019, 0.019, 0.019, 0.02,
        0.019, 0.019, 0.018, 0.019, 0.019, 0.018, 0.018, 0.018, 0.019, 0.018, 0.018, 0.018, 0.018,
        0.016, 0.018, 0.018, 0.018, 0.018, 0.018, 0.019, 0.019, 0.019, 0.019, 0.019, 0.019, 0.019,
        0.019, 0.019, 0.018, 0.019, 0.019, 0.018, 0.018, 0.017, 0.017, 0.017, 0.018, 0.017, 0.017,
        0.017, 0.018, 0.019, 0.02, 0.02, 0.02, 0.019, 0.019, 0.019, 0.019, 0.019, 0.019, 0.018,
        0.018, 0.018, 0.018, 0.018, 0.018, 0.018, 0.018, 0.019, 0.019, 0.019, 0.019, 0.019, 0.018,
        0.018, 0.018, 0.019, 0.019, 0.019, 0.019, 0.02, 0.019, 0.019, 0.019, 0.02, 0.02, 0.021,
        0.02, 0.02, 0.021, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.021,
        0.02, 0.021, 0.02, 0.02, 0.02, 0.019, 0.02, 0.019, 0.019, 0.019, 0.018, 0.018, 0.019,
        0.019, 0.019, 0.019, 0.018, 0.019, 0.019, 0.019, 0.019, 0.019, 0.019, 0.019, 0.02, 0.019,
        0.019, 0.019, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02,
        0.019, 0.019, 0.019, 0.02, 0.021, 0.021, 0.021, 0.021, 0.021, 0.021, 0.021, 0.021, 0.02,
        0.02, 0.021, 0.021, 0.021, 0.021, 0.02, 0.02, 0.021, 0.021, 0.02, 0.02, 0.02, 0.02, 0.02,
        0.02, 0.02, 0.02, 0.02, 0.021, 0.021, 0.021, 0.02, 0.02, 0.02, 0.021, 0.02, 0.02, 0.019,
        0.019, 0.02, 0.02, 0.019, 0.019, 0.019, 0.02, 0.019, 0.019, 0.019, 0.02, 0.02, 0.02, 0.019,
        0.019, 0.019, 0.02, 0.021, 0.02, 0.02, 0.02, 0.02, 0.021, 0.02, 0.02, 0.02, 0.02, 0.021,
        0.021, 0.021, 0.02, 0.02, 0.021, 0.021, 0.021, 0.021, 0.021, 0.022, 0.023, 0.023, 0.022,
        0.022, 0.022, 0.022, 0.022, 0.022, 0.021, 0.021, 0.021, 0.021, 0.021, 0.02, 0.02, 0.02,
        0.02, 0.02, 0.019, 0.019, 0.019, 0.02, 0.02, 0.02, 0.019, 0.019, 0.021, 0.021, 0.02, 0.02,
        0.02, 0.021, 0.021, 0.021, 0.021, 0.02, 0.02, 0.021, 0.021, 0.021, 0.02, 0.021, 0.022,
        0.023, 0.023, 0.022, 0.022, 0.023, 0.024, 0.024, 0.023, 0.023, 0.024, 0.024, 0.024, 0.024,
        0.023, 0.023, 0.024, 0.024, 0.023, 0.023, 0.023, 0.024, 0.025, 0.025, 0.025, 0.025, 0.026,
        0.026, 0.026, 0.026, 0.026, 0.027, 0.028, 0.028, 0.028, 0.028, 0.029, 0.029, 0.03, 0.03,
        0.029, 0.029, 0.03, 0.031, 0.031, 0.031, 0.031, 0.031, 0.031, 0.032, 0.032, 0.031, 0.031,
        0.032, 0.032, 0.032, 0.031, 0.03, 0.031, 0.031, 0.031, 0.03, 0.03, 0.03, 0.031, 0.031,
        0.031, 0.03, 0.03, 0.031, 0.032, 0.032, 0.031, 0.03, 0.031, 0.031, 0.032, 0.032, 0.032,
        0.032, 0.033, 0.034, 0.034, 0.034, 0.034, 0.035, 0.036, 0.036, 0.036, 0.035, 0.036, 0.037,
        0.038, 0.037, 0.037, 0.038, 0.039, 0.039, 0.039, 0.039, 0.039, 0.04, 0.041, 0.041, 0.041,
        0.041, 0.042, 0.044, 0.045, 0.045, 0.044, 0.045, 0.046, 0.047, 0.047, 0.047, 0.047, 0.048,
        0.048, 0.048, 0.047, 0.045, 0.044, 0.055, 0.056, 0.051, 0.054, 0.053, 0.057, 0.055, 0.053,
        0.054, 0.058, 0.068, 0.067, 0.062, 0.061, 0.063, 0.074, 0.08, 0.073, 0.063, 0.064, 0.074,
        0.089, 0.091, 0.088, 0.089, 0.088, 0.094, 0.097, 0.103, 0.108, 0.112, 0.12, 0.13, 0.139,
        0.166, 0.239, 0.355, 0.484, 0.608, 0.719, 0.822, 0.915, 1.002, 1.088, 1.176, 1.266, 1.356,
        1.439, 1.524, 1.618, 1.721, 1.833, 1.948, 2.064, 2.178, 2.311, 2.466, 2.645, 2.838, 3.049,
        3.326, 3.683, 4.25, 3.46, 3.626, 4.26, 4.919, 5.835, 6.996, 8.491, 3.839, 1.299, 0.642,
        0.42, 0.48, 0.662, 0.591, 0.573, 1.22, 2.824, 4.73, 7.373, 11.173, 1.231, 0.312, 0.108,
        0.066, 0.606, 1.312, 0.108, -0.069, -0.089, -0.043, 0.01, 0.046, 0.019, 0.013, 0.077,
        0.077, 0.067, 0.048, 0.028, 0.013, 0.021, 0.028, 0.028, 0.018, 0.009, 0.011,
    ]
};
