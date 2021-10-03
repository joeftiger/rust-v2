use core::ops::{Div, Index, IndexMut, Mul};
use core::slice::SliceIndex;

use serde::{Deserialize, Serialize};

use crate::{Float, Spectrum, UVec2, PACKET_SIZE};

serde_big_array::big_array! {
    PixelSerde;
    Spectrum::size()
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Pixel {
    pub position: UVec2,
    pub average: Spectrum,
    samples: SampleCounter,
}

impl Pixel {
    pub fn new(position: UVec2) -> Self {
        Self {
            position,
            average: Spectrum::splat(0.0),
            samples: SampleCounter::default(),
        }
    }

    pub fn add_none(&mut self) {
        let avg = self.average * self.samples;
        self.samples.inc_all();
        self.average = avg / self.samples;
    }

    pub fn add_none_packet(&mut self, indices: &[usize; PACKET_SIZE]) {
        for &index in indices {
            let avg = self.average[index] * self.samples[index] as Float;
            self.samples.inc(index);
            self.average[index] = avg / self.samples[index] as Float;
        }
    }

    pub fn add(&mut self, spectrum: Spectrum) {
        let mut avg = self.average * self.samples;
        avg += spectrum;
        self.samples.inc_all();

        self.average = avg / self.samples;
    }

    pub fn add_packet(&mut self, spectrum: &[Float; PACKET_SIZE], indices: &[usize; PACKET_SIZE]) {
        for i in 0..PACKET_SIZE {
            let index = indices[i];
            let mut avg = self.average[index] * self.samples[index] as Float;
            avg += spectrum[i];
            self.samples.inc(index);

            self.average[index] = avg / self.samples[index] as Float;
        }
    }

    pub fn add_lambda(&mut self, lambda: Float, index: usize) {
        let mut avg = self.average[index] * self.samples[index] as Float;
        avg += lambda;
        self.samples.inc(index);

        self.average[index] = avg / self.samples[index] as Float;
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Self {
            position: UVec2::new(0, 0),
            average: Default::default(),
            samples: Default::default(),
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
struct SampleCounter {
    #[serde(with = "PixelSerde")]
    data: [u32; Spectrum::size()],
}

impl SampleCounter {
    fn inc(&mut self, index: usize) {
        self.data[index] += 1;
    }

    fn inc_all(&mut self) {
        self.data.iter_mut().for_each(|v| *v += 1);
    }
}

#[allow(clippy::derivable_impls)]
impl Default for SampleCounter {
    fn default() -> Self {
        Self {
            data: [0; Spectrum::size()],
        }
    }
}

impl<Idx: SliceIndex<[u32]>> Index<Idx> for SampleCounter {
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.data[index]
    }
}
impl<Idx: SliceIndex<[u32]>> IndexMut<Idx> for SampleCounter {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl Mul<SampleCounter> for Spectrum {
    type Output = Self;

    fn mul(mut self, rhs: SampleCounter) -> Self::Output {
        for i in 0..Spectrum::size() {
            self[i] *= rhs[i] as Float;
        }
        self
    }
}
impl Div<SampleCounter> for Spectrum {
    type Output = Self;

    fn div(mut self, rhs: SampleCounter) -> Self::Output {
        for i in 0..Spectrum::size() {
            self[i] /= rhs[i] as Float;
        }
        self
    }
}
