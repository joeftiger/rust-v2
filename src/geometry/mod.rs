pub use aabb::*;
pub use bubble::*;
pub use disk::*;
pub use mesh::*;
pub use plane::*;
pub use point::*;
pub use ray::*;
pub use sphere::*;

use crate::util::floats::BIG_EPSILON;
use crate::{Float, Vec3};
use cgmath::InnerSpace;

pub mod aabb;
pub mod bubble;
pub mod bvh;
pub mod disk;
pub mod mesh;
pub mod plane;
pub mod point;
pub mod ray;
pub mod sphere;

/// The unit vectors in all directions.
#[rustfmt::skip]
pub const UNIT_VECTORS: [Vec3; 6] = [
    Vec3 { x:  1.0, y:  0.0, z:  0.0 },
    Vec3 { x: -1.0, y:  0.0, z:  0.0 },
    Vec3 { x:  0.0, y:  1.0, z:  0.0 },
    Vec3 { x:  0.0, y: -1.0, z:  0.0 },
    Vec3 { x:  0.0, y:  0.0, z:  1.0 },
    Vec3 { x:  0.0, y:  0.0, z: -1.0 },
];

#[inline]
pub fn min2(a: Vec3, b: Vec3) -> Vec3 {
    Vec3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z))
}
#[inline]
pub fn min3(a: Vec3, b: Vec3, c: Vec3) -> Vec3 {
    min2(min2(a, b), c)
}
#[inline]
pub fn max2(a: Vec3, b: Vec3) -> Vec3 {
    Vec3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z))
}
#[inline]
pub fn max3(a: Vec3, b: Vec3, c: Vec3) -> Vec3 {
    max2(max2(a, b), c)
}
#[inline]
pub fn min_val(v: Vec3) -> Float {
    v.x.min(v.y).min(v.z)
}
#[inline]
pub fn max_val(v: Vec3) -> Float {
    v.x.max(v.y).max(v.z)
}
#[inline]
pub fn max_index(v: Vec3) -> usize {
    if v.x > v.y {
        if v.x > v.z {
            return 0;
        }
    } else if v.y > v.z {
        return 1;
    }

    2
}
#[inline]
pub fn abs(v: Vec3) -> Vec3 {
    Vec3::new(v.x.abs(), v.y.abs(), v.z.abs())
}

/// Offsets a point by an epsilon into the normal direction, depending on the angle to the given
/// direction.
///
/// # Constraints
/// * `point`: ALl values should be finite (neither infinite nor `NaN`).
/// * `normal`: All values should be finite.
///              Should be normalized.
/// * `direction`: Should be finite.
///                 (Does not need to be normalized.)
///
/// # Arguments
/// * `point`: The starting point
/// * `normal`: The normal vector to offset towards
/// * `direction`: The direction helper to decide whether to invert the normal
///
/// # Returns
/// * The offset point
pub fn offset_point(point: Vec3, normal: Vec3, direction: Vec3) -> Vec3 {
    let offset = if direction.dot(normal) >= 0.0 {
        normal * BIG_EPSILON
    } else {
        normal * -BIG_EPSILON
    };

    point + offset
}

/// Offsets a point by an epsilon into the normal direction, depending on the angle to the given
/// direction and creates a ray from it.
///
/// If the parameter `direction` shows into the same general direction of this intersection
/// normal, the ray origin will be offset by an epsilon into the intersection normal.
/// Otherwise, the opposite normal will be used.
///
/// # Constraints
/// * `point`: ALl values should be finite (neither infinite nor `NaN`).
/// * `normal`: All values should be finite.
///              Should be normalized.
/// * `direction`: Should be finite.
///                 Should be normalized.
///
/// # Arguments
/// * `point`: The starting point
/// * `normal`: The normal vector to offset towards
/// * `direction`: The direction of the ray
///
/// # Returns
/// * Ray from this intersection, offset by an epsilon
pub fn offset_ray_towards(point: Vec3, normal: Vec3, direction: Vec3) -> Ray {
    let origin = offset_point(point, normal, direction);

    Ray::new(origin, direction)
}

/// Offsets a point by an epsilon into the normal direction, depending on the angle to the given
/// direction and creates a ray to the target from it.
///
/// If the direction to the parameter `target` shows into the same general direction of this
/// intersection normal, the ray origin will be offset by an epsilon into the intersection
/// normal.
/// Otherwise, the opposite normal will be used.
///
/// # Constraints
/// * `point`: ALl values should be finite (neither infinite nor `NaN`).
/// * `normal`: All values should be finite.
///              Should be normalized.
/// * `target`: Should be finite.
///
/// # Arguments
/// * `point`: The starting point
/// * `normal`: The normal vector to offset towards
/// * `target`: The target position
///
/// # Returns
/// * Ray from this intersection to the target, offset by an epsilon
pub fn offset_ray_to(point: Vec3, normal: Vec3, target: Vec3) -> Ray {
    let dir = target - point;
    let origin = offset_point(point, normal, dir);
    let direction = target - origin;

    Ray::new2(origin, direction.normalize(), 0.0, direction.magnitude())
}

