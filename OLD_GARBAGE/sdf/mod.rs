pub trait MinFunction {
    fn min(&self, a: f64, b: f64) -> f64;
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Min;
impl MinFunction for Min {
    #[inline]
    fn min(&self, a: f64, b: f64) -> f64 {
        a.min(b)
    }
}

/// Quadratic smooth minimum.
///
/// `k = 0.1` is the default.
///
/// # Continuity
/// Continuous up to `C1`.
#[derive(Copy, Clone, Debug)]
pub struct QuadraticSmoothMin {
    pub k: f64,
}
impl QuadraticSmoothMin {
    pub const fn new(k: f64) -> Self {
        Self { k }
    }
}
impl Default for QuadraticSmoothMin {
    #[inline]
    fn default() -> Self {
        Self::new(0.1)
    }
}
impl MinFunction for QuadraticSmoothMin {
    fn min(&self, a: f64, b: f64) -> f64 {
        let h = f64::max(self.k - (a - b).abs(), 0.0) / self.k;
        a.min(b) - h * h * self.k * (1.0 / 4.0)
    }
}

/// Cubic smooth minimum
///
/// `k = 0.1` is the default.
///
/// # Continuity
/// Continuous up to `C2`.
#[derive(Copy, Clone, Debug)]
pub struct CubicSmoothMin {
    pub k: f64,
}
impl CubicSmoothMin {
    pub const fn new(k: f64) -> Self {
        Self { k }
    }
}
impl Default for CubicSmoothMin {
    #[inline]
    fn default() -> Self {
        Self::new(0.1)
    }
}
impl MinFunction for CubicSmoothMin {
    fn min(&self, a: f64, b: f64) -> f64 {
        let h = f64::max(self.k - (a - b).abs(), 0.0) / self.k;
        a.min(b) - h * h * h * self.k * (1.0 / 6.0)
    }
}
