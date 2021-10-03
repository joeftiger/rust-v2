use crate::camera::sensor::Pixel;
use crate::{UVec2, SENSOR_TILE_WIDTH};

use serde::{Deserialize, Serialize};

pub const SENSOR_PIXEL_LEN: usize = (SENSOR_TILE_WIDTH * SENSOR_TILE_WIDTH) as usize;

serde_big_array::big_array! {
    SensorSerde;
    SENSOR_PIXEL_LEN
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SensorTile {
    #[serde(with = "SensorSerde")]
    pub pixels: [Pixel; SENSOR_PIXEL_LEN],
}

impl SensorTile {
    /// Creates a new sensor tile with a square size of [SENSOR_TILE_WIDTH].
    ///
    /// # Arguments
    /// * `start`: The start position of the top left pixel
    pub fn new(start: UVec2) -> Self {
        let mut pixels = [Pixel::default(); SENSOR_PIXEL_LEN];

        let mut i = 0;
        for x in 0..SENSOR_TILE_WIDTH {
            for y in 0..SENSOR_TILE_WIDTH {
                let offset = UVec2::new(x as u32, y as u32);
                pixels[i].position = start + offset;

                i += 1;
            }
        }

        Self { pixels }
    }
}
