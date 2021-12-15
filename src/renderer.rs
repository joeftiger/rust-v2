use crate::camera::sensor::Sensor;
use crate::camera::Camera;
use crate::config::Config;
use crate::integrator::Integrator;
use crate::scene::{Scene, SceneData, SceneDataRef};
use crate::Spectrum;
use image::{ImageBuffer, Rgb};
use serde::{Deserialize, Serialize, Serializer};

#[derive(Deserialize)]
#[serde(from = "RendererData")]
pub struct Renderer {
    pub config: Config,
    pub camera: Box<dyn Camera>,
    // TODO: remove pub
    sensor: Sensor,
    pub integrator: Box<dyn Integrator>,
    #[serde(default)]
    pub scene: Scene,
}

impl Renderer {
    pub fn new(
        config: Config,
        camera: Box<dyn Camera>,
        sensor: Option<Sensor>,
        integrator: Box<dyn Integrator>,
        scene: Scene,
    ) -> Self {
        let res = camera.resolution();
        Self {
            config,
            camera,
            sensor: sensor.unwrap_or_else(|| Sensor::new(res)),
            integrator,
            scene,
        }
    }

    pub fn sensor(&self) -> &Sensor {
        &self.sensor
    }

    pub fn integrate(&self, index: usize) {
        let mut tile = self.sensor.tile(index).lock();

        for px in &mut tile.pixels {
            let ray = self.camera.primary_ray(px.position);

            self.integrator.integrate(&self.scene, ray, px);
        }
    }

    pub fn get_image<T: 'static + image::Primitive>(&self) -> ImageBuffer<Rgb<T>, Vec<T>>
    where
        Rgb<T>: From<Spectrum>,
    {
        let res = self.sensor.resolution;
        let mut buffer = ImageBuffer::new(res.x, res.y);

        for sensor_tile in &self.sensor.tiles {
            for px in &sensor_tile.lock().pixels {
                buffer.put_pixel(px.position.x, px.position.y, Rgb::from(px.average));
            }
        }

        buffer
    }

    pub fn save_image(&self, appendix: Option<usize>) {
        log::info!(target: "Renderer", "saving image...");
        let image = self.get_image::<u16>();

        let path = match appendix {
            None => format!("{}.png", &self.config.output),
            Some(num) => format!("{}-{}.png", &self.config.output, num),
        };

        match image.save(path) {
            Ok(_) => log::info!(target: "Renderer", "saved image!"),
            Err(e) => log::error!(target: "Renderer", "unable to save image: {}", e),
        }
    }
}

// ===================== Serialization into a full ref representation ==============================
// Note the sensor being `Option` due to deserialization requiring it!
#[derive(Serialize)]
pub struct RendererDataRef<'a> {
    config: &'a Config,
    camera: &'a dyn Camera,
    sensor: Option<&'a Sensor>,
    integrator: &'a dyn Integrator,
    scene_data: SceneDataRef<'a>,
}
impl<'a> From<&'a Renderer> for RendererDataRef<'a> {
    fn from(r: &'a Renderer) -> Self {
        Self {
            config: &r.config,
            camera: &*r.camera,
            sensor: Some(&r.sensor),
            integrator: &*r.integrator,
            scene_data: (&r.scene).into(),
        }
    }
}
impl Serialize for Renderer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        RendererDataRef::from(self).serialize(serializer)
    }
}

// ===================== Deserialization from full representation (potentially withou sensor) ======
// Note we convert Renderer to RendererData with a `None` sensor!
#[derive(Deserialize, Serialize)]
pub struct RendererData {
    config: Config,
    camera: Box<dyn Camera>,
    #[serde(default)]
    sensor: Option<Sensor>,
    integrator: Box<dyn Integrator>,
    scene: SceneData,
}
impl From<RendererData> for Renderer {
    fn from(r: RendererData) -> Self {
        Self::new(r.config, r.camera, r.sensor, r.integrator, r.scene.into())
    }
}
impl From<Renderer> for RendererData {
    fn from(r: Renderer) -> Self {
        Self {
            config: r.config,
            camera: r.camera,
            sensor: None,
            integrator: r.integrator,
            scene: r.scene.into(),
        }
    }
}
