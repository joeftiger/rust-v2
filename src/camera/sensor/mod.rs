pub mod pixel;
pub mod tile;

use parking_lot::Mutex;
pub use pixel::*;
pub use tile::*;

use crate::{UVec2, SENSOR_TILE_WIDTH};
use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Deserialize)]
#[serde(from = "SensorDe")]
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

    pub const fn new2(resolution: UVec2, tiles: Vec<Mutex<SensorTile>>) -> Self {
        Self { resolution, tiles }
    }

    pub fn reset(&mut self) {
        for t in &self.tiles {
            t.lock().pixels.iter_mut().for_each(Pixel::reset);
        }
    }

    pub fn num_tiles(&self) -> usize {
        self.tiles.len()
    }

    pub fn tile(&self, index: usize) -> &Mutex<SensorTile> {
        &self.tiles[index]
    }
}

impl From<SensorDe> for Sensor {
    fn from(de: SensorDe) -> Self {
        match de {
            SensorDe::Config(resolution) => Self::new(resolution),
            SensorDe::Checkpoint(resolution, tiles) => Self::new2(resolution, tiles),
        }
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

#[derive(Serialize)]
enum SensorSer<'a> {
    Checkpoint(UVec2, &'a [Mutex<SensorTile>]),
}

#[derive(Deserialize)]
enum SensorDe {
    Checkpoint(UVec2, Vec<Mutex<SensorTile>>),
    Config(UVec2),
}
