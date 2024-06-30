#![cfg_attr(any(feature = "f16", feature = "f16_softfloat",), feature(f16))]
#![cfg_attr(any(feature = "f128", feature = "f128_softfloat",), feature(f128))]
#![cfg_attr(test, feature(float_next_up_down))]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! Floating-point's four arithmetic operations (including fused multiply-add) and square root
//! with strict rounding mode.
//!
//! ```
//! use srmfpa::prelude::*;
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
//! use srmfpa::f64::{ciel_add, floor_add};
//! assert_eq!(ciel_add(0.1, 0.2), 0.30000000000000004);
//! assert_eq!(floor_add(0.1, 0.2), 0.3);
//! ```
//!
//! # Features
//!
//! - `softfloat`: use softfloat for `f32` and `f64`
//!   (enable `f32_softfloat` and `f64_softfloat`).
//! - `f32_softfloat`: support softfloat `f32` by [Berkeley SoftFloat 3][softfloat].
//! - `f64_softfloat`: support softfloat `f64` by [Berkeley SoftFloat 3][softfloat].
//! - `f16_softfloat`: support softfloat `f16` by [Berkeley SoftFloat 3][softfloat].
//! - `f128_softfloat`: support softfloat `f128` by [Berkeley SoftFloat 3][softfloat].
//!
//! # Notes on Correctness and Configuration
//!
//! As default, [`srmfpa`][mod@self] uses C lang floating-point ops,
//! it maps `f32` (`f64`) to `float` (`double`) and controls rounding mode by `<fenv.h>`.
//! Thus, rounding correctness depends on the environment (C compiler, libc, CPU etc.).
//!
//! [`srmfpa`][mod@self] supports softfloat ops by [Berkeley SoftFloat 3][softfloat] with `fN_softfloat` features.
//! It provides correct rounding ops for evey IEEE 754 rounding modes.
//!
//! [`srmfpa`][mod@self] uses the default C compiler options of `cc`,
//! and does not (explicitly) specify other options.
//! It is recommended to pass corresponding options (`-std=c11`, `-lm`, `-frounding-math`, `-mfma` etc.)
//! to obtain the desired result.
//! See [`cc` crate document][cc_doc] for detail of configuration.
//!
//! [softfloat]: https://github.com/ucb-bar/berkeley-softfloat-3
//! [cc_doc]: https://docs.rs/cc/latest/cc/index.html
#[cfg(all(feature = "f128", feature = "f128_softfloat"))]
compile_error!("not supported features combination, `f128` and `f128-softfloat`");
#[cfg(all(feature = "f16", feature = "f16_softfloat"))]
compile_error!("not supported features combination, `f16` and `f16-softfloat`");

use core::ffi::c_int;

#[cfg(any(feature = "f128", feature = "f128_softfloat"))]
#[cfg_attr(docsrs, doc(cfg(feature = "f128_softfloat")))]
pub use r#impl::f128;
#[cfg(any(feature = "f16", feature = "f16_softfloat"))]
#[cfg_attr(docsrs, doc(cfg(feature = "f16_softfloat")))]
pub use r#impl::f16;
pub use r#impl::f32;
pub use r#impl::f64;

mod r#impl;
mod internal;

/// [fpa_specr][mod@self]’s prelude.
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

    fn c_supported(round: c_int) -> bool;
}

/// IEEE 754 rounding mode
#[derive(Debug, Copy, Clone)]
pub enum RoundingMode {
    /// To nearest, ties to even.
    NearestTiesEven,
    /// Toward 0 (aka trunc).
    TowardZero,
    /// Toward +∞ (aka ciel).
    TowardPosInf,
    /// Toward -∞ (aka floor).
    TowardNegInf,
}

impl RoundingMode {
    #[inline]
    pub(crate) fn as_c_int(&self) -> c_int {
        match self {
            Self::NearestTiesEven => unsafe { c_TO_NEAREST },
            Self::TowardPosInf => unsafe { c_UPWARD },
            Self::TowardNegInf => unsafe { c_DOWNWARD },
            Self::TowardZero => unsafe { c_TOWARD_ZERO },
        }
    }

    /// Returns `true` if the mode is supported.
    ///
    /// Notes, all rounding modes are supported on softfloat,
    /// even when this returns `false`.
    ///
    /// # Implementation Notes
    ///
    /// This tests the corresponding C macro's value is greater than or equals to 0,
    /// e.g., `0 <= TONEAREST`.
    #[must_use]
    #[inline]
    pub fn supported(&self) -> bool {
        unsafe { c_supported(self.as_c_int()) }
    }
}

mod sealed {
    pub trait Sealed {}

    #[cfg(any(feature = "f16", feature = "f16_softfloat",))]
    impl Sealed for f16 {}
    #[cfg(any(feature = "f128", feature = "f128_softfloat",))]
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
