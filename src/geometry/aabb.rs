use crate::geometry::{Geometry, Intersection, Ray};
use crate::{Float, Vec3};
use cgmath::{Bounded, ElementWise, InnerSpace};
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    /// Creates a new cube.
    ///
    /// # Example
    /// ```rust
    /// use rust_v2::geometry::{Aabb, Vec3};
    ///
    /// let aabb = Aabb::new(Vec3::ZERO, Vec3::ONE);
    /// assert_eq!(Vec3::ZERO, aabb.min);
    /// assert_eq!(Vec3::ONE, aabb.max);
    /// ```
    #[inline]
    pub const fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    #[inline]
    pub const fn unit() -> Self {
        Self::new(Vec3::new(0., 0., 0.), Vec3::new(1., 1., 1.))
    }

    /// Returns the "empty cube".
    ///
    /// This cube is effectively **invalid**, but might be useful to compute bounding boxes
    /// of many objects, taking this empty cube as starting point.
    ///
    /// # Example
    /// ```rust
    /// use rust_v2::geometry::{Aabb, Vec3};
    /// use cgmath::Bounded;
    ///
    /// let aabb = Aabb::empty();
    /// assert_eq!(Vec3::max_value(), aabb.min);
    /// assert_eq!(Vec3::min_value(), aabb.max);
    /// ```
    #[inline]
    pub fn empty() -> Self {
        Self::new(Vec3::max_value(), Vec3::min_value())
    }

    /// Returns the "maximum cube".
    ///
    /// # Example
    /// ```rust
    /// use rust_v2::geometry::{Aabb, Vec3};
    /// use cgmath::Bounded;
    ///
    /// let aabb = Aabb::max();
    /// assert_eq!(Vec3::min_value(), aabb.min);
    /// assert_eq!(Vec3::max_value(), aabb.max);
    /// ```
    #[inline]
    pub fn max() -> Self {
        Self::new(Vec3::min_value(), Vec3::max_value())
    }

    /// Returns the size of this cube in all 3 dimensions.
    #[inline]
    pub fn size(&self) -> Vec3 {
        self.max - self.min
    }

    /// Returns the volume of this cube.
    #[inline]
    pub fn volume(&self) -> Float {
        let size = self.size();
        size.x * size.y * size.z
    }

    /// Returns the center of this cube.
    #[inline]
    pub fn center(&self) -> Vec3 {
        (self.min + self.max) / 2.0
    }

    /// Joins this cube with another one, effectively creating a cube spanning both cubes.
    ///
    /// # Arguments
    /// * `other`: Another cube
    ///
    /// # Returns
    /// * The outer join
    #[must_use]
    pub fn join(&self, other: Self) -> Self {
        let min = self.min.zip(other.min, |a, b| a.min(b));
        let max = self.max.zip(other.max, |a, b| a.max(b));
        Self::new(min, max)
    }

    /// Joins this cube with a vector, effectively creating a cube spanning both.
    ///
    /// # Arguments
    /// * `other`: a vector
    ///
    /// # Returns
    /// * The outer join
    #[must_use]
    pub fn join2(&self, other: Vec3) -> Self {
        let min = self.min.zip(other, |a, b| a.min(b));
        let max = self.max.zip(other, |a, b| a.max(b));
        Self::new(min, max)
    }
}

#[typetag::serde]
impl Geometry for Aabb {
    fn contains(&self, point: Vec3) -> Option<bool> {
        Some(
            self.min.x <= point.x
                && point.x <= self.max.x
                && self.min.y <= point.y
                && point.y <= self.max.y
                && self.min.z <= point.z
                && point.z <= self.max.z,
        )
    }

    fn bounds(&self) -> Aabb {
        *self
    }

    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        let t1 = (self.min - ray.origin).div_element_wise(ray.direction);
        let t2 = (self.max - ray.origin).div_element_wise(ray.direction);

        let vec_min = t1.zip(t2, |a, b| a.min(b));
        let vec_max = t1.zip(t2, |a, b| a.max(b));

        let t_min = vec_min.x.max(vec_min.y).max(vec_min.z);
        let t_max = vec_max.x.min(vec_max.y).min(vec_max.z);

        if t_min > t_max {
            return None;
        }

        let t = if ray.contains(t_min) {
            t_min
        } else if ray.contains(t_max) {
            t_max
        } else {
            return None;
        };

        let point = ray.at(t);
        let half_size = self.size() / 2.0;
        let center = self.min + half_size;
        let direction = point - center;
        let bias = 1.01;

        // Used for debugging. The bias fom
        // let a = direction * bias;
        // let b = a.div_element_wise(half_size);
        // let c = b.map(|f| f as i64);
        // let d = c.map(|f| f as Float);
        // let normal = d.normalize();
        let normal = (direction * bias)
            .div_element_wise(half_size)
            .map(|f| f as i64 as Float)
            .normalize();

        Some(Intersection::new(point, normal, ray.direction, t))
    }

    fn intersects(&self, ray: Ray) -> bool {
        let t1 = (self.min - ray.origin).div_element_wise(ray.direction);
        let t2 = (self.max - ray.origin).div_element_wise(ray.direction);

        let vec_min = t1.zip(t2, |a, b| a.min(b));
        let vec_max = t1.zip(t2, |a, b| a.max(b));

        let t_min = vec_min.x.max(vec_min.y).max(vec_min.z);
        let t_max = vec_max.x.min(vec_max.y).min(vec_max.z);

        t_min <= t_max && (ray.contains(t_min) || ray.contains(t_max))
    }
}
