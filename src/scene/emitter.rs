use crate::bxdf::BSDF;
use crate::geometry::{Aabb, Geometry, Intersection, Ray};
use crate::scene::{Sampleable, Scene};
use crate::util::floats;
use crate::{Float, Spectrum, Vec2, Vec3, PACKET_SIZE};
use cgmath::InnerSpace;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Emitter {
    geometry: Box<dyn Sampleable>,
    #[serde(default)]
    pub bsdf: BSDF,
    pub emission: Spectrum,
    #[serde(default)]
    pub tag: String,
}

impl Emitter {
    /// Computes the radiance of this emitter.
    #[inline]
    pub fn radiance(&self) -> Spectrum {
        self.emission
    }

    /// Computes the radiance of this emitter.
    ///
    /// # Constraints
    /// * `indices`: All values should be within `[0, `[Spectrum::size]`)`.
    ///
    /// # Arguments
    /// * `indices`: The spectral indices
    #[inline]
    pub fn radiance_packet(&self, indices: &[usize; PACKET_SIZE]) -> [Float; PACKET_SIZE] {
        indices.map(|i| self.emission[i])
    }

    /// Computes the radiance of this emitter.
    ///
    /// # Constraints
    /// * `index`: Should be within `[0, `[Spectrum::size]`)`.
    ///
    /// # Arguments
    /// * `index`: The spectral index
    #[inline]
    pub fn radiance_lambda(&self, index: usize) -> Float {
        self.emission[index]
    }

    /// Samples the emitter.
    ///
    /// # Constraints
    /// * `point` - ALl values should be finite (neither infinite nor `NaN`).
    /// * `sample` - All values should be within `[0, 1)`.
    ///
    /// # Arguments
    /// * `point` - The point from which we sample the emitter
    /// * `sample` - A random sample
    pub fn sample(&self, point: Vec3, sample: Vec2) -> EmitterSample<Spectrum> {
        let surface_sample = self.geometry.sample_surface(point, sample);
        let occlusion = OcclusionTester::between(point, surface_sample.point);

        EmitterSample::new(self.radiance(), occlusion.ray.direction, occlusion)
    }

    /// Samples the emitter.
    ///
    /// # Constraints
    /// * `point` - ALl values should be finite (neither infinite nor `NaN`).
    /// * `sample` - All values should be within `[0, 1)`.
    /// * `indices`: All values should be within `[0, `[Spectrum::size]`)`.
    ///
    /// # Arguments
    /// * `point` - The point from which we sample the emitter
    /// * `sample` - A random sample
    /// * `indices`: The spectral indices
    pub fn sample_packet(
        &self,
        point: Vec3,
        sample: Vec2,
        indices: &[usize; PACKET_SIZE],
    ) -> EmitterSample<[Float; PACKET_SIZE]> {
        let surface_sample = self.geometry.sample_surface(point, sample);
        let occlusion = OcclusionTester::between(point, surface_sample.point);

        EmitterSample::new(
            self.radiance_packet(indices),
            occlusion.ray.direction,
            occlusion,
        )
    }

    /// Samples the emitter.
    ///
    /// # Constraints
    /// * `point` - ALl values should be finite (neither infinite nor `NaN`).
    /// * `sample` - All values should be within `[0, 1)`.
    /// * `index`: Should be within `[0, `[Spectrum::size]`)`.
    ///
    /// # Arguments
    /// * `point` - The point from which we sample the emitter
    /// * `sample` - A random sample
    /// * `index`: The spectral index
    pub fn sample_lambda(&self, point: Vec3, sample: Vec2, index: usize) -> EmitterSample<Float> {
        let surface_sample = self.geometry.sample_surface(point, sample);
        let occlusion_tester = OcclusionTester::between(point, surface_sample.point);

        EmitterSample::new(
            self.radiance_lambda(index),
            occlusion_tester.ray.direction,
            occlusion_tester,
        )
    }
}

#[typetag::serde]
impl Geometry for Emitter {
    #[inline]
    fn contains(&self, point: Vec3) -> Option<bool> {
        self.geometry.contains(point)
    }

    #[inline]
    fn bounds(&self) -> Aabb {
        self.geometry.bounds()
    }

    #[inline]
    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        self.geometry.intersect(ray)
    }

    #[inline]
    fn intersects(&self, ray: Ray) -> bool {
        self.geometry.intersects(ray)
    }
}

pub struct EmitterSample<T> {
    pub radiance: T,
    pub incident: Vec3,
    pub occlusion: OcclusionTester,
}
impl<T> EmitterSample<T> {
    pub const fn new(radiance: T, incident: Vec3, occlusion: OcclusionTester) -> Self {
        Self {
            radiance,
            incident,
            occlusion,
        }
    }
}

#[derive(Copy, Clone)]
pub struct OcclusionTester {
    ray: Ray,
}
impl OcclusionTester {
    /// Creates a new occlusion tester between the two given points.
    /// The created ray partition will be clamped to `[e, distance - e]`, with `e` denoting an epsilon
    /// and `distance` the distance between the points.
    /// This is to work around floating point imprecision that might occur in the intersection code.
    ///
    /// # Constraints
    /// * `origin` - All values should be finite (neither infinite nor `NaN`).
    /// * `target` - All values should be finite.
    ///
    /// # Arguments
    /// * `origin` - The origin of the ray
    /// * `target` - The target of the ray
    pub fn between(origin: Vec3, target: Vec3) -> Self {
        let direction = target - origin;
        let distance = direction.magnitude();

        let t_start = floats::BIG_EPSILON;
        let t_end = distance - floats::BIG_EPSILON;

        let ray = Ray::new2(origin, direction / distance, t_start, t_end);
        Self { ray }
    }

    /// Tests for unocclusion.
    ///
    /// # Arguments
    /// * `scene` - The scene to intersect against
    pub fn unoccluded(&self, scene: &Scene) -> bool {
        !scene.intersects(self.ray)
    }

    #[inline]
    pub fn decay(&self) -> Float {
        (self.ray.t_end - self.ray.t_start).powi(2)
    }
}
