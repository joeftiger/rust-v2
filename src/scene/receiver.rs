use crate::bxdf::bsdf::BSDF;
use crate::geometry::Geometry;
use serde::{Deserialize, Serialize};

/// A receiver consists of a geometry and a BSDF.
#[derive(Serialize, Deserialize)]
pub struct Receiver {
    geometry: Box<dyn Geometry>,
    pub bsdf: BSDF,
}