/// Converts spherical coordinates to cartesian coordinates in the given frame wth following description:
/// * `frame.x_axis`: to your right
/// * `frame.y_axis`: to your top
/// * `frame.z_axis`: towards you
///
/// # Constraints
/// * `theta`: Should be within `[0, 2π]`.
/// * `phi`: Should be within `[0, π]`.
///
/// # Arguments
/// * `theta`: The angle between the `z`-axis and the spherical direction in the `zx` plane
/// * `phi`: The angle between the  `y`-axis and the spherical direction
/// * `frame`: The coordinate system/frame to use
///
/// # Returns
/// * The corresponding cartesian vector
pub fn spherical_to_cartesian_frame(theta: Float, phi: Float, frame: &CoordinateSystem) -> Vec3 {
    let (sin_theta, cos_theta) = theta.sin_cos();
    let (sin_phi, cos_phi) = phi.sin_cos();

    spherical_to_cartesian_frame_trig(sin_theta, cos_theta, sin_phi, cos_phi, frame)
}

/// Converts spherical coordinates to cartesian coordinates in the given frame wth following description:
/// * `frame.x_axis`: to your right
/// * `frame.y_axis`: to your top
/// * `frame.z_axis`: towards you
///
/// To make below descriptions easier, we define the following:
/// * `theta`: The angle between the `z`-axis and the spherical direction in the `zx` plane.
/// * `phi`: The angle between the  `y`-axis and the spherical direction.
///
/// # Constraints
/// * `sin_theta`: Should be within `[-1, 1]`
/// * `cos_theta`: Should be within `[-1, 1]`
/// * `sin_phi`: Should be within `[-1, 1]`
/// * `cos_phi`: Should be within `[-1, 1]`
///
/// # Arguments
/// * `sin_theta`: The sine of `theta`
/// * `cos_theta`: The cosine of `theta`
/// * `sin_phi`: The sine of `phi`
/// * `cos_phi`: The cosine of `phi`
/// * `frame`: The coordinate system/frame to use
///
/// # Returns
/// * The corresponding cartesian vector
pub fn spherical_to_cartesian_frame_trig(
    sin_theta: Float,
    cos_theta: Float,
    sin_phi: Float,
    cos_phi: Float,
    frame: &CoordinateSystem,
) -> Vec3 {
    let x = frame.x_axis * sin_phi * sin_theta;
    let y = frame.y_axis * cos_phi;
    let z = frame.z_axis * sin_phi * cos_theta;

    x + y + z
}

#[inline]
pub fn spherical_to_cartesian1(sin_theta: Float, cos_theta: Float, phi: Float) -> Vec3 {
    Vec3::new(sin_theta * phi.cos(), sin_theta * phi.sin(), cos_theta)
}

/// Converts spherical coordinates to cartesian coordinates in the following describe frame:
/// * x-axis: to your right
/// * y-axis: to your top
/// * z-axis: towards you
///
/// # Constraints
/// * `theta`: Should be within `[0, 2π]`.
/// * `phi`: Should be within `[0, π]`.
///
/// # Arguments
/// * `theta`: The angle between the `z`-axis and the spherical direction in the `zx` plane.
/// * `phi`: The angle between the  `y`-axis and the spherical direction .
///
/// # Returns
/// * The corresponding cartesian vector
pub fn spherical_to_cartesian(theta: Float, phi: Float) -> Vec3 {
    let (sin_theta, cos_theta) = theta.sin_cos();
    let (sin_phi, cos_phi) = phi.sin_cos();

    spherical_to_cartesian_trig(sin_theta, cos_theta, sin_phi, cos_phi)
}

/// Converts spherical coordinates to cartesian coordinates in the following described frame:
/// * x-axis: to your right
/// * y-axis: to your top
/// * z-axis: towards you
///
/// To make below descriptions easier, we define the following:
/// * `theta`: The angle between the `z`-axis and the spherical direction in the `zx` plane.
/// * `phi`: The angle between the  `y`-axis and the spherical direction .
///
/// # Constraints
/// * `sin_theta`: Should be within `[-1, 1]`
/// * `cos_theta`: Should be within `[-1, 1]`
/// * `sin_phi`: Should be within `[-1, 1]`
/// * `cos_phi`: Should be within `[-1, 1]`
///
/// # Arguments
/// * `sin_theta`: The sine of `theta`
/// * `cos_theta`: The cosine of `theta`
/// * `sin_phi`: The sine of `phi`
/// * `cos_phi`: The cosine of `phi`
///
/// # Returns
/// * The corresponding cartesian vector
pub fn spherical_to_cartesian_trig(
    sin_theta: Float,
    cos_theta: Float,
    sin_phi: Float,
    cos_phi: Float,
) -> Vec3 {
    let x = sin_phi * sin_theta;
    let y = cos_phi;
    let z = sin_phi * cos_theta;

    Vec3::new(x, y, z)
}

