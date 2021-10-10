use crate::camera::sensor::Pixel;
use crate::geometry::Ray;
use crate::integrator::Integrator;
use crate::scene::Scene;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct DummyIntegrator;

#[typetag::serde]
impl Integrator for DummyIntegrator {
    fn integrate(&self, _: &Scene, _: Ray, _: &mut Pixel) {
        unimplemented!()
    }
}
