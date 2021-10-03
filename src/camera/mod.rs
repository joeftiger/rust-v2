pub use perspective::*;

use crate::geometry::Ray;
use crate::UVec2;

pub mod perspective;
pub mod sensor;

#[typetag::serde]
pub trait Camera: Send + Sync {
    /// Creates a "primary" (camera) ray
    fn primary_ray(&self, pixel: UVec2) -> Ray;
}
