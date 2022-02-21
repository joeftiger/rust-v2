use crate::Float;

pub trait Lerp {
    /// Lerps `start` to `end` by `self`.
    #[must_use]
    fn lerp(self, start: Self, end: Self) -> Self;
}

macro_rules! impl_lerp {
    ($($t:ident),+) => {$(
        impl Lerp for $t {
            #[must_use]
            fn lerp(self, start: Self, end: Self) -> Self {
                // consistent
                if start == end {
                    start

                // exact/monotonic
                } else {
                    self.mul_add(end, (-self).mul_add(start, start))
                }
            }
        })+
    };
}

impl_lerp!(f32, f64);

/// Solves a quadratic equation, handling generics.
///
/// `a`x^2 + `b`x + `c`
///
/// # Constraints
/// * `a`: Should be finite (neither infinite nor `NaN`).
/// * `b`: Should be finite.
/// * `c`: Should be finite.
///
/// # Arguments
/// * `a`: The parameter for `x^2`
/// * `b`: The parameter for `x`
/// * `c`: The constant parameter
///
/// # Returns
/// * `Option<(f32, f32)>`: The solutions in ascending order (if any)
#[inline]
#[must_use]
pub fn solve_quadratic(a: Float, b: Float, c: Float) -> Option<(Float, Float)> {
    debug_assert!(a.is_finite());
    debug_assert!(b.is_finite());
    debug_assert!(c.is_finite());

    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return None;
    }

    let q = -0.5 * (b + Float::copysign(discriminant.sqrt(), b));
    let t0 = q / a;
    let t1 = c / q;

    match t0 < t1 {
        true => Some((t0, t1)),
        false => Some((t1, t0)),
    }

    // NOTE: below would be "faster in a corner case" but the discriminant will never get 0 with
    // very high certainty.
    //
    // match discriminant.total_cmp(&0.0) {
    //     Ordering::Less => None,
    //     Ordering::Equal => {
    //         let t = -0.5 * b / a;
    //         Some((t, t))
    //     }
    //     Ordering::Greater => {
    //         let q = -0.5 * (b + Float::copysign(discriminant.sqrt(), b));
    //         let t0 = q / a;
    //         let t1 = c / q;
    //
    //         match t0 < t1 {
    //             true => Some((t0, t1)),
    //             false => Some((t1, t0)),
    //         }
    //     }
    // }
}
