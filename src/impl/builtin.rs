#[cfg(feature = "f128")]
pub(crate) mod f128;
#[cfg(feature = "f16")]
pub(crate) mod f16;
#[cfg(not(feature = "f32-softfloat"))]
pub(crate) mod f32;
#[cfg(not(feature = "f64-softfloat"))]
pub(crate) mod f64;