/// A coordinate system represents 3 (orthogonal) vectors in 3D space.
#[derive(Copy, Clone, Debug)]
pub struct CoordinateSystem {
    pub x_axis: Vec3,
    pub y_axis: Vec3,
    pub z_axis: Vec3,
}

impl Default for CoordinateSystem {
    fn default() -> Self {
        Self::new(Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z())
    }
}

impl CoordinateSystem {
    /// Creates a new coordinate system.
    ///
    /// # Constraints
    /// * `x`: All values must be finite (neither infinite nor `NaN`).
    ///          Should be normalized.
    /// * `y`: All values must be finite.
    ///          Should be normalized.
    /// * `z`: All values must be finite.
    ///          Should be normalized.
    ///
    /// # Arguments
    /// * `x`: The first vector
    /// * `y`: The second vector
    /// * `z`: The third vector
    pub fn new(x: Vec3, y: Vec3, z: Vec3) -> Self {
        Self {
            x_axis: x,
            y_axis: y,
            z_axis: z,
        }
    }

    /// Creates a new coordinate system around the given `x` direction vector.
    ///
    /// # Constraints
    /// * `x_axis`: All values must be finite (neither infinite nor `NaN`).
    ///              Should be normalized.
    ///
    /// # Arguments
    /// * `x_axis`: The x direction vector
    // TODO: Make more efficient
    pub fn from_x(x_axis: Vec3) -> Self {
        if x_axis == Vec3::unit_x() {
            Self::new(Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z())
        } else {
            let z = x_axis.cross(Vec3::unit_x()).normalize();
            let y = z.cross(x_axis).normalize();

            Self::new(x_axis, y, z)
        }
    }

    /// Creates a new coordinate system around the given `y` direction vector.
    ///
    /// # Constraints
    /// * `y_axis`: All values must be finite (neither infinite nor `NaN`).
    ///              Should be normalized.
    ///
    /// # Arguments
    /// * `y_axis`: The y direction vector
    // TODO: Make more efficient
    pub fn from_y(y_axis: Vec3) -> Self {
        let s = if y_axis.x.abs() > y_axis.y.abs() {
            let l = (y_axis.x * y_axis.x + y_axis.z * y_axis.z).sqrt();
            Vec3::new(-y_axis.z / l, 0.0, y_axis.x / l)
        } else {
            let l = (y_axis.y * y_axis.y + y_axis.z * y_axis.z).sqrt();
            Vec3::new(0.0, y_axis.z / l, -y_axis.y / l)
        };
        Self::new(s, y_axis, y_axis.cross(s))
    }

    /// Creates a new coordinate system around the given `z` direction vector.
    ///
    /// # Constraints
    /// * `z_axis`: All values must be finite (neither infinite nor `NaN`).
    ///              Should be normalized.
    ///
    /// # Arguments
    /// * `z_axis`: The z direction vector
    // TODO: Make more efficient
    pub fn from_z(z_axis: Vec3) -> Self {
        if z_axis == Vec3::unit_z() {
            Self::new(Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z())
        } else {
            let x = z_axis.cross(Vec3::unit_z()).normalize();
            let y = z_axis.cross(x).normalize();

            Self::new(x, y, z_axis)
        }
    }
}

#[derive(Copy, Clone)]
pub struct Intersection {
    pub point: Vec3,
    pub normal: Vec3,
    pub incoming: Vec3,
    pub t: Float,
}

impl Intersection {
    pub fn point(&self) -> Vec3 {
        self.normal
    }

    pub const fn new(point: Vec3, normal: Vec3, incoming: Vec3, t: Float) -> Self {
        Self {
            point,
            normal,
            incoming,
            t,
        }
    }
}

#[typetag::serde]
pub trait Geometry: Send + Sync {
    /// Returns whether this geometry contains the given point.
    ///
    /// # Arguments
    /// - `point`: a point in space
    ///
    /// # Returns
    /// - `None`: no implementation for this check
    /// - `Some(true)`: this geometry contains the point
    /// - `Some(false): this geometry does not contain the point
    fn contains(&self, point: Vec3) -> Option<bool>;

    /// Returns the bounding box.
    ///
    /// # Returns
    /// - `Aabb`: the bounding box
    fn bounds(&self) -> Aabb;

    /// Intersects a ray with this geometry.
    ///
    /// # Arguments
    /// - `ray`: the ray to intersect
    ///
    /// # Returns
    /// - `None`: no intersection happens
    /// - `Some`: the intersection details
    fn intersect(&self, ray: Ray) -> Option<Intersection>;

    /// Returns whether a ray intersects with this geometry.
    ///
    /// # Arguments
    /// - `ray`: the ray to intersect
    ///
    /// # Returns
    /// - `bool`: whether an intersection occurs
    ///
    /// # Default implementation
    /// Returns whether [Self::intersect] is some.
    fn intersects(&self, ray: Ray) -> bool {
        self.intersect(ray).is_some()
    }
}
