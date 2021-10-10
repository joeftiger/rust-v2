pub use perspective::*;

use crate::geometry::Ray;
use crate::UVec2;

pub mod dummy;
pub mod perspective;
pub mod sensor;

#[typetag::serde]
pub trait Camera: Send + Sync {
    /// Returns the resolution of the camear.
    fn resolution(&self) -> UVec2;

    /// Creates a "primary" (camera) ray
    fn primary_ray(&self, pixel: UVec2) -> Ray;
}
