#[cfg(feature = "f128_softfloat")]
pub(crate) mod f128;
#[cfg(feature = "f16_softfloat")]
pub(crate) mod f16;
#[cfg(feature = "f32_softfloat")]
pub(crate) mod f32;
#[cfg(feature = "f64_softfloat")]
pub(crate) mod f64;
#[cfg(any(
    feature = "f128_softfloat",
    feature = "f16_softfloat",
    feature = "f32_softfloat",
    feature = "f64_softfloat"
))]
pub(crate) mod rounding_mode;
