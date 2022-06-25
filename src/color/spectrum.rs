use crate::color::cie::{lambda_to_xyz_approx, CIE_Y_INTEGRAL};
use crate::color::color_data::{LAMBDA_RANGE, LAMBDA_STEP};
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
        let xyz: Xyz = spectrum
            .as_light_waves()
            .iter()
            .map(|l| lambda_to_xyz_approx(l.lambda) * l.intensity)
            .sum();

        xyz * LAMBDA_STEP

        // const SCALE: Float = LAMBDA_RANGE / (CIE_Y_INTEGRAL * Spectrum::size() as Float);
        //
        // xyz * SCALE
    }
}
