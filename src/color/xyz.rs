use crate::color::{ColorSerde, Spectrum, Srgb};
use crate::Float;
use core::convert::TryFrom;

crate::color!(
    Xyz => Float, 3, crate::color::color_data::xyz
);

impl TryFrom<ColorSerde> for Xyz {
    type Error = ();

    fn try_from(value: ColorSerde) -> Result<Self, Self::Error> {
        let xyz = match value {
            ColorSerde::Srgb(data) => Srgb::new(data).into(),
            ColorSerde::Xyz(data) => Self::new(data),
            ColorSerde::Spectrum(data) => Spectrum::new(data).into(),
            ColorSerde::Color(c) => Self::from(c),
            ColorSerde::MulColor(mul, c) => Self::from(c) * mul,
            ColorSerde::Constant(c) => Self::splat(c),
        };

        Ok(xyz)
    }
}

impl From<Xyz> for Srgb {
    #[rustfmt::skip]
    fn from(xyz: Xyz) -> Self {
        let r =  3.2404542 * xyz[0] - 1.5371385 * xyz[1] - 0.4985314 * xyz[2];
        let g = -0.9692660 * xyz[0] + 1.8760108 * xyz[1] + 0.0415560 * xyz[2];
        let b =  0.0556434 * xyz[0] - 0.2040259 * xyz[1] + 1.0572252 * xyz[2];

        Self::new([compand(r), compand(g), compand(b)])
    }
}

#[inline]
fn compand(val: Float) -> Float {
    // https://entropymine.com/imageworsener/srgbformula/
    if val <= 0.00313066844250063 {
        val * 12.92
    } else {
        1.055 * val.powf(1.0 / 2.4) - 0.055
    }
}
