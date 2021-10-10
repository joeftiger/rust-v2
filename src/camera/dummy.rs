use crate::camera::Camera;
use crate::geometry::Ray;
use crate::UVec2;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct DummyCamera;

#[typetag::serde]
impl Camera for DummyCamera {
    fn resolution(&self) -> UVec2 {
        UVec2::new(0, 0)
    }

    fn primary_ray(&self, _: UVec2) -> Ray {
        unimplemented!()
    }
}
