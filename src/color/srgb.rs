use crate::color::{ColorSerde, Spectrum, Xyz};
use crate::{Float, Vec3};
use cgmath::Matrix3;
use core::convert::TryFrom;

crate::color!(
    Srgb => Float, 3, crate::color::color_data::srgb
);

impl TryFrom<ColorSerde> for Srgb {
    type Error = ();

    fn try_from(value: ColorSerde) -> Result<Self, Self::Error> {
        let srgb = match value {
            ColorSerde::Srgb(data) => Self::new(data),
            ColorSerde::Xyz(data) => Xyz::new(data).into(),
            ColorSerde::Spectrum(data) => Spectrum::new(data).into(),
            ColorSerde::Color(c) => Self::from(c),
            ColorSerde::MulColor(mul, c) => Self::from(c) * mul,
            ColorSerde::Constant(c) => Self::splat(c),
        };

        Ok(srgb)
    }
}

#[rustfmt::skip]
#[allow(clippy::excessive_precision)]
const RGB_TO_XYZ: Matrix3<Float> = Matrix3::new(
    0.4124564,0.2126729,0.0193339,
    0.3575761, 0.7151522, 0.1191920,
    0.1804375, 0.0721750, 0.9503041,
);
impl From<Srgb> for Xyz {
    #[allow(clippy::excessive_precision)]
    fn from(srgb: Srgb) -> Self {
        let rgb = Vec3::new(uncompand(srgb[0]), uncompand(srgb[1]), uncompand(srgb[2]));
        let data = RGB_TO_XYZ * rgb;

        Self::new(*data.as_ref())
    }
}

#[allow(clippy::excessive_precision)]
#[inline]
fn uncompand(val: Float) -> Float {
    // https://entropymine.com/imageworsener/srgbformula/
    if val <= 0.0404482362771082 {
        val / 12.92
    } else {
        ((val + 0.055) / 1.055).powf(2.4)
    }
}
