use crate::color::{Spectrum, Srgb, Xyz};
use crate::Float;
use image::Rgb;

const fn bit_size_of<T>() -> usize {
    core::mem::size_of::<T>() * 8
}

macro_rules! srgb_conv {
    (Srgb => $($t:ident),*) => {
        $(
            impl From<Srgb> for Rgb<$t> {
                fn from(mut srgb: Srgb) -> Self {
                    srgb *= 2u64.pow(bit_size_of::<$t>() as u32) as Float;

                    // IMPORTANT: note the inversion as our Srgb ranks lambda first => blue first
                    Self::from([
                        srgb.data[2] as $t,
                        srgb.data[1] as $t,
                        srgb.data[0] as $t,
                    ])
                }
            }
        )*
    };
    (Srgb no-scale => $($t:ident),*) => {
        $(
            impl From<Srgb> for Rgb<$t> {
                fn from(srgb: Srgb) -> Self {
                    // IMPORTANT: note the inversion as our Srgb ranks lambda first => blue first
                    Self::from([
                        srgb.data[2] as $t,
                        srgb.data[1] as $t,
                        srgb.data[0] as $t,
                    ])
                }
            }
        )*
    }
}
srgb_conv!(Srgb => u8, u16, u32, u64, usize);
srgb_conv!(Srgb no-scale => f32, f64);

macro_rules! xyz_conv {
    (Xyz => $($t:ident),*) => {
        $(
            impl From<Xyz> for Rgb<$t> {
                fn from(xyz: Xyz) -> Self {
                    let mut srgb = Srgb::from(xyz);
                    srgb *= 2u64.pow(bit_size_of::<$t>() as u32) as Float;

                    // IMPORTANT: note the inversion as our Xyz ranks lambda first => blue first
                    Self::from([
                        srgb.data[2] as $t,
                        srgb.data[1] as $t,
                        srgb.data[0] as $t,
                    ])
                }
            }
        )*
    };
    (Xyz no-scale => $($t:ident),*) => {
        $(
            impl From<Xyz> for Rgb<$t> {
                fn from(xyz: Xyz) -> Self {
                    let srgb = Srgb::from(xyz);

                    // IMPORTANT: note the inversion as our Xyz ranks lambda first => blue first
                    Self::from([
                        srgb.data[2] as $t,
                        srgb.data[1] as $t,
                        srgb.data[0] as $t,
                    ])
                }
            }
        )*
    }
}
xyz_conv!(Xyz => u8, u16, u32, u64, usize);
xyz_conv!(Xyz no-scale => f32, f64);

macro_rules! spectral_conv {
    (Spectrum => $($t:ident),*) => {
        $(
            impl From<Spectrum> for Rgb<$t> {
                fn from(o: Spectrum) -> Self {
                    let mut srgb = Srgb::from(o);
                    srgb *= 2u64.pow(bit_size_of::<$t>() as u32) as Float;

                    Self::from([
                        srgb.data[0] as $t,
                        srgb.data[1] as $t,
                        srgb.data[2] as $t,
                    ])
                }
            }
        )*
    };
    (Spectrum no-scale => $($t:ident),*) => {
        $(
            impl From<Spectrum> for Rgb<$t> {
                fn from(o: Spectrum) -> Self {
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
spectral_conv!(Spectrum => u8, u16, u32, u64, usize);
spectral_conv!(Spectrum no-scale => f32, f64);
