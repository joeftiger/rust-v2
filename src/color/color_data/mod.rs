pub mod spectral;
pub mod srgb;
pub mod xyz;

/// micro meters
pub const LAMBDA_START: f64 = 0.38;
/// micro meters
pub const LAMBDA_END: f64 = 0.73;
/// micro meters
pub const LAMBDA_RANGE: f64 = LAMBDA_END - LAMBDA_START;
pub const LAMBDA_STEP: usize = 10;
pub const LAMBDA_NUM: usize = 36; //((LAMBDA_END - LAMBDA_START) / LAMBDA_STEP as f64 + 0.1) as u16;
