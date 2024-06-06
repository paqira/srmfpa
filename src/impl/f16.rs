#![allow(unused_imports)]

//! Functions for [`f16`].

#[cfg(feature = "f16-arithmetics")]
pub use arithmetic::*;
#[cfg(feature = "f16-math")]
pub use math::*;

use crate::RoundMode;
use crate::{CielArithmetic, FloorArithmetic, RoundTiesEvenArithmetic, TruncArithmetic};
use crate::{CielMath, FloorMath, RoundTiesEvenMath, TruncMath};
use crate::{RoundingArithmetic, RoundingMath};

#[cfg(feature = "f16-arithmetics")]
mod arithmetic {
    use std::ffi::c_int;

    use super::*;
    use crate::internal::*;

    extern "C" {
        fn c_add_f16(mode: c_int, a: f16, b: f16, dst: *mut f16) -> c_int;
        fn c_sub_f16(mode: c_int, a: f16, b: f16, dst: *mut f16) -> c_int;
        fn c_mul_f16(mode: c_int, a: f16, b: f16, dst: *mut f16) -> c_int;
        fn c_div_f16(mode: c_int, a: f16, b: f16, dst: *mut f16) -> c_int;
        fn c_fma_f16(mode: c_int, a: f16, b: f16, c: f16, dst: *mut f16) -> c_int;
    }

    impl_round_func_binary_all!(
        f16,
        round_add => c_add_f16,
        round_sub => c_sub_f16,
        round_mul => c_mul_f16,
        round_div => c_div_f16,
        round_mul_add => c_fma_f16,
    );

    impl_non_round_func_binary_all!(
        f16, NearestTiesEven, "to nearest, ties to even",
        round_ties_even_add => round_add,
        round_ties_even_sub => round_sub,
        round_ties_even_mul => round_mul,
        round_ties_even_div => round_div,
        round_ties_even_mul_add => round_mul_add,
    );

    impl_non_round_func_binary_all!(
        f16, TowardPosInf, "toward +∞",
        ciel_add => round_add,
        ciel_sub => round_sub,
        ciel_mul => round_mul,
        ciel_div => round_div,
        ciel_mul_add => round_mul_add,
    );

    impl_non_round_func_binary_all!(
        f16, TowardNegInf, "toward -∞",
        floor_add => round_add,
        floor_sub => round_sub,
        floor_mul => round_mul,
        floor_div => round_div,
        floor_mul_add => round_mul_add,
    );

    impl_non_round_func_binary_all!(
        f16, TowardZero, "toward 0",
        trunc_add => round_add,
        trunc_sub => round_sub,
        trunc_mul => round_mul,
        trunc_div => round_div,
        trunc_mul_add => round_mul_add,
    );

    impl RoundingArithmetic for f16 {
        type Output = Self;

        impl_round_binary!(round_add);
        impl_round_binary!(round_sub);
        impl_round_binary!(round_mul);
        impl_round_binary!(round_div);
        impl_round_trialy!(round_mul_add);
    }

    impl RoundTiesEvenArithmetic for f16 {
        type Output = Self;

        impl_non_round_binary!(round_ties_even_add);
        impl_non_round_binary!(round_ties_even_sub);
        impl_non_round_binary!(round_ties_even_mul);
        impl_non_round_binary!(round_ties_even_div);
        impl_non_round_trialy!(round_ties_even_mul_add);
    }

    impl CielArithmetic for f16 {
        type Output = Self;

        impl_non_round_binary!(ciel_add);
        impl_non_round_binary!(ciel_sub);
        impl_non_round_binary!(ciel_mul);
        impl_non_round_binary!(ciel_div);
        impl_non_round_trialy!(ciel_mul_add);
    }

    impl FloorArithmetic for f16 {
        type Output = Self;

        impl_non_round_binary!(floor_add);
        impl_non_round_binary!(floor_sub);
        impl_non_round_binary!(floor_mul);
        impl_non_round_binary!(floor_div);
        impl_non_round_trialy!(floor_mul_add);
    }

    impl TruncArithmetic for f16 {
        type Output = Self;

        impl_non_round_binary!(trunc_add);
        impl_non_round_binary!(trunc_sub);
        impl_non_round_binary!(trunc_mul);
        impl_non_round_binary!(trunc_div);
        impl_non_round_trialy!(trunc_mul_add);
    }
}

#[cfg(feature = "f16-math")]
mod math {
    use std::ffi::c_int;

    use super::*;
    use crate::internal::*;

    extern "C" {
        fn c_sqrt_f16(mode: c_int, a: f16, dst: *mut f16) -> c_int;
    }

    impl_func_unary!(
        /// Returns `a.sqrt()` as specific rounding mode.
        ///
        /// # Safety
        ///
        /// Panics when fail to set/rest rounding mode.
        #[inline]
        => f16, round_sqrt, c_sqrt_f16
    );
    impl_func_unary!(
        /// Returns `a.sqrt()` as rounding to nearest, ties to even.
        ///
        /// # Safety
        ///
        /// Panics when fail to set/rest rounding mode.
        #[inline]
        => f16, round_ties_even_sqrt, round_sqrt, NearestTiesEven
    );
    impl_func_unary!(
        /// Returns `a.sqrt()` as rounding toward +∞.
        ///
        /// # Safety
        ///
        /// Panics when fail to set/rest rounding mode.
        #[inline]
        => f16, ciel_sqrt, round_sqrt, TowardPosInf
    );
    impl_func_unary!(
        /// Returns `a.sqrt()` as rounding toward -∞.
        ///
        /// # Safety
        ///
        /// Panics when fail to set/rest rounding mode.
        #[inline]
        => f16, floor_sqrt, round_sqrt, TowardNegInf
    );
    impl_func_unary!(
        /// Returns `a.sqrt()` as rounding toward 0.
        ///
        /// # Safety
        ///
        /// Panics when fail to set/rest rounding mode.
        #[inline]
        => f16, trunc_sqrt, round_sqrt, TowardZero
    );

    impl RoundingMath for f16 {
        type Output = Self;

        #[inline]
        fn round_sqrt(self, mode: &RoundMode) -> Self::Output {
            round_sqrt(self, mode)
        }
    }

    impl RoundTiesEvenMath for f16 {
        type Output = Self;

        #[inline]
        fn round_ties_even_sqrt(self) -> Self::Output {
            round_sqrt(self, &RoundMode::NearestTiesEven)
        }
    }

    impl CielMath for f16 {
        type Output = Self;

        #[inline]
        fn ciel_sqrt(self) -> Self::Output {
            round_sqrt(self, &RoundMode::TowardPosInf)
        }
    }

    impl FloorMath for f16 {
        type Output = Self;

        #[inline]
        fn floor_sqrt(self) -> Self::Output {
            round_sqrt(self, &RoundMode::TowardNegInf)
        }
    }

    impl TruncMath for f16 {
        type Output = Self;

        #[inline]
        fn trunc_sqrt(self) -> Self::Output {
            round_sqrt(self, &RoundMode::TowardZero)
        }
    }
}
