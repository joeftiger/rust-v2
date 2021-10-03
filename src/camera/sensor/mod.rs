pub mod pixel;
pub mod tile;

pub use pixel::*;
use std::sync::Mutex;
pub use tile::*;

use crate::{UVec2, SENSOR_TILE_WIDTH};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug)]
pub struct Sensor {
    pub resolution: UVec2,
    pub tiles: Vec<Mutex<SensorTile>>,
}

impl Sensor {
    /// Creates a new sensor.
    ///
    /// # Conditions
    /// * `resolution`: The resolution must be a multiple of [SENSOR_TILE_WIDTH].
    ///                 This function will otherwise panic.
    ///
    /// # Arguments
    /// * `resolution`: The resolution of the camera.
    pub fn new(resolution: UVec2) -> Self {
        assert_eq!(
            0,
            resolution.x % SENSOR_TILE_WIDTH,
            "sensor resolution width must be a multiple of '{}'",
            SENSOR_TILE_WIDTH
        );
        assert_eq!(
            0,
            resolution.y % SENSOR_TILE_WIDTH,
            "sensor resolution height must be a multiple of '{}'",
            SENSOR_TILE_WIDTH
        );

        let grid = resolution / SENSOR_TILE_WIDTH;
        let num_tiles = grid.x * grid.y;

        let mut tiles = Vec::with_capacity(num_tiles as usize);

        for x in 0..grid.x {
            for y in 0..grid.y {
                let start = UVec2::new(x, y) * SENSOR_TILE_WIDTH;
                let tile = SensorTile::new(start);
                let tile = Mutex::new(tile);
                tiles.push(tile);
            }
        }

        Self { resolution, tiles }
    }

    pub fn num_tiles(&self) -> usize {
        self.tiles.len()
    }

    pub fn tile(&self, index: usize) -> &Mutex<SensorTile> {
        &self.tiles[index]
    }
}

impl Serialize for Sensor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        SensorSer::Checkpoint(self.resolution, &self.tiles).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Sensor {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        match SensorDe::deserialize(deserializer)? {
            SensorDe::Checkpoint(resolution, tiles) => Ok(Sensor { resolution, tiles }),
            SensorDe::Config(res) => Ok(Self::new(res)),
        }
    }
}

#[derive(Serialize)]
enum SensorSer<'a> {
    Checkpoint(UVec2, &'a [Mutex<SensorTile>]),
}

#[derive(Deserialize)]
enum SensorDe {
    Config(UVec2),
    Checkpoint(UVec2, Vec<Mutex<SensorTile>>),
}
