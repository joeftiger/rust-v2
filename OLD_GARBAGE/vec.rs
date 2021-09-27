use crate::util::floats;
use crate::util::Index;
use core::fmt;
use core::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

macro_rules! vecs {
    ($name:ident => $size:expr, $t:ident, [$first:ident, $($var:ident),*]) => {
        #[derive(Copy, Clone, Default, Debug, PartialEq)]
        #[doc = concat!("A `", stringify!($size), "`-dimensional vector.")]
        pub struct $name {
            #[doc = concat!("The `", stringify!($first), "` component of the vector.")]
            pub $first: $t,
            $(
                #[doc = concat!("The `", stringify!($var), "` component of the vector.")]
                pub $var: $t
            ),*
        }

        impl $name {
            /// The `0`-vector.
            pub const ZERO: Self = Self::splat(0.0);
            /// The `1`-vector.
            pub const ONE: Self = Self::splat(1.0);

            /// Creates a new vector.
            #[inline]
            pub const fn new($first: $t, $($var: $t),*) -> Self {
                Self { $first, $($var),* }
            }

            /// Creates a new vector with a splatted value.
            #[inline]
            pub const fn splat(all: $t) -> Self {
                Self {
                    $first: all,
                    $($var: all),*
                }
            }

            #[doc = concat!(
                "Creates a new vector from an array.",
                "If the array has insufficient length, the remaining fields will be filled with",
                "the default value of [`", stringify!($t), "`]",
            )]
            pub fn from_partial(ar: &[$t]) -> Self {
                let mut index = Index::new();

                #[allow(unused_assignments)]
                Self {
                    $first: ar.get(index.get_and_inc()).copied().unwrap_or_default(),
                    $($var: ar.get(index.get_and_inc()).copied().unwrap_or_default()),*
                }
            }

            #[doc = concat!("Returns the absolute `", stringify!($name), "`")]
            #[inline]
            pub fn abs(&self) -> Self {
                Self::new(self.$first.abs(), $(self.$var.abs()),*)
            }

            /// Computes the dot product.
            #[inline]
            pub fn dot(&self, other: Self) -> $t {
                self.$first * other.$first
                $(
                    + self.$var * other.$var
                )*
            }

            /// Computes the dot product.
            #[inline]
            pub fn dot2(&self) -> $t {
                self.dot(*self)
            }

            /// Computes the angle between two vectors.
            pub fn angle_to(&self, other: Self) -> $t {
                $t::acos(self.dot(other) / (self.mag() * other.mag()))
            }

            /// Computes the magnitude.
            #[inline]
            pub fn mag(&self) -> $t {
                self.dot2().sqrt()
            }

            /// Normalizes the vector.
            #[inline]
            pub fn normalized(&self) -> Self {
                *self / self.mag()
            }

            /// Returns whether this vector is [approximately](floats::approx_eq) normalized.
            pub fn is_normalized(&self) -> bool {
                floats::approx_eq(1.0, self.mag())
            }

            /// Returns whether two vectors are [approximately](floats::approx_eq) the same.
            pub fn eq_approx(&self, other: Self) -> bool {
                floats::approx_eq(self.$first, other.$first)
                $(
                    && floats::approx_eq(self.$var, other.$var)
                )*
            }

            /// Applies a function to all vector fields.
            #[inline]
            pub fn apply<F>(&self, f: F) -> Self where F: Fn($t) -> $t {
                Self::new(
                    f(self.$first),
                    $(f(self.$var)),*
                )
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_tuple(stringify!($name))
                    .field(&self.$first)
                    $(.field(&self.$var))*
                    .finish()
            }
        }

        impl From<[$t; $size]> for $name {
            fn from(ar: [$t; $size]) -> Self {
                let mut index = Index::new();

                #[allow(unused_assignments)]
                Self {
                    $first: ar[index.get_and_inc()],
                    $($var: ar[index.get_and_inc()]),*
                }
            }
        }

        impl Neg for $name {
            type Output = Self;

            #[inline]
            fn neg(self) -> Self::Output {
                Self::new(-self.$first, $(-self.$var),*)
            }
        }

        impl Add for $name {
            type Output = Self;

            #[inline]
            fn add(self, rhs: Self) -> Self::Output {
                Self::new(self.$first + rhs.$first, $(self.$var + rhs.$var),*)
            }
        }

        impl AddAssign for $name {
            #[inline]
            fn add_assign(&mut self, rhs: Self) {
                self.$first += rhs.$first;
                $(self.$var += rhs.$var);*
            }
        }

        impl Sub for $name {
            type Output = Self;

            #[inline]
            fn sub(self, rhs: Self) -> Self::Output {
                Self::new(self.$first - rhs.$first, $(self.$var - rhs.$var),*)
            }
        }

        impl SubAssign for $name {
            #[inline]
            fn sub_assign(&mut self, rhs: Self) {
                self.$first -= rhs.$first;
                $(self.$var -= rhs.$var);*
            }
        }

        impl Mul for $name {
            type Output = Self;

            #[inline]
            fn mul(self, rhs: Self) -> Self::Output {
                Self::new(self.$first * rhs.$first, $(self.$var * rhs.$var),*)
            }
        }

        impl MulAssign for $name {
            #[inline]
            fn mul_assign(&mut self, rhs: Self) {
                self.$first *= rhs.$first;
                $(self.$var *= rhs.$var);*
            }
        }

        impl Mul<$t> for $name {
            type Output = Self;

            #[inline]
            fn mul(self, rhs: $t) -> Self::Output {
                Self::new(self.$first * rhs, $(self.$var * rhs),*)
            }
        }

        impl Mul<$name> for f64 {
            type Output = $name;

            #[inline]
            fn mul(self, rhs: $name) -> Self::Output {
                rhs * self
            }
        }

        impl MulAssign<$t> for $name {
            #[inline]
            fn mul_assign(&mut self, rhs: $t) {
                self.$first *= rhs;
                $(self.$var *= rhs);*
            }
        }

        impl Div for $name {
            type Output = Self;

            #[inline]
            fn div(self, rhs: Self) -> Self::Output {
                Self::new(self.$first / rhs.$first, $(self.$var / rhs.$var),*)
            }
        }

        impl DivAssign for $name {
            #[inline]
            fn div_assign(&mut self, rhs: Self) {
                self.$first /= rhs.$first;
                $(self.$var /= rhs.$var);*
            }
        }

        impl Div<$t> for $name {
            type Output = Self;

            #[inline]
            fn div(self, rhs: $t) -> Self::Output {
                Self::new(self.$first / rhs, $(self.$var / rhs),*)
            }
        }

        impl Div<$name> for $t {
            type Output = $name;

            #[inline]
            fn div(self, rhs: $name) -> Self::Output {
                $name::new(self / rhs.$first, $(self / rhs.$var),*)
            }
        }

        impl DivAssign<$t> for $name {
            #[inline]
            fn div_assign(&mut self, rhs: $t) {
                self.$first /= rhs;
                $(self.$var /= rhs);*
            }
        }

        impl Rem for $name {
            type Output = Self;

            #[inline]
            fn rem(self, rhs: Self) -> Self::Output {
                Self::new(self.$first % rhs.$first, $(self.$var % rhs.$var),*)
            }
        }

        impl RemAssign for $name {
            #[inline]
            fn rem_assign(&mut self, rhs: Self) {
                self.$first %= rhs.$first;
                $(self.$var %= rhs.$var);*
            }
        }

        impl Rem<$t> for $name {
            type Output = Self;

            #[inline]
            fn rem(self, rhs: $t) -> Self::Output {
                Self::new(self.$first % rhs, $(self.$var % rhs),*)
            }
        }

        impl RemAssign<f64> for $name {
            #[inline]
            fn rem_assign(&mut self, rhs: f64) {
                self.$first %= rhs;
                $(self.$var %= rhs);*
            }
        }
    };
}

