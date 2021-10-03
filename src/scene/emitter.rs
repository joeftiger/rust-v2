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
}

impl Emitter {
    /// Computes the radiance of this emitter.
    ///
    /// # Constraints
    /// * `incident`: All values should be finite (neither infinite nor `NaN`).
    ///                Should be normalized.
    /// * `normal`: All values should be finite.
    ///              Should be normalized.
    ///
    /// # Arguments
    /// * `incident`: The incident on the surface of an object
    /// * `normal`: The normal on the surface of an object
    #[inline]
    pub fn radiance(&self, incident: Vec3, normal: Vec3) -> Spectrum {
        let cos_theta = incident.dot(normal);

        if cos_theta > 0.0 {
            self.emission
        } else {
            Spectrum::splat(0.0)
        }
    }

    /// Computes the radiance of this emitter.
    ///
    /// # Constraints
    /// * `incident`: All values should be finite (neither infinite nor `NaN`).
    ///                Should be normalized.
    /// * `normal`: All values should be finite.
    ///              Should be normalized.
    /// * `indices`: All values should be within `[0, `[Spectrum::size]`)`.
    ///
    /// # Arguments
    /// * `incident`: The incident on the surface of an object
    /// * `normal`: The normal on the surface of an object
    /// * `indices`: The spectral indices
    pub fn radiance_packet(
        &self,
        incident: Vec3,
        normal: Vec3,
        indices: &[usize; PACKET_SIZE],
    ) -> [Float; PACKET_SIZE] {
        let mut radiance = [0.0; PACKET_SIZE];

        let cos_theta = incident.dot(normal);
        if cos_theta > 0.0 {
            for i in 0..PACKET_SIZE {
                radiance[i] = self.emission[indices[i]];
            }
        }

        radiance
    }

    /// Computes the radiance of this emitter.
    ///
    /// # Constraints
    /// * `incident`: All values should be finite (neither infinite nor `NaN`).
    ///                Should be normalized.
    /// * `normal`: All values should be finite.
    ///              Should be normalized.
    /// * `index`: Should be within `[0, `[Spectrum::size]`)`.
    ///
    /// # Arguments
    /// * `incident`: The incident on the surface of an object
    /// * `normal`: The normal on the surface of an object
    /// * `index`: The spectral index
    pub fn radiance_lambda(&self, incident: Vec3, normal: Vec3, index: usize) -> Float {
        let cos_theta = incident.dot(normal);

        if cos_theta > 0.0 {
            self.emission[index]
        } else {
            0.0
        }
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
        let incident = occlusion.ray.direction;
        let radiance = self.radiance(-incident, surface_sample.normal);

        EmitterSample::new(radiance, incident, surface_sample.pdf, occlusion)
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
        let incident = occlusion.ray.direction;
        let radiances = self.radiance_packet(-incident, surface_sample.normal, indices);

        EmitterSample::new(radiances, incident, surface_sample.pdf, occlusion)
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
        let incident = occlusion_tester.ray.direction;
        let radiance = self.radiance_lambda(-incident, surface_sample.normal, index);

        EmitterSample::new(radiance, incident, surface_sample.pdf, occlusion_tester)
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
    pub pdf: Float,
    pub occlusion: OcclusionTester,
}
impl<T> EmitterSample<T> {
    pub const fn new(radiance: T, incident: Vec3, pdf: Float, occlusion: OcclusionTester) -> Self {
        Self {
            radiance,
            incident,
            pdf,
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
    /// * `origin` - All values should be finite (neither infinite nor `NaNN`).
    /// * `target` - All values should be finite.
    ///
    /// # Arguments
    /// * `origin` - The origin of the ray
    /// * `target` - The target of the ray
    pub fn between(origin: Vec3, target: Vec3) -> Self {
        let direction = target - origin;
        let distance = direction.magnitude();

        let t_start = floats::EPSILON;
        let t_end = distance - floats::EPSILON;

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
}
