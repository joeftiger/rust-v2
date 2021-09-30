use crate::sensor::Pixel;
use crate::{UVec2, SENSOR_TILE_WIDTH};

use serde::{Deserialize, Serialize};

const SIZE: usize = SENSOR_TILE_WIDTH * SENSOR_TILE_WIDTH;

serde_big_array::big_array! {
    SensorSerde;
    SIZE
}

#[derive(Serialize, Deserialize)]
pub struct SensorTile {
    #[serde(with = "SensorSerde")]
    pub pixels: [Pixel; SIZE],
}

impl SensorTile {
    pub fn new(start: UVec2) -> Self {
        let mut pixels = [Pixel::default(); SIZE];

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