vecs!(Vec2 => 2, f64, [x, y]);
impl Vec2 {
    /// The unit vector in `x` direction.
    pub const UNIT_X: Self = Self::new(1.0, 0.0);
    /// The unit vector in `y` direction.
    pub const UNIT_Y: Self = Self::new(0.0, 1.0);

    /// Extends this vector to a [Vec3] with given `z`.
    pub fn extend(&self, z: f64) -> Vec3 {
        Vec3::new(self.x, self.y, z)
    }
}

vecs!(Vec3 => 3, f64, [x, y, z]);
impl Vec3 {
    /// The unit vector in `x` direction.
    pub const UNIT_X: Self = Self::new(1.0, 0.0, 0.0);
    /// The unit vector in `y` direction.
    pub const UNIT_Y: Self = Self::new(0.0, 1.0, 0.0);
    /// The unit vector in `z` direction.
    pub const UNIT_Z: Self = Self::new(0.0, 0.0, 1.0);

    /// Computes the cross product.
    #[inline]
    pub fn cross(&self, other: Self) -> Self {
        Self::new(
            (self.y * other.z) + (-self.z * other.y),
            (self.z * other.x) + (-self.x * other.z),
            (self.x * other.y) + (-self.y * other.x),
        )
    }

    /// Computes the reflection along a normal.
    #[inline]
    pub fn reflect(&self, normal: Self) -> Self {
        *self - 2.0 * self.dot(normal) * normal
    }

