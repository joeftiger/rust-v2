use core::convert::TryFrom;

use crate::Float;
use color_data::LAMBDA_NUM;
use serde_big_array::BigArray;
pub use spectrum::*;
pub use srgb::*;
pub use xyz::*;

pub mod cie;
pub mod color_data;
pub mod image_conv;
pub mod spectrum;
pub mod srgb;
pub mod xyz;

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum ColorSerde {
    Srgb([Float; 3]),
    Xyz([Float; 3]),
    #[serde(with = "BigArray")]
    Spectrum([Float; 36]),
    Color(Color),
    MulColor(Float, Color),
    Constant(Float),
}

/// A light wave is described by a wavelength (lambda) in `μm` and an intensity (associated with amplitude).
#[derive(Copy, Clone, Default)]
pub struct LightWave {
    /// The wavelength in `μm`.
    pub lambda: Float,
    /// The intensity of the light wave.
    pub intensity: Float,
}

#[macro_export]
macro_rules! color {
    ($name:ident => $t:ident, $size:expr, $path:ident $(::$path2:ident)*) => {
        use core::ops::{Index, IndexMut, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
        use core::slice::SliceIndex;
        use core::iter::Sum;

        use $crate::color::*;
        use $crate::util::floats;
        use $crate::util::math::Lerp;
        use serde;
        use serde::de::Error;

        #[derive(Clone, Copy, Debug, PartialEq)]
        pub struct $name {
            pub data: [$t; $size],
        }

        impl serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
                S: serde::Serializer {
                for c in Color::variants() {
                    if self.eq(&c) {
                        return ColorSerde::Color(c).serialize(serializer);
                    }
                }

                let first = self.data[0];
                #[allow(clippy::float_cmp)]
                if self.data.iter().skip(1).all(|v| *v == first) {
                    return ColorSerde::Constant(first).serialize(serializer);
                }

                ColorSerde::$name(self.data).serialize(serializer)
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
                D: serde::Deserializer<'de> {
                let c = ColorSerde::deserialize(deserializer)?;
                Self::try_from(c.clone()).map_err(|_| D::Error::custom(format!("Unable to parse {} from {:?}", core::any::type_name::<$name>(), c)))
            }
        }

        impl $name {
            /// Creates a new color ([Srgb], [Xyz] or [Spectrum]).
            ///
            /// # Example
            /// ```rust
            /// use rust_v2::color::Srgb;
            ///
            /// let color = Srgb::new([0.25, 0.5, 0.75]);
            /// assert_eq!([0.25, 0.5, 0.75], color[..]);
            /// ```
            pub const fn new(data: [$t; $size]) -> Self {
                Self { data }
            }

            /// Creates a new color ([Srgb], [Xyz] or [Spectrum]) with a splatted value.
            ///
            /// # Example
            /// ```rust
            /// use rust_v2::color::Srgb;
            ///
            /// let white = Srgb::splat(1.0);
            /// assert_eq!([1.0, 1.0, 1.0], white[..]);
            /// ```
            pub const fn splat(value: $t) -> Self {
                Self::new([value; $size])
            }

            /// Returns the size of the color ([Srgb], [Xyz] or [Spectrum]).
            ///
            /// # Example
            /// ```rust
            /// use rust_v2::color::{Srgb, Xyz, Spectrum};
            ///
            /// assert_eq!(3, Srgb::size());
            /// assert_eq!(3, Xyz::size());
            /// assert_eq!(36, Spectrum::size());
            /// ```
            #[inline]
            pub const fn size() -> usize {
                $size
            }

            /// Returns the sum of all values.
            pub fn sum_values(&self) -> $t {
                self.data.iter().sum()
            }

            /// Returns whether this color is black (all values `0`).
            ///
            /// # Example
            /// ```rust
            /// use rust_v2::color::Srgb;
            ///
            /// let black = Srgb::splat(0.0);
            /// assert!(black.is_black());
            ///
            /// let red = Srgb::new([1.0, 0.0, 0.0]);
            /// assert!(!red.is_black());
            /// ```
            #[inline]
            pub fn is_black(&self) -> bool {
                self.data.iter().all(|value| *value == 0.0)
            }

            /// Clamps this color.
            ///
            /// # Example
            /// ```rust
            /// use rust_v2::color::Srgb;
            ///
            /// let mut color = Srgb::new([0.0, 0.5, 1.0]);
            /// color.clamp(0.25, 0.75);
            ///
            /// assert_eq!([0.25, 0.5, 0.75], color[..]);
            /// ```
            pub fn clamp(&mut self, min: $t, max: $t) {
                for d in &mut self.data {
                    *d = d.clamp(min, max);
                }
            }

            /// Clamps this color and returns it.
            ///
            /// # Example
            /// ```rust
            /// use rust_v2::color::Srgb;
            ///
            /// let color = Srgb::new([0.0, 0.5, 1.0]);
            /// let clamped = color.clamped(0.25, 0.75);
            ///
            /// assert_eq!([0.25, 0.5, 0.75], clamped[..]);
            /// ```
            #[must_use]
            pub fn clamped(&self, min: $t, max: $t) -> Self {
                let mut new = self.clone();
                new.clamp(min, max);
                new
            }

            /// Clamps this color between two others.
            pub fn clamp2(&mut self, min: Self, max: Self) {
                for i in 0..$size {
                    self[i] = self.data[i].clamp(min[i], max[i]);
                }
            }

            /// Clamps this color between two others and returns it
            #[must_use]
            pub fn clamped2(&self, min: Self, max: Self) -> Self {
                let mut new = self.clone();
                new.clamp2(min, max);
                new
            }

            pub fn sqrt(&mut self) {
                for d in &mut self.data {
                    *d = d.sqrt();
                }
            }

            #[must_use]
            pub fn sqrted(&self) -> Self {
                let mut new = self.clone();
                new.sqrt();
                new
            }

            pub fn lerp(&mut self, other: &Self, t: $t) {
                for i in 0..$size {
                    self[i] = t.lerp(self[i], other[i]);
                }
            }

            #[must_use]
            pub fn lerped(&self, other: &Self, t: $t) -> Self {
                let mut new = self.clone();
                new.lerp(other, t);
                new
            }

            pub fn min_value(&self) -> $t {
                let mut min = $t::MAX;
                for d in self.data {
                    if d < min {
                        min = d;
                    }
                }

                min
            }

            pub fn max_value(&self) -> $t {
                let mut max = $t::MIN;
                for d in self.data {
                    if d > max {
                        max = d;
                    }
                }

                max
            }

            #[inline]
            pub const fn lambda(index: usize) -> $t {
                $path $(::$path2)* ::LAMBDAS[index]
                // let t = index as $t / ($size - 1) as $t;
                // t.lerp(LAMBDA_START, LAMBDA_END)
            }

            #[inline]
            pub fn as_light_wave(&self, light_wave_index: usize) -> LightWave {
                let lambda = Self::lambda(light_wave_index);
                let intensity = self[light_wave_index];

                LightWave {
                    lambda,
                    intensity,
                }
            }

            pub fn as_light_waves(&self) -> [LightWave; $size] {
                let mut light_waves = [LightWave::default(); $size];

                for i in 0..$size {
                    light_waves[i].lambda = Self::lambda(i);
                    light_waves[i].intensity = self[i]
                }

                light_waves
            }

            pub fn approx_eq(&self, other: &Self) -> bool {
                self.data
                    .iter()
                    .zip(other.data.iter())
                    .all(|(&d0, &d1)| floats::approx_eq(d0, d1))
            }
        }

        impl From<Color> for $name {
            #[rustfmt::skip]
            fn from(c: Color) -> Self {
                let data = match c {
                    Color::DarkSkin     => $path $(::$path2)* ::DARK_SKIN,
                    Color::LightSkin    => $path $(::$path2)* ::LIGHT_SKIN,
                    Color::BlueSky      => $path $(::$path2)* ::BLUE_SKY,
                    Color::Foliage      => $path $(::$path2)* ::FOLIAGE,
                    Color::BlueFlower   => $path $(::$path2)* ::BLUE_FLOWER,
                    Color::BluishGreen  => $path $(::$path2)* ::BLUISH_GREEN,
                    Color::Orange       => $path $(::$path2)* ::ORANGE,
                    Color::PurplishBlue => $path $(::$path2)* ::PURPLISH_BLUE,
                    Color::ModerateRed  => $path $(::$path2)* ::MODERATE_RED,
                    Color::Purple       => $path $(::$path2)* ::PURPLE,
                    Color::YellowGreen  => $path $(::$path2)* ::YELLOW_GREEN,
                    Color::OrangeYellow => $path $(::$path2)* ::ORANGE_YELLOW,
                    Color::Blue         => $path $(::$path2)* ::BLUE,
                    Color::Green        => $path $(::$path2)* ::GREEN,
                    Color::Red          => $path $(::$path2)* ::RED,
                    Color::Yellow       => $path $(::$path2)* ::YELLOW,
                    Color::Magenta      => $path $(::$path2)* ::MAGENTA,
                    Color::Cyan         => $path $(::$path2)* ::CYAN,
                    Color::White        => $path $(::$path2)* ::WHITE,
                    Color::Grey1        => $path $(::$path2)* ::GREY_1,
                    Color::Grey2        => $path $(::$path2)* ::GREY_2,
                    Color::Grey3        => $path $(::$path2)* ::GREY_3,
                    Color::Grey4        => $path $(::$path2)* ::GREY_4,
                    Color::Black        => $path $(::$path2)* ::BLACK,
                };

                Self::new(data)
            }
        }
        impl From<&Color> for $name {
            #[inline]
            fn from(c: &Color) -> Self {
                Self::from(*c)
            }
        }

        impl Default for $name {
            fn default() -> Self {
                let data = [$t::default(); $size];
                Self::new(data)
            }
        }

        impl Add for $name {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                let mut data = self.data;
                for i in 0..data.len() {
                    data[i] += rhs.data[i];
                }

                Self::new(data)
            }
        }
        impl AddAssign for $name {
            fn add_assign(&mut self, rhs: Self) {
                for i in 0..self.data.len() {
                    self.data[i] += rhs.data[i];
                }
            }
        }

        impl Add<$t> for $name {
            type Output = Self;

            fn add(self, rhs: $t) -> Self::Output {
                let mut data = self.data;
                for d in &mut data {
                    *d += rhs;
                }

                Self::new(data)
            }
        }
        impl AddAssign<$t> for $name {
            fn add_assign(&mut self, rhs: $t) {
                for d in &mut self.data {
                    *d += rhs;
                }
            }
        }

        impl Sub for $name {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                let mut data = self.data;
                for i in 0..data.len() {
                    data[i] -= rhs.data[i];
                }

                Self::new(data)
            }
        }
        impl SubAssign for $name {
            fn sub_assign(&mut self, rhs: Self) {
                for i in 0..self.data.len() {
                    self.data[i] -= rhs.data[i];
                }
            }
        }

        impl Sub<$t> for $name {
            type Output = Self;

            fn sub(self, rhs: $t) -> Self::Output {
                let mut data = self.data;
                for d in &mut data {
                    *d -= rhs;
                }

                Self::new(data)
            }
        }
        impl SubAssign<$t> for $name {
            fn sub_assign(&mut self, rhs: $t) {
                for d in &mut self.data {
                    *d -= rhs;
                }
            }
        }

        impl Mul for $name {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self::Output {
                let mut data = self.data;
                for i in 0..data.len() {
                    data[i] *= rhs.data[i];
                }

                Self::new(data)
            }
        }
        impl MulAssign for $name {
            fn mul_assign(&mut self, rhs: Self) {
                for i in 0..self.data.len() {
                    self.data[i] *= rhs.data[i];
                }
            }
        }

        impl Mul<$t> for $name {
            type Output = Self;

            fn mul(self, rhs: $t) -> Self::Output {
                let mut data = self.data;
                for d in &mut data {
                    *d *= rhs;
                }

                Self::new(data)
            }
        }
        impl MulAssign<$t> for $name {
            fn mul_assign(&mut self, rhs: $t) {
                for d in &mut self.data {
                    *d *= rhs;
                }
            }
        }

        impl Mul<$name> for $t {
            type Output = $name;

            fn mul(self, rhs: $name) -> Self::Output {
                let mut data = rhs.data;
                for d in &mut data {
                    *d *= self;
                }

                $name::new(data)
            }
        }

        impl Div for $name {
            type Output = Self;

            fn div(self, rhs: Self) -> Self::Output {
                let mut data = self.data;
                for i in 0..data.len() {
                    data[i] /= rhs.data[i];
                }

                Self::new(data)
            }
        }
        impl DivAssign for $name {
            fn div_assign(&mut self, rhs: Self) {
                for i in 0..self.data.len() {
                    self.data[i] /= rhs.data[i];
                }
            }
        }

        impl Div<$t> for $name {
            type Output = Self;

            fn div(self, rhs: $t) -> Self::Output {
                let mut data = self.data;
                for d in &mut data {
                    *d /= rhs;
                }

                Self::new(data)
            }
        }
        impl DivAssign<$t> for $name {
            fn div_assign(&mut self, rhs: $t) {
                for d in &mut self.data {
                    *d /= rhs;
                }
            }
        }

        impl Div<$name> for $t {
            type Output = $name;

            fn div(self, rhs: $name) -> Self::Output {
                let mut data = rhs.data;
                for d in &mut data {
                    *d /= self;
                }

                $name::new(data)
            }
        }

        impl<Idx: SliceIndex<[$t]>> Index<Idx> for $name {
            type Output = Idx::Output;

            fn index(&self, index: Idx) -> &Self::Output {
                &self.data[index]
            }
        }
        impl<Idx: SliceIndex<[$t]>> IndexMut<Idx> for $name {
            fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
                &mut self.data[index]
            }
        }

        impl PartialEq<Color> for $name {
            fn eq(&self, c: &Color) -> bool {
                let color = Self::from(c);
                self == &color
            }
        }

        impl Sum for $name {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold($name::default(), |a, b| a + b)
            }
        }
    };
}

