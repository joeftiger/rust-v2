use crate::color::cie::{lambda_to_xyz_approx, CIE_Y_INTEGRAL};
use crate::color::color_data::LAMBDA_RANGE;
use crate::color::{ColorSerde, Srgb};
use crate::Float;
use core::convert::TryFrom;

crate::color!(
    Spectrum => Float, LAMBDA_NUM, crate::color::color_data::spectral
);

impl TryFrom<ColorSerde> for Spectrum {
    type Error = ();

    fn try_from(value: ColorSerde) -> Result<Self, Self::Error> {
        let spectrum = match value {
            ColorSerde::Spectrum(data) => Self::new(data),
            ColorSerde::Color(c) => Self::from(c),
            ColorSerde::MulColor(mul, c) => mul * Self::from(c),
            ColorSerde::Constant(c) => Self::splat(c),
            _ => return Err(()),
        };

        Ok(spectrum)
    }
}

impl From<Spectrum> for Srgb {
    fn from(spectrum: Spectrum) -> Self {
        Srgb::from(Xyz::from(spectrum))
    }
}

impl From<Spectrum> for Xyz {
    fn from(spectrum: Spectrum) -> Self {
        let xyz = spectrum
            .as_light_waves()
            .iter()
            .fold(Xyz::splat(0.0), |acc, next| {
                acc + lambda_to_xyz_approx(next.lambda) * next.intensity
            });

        let scale = LAMBDA_RANGE / (CIE_Y_INTEGRAL * Spectrum::size() as Float);

        xyz * scale
    }
}
