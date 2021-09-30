use crate::{Spectrum, PACKET_SIZE};
use fastrand::usize as rand;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum SpectralSampler {
    /// Fully randomized samples
    Random,
    /// Hero wavelength sampling with a given index spread between samples
    Hero,
}

impl SpectralSampler {
    pub fn fill(self, indices: &mut [usize; PACKET_SIZE]) {
        match self {
            SpectralSampler::Random => indices
                .iter_mut()
                .for_each(|idx| *idx = rand(0..Spectrum::size())),
            SpectralSampler::Hero => {
                if PACKET_SIZE == Spectrum::size() {
                    indices.iter_mut().enumerate().for_each(|(i, idx)| *idx = i);
                } else {
                    let hero = fastrand::usize(0..Spectrum::size());

                    indices
                        .iter_mut()
                        .enumerate()
                        .for_each(|(j, idx)| *idx = Self::hero(j, hero));
                }
            }
        }
    }

    /// The rotation function according to
    /// - authors: A. Wilkie & S. Nawaz & M. Droske & A. Weidlich & J. Hanika
    /// - paper: Hero Wavelength Spectral Sampling
    /// - year: 2014
    /// - page: 3
    /// - equation: 5
    #[inline]
    fn hero(j: usize, hero_index: usize) -> usize {
        let percentage = j as f32 / PACKET_SIZE as f32;

        let spread = percentage * Spectrum::size() as f32;

        (hero_index + spread.round() as usize) % Spectrum::size()
    }
}
