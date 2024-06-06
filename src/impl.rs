#[cfg(any(feature = "f128-arithmetics", feature = "f128-math"))]
pub mod f128;
#[cfg(any(feature = "f16-arithmetics", feature = "f16-math"))]
pub mod f16;
pub mod f32;
pub mod f64;

/// wrapper of panic!(..)
#[cold]
pub(crate) fn error() -> ! {
    panic!("fail to set/restore rounding mode")
}