    /// Computes the refraction along a normal with refraction index `eta`.
    ///
    /// # Returns
    /// - `None`: if total internal refraction occurs
    /// - `Some`: otherwise
    pub fn refract(&self, normal: Self, eta: f64) -> Option<Self> {
        let cos_theta_i = self.dot(normal);
        let sin2_theta_i = f64::max(0.0, 1.0 - cos_theta_i * cos_theta_i);
        let sin2_theta_t = eta * eta * sin2_theta_i;

        if sin2_theta_t >= 1.0 {
            return None;
        }

        let cos_theta_t = f64::sqrt(1.0 - sin2_theta_t);

        let refraction = eta * -*self + (eta * cos_theta_i - cos_theta_t) * normal;
        Some(refraction)
    }

    /// Extends this vector to a [Vec4] with given `w`.
    pub fn extend(&self, w: f64) -> Vec4 {
        Vec4::new(self.x, self.y, self.z, w)
    }

    /// Truncates this vector to a [Vec2], dropping the `z` value.
    pub fn truncate(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

vecs!(Vec4 => 4, f64, [x, y, z, w]);
impl Vec4 {
    /// The unit vector in `x` direction.
    pub const UNIT_X: Self = Self::new(1.0, 0.0, 0.0, 0.0);
    /// The unit vector in `y` direction.
    pub const UNIT_Y: Self = Self::new(0.0, 1.0, 0.0, 0.0);
    /// The unit vector in `z` direction.
    pub const UNIT_Z: Self = Self::new(0.0, 0.0, 1.0, 0.0);
    /// The unit vector in `z` direction.
    pub const UNIT_W: Self = Self::new(0.0, 0.0, 0.0, 1.0);

    /// Truncates this vector to a [Vec3], dropping the `w` value.
    pub fn truncate(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod vec3_tests {
    use crate::geometry::vec::Vec3;
    use crate::util::floats::approx_eq;
    use core::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI};

    #[test]
    fn debug_display() {
        assert_eq!(
            "Vec3 { x: 1.0, y: 1.0, z: 1.0 }",
            format!("{:?}", Vec3::ONE)
        );
    }

    #[test]
    fn display() {
        assert_eq!("Vec3(1.0, 1.0, 1.0)", format!("{}", Vec3::ONE));
    }

    #[test]
    fn new() {
        for i in 0..10 {
            let f = i as f64;
            assert_eq!(
                Vec3 {
                    x: f,
                    y: f * f,
                    z: f * f * f,
                },
                Vec3::new(f, f * f, f * f * f),
            )
        }
    }

    #[test]
    fn splat() {
        for i in 0..10 {
            let f = i as f64;
            assert_eq!(Vec3 { x: f, y: f, z: f }, Vec3::splat(f),)
        }
    }

    #[test]
    fn from_partial() {
        assert_eq!(
            Vec3 {
                x: f64::default(),
                y: f64::default(),
                z: f64::default(),
            },
            Vec3::from_partial(&[]),
        );
        assert_eq!(
            Vec3 {
                x: 1.0,
                y: f64::default(),
                z: f64::default(),
            },
            Vec3::from_partial(&[1.0]),
        );
        assert_eq!(
            Vec3 {
                x: 1.0,
                y: 2.0,
                z: f64::default(),
            },
            Vec3::from_partial(&[1.0, 2.0]),
        );
        assert_eq!(
            Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            Vec3::from_partial(&[1.0, 2.0, 3.0]),
        );
        assert_eq!(
            Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            Vec3::from_partial(&[1.0, 2.0, 3.0, 4.0]),
        );
    }

    #[test]
    fn abs() {
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    let x = x as f64;
                    let y = y as f64;
                    let z = z as f64;

                    assert_eq!(
                        Vec3 {
                            x: x.abs(),
                            y: y.abs(),
                            z: z.abs()
                        },
                        Vec3::new(x, y, z).abs(),
                    )
                }
            }
        }
    }

