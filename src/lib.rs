#![feature(float_interpolation)]
#![feature(total_cmp)]

use cgmath::{Basis3, Vector3};

pub mod color;
pub mod geometry;
pub mod util;
pub mod bxdf;

pub type Float = f64;
pub type Vec3 = Vector3<Float>;
pub type Rot3 = Basis3<Float>;
