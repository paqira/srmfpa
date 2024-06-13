//! Functions for [`f128`].

use crate::internal::*;
use crate::RoundingMode;
use crate::{CielArithmetic, FloorArithmetic, RoundTiesEvenArithmetic, TruncArithmetic};
use crate::{CielMath, FloorMath, RoundTiesEvenMath, TruncMath};
use crate::{RoundingArithmetic, RoundingMath};

#[cfg(feature = "f128")]
pub use crate::r#impl::builtin::f128::*;
#[cfg(feature = "f128-softfloat")]
pub use crate::r#impl::softfloat::f128::*;

impl_non_round_func_binary_all!(
    f128, NearestTiesEven, "to nearest, ties to even",
    round_ties_even_add => round_add,
    round_ties_even_sub => round_sub,
    round_ties_even_mul => round_mul,
    round_ties_even_div => round_div,
    round_ties_even_mul_add => round_mul_add,
);

impl_non_round_func_binary_all!(
    f128, TowardPosInf, "toward +∞",
    ciel_add => round_add,
    ciel_sub => round_sub,
    ciel_mul => round_mul,
    ciel_div => round_div,
    ciel_mul_add => round_mul_add,
);

impl_non_round_func_binary_all!(
    f128, TowardNegInf, "toward -∞",
    floor_add => round_add,
    floor_sub => round_sub,
    floor_mul => round_mul,
    floor_div => round_div,
    floor_mul_add => round_mul_add,
);

impl_non_round_func_binary_all!(
    f128, TowardZero, "toward 0",
    trunc_add => round_add,
    trunc_sub => round_sub,
    trunc_mul => round_mul,
    trunc_div => round_div,
    trunc_mul_add => round_mul_add,
);

impl_func_unary!(
    /// Returns `a.sqrt()` as rounding to nearest, ties to even.
    ///
    /// # Safety
    ///
    /// Panics when fail to set/rest rounding mode.
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    => f128, round_ties_even_sqrt, round_sqrt, NearestTiesEven
);
impl_func_unary!(
    /// Returns `a.sqrt()` as rounding toward +∞.
    ///
    /// # Safety
    ///
    /// Panics when fail to set/rest rounding mode.
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    => f128, ciel_sqrt, round_sqrt, TowardPosInf
);
impl_func_unary!(
    /// Returns `a.sqrt()` as rounding toward -∞.
    ///
    /// # Safety
    ///
    /// Panics when fail to set/rest rounding mode.
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    => f128, floor_sqrt, round_sqrt, TowardNegInf
);
impl_func_unary!(
    /// Returns `a.sqrt()` as rounding toward 0.
    ///
    /// # Safety
    ///
    /// Panics when fail to set/rest rounding mode.
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    => f128, trunc_sqrt, round_sqrt, TowardZero
);

impl RoundingArithmetic for f128 {
    type Output = Self;

    impl_round_binary!(round_add);
    impl_round_binary!(round_sub);
    impl_round_binary!(round_mul);
    impl_round_binary!(round_div);
    impl_round_ternary!(round_mul_add);
}

impl RoundingMath for f128 {
    type Output = Self;

    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn round_sqrt(self, mode: &RoundingMode) -> Self::Output {
        round_sqrt(self, mode)
    }
}

impl RoundTiesEvenArithmetic for f128 {
    type Output = Self;

    impl_non_round_binary!(round_ties_even_add);
    impl_non_round_binary!(round_ties_even_sub);
    impl_non_round_binary!(round_ties_even_mul);
    impl_non_round_binary!(round_ties_even_div);
    impl_non_round_ternary!(round_ties_even_mul_add);
}

impl RoundTiesEvenMath for f128 {
    type Output = Self;

    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn round_ties_even_sqrt(self) -> Self::Output {
        round_ties_even_sqrt(self)
    }
}

impl CielArithmetic for f128 {
    type Output = Self;

    impl_non_round_binary!(ciel_add);
    impl_non_round_binary!(ciel_sub);
    impl_non_round_binary!(ciel_mul);
    impl_non_round_binary!(ciel_div);
    impl_non_round_ternary!(ciel_mul_add);
}

impl CielMath for f128 {
    type Output = Self;

    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn ciel_sqrt(self) -> Self::Output {
        ciel_sqrt(self)
    }
}

impl FloorArithmetic for f128 {
    type Output = Self;

    impl_non_round_binary!(floor_add);
    impl_non_round_binary!(floor_sub);
    impl_non_round_binary!(floor_mul);
    impl_non_round_binary!(floor_div);
    impl_non_round_ternary!(floor_mul_add);
}

impl FloorMath for f128 {
    type Output = Self;

    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn floor_sqrt(self) -> Self::Output {
        floor_sqrt(self)
    }
}

impl TruncArithmetic for f128 {
    type Output = Self;

    impl_non_round_binary!(trunc_add);
    impl_non_round_binary!(trunc_sub);
    impl_non_round_binary!(trunc_mul);
    impl_non_round_binary!(trunc_div);
    impl_non_round_ternary!(trunc_mul_add);
}

impl TruncMath for f128 {
    type Output = Self;

    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn trunc_sqrt(self) -> Self::Output {
        trunc_sqrt(self)
    }
}