    #[test]
    fn dot() {
        assert_eq!(0.0, Vec3::ZERO.dot(Vec3::ONE));
        assert!(approx_eq(3.0, Vec3::ONE.dot(Vec3::ONE)));
        assert!(approx_eq(12.0, Vec3::ONE.dot(Vec3::new(2.0, 4.0, 6.0))));
    }

    #[test]
    fn dot2() {
        assert!(approx_eq(3.0, Vec3::ONE.dot2()));
        assert!(approx_eq(14.0, Vec3::new(1.0, 2.0, 3.0).dot2()));
    }

    #[test]
    fn angle_to() {
        assert!(approx_eq(0.0, Vec3::UNIT_X.angle_to(Vec3::UNIT_X)));
        assert!(approx_eq(PI, Vec3::UNIT_X.angle_to(-Vec3::UNIT_X)));

        assert!(approx_eq(FRAC_PI_2, Vec3::UNIT_X.angle_to(Vec3::UNIT_Y)));
        assert!(approx_eq(FRAC_PI_2, Vec3::UNIT_X.angle_to(Vec3::UNIT_Z)));
        assert!(approx_eq(FRAC_PI_2, Vec3::UNIT_Y.angle_to(Vec3::UNIT_Z)));

        assert!(approx_eq(
            FRAC_PI_4,
            Vec3::UNIT_X.angle_to(Vec3::UNIT_Y + Vec3::UNIT_X)
        ));
    }

    #[test]
    fn mag() {
        assert!(approx_eq(1.0, Vec3::UNIT_X.mag()));
        assert!(approx_eq(1.0, Vec3::UNIT_Y.mag()));
        assert!(approx_eq(1.0, Vec3::UNIT_Z.mag()));

        assert!(approx_eq(2_f64.sqrt(), Vec3::new(1.0, 1.0, 0.0).mag()));
        assert!(approx_eq(3_f64.sqrt(), Vec3::ONE.mag()));

        assert!(approx_eq(14_f64.sqrt(), Vec3::new(1.0, 2.0, 3.0).mag()));
    }

    #[test]
    fn normalized() {
        assert!(Vec3::new(2.0, 2.0, 2.0)
            .normalized()
            .eq_approx(Vec3::splat(1.0 / 3_f64.sqrt())));
    }
}
