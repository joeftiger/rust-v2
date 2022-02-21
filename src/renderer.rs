use crate::camera::sensor::Sensor;
use crate::camera::Camera;
use crate::config::Config;
use crate::integrator::Integrator;
use crate::scene::Scene;
use crate::Image;
use image::{ImageBuffer, ImageFormat, Pixel, Rgb};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Serialize)]
pub struct Renderer {
    pub config: Config,
    pub camera: Box<dyn Camera>,
    #[serde(skip_serializing_if = "Sensor::is_empty")]
    pub sensor: Sensor,
    pub integrator: Box<dyn Integrator>,
    #[serde(skip_serializing_if = "Scene::is_empty")]
    pub scene: Scene,
}

impl Renderer {
    pub fn new(
        config: Config,
        camera: Box<dyn Camera>,
        sensor: Sensor,
        integrator: Box<dyn Integrator>,
        scene: Scene,
    ) -> Self {
        Self {
            config,
            camera,
            sensor,
            integrator,
            scene,
        }
    }

    pub fn reset(&mut self) {
        self.sensor.reset();
    }

    pub fn integrate(&self, index: usize) {
        let mut tile = self.sensor.tile(index).lock();

        for px in &mut tile.pixels {
            let ray = self.camera.primary_ray(px.position);

            self.integrator.integrate(&self.scene, ray, px);
        }
    }

    pub fn get_image_openexr_png(&self) -> (Image<f32>, Image<u16>) {
        let res = self.sensor.resolution;
        let mut exr = ImageBuffer::new(res.x, res.y);
        let mut png = ImageBuffer::new(res.x, res.y);

        for sensor_tile in &self.sensor.tiles {
            for px in &sensor_tile.lock().pixels {
                exr.put_pixel(px.position.x, px.position.y, Rgb::from(px.average));
                png.put_pixel(px.position.x, px.position.y, Rgb::from(px.average));
            }
        }

        (exr, png)
    }

    pub fn get_image<T>(&self) -> Image<T>
    where
        Rgb<T>: From<crate::Spectrum> + Pixel,
    {
        let res = self.sensor.resolution;
        let mut image = ImageBuffer::new(res.x, res.y);

        for sensor_tile in &self.sensor.tiles {
            for px in &sensor_tile.lock().pixels {
                image.put_pixel(px.position.x, px.position.y, Rgb::from(px.average));
            }
        }

        image
    }

    pub fn get_image_openexr(&self) -> Image<f32> {
        self.get_image()
    }

    pub fn get_image_png(&self) -> Image<u16> {
        self.get_image()
    }

    pub fn save_image(&self, appendix: Option<usize>) {
        log::info!(target: "Renderer", "saving image...");
        let (exr, png) = self.get_image_openexr_png();

        let path = match appendix {
            None => self.config.output.clone(),
            Some(num) => format!("{}-{}.exr", &self.config.output, num),
        };

        match exr.save_with_format(format!("{}.exr", &path), ImageFormat::OpenExr) {
            Ok(_) => log::info!(target: "Renderer", "saved image! (OpenEXR)"),
            Err(e) => log::error!(target: "Renderer", "unable to save image (OpenEXR): {}", e),
        }

        match png.save_with_format(format!("{}.png", &path), ImageFormat::Png) {
            Ok(_) => log::info!(target: "Renderer", "saved image! (PNG)"),
            Err(e) => log::error!(target: "Renderer", "unable to save image (PNG): {}", e),
        }
    }
}

impl<'de> Deserialize<'de> for Renderer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (config, camera, sensor, integrator, scene) = {
            #[derive(Deserialize)]
            struct Renderer {
                config: Config,
                camera: Box<dyn Camera>,
                #[serde(default)]
                sensor: Sensor,
                integrator: Box<dyn Integrator>,
                #[serde(default)]
                scene: Scene,
            }

            let Renderer {
                config,
                camera,
                mut sensor,
                integrator,
                scene,
            } = Renderer::deserialize(deserializer)?;
            if sensor.resolution == Sensor::default().resolution && sensor.is_empty() {
                sensor = Sensor::new(camera.resolution())
            }

            (config, camera, sensor, integrator, scene)
        };

        Ok(Self::new(config, camera, sensor, integrator, scene))
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::camera::dummy::DummyCamera;
//     use crate::integrator::dummy::DummyIntegrator;
//
//     #[test]
//     fn serde_empty() {
//         let config = Config {
//             output: "".to_string(),
//             passes: 0,
//             threads: None,
//         };
//         let camera = Box::new(DummyCamera);
//         let sensor = Sensor::new(camera.resolution());
//         let integrator = Box::new(DummyIntegrator);
//         let scene = Scene::default();
//
//         Renderer::new(config, camera, sensor, integrator, scene);
//     }
// }
