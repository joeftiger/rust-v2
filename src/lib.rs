#![feature(float_interpolation)]
#![feature(total_cmp)]

use cgmath::{Basis3, Vector2, Vector3};

pub mod bxdf;
pub mod color;
pub mod geometry;
pub mod sampler;
pub mod scene;
pub mod sensor;
pub mod util;

#[cfg(feature = "spectral")]
pub type Spectrum = crate::color::Spectrum;
#[cfg(feature = "srgb")]
pub type Spectrum = crate::color::Spectrum;
#[cfg(feature = "xyz")]
pub type Spectrum = crate::color::Spectrum;

#[cfg(not(feature = "f64"))]
pub type Float = f32;
#[cfg(feature = "f64")]
pub type Float = f64;
pub type UVec2 = Vector2<u32>;
pub type Vec2 = Vector2<Float>;
pub type Vec3 = Vector3<Float>;
pub type Rot3 = Basis3<Float>;

pub const PACKET_SIZE: usize = 4;
pub const SENSOR_TILE_WIDTH: usize = 16;
