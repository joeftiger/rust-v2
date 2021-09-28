use crate::color::{Spectrum, Srgb, Xyz};
use crate::Float;
use image::Rgb;

const fn bit_size_of<T>() -> usize {
    core::mem::size_of::<T>() * 8
}

macro_rules! conv {
    ($($origin:ident => $target:ident<$t:ident>),*) => {
        $(
            impl From<$origin> for $target<$t> {
                fn from(o: $origin) -> Self {
                    let mut srgb = Srgb::from(o);
                    srgb *= bit_size_of::<$t>() as Float;

                    Self::from([
                        srgb.data[0] as $t,
                        srgb.data[1] as $t,
                        srgb.data[2] as $t,
                    ])
                }
            }
        )*
    };
    (no-scale $($origin:ident => $target:ident<$t:ident>),*) => {
        $(
            impl From<$origin> for $target<$t> {
                fn from(o: $origin) -> Self {
                    let srgb = Srgb::from(o);

                    Self::from([
                        srgb.data[0] as $t,
                        srgb.data[1] as $t,
                        srgb.data[2] as $t,
                    ])
                }
            }
        )*
    }
}

conv!(
    Srgb => Rgb<u8>,
    Srgb => Rgb<u16>,
    Srgb => Rgb<u32>,
    Srgb => Rgb<u64>,
    Srgb => Rgb<usize>,

    Xyz => Rgb<u8>,
    Xyz => Rgb<u16>,
    Xyz => Rgb<u32>,
    Xyz => Rgb<u64>,
    Xyz => Rgb<usize>,

    Spectrum => Rgb<u8>,
    Spectrum => Rgb<u16>,
    Spectrum => Rgb<u32>,
    Spectrum => Rgb<u64>,
    Spectrum => Rgb<usize>
);

conv!(
    no-scale

    Srgb => Rgb<f32>,
    Srgb => Rgb<f64>,
    Xyz => Rgb<f32>,
    Xyz => Rgb<f64>,
    Spectrum => Rgb<f32>,
    Spectrum => Rgb<f64>
);
