#![cfg_attr(any(feature = "f16-arithmetics", feature = "f16-math"), feature(f16))]
#![cfg_attr(
    any(feature = "f128-arithmetics", feature = "f128-math"),
    feature(f128)
)]
#![cfg_attr(test, feature(float_next_up_down))]

//! Floating point arithmetics (add, sub, mul, div and mul_add) and `sqrt` with specified rounding mode.
//!
//! ```
//! use fpa_specr::prelude::*;
//!
//! // Support add, sub, mul, div, mul_add (fma) and sqrt
//!
//! assert_eq!(0.1f64.round_ties_even_add(0.2), 0.30000000000000004);
//! assert_eq!(0.1f64.ciel_add(0.2), 0.30000000000000004);
//! assert_eq!(0.1f64.floor_add(0.2), 0.3);
//! assert_eq!(0.1f64.trunc_add(0.2), 0.3);
//!
//! assert_eq!((-0.1f64).round_ties_even_add(-0.2), -0.30000000000000004);
//! assert_eq!((-0.1f64).ciel_add(-0.2), -0.3);
//! assert_eq!((-0.1f64).floor_add(-0.2), -0.30000000000000004);
//! assert_eq!((-0.1f64).trunc_add(-0.2), -0.3);
//!
//! // Generic ops
//! assert_eq!(0.1.round_add(0.2, &RoundingMode::NearestTiesEven), 0.30000000000000004);
//!
//! // Functions are available
//! use fpa_specr::f64::{ciel_add, floor_add};
//! assert_eq!(ciel_add(0.1, 0.2), 0.30000000000000004);
//! assert_eq!(floor_add(0.1, 0.2), 0.3);
//! ```
//!
//! # Notes on Correctness and Configuration
//!
//! Correctness depends on using C compiler and libc,
//! because APIs of [`fpa_specr`][mod@self] call floating point ops implemented in C with `<fenv.h>`.
//!
//! [`fpa_specr`][mod@self] uses the default C compiler options of `cc`,
//! and does not (explicitly) specify other options.
//! It is recommended to pass corresponding options (`-frounding-math`, `-std=c23`, `-mfma` etc.)
//! to obtain the desired result.
//! See [`cc` crate document][cc_doc] for detail of configuration.
//!
//! [cc_doc]: https://docs.rs/cc/latest/cc/index.html

use core::ffi::c_int;

mod r#impl;
mod internal;

#[cfg(any(feature = "f128-arithmetics", feature = "f128-math"))]
pub use r#impl::f128;
#[cfg(any(feature = "f16-arithmetics", feature = "f16-math"))]
pub use r#impl::f16;
pub use r#impl::f32;
pub use r#impl::f64;

/// rma_arith’s prelude.
pub mod prelude {
    // provides RoundingMode and traits only.
    pub use crate::RoundingMode;
    pub use crate::{CielArithmetic, FloorArithmetic, RoundTiesEvenArithmetic, TruncArithmetic};
    pub use crate::{CielMath, FloorMath, RoundTiesEvenMath, TruncMath};
    pub use crate::{RoundingArithmetic, RoundingMath};
}

extern "C" {
    static c_TO_NEAREST: c_int;
    static c_UPWARD: c_int;
    static c_DOWNWARD: c_int;
    static c_TOWARD_ZERO: c_int;
}

/// Rounding mode
#[derive(Debug, Copy, Clone)]
pub enum RoundingMode {
    /// To nearest, ties to even (`TONEAREST` in C).
    NearestTiesEven,
    /// Toward 0 (trunc, `TOWARDZERO` in C).
    TowardZero,
    /// Toward +∞ (ciel, `UPWARD` in C).
    TowardPosInf,
    /// Toward -∞ (floor, `DOWNWARD` in C).
    TowardNegInf,

}

impl RoundingMode {
    pub(crate) fn as_c_int(&self) -> c_int {
        match self {
            Self::NearestTiesEven => unsafe { c_TO_NEAREST },
            Self::TowardZero => unsafe { c_TOWARD_ZERO },
            Self::TowardPosInf => unsafe { c_UPWARD },
            Self::TowardNegInf => unsafe { c_DOWNWARD },
        }
    }

    /// Returns `true` if the mode is supported.
    pub fn supported(&self) -> bool {
        0 <= self.as_c_int()
    }
}

mod sealed {
    pub trait Sealed {}

