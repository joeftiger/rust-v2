use crate::util::Index;
use crate::{Spectrum, PACKET_SIZE};
use core::ops::RangeBounds;
use serde::{Deserialize, Serialize};

thread_local! {
    static RNG: fastrand::Rng = fastrand::Rng::with_seed(0);
}

fn rand(range: impl RangeBounds<usize>) -> usize {
    RNG.with(|rng| rng.usize(range))
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum SpectralSampler {
    /// Fully randomized samples
    Random,
    /// Hero wavelength sampling with a given index spread between samples
    Hero,
}

impl SpectralSampler {
    pub fn create(self) -> [usize; PACKET_SIZE] {
        if PACKET_SIZE == Spectrum::size() {
            let mut i = Index::new();
            return [0; PACKET_SIZE].map(|_| i.get_and_inc());
        }

        match self {
            SpectralSampler::Random => [0; PACKET_SIZE].map(|_| rand(0..Spectrum::size())),
            SpectralSampler::Hero => {
                let hero = rand(0..Spectrum::size());

                let mut i = Index::new();
                [0; PACKET_SIZE].map(|_| Self::hero(i.get_and_inc(), hero))
            }
        }
    }

    pub fn fill(self, indices: &mut [usize; PACKET_SIZE]) {
        match self {
            SpectralSampler::Random => indices
                .iter_mut()
                .for_each(|idx| *idx = rand(0..Spectrum::size())),
            SpectralSampler::Hero => {
                if PACKET_SIZE == Spectrum::size() {
                    indices.iter_mut().enumerate().for_each(|(i, idx)| *idx = i);
                } else {
                    let hero = rand(0..Spectrum::size());

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
