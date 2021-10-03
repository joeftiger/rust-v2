use crate::camera::sensor::Sensor;
use crate::camera::Camera;
use crate::config::Config;
use crate::integrator::Integrator;
use crate::scene::Scene;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Renderer {
    pub config: Config,
    pub camera: Box<dyn Camera>,
    sensor: Sensor,
    integrator: Box<dyn Integrator>,
    scene: Scene,
}

impl Renderer {
    pub fn sensor(&self) -> &Sensor {
        &self.sensor
    }

    pub fn integrate(&self, index: usize) {
        let mut tile = self.sensor.tile(index).lock().unwrap();

        for px in &mut tile.pixels {
            let ray = self.camera.primary_ray(px.position);

            self.integrator.integrate(&self.scene, ray, px);
        }
    }
}