    #[cfg(any(feature = "f16-arithmetics", feature = "f16-math"))]
    impl Sealed for f16 {}
    #[cfg(any(feature = "f128-arithmetics", feature = "f128-math"))]
    impl Sealed for f128 {}
    impl Sealed for f32 {}
    impl Sealed for f64 {}
}

/// Provides arithmetics (add, sub, mul, div and mul_add) with specified rounding mode.
pub trait RoundingArithmetic<T = Self>: sealed::Sealed {
    /// The resulting type.
    type Output;

    /// Returns `self + other` with specified rounding mode.
    ///
    /// # Safety
    ///
    /// Panics when fail to set/restore rounding mode.
    fn round_add(self, other: T, mode: &RoundingMode) -> Self::Output;
    /// Returns `self - other` with specified rounding mode.
    ///
    /// # Safety
    ///
    /// Panics when fail to set/restore rounding mode.
    fn round_sub(self, other: T, mode: &RoundingMode) -> Self::Output;
    /// Returns `self * other` with specified rounding mode.
    ///
    /// # Safety
    ///
    /// Panics when fail to set/restore rounding mode.
    fn round_mul(self, other: T, mode: &RoundingMode) -> Self::Output;
    /// Returns `self / other` with specified rounding mode.
    ///
    /// # Safety
    ///
    /// Panics when fail to set/restore rounding mode.
    fn round_div(self, other: T, mode: &RoundingMode) -> Self::Output;
    /// Returns `self * a + b` with single rounding (fused multiply-add) with specified rounding mode.
    ///
    /// # Safety
    ///
    /// Panics when fail to set/restore rounding mode.
    fn round_mul_add(self, a: T, b: T, mode: &RoundingMode) -> Self::Output;
}

/// Provides arithmetics (add, sub, mul, div and mul_add) as rounding to nearest, ties to even.
pub trait RoundTiesEvenArithmetic<T = Self>: sealed::Sealed {
    /// The resulting type.
    type Output;

    /// Returns `self + other` as rounding to nearest, ties to even.
    ///
    /// # Safety
    ///
    /// Panics when fail to set/restore rounding mode.
    fn round_ties_even_add(self, other: T) -> Self::Output;
    /// Returns `self - other` as rounding to nearest, ties to even.
    ///
    /// # Safety
    ///
    /// Panics when fail to set/restore rounding mode.
    fn round_ties_even_sub(self, other: T) -> Self::Output;
    /// Returns `self * other` as rounding to nearest, ties to even.
    ///
    /// # Safety
    ///
    /// Panics when fail to set/restore rounding mode.
    fn round_ties_even_mul(self, other: T) -> Self::Output;
    /// Returns `self / other` as rounding to nearest, ties to even.
    ///
    /// # Safety
    ///
    /// Panics when fail to set/restore rounding mode.
    fn round_ties_even_div(self, other: T) -> Self::Output;
    /// Returns `self * a + b` with single rounding (fused multiply-add) as rounding to nearest, ties to even.
    ///
    /// # Safety
    ///
    /// Panics when fail to set/restore rounding mode.
    fn round_ties_even_mul_add(self, a: T, b: T) -> Self::Output;
}

/// Provides arithmetics (add, sub, mul, div and mul_add) as rounding toward +∞.
pub trait CielArithmetic<T = Self>: sealed::Sealed {
    /// The resulting type.
    type Output;

    /// Returns `self + other` as rounding toward +∞.
    ///
    /// # Safety
    ///
    /// Panics when fail to set/restore rounding mode.
    fn ciel_add(self, other: T) -> Self::Output;
    /// Returns `self - other` as rounding toward +∞.
    ///
    /// # Safety
    ///
    /// Panics when fail to set/restore rounding mode.
    fn ciel_sub(self, other: T) -> Self::Output;
    /// Returns `self * other` as rounding toward +∞.
    ///
    /// # Safety
    ///
    /// Panics when fail to set/restore rounding mode.
    fn ciel_mul(self, other: T) -> Self::Output;
    /// Returns `self / other` as rounding toward +∞.
    ///
    /// # Safety
    ///
    /// Panics when fail to set/restore rounding mode.
    fn ciel_div(self, other: T) -> Self::Output;
    /// Returns `self * a + b` with single rounding (fused multiply-add) as rounding toward +∞.
    ///
    /// # Safety
    ///
    /// Panics when fail to set/restore rounding mode.
    fn ciel_mul_add(self, a: T, b: T) -> Self::Output;
}

/// Provides arithmetics (add, sub, mul, div and mul_add) as rounding toward -∞.
pub trait FloorArithmetic<T = Self>: sealed::Sealed {
    /// The resulting type.
    type Output;

