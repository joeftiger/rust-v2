use crate::camera::sensor::Sensor;
use crate::camera::Camera;
use crate::config::Config;
use crate::integrator::Integrator;
use crate::scene::Scene;
use crate::Spectrum;
use core::fmt;
use image::{ImageBuffer, Rgb};
use serde::{de, Deserialize, Deserializer, Serialize};

#[derive(Serialize)]
pub struct Renderer {
    pub config: Config,
    pub camera: Box<dyn Camera>,
    sensor: Sensor,
    integrator: Box<dyn Integrator>,
    scene: Scene,
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
}

impl<'de> Deserialize<'de> for Renderer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Config,
            Camera,
            Sensor,
            Integrator,
            Scene,
        }
        impl<'d> Deserialize<'d> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'d>,
            {
                struct FieldVisitor;
                impl<'v> de::Visitor<'v> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        write!(
                            formatter,
                            "'config', 'camera', 'sensor' (optional), 'integrator', 'scene'"
                        )
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        match v {
                            "config" => Ok(Field::Config),
                            "camera" => Ok(Field::Camera),
                            "sensor" => Ok(Field::Sensor),
                            "integrator" => Ok(Field::Integrator),
                            "scene" => Ok(Field::Scene),
                            _ => Err(de::Error::unknown_field(v, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct RendererVisitor;
        impl<'v> de::Visitor<'v> for RendererVisitor {
            type Value = Renderer;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "struct Renderer (reduced or full)")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: de::MapAccess<'v>,
            {
                let mut config = None;
                let mut camera: Option<Box<dyn Camera>> = None;
                let mut sensor = None;
                let mut integrator = None;
                let mut scene = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Config => {
                            if config.replace(map.next_value()?).is_some() {
                                return Err(de::Error::duplicate_field("config"));
                            }
                        }
                        Field::Camera => {
                            if camera.replace(map.next_value()?).is_some() {
                                return Err(de::Error::duplicate_field("camera"));
                            }
                        }
                        Field::Sensor => {
                            if sensor.replace(map.next_value()?).is_some() {
                                return Err(de::Error::duplicate_field("sensor"));
                            }
                        }
                        Field::Integrator => {
                            if integrator.replace(map.next_value()?).is_some() {
                                return Err(de::Error::duplicate_field("integrator"));
                            }
                        }
                        Field::Scene => {
                            if scene.replace(map.next_value()?).is_some() {
                                return Err(de::Error::duplicate_field("scene"));
                            }
                        }
                    }
                }

                let config = config.ok_or_else(|| de::Error::missing_field("config"))?;
                let camera = camera.ok_or_else(|| de::Error::missing_field("camera"))?;
                let sensor = sensor.unwrap_or_else(|| Sensor::new(camera.resolution()));
                let integrator =
                    integrator.ok_or_else(|| de::Error::missing_field("integrator"))?;
                let scene = scene.ok_or_else(|| de::Error::missing_field("scene"))?;

                Ok(Renderer::new(config, camera, sensor, integrator, scene))
            }
        }

        const FIELDS: &[&str] = &["config", "camera", "sensor", "integrator", "scene"];
        deserializer.deserialize_struct("Renderer", FIELDS, RendererVisitor)
    }
}
