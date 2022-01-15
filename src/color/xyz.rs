use crate::color::{ColorSerde, Spectrum, Srgb};
use crate::{Float, Vec3};
use cgmath::Matrix3;
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

#[rustfmt::skip]
#[allow(clippy::excessive_precision)]
const XYZ_TO_RGB: Matrix3<Float> = Matrix3::new(
    3.240454836021409, -0.9692663898756538, 0.05564341960421366,
    -1.5371388501025753, 1.8760109288424913, -0.20402585426769815,
    -0.498531546868481, 0.041556082346673524, 1.0572251624579287,
);
impl From<Xyz> for Srgb {
    #[allow(clippy::excessive_precision)]
    fn from(xyz: Xyz) -> Self {
        let vec = Vec3::from(xyz.data);
        let rgb = XYZ_TO_RGB * vec;

        Self::new([compand(rgb.x), compand(rgb.y), compand(rgb.z)])
    }
}

#[allow(clippy::excessive_precision)]
#[inline]
fn compand(val: Float) -> Float {
    // https://entropymine.com/imageworsener/srgbformula/
    if val <= 0.00313066844250063 {
        val * 12.92
    } else {
        1.055 * val.powf(1.0 / 2.4) - 0.055
    }
}