/// Describes colors.
#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum Color {
    DarkSkin,
    LightSkin,
    BlueSky,
    Foliage,
    BlueFlower,
    BluishGreen,
    Orange,
    PurplishBlue,
    ModerateRed,
    Purple,
    YellowGreen,
    OrangeYellow,
    Blue,
    Green,
    Red,
    Yellow,
    Magenta,
    Cyan,
    White,
    Grey1,
    Grey2,
    Grey3,
    Grey4,
    Black,
}

impl Color {
    pub fn variants() -> [Self; 24] {
        [
            Self::DarkSkin,
            Self::LightSkin,
            Self::BlueSky,
            Self::Foliage,
            Self::BlueFlower,
            Self::BluishGreen,
            Self::Orange,
            Self::PurplishBlue,
            Self::ModerateRed,
            Self::Purple,
            Self::YellowGreen,
            Self::OrangeYellow,
            Self::Blue,
            Self::Green,
            Self::Red,
            Self::Yellow,
            Self::Magenta,
            Self::Cyan,
            Self::White,
            Self::Grey1,
            Self::Grey2,
            Self::Grey3,
            Self::Grey4,
            Self::Black,
        ]
    }
}

impl TryFrom<&str> for Color {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let c = match value {
            "DarkSkin" => Self::DarkSkin,
            "LightSkin" => Self::LightSkin,
            "BlueSky" => Self::BlueSky,
            "Foliage" => Self::Foliage,
            "BlueFlower" => Self::BlueFlower,
            "BluishGreen" => Self::BluishGreen,
            "Orange" => Self::Orange,
            "PurplishBlue" => Self::PurplishBlue,
            "ModerateRed" => Self::ModerateRed,
            "Purple" => Self::Purple,
            "YellowGreen" => Self::YellowGreen,
            "OrangeYellow" => Self::OrangeYellow,
            "Blue" => Self::Blue,
            "Green" => Self::Green,
            "Red" => Self::Red,
            "Yellow" => Self::Yellow,
            "Magenta" => Self::Magenta,
            "Cyan" => Self::Cyan,
            "White" => Self::White,
            "Grey1" => Self::Grey1,
            "Grey2" => Self::Grey2,
            "Grey3" => Self::Grey3,
            "Grey4" => Self::Grey4,
            "Black" => Self::Black,
            _ => return Err(format!("Unable to parse Color: {}", value)),
        };

        Ok(c)
    }
}
