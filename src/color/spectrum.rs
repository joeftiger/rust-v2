use crate::color::cie::*;
use crate::color::color_data::LAMBDA_STEP;
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
        // use gaussian approximation (as in my paper)
        let xyz: Xyz = spectrum
            .as_light_waves()
            .iter()
            .map(|l| lambda_to_xyz_approx(l.lambda) * l.intensity)
            .sum();

        // use spectral data
        // let x = (spectrum * Spectrum::new(CIE_X_BAR)).sum_values();
        // let y = (spectrum * Spectrum::new(CIE_Y_BAR)).sum_values();
        // let z = (spectrum * Spectrum::new(CIE_Z_BAR)).sum_values();
        // let xyz = Self::new([x, y, z]);

        xyz * (LAMBDA_STEP / CIE_Y_INTEGRAL)
    }
}
