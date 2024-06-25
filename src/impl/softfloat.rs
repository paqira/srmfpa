#[cfg(feature = "f128-softfloat")]
pub(crate) mod f128;
#[cfg(feature = "f16-softfloat")]
pub(crate) mod f16;
#[cfg(feature = "f32-softfloat")]
pub(crate) mod f32;
#[cfg(feature = "f64-softfloat")]
pub(crate) mod f64;
#[cfg(any(
    feature = "f128-softfloat",
    feature = "f16-softfloat",
    feature = "f32-softfloat",
    feature = "f64-softfloat"
))]
pub(crate) mod rounding_mode;
