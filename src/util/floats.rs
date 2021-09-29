use crate::Float;

pub const EPSILON: Float = 16.0 * Float::EPSILON;
pub const BIG_EPSILON: Float = 128.0 * Float::EPSILON;

#[inline]
pub fn approx_eq(a: Float, b: Float) -> bool {
    approx_eq_epsilon(a, b, EPSILON)
}

pub fn approx_eq_epsilon(a: Float, b: Float, epsilon: Float) -> bool {
    let a_abs = a.abs();
    let b_abs = b.abs();
    let diff = (b - a).abs();

    // shortcut, handles infinities
    #[allow(clippy::float_cmp)]
    if a == b {
        true
    }
    // a or b is zero or both are extremely close to it
    // relative error is less meaningful here
    else if a == 0.0 || b == 0.0 || (a_abs + b_abs) < Float::MIN_POSITIVE {
        diff < epsilon * Float::MIN_POSITIVE
    }
    // use relative error
    else {
        diff / Float::MAX.min(a_abs + b_abs) < epsilon
    }
}

/// Returns the scaling parameter of `value` in-between `[start, end]`.
/// If `value` is on the outside like `)start, end(`, the returned value will then be in `)0, 1(`.
#[inline]
pub fn lerp_inv(value: Float, start: Float, end: Float) -> Float {
    (value - start) / (end - start)
}
