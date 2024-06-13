#[cfg(not(all(
    feature = "f16-softfloat",
    feature = "f32-softfloat",
    feature = "f64-softfloat",
    feature = "f128-softfloat"
)))]
pub(crate) mod builtin;
#[cfg(any(
    feature = "f16-softfloat",
    feature = "f32-softfloat",
    feature = "f64-softfloat",
    feature = "f128-softfloat"
))]
pub(crate) mod softfloat;

#[cfg(any(feature = "f128", feature = "f128-softfloat"))]
pub mod f128;
#[cfg(any(feature = "f16", feature = "f16-softfloat"))]
pub mod f16;
pub mod f32;
pub mod f64;

/// wrapper of panic!(..)
#[cold]
#[allow(dead_code)]
pub(crate) fn error() -> ! {
    panic!("fail to set/restore rounding mode")
}
