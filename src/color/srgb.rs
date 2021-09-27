use crate::color::{ColorSerde, Spectrum, Xyz};
use crate::Float;
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

impl From<Srgb> for Xyz {
    #[allow(clippy::many_single_char_names)]
    fn from(srgb: Srgb) -> Self {
        let r = uncompand(srgb[0]);
        let g = uncompand(srgb[1]);
        let b = uncompand(srgb[2]);

        let x = 0.4124564 * r + 0.3575761 * g + 0.1804375 * b;
        let y = 0.2126729 * r + 0.7151522 * g + 0.0721750 * b;
        let z = 0.0193339 * r + 0.1191920 * g + 0.9503041 * b;

        Self::new([x, y, z])
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