    /// Returns `self + other` as rounding toward -∞.
    ///
    /// # Safety
    ///
    /// Panics when fail to set/restore rounding mode.
    fn floor_add(self, other: T) -> Self::Output;
    /// Returns `self - other` as rounding toward -∞.
    ///
    ///
    /// # Safety
    ///
    /// Panics when fail to set/restore rounding mode.
    fn floor_sub(self, other: T) -> Self::Output;
    /// Returns `self * other` as rounding toward -∞.
    ///
    /// # Safety
    ///
    /// Panics when fail to set/restore rounding mode.
    fn floor_mul(self, other: T) -> Self::Output;
    /// Returns `self / other` as rounding toward -∞.
    ///
    /// # Safety
    ///
    /// Panics when fail to set/restore rounding mode.
    fn floor_div(self, other: T) -> Self::Output;
    /// Returns `self * a + b` with single rounding (fused multiply-add) as rounding toward -∞.
    ///
    /// # Safety
    ///
    /// Panics when fail to set/restore rounding mode.
    fn floor_mul_add(self, a: T, b: T) -> Self::Output;
}

/// Provides arithmetics (add, sub, mul, div and mul_add) as rounding toward 0.
pub trait TruncArithmetic<T = Self>: sealed::Sealed {
    /// The resulting type.
    type Output;

    /// Returns `self + other` as rounding toward 0.
    ///
    /// # Safety
    ///
    /// Panics when fail to set/restore rounding mode.
    fn trunc_add(self, other: T) -> Self::Output;
    /// Returns `self - other` as rounding toward 0.
    ///
    /// # Safety
    ///
    /// Panics when fail to set/restore rounding mode.
    fn trunc_sub(self, other: T) -> Self::Output;
    /// Returns `self * other` as rounding toward 0.
    ///
    /// # Safety
    ///
    /// Panics when fail to set/restore rounding mode.
    fn trunc_mul(self, other: T) -> Self::Output;
    /// Returns `self / other` as rounding toward 0.
    ///
    /// # Safety
    ///
    /// Panics when fail to set/restore rounding mode.
    fn trunc_div(self, other: T) -> Self::Output;
    /// Returns `self * a + b` with single rounding (fused multiply-add) as rounding toward 0.
    ///
    /// # Safety
    ///
    /// Panics when fail to set/restore rounding mode.
    fn trunc_mul_add(self, a: T, b: T) -> Self::Output;
}

/// Provides a math function (`sqrt`) with specified rounding mode.
pub trait RoundingMath<T = Self>: sealed::Sealed {
    /// The resulting type.
    type Output;

    /// Returns `self.sqrt()` with specified rounding mode.
    ///
    /// # Safety
    ///
    /// Panics when fail to set/restore rounding mode.
    fn round_sqrt(self, mode: &RoundingMode) -> Self::Output;
}

/// Provides a math function (`sqrt`) as rounding to nearest, ties to even.
pub trait RoundTiesEvenMath<T = Self>: sealed::Sealed {
    /// The resulting type.
    type Output;

    /// Returns `self.sqrt()` as rounding to nearest, ties to even.
    ///
    /// # Safety
    ///
    /// Panics when fail to set/restore rounding mode.
    fn round_ties_even_sqrt(self) -> Self::Output;
}

/// Provides a math function (`sqrt`) as rounding toward +∞.
pub trait CielMath<T = Self>: sealed::Sealed {
    /// The resulting type.
    type Output;

    /// Returns `self.sqrt()` as rounding toward +∞.
    ///
    /// # Safety
    ///
    /// Panics when fail to set/restore rounding mode.
    fn ciel_sqrt(self) -> Self::Output;
}

/// Provides a math function (`sqrt`) as rounding toward -∞.
pub trait FloorMath<T = Self>: sealed::Sealed {
    /// The resulting type.
    type Output;

    /// Returns `self.sqrt()` as rounding toward -∞.
    ///
    /// # Safety
    ///
    /// Panics when fail to set/restore rounding mode.
    fn floor_sqrt(self) -> Self::Output;
}

/// Provides a math function (`sqrt`) as rounding toward 0.
pub trait TruncMath<T = Self>: sealed::Sealed {
    /// The resulting type.
    type Output;

    /// Returns `self.sqrt()` as rounding toward 0.
    ///
    /// # Safety
    ///
    /// Panics when fail to set/restore rounding mode.
    fn trunc_sqrt(self) -> Self::Output;
}
