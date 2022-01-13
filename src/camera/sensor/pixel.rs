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

    pub fn reset(&mut self) {
        self.average.data.iter_mut().for_each(|a| *a = 0.0);
        self.samples.data.iter_mut().for_each(|s| *s = 0);
    }

    pub fn add_none(&mut self) {
        let avg = self.average * self.samples;
        self.samples.inc_all();
        self.average = avg / self.samples;
    }

    #[inline]
    pub fn add_none_packet(&mut self, indices: &[usize; PACKET_SIZE]) {
        indices.iter().for_each(|&i| self.add_none_lambda(i));
        // for &index in indices {
        //     let avg = self.average[index] * self.samples[index] as Float;
        //     self.samples.inc(index);
        //     self.average[index] = avg / self.samples[index] as Float;
        // }
    }

    #[inline]
    pub fn add_none_lambda(&mut self, index: usize) {
        let avg = self.average[index] * self.samples[index] as Float;
        self.samples.inc(index);
        self.average[index] = avg / self.samples[index] as Float;
    }

    pub fn add(&mut self, spectrum: Spectrum) {
        let avg = self.average * self.samples + spectrum;
        self.samples.inc_all();
        self.average = avg / self.samples;
    }

    #[inline]
    pub fn add_packet(&mut self, spectrum: &[Float; PACKET_SIZE], indices: &[usize; PACKET_SIZE]) {
        for i in 0..PACKET_SIZE {
            self.add_lambda(spectrum[i], indices[i]);
        }
    }

    #[inline]
    pub fn add_lambda(&mut self, lambda: Float, index: usize) {
        let avg = self.average[index] * self.samples[index] as Float + lambda;
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
    #[inline(always)]
    fn inc(&mut self, index: usize) {
        self.data[index] += 1;
    }

    #[inline]
    fn inc_all(&mut self) {
        self.data.iter_mut().for_each(|v| *v += 1);
    }
}

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