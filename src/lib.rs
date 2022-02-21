#![feature(total_cmp)]

extern crate image;

use cgmath::{Basis3, Matrix4, Vector2, Vector3};
use image::{ImageBuffer, Pixel, Rgb};

pub mod bxdf;
pub mod camera;
pub mod color;
pub mod config;
pub mod geometry;
pub mod integrator;
pub mod renderer;
pub mod runtime;
pub mod sampler;
pub mod scene;
pub mod util;

#[cfg(feature = "spectral")]
pub type Spectrum = crate::color::Spectrum;
#[cfg(feature = "srgb")]
pub type Spectrum = crate::color::Srgb;
#[cfg(feature = "xyz")]
pub type Spectrum = crate::color::Xyz;

#[cfg(not(feature = "f64"))]
pub type Float = f32;
#[cfg(feature = "f64")]
pub type Float = f64;
pub type UVec2 = Vector2<u32>;
pub type Vec2 = Vector2<Float>;
pub type Vec3 = Vector3<Float>;
pub type Rot3 = Basis3<Float>;
pub type Mat4 = Matrix4<Float>;

pub type Image<T> = ImageBuffer<Rgb<T>, Vec<<Rgb<T> as Pixel>::Subpixel>>;

#[cfg(feature = "spectral")]
pub const PACKET_SIZE: usize = 4;
#[cfg(any(feature = "srgb", feature = "xyz"))]
pub const PACKET_SIZE: usize = Spectrum::size(); // DO NOT CHANGE
pub const SENSOR_TILE_WIDTH: u32 = 16;
