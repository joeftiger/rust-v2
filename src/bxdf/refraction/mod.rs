pub mod air;
pub mod glass;
pub mod sapphire;
pub mod water;

use crate::color::color_data::{LAMBDA_END, LAMBDA_START};
use crate::util::floats;
use crate::Float;
use serde::{Deserialize, Serialize};

///! In optics, the **refractive index** of a material is a dimensionless number that describes
///! how fast light travels through the material.
///!
///! This trait helps describe the different spectra of refractive indices, as different wavelengths
///! refract differently.
///!
///! To complement the refractive index, this trait also specifies to return an **optional extinction
///! coefficient**. The extinction coefficient describes how strongly a material absorbs light at given
///! wavelength.

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum RefractiveType {
    Air,
    Vacuum,
    Water,
    Glass,
    Sapphire,
    Linear(Float, Float),
}

impl RefractiveType {
    /// Returns the refractive index (inaccurate for different wavelengths).
    ///
    /// # Returns
    /// * The refractive index
    #[inline]
    pub fn n_uniform(self) -> Float {
        match self {
            RefractiveType::Air => 1.00029,
            RefractiveType::Vacuum => 1.0,
            RefractiveType::Water => 1.3325,
            RefractiveType::Glass => 1.5168,
            RefractiveType::Sapphire => 1.7490,
            RefractiveType::Linear(min, max) => 0.5 * (min + max),
        }
    }

    /// Returns the extinction coefficient (if it exists).
    ///
    /// # Returns
    /// * `Some` extinction coefficient, or
    /// * `None`
    #[inline]
    pub fn k_uniform(self) -> Option<Float> {
        match self {
            RefractiveType::Water => Some(7.2792e-9),
            RefractiveType::Glass => Some(9.7525e-9),
            RefractiveType::Sapphire => Some(0.020900),
            _ => None,
        }
    }

    /// Returns the refractive index at a given wavelength.
    ///
    /// # Arguments
    /// * `lambda` - The wavelength in **µm**
    ///
    /// # Returns
    /// * The corresponding refractive index
    #[inline]
    pub fn n(self, lambda: Float) -> Float {
        match self {
            RefractiveType::Air => air::sellmeier_n(lambda),
            RefractiveType::Vacuum => 1.0,
            RefractiveType::Water => search_lambda(&water::INDEX, &water::N, lambda),
            RefractiveType::Glass => glass::sellmeier_n(lambda),
            RefractiveType::Sapphire => sapphire::sellmeier_n(lambda),
            RefractiveType::Linear(min, max) => {
                let t = floats::lerp_inv(lambda, LAMBDA_START, LAMBDA_END);
                t.lerp(min, max)
            }
        }
    }

    /// Returns the extinction coefficient at a given wavelength (if it exists).
    ///
    /// # Arguments
    /// * `lambda` - The wavelength in **µm**
    ///
    /// # Returns
    /// * `Some` corresponding extinction coefficient, or
    /// * `None`
    pub fn k(self, lambda: Float) -> Option<Float> {
        match self {
            RefractiveType::Water => Some(search_lambda(&water::INDEX, &water::K, lambda)),
            RefractiveType::Glass => Some(search_lambda(&glass::INDEX_K, &glass::K, lambda)),
            RefractiveType::Sapphire => {
                Some(search_lambda(&sapphire::INDEX_K, &sapphire::K, lambda))
            }
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum SearchResult {
    Single(usize),
    Lerp(usize, usize),
}
fn search_index(values: &[Float], search_index: Float) -> SearchResult {
    match values.binary_search_by(|&a| a.total_cmp(&search_index)) {
        Ok(i) => SearchResult::Single(i),
        Err(i) if i >= values.len() => SearchResult::Single(i - 1),
        Err(i) => SearchResult::Lerp(i - 1, i),
    }
}
fn search_lambda(indices: &[Float], values: &[Float], lambda: Float) -> Float {
    match search_index(indices, lambda) {
        SearchResult::Single(i) => values[i],
        SearchResult::Lerp(min, max) => {
            let t = floats::lerp_inv(lambda, indices[min], indices[max]);
            t.lerp(values[min], values[max])
        }
    }
}
