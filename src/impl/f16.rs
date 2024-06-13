//! Functions for [`f16`].

use crate::internal::*;
use crate::RoundingMode;
use crate::{CielArithmetic, FloorArithmetic, RoundTiesEvenArithmetic, TruncArithmetic};
use crate::{CielMath, FloorMath, RoundTiesEvenMath, TruncMath};
use crate::{RoundingArithmetic, RoundingMath};

#[cfg(feature = "f16")]
pub use crate::r#impl::builtin::f16::*;
#[cfg(feature = "f16-softfloat")]
pub use crate::r#impl::softfloat::f16::*;

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

impl RoundingArithmetic for f16 {
    type Output = Self;

    impl_round_binary!(round_add);
    impl_round_binary!(round_sub);
    impl_round_binary!(round_mul);
    impl_round_binary!(round_div);
    impl_round_ternary!(round_mul_add);
}

impl RoundingMath for f16 {
    type Output = Self;

    #[inline]
    fn round_sqrt(self, mode: &RoundingMode) -> Self::Output {
        round_sqrt(self, mode)
    }
}

impl RoundTiesEvenArithmetic for f16 {
    type Output = Self;

    impl_non_round_binary!(round_ties_even_add);
    impl_non_round_binary!(round_ties_even_sub);
    impl_non_round_binary!(round_ties_even_mul);
    impl_non_round_binary!(round_ties_even_div);
    impl_non_round_ternary!(round_ties_even_mul_add);
}

impl RoundTiesEvenMath for f16 {
    type Output = Self;

    #[inline]
    fn round_ties_even_sqrt(self) -> Self::Output {
        round_ties_even_sqrt(self)
    }
}

impl CielArithmetic for f16 {
    type Output = Self;

    impl_non_round_binary!(ciel_add);
    impl_non_round_binary!(ciel_sub);
    impl_non_round_binary!(ciel_mul);
    impl_non_round_binary!(ciel_div);
    impl_non_round_ternary!(ciel_mul_add);
}

impl CielMath for f16 {
    type Output = Self;

    #[inline]
    fn ciel_sqrt(self) -> Self::Output {
        ciel_sqrt(self)
    }
}

impl FloorArithmetic for f16 {
    type Output = Self;

    impl_non_round_binary!(floor_add);
    impl_non_round_binary!(floor_sub);
    impl_non_round_binary!(floor_mul);
    impl_non_round_binary!(floor_div);
    impl_non_round_ternary!(floor_mul_add);
}

impl FloorMath for f16 {
    type Output = Self;

    #[inline]
    fn floor_sqrt(self) -> Self::Output {
        floor_sqrt(self)
    }
}

impl TruncArithmetic for f16 {
    type Output = Self;

    impl_non_round_binary!(trunc_add);
    impl_non_round_binary!(trunc_sub);
    impl_non_round_binary!(trunc_mul);
    impl_non_round_binary!(trunc_div);
    impl_non_round_ternary!(trunc_mul_add);
}

impl TruncMath for f16 {
    type Output = Self;

    #[inline]
    fn trunc_sqrt(self) -> Self::Output {
        trunc_sqrt(self)
    }
}

#[cfg(test)]
mod test_fn_arith {
    use crate::RoundingMode as Mode;

    use super::*;

    #[test]
    fn test_add() {
        let (a, b) = (0.1f16, 0.2f16);
        let (e1, e2) = (0.3f16, 0.2998f16);

        assert_eq!(round_add(a, b, &Mode::NearestTiesEven), e2);
        assert_eq!(round_add(a, b, &Mode::TowardPosInf), e1);
        assert_eq!(round_add(a, b, &Mode::TowardNegInf), e2);
        assert_eq!(round_add(a, b, &Mode::TowardZero), e2);

        let (a, b) = (-0.1f16, -0.2f16);
        let (e1, e2) = (-0.3f16, -0.2998f16);

        assert_eq!(round_add(a, b, &Mode::NearestTiesEven), e2);
        assert_eq!(round_add(a, b, &Mode::TowardPosInf), e2);
        assert_eq!(round_add(a, b, &Mode::TowardNegInf), e1);
        assert_eq!(round_add(a, b, &Mode::TowardZero), e2);
    }

    #[test]
    fn test_sub() {
        let (a, b) = (0.3f16, 0.01f16);
        let (e1, e2) = (0.29f16, 0.2902f16);

        assert_eq!(round_sub(a, b, &Mode::NearestTiesEven), e1);
        assert_eq!(round_sub(a, b, &Mode::TowardPosInf), e2);
        assert_eq!(round_sub(a, b, &Mode::TowardNegInf), e1);
        assert_eq!(round_sub(a, b, &Mode::TowardZero), e1);

        let (a, b) = (-0.3f16, -0.01f16);
        let (e1, e2) = (-0.29f16, -0.2902f16);

        assert_eq!(round_sub(a, b, &Mode::NearestTiesEven), e1);
        assert_eq!(round_sub(a, b, &Mode::TowardPosInf), e1);
        assert_eq!(round_sub(a, b, &Mode::TowardNegInf), e2);
        assert_eq!(round_sub(a, b, &Mode::TowardZero), e1);
    }

    #[test]
    fn test_mul() {
        let (a, b) = (0.3f16, 0.3f16);
        let (e1, e2) = (0.09f16, 0.0901f16);

        assert_eq!(round_mul(a, b, &Mode::NearestTiesEven), e1);
        assert_eq!(round_mul(a, b, &Mode::TowardPosInf), e2);
        assert_eq!(round_mul(a, b, &Mode::TowardNegInf), e1);
        assert_eq!(round_mul(a, b, &Mode::TowardZero), e1);

        let (a, b) = (-0.3f16, 0.3f16);
        let (e1, e2) = (-0.09f16, -0.0901f16);

        assert_eq!(round_mul(a, b, &Mode::NearestTiesEven), e1);
        assert_eq!(round_mul(a, b, &Mode::TowardPosInf), e1);
        assert_eq!(round_mul(a, b, &Mode::TowardNegInf), e2);
        assert_eq!(round_mul(a, b, &Mode::TowardZero), e1);
    }

    #[test]
    fn test_div() {
        let (a, b) = (0.3f16, 0.2f16);
        let (e1, e2) = (1.501f16, 1.5f16);

        assert_eq!(round_div(a, b, &Mode::NearestTiesEven), e1);
        assert_eq!(round_div(a, b, &Mode::TowardPosInf), e1);
        assert_eq!(round_div(a, b, &Mode::TowardNegInf), e2);
        assert_eq!(round_div(a, b, &Mode::TowardZero), e2);

        let (a, b) = (0.3f16, -0.2f16);
        let (e1, e2) = (-1.501f16, -1.5f16);

        assert_eq!(round_div(a, b, &Mode::NearestTiesEven), e1);
        assert_eq!(round_div(a, b, &Mode::TowardPosInf), e2);
        assert_eq!(round_div(a, b, &Mode::TowardNegInf), e1);
        assert_eq!(round_div(a, b, &Mode::TowardZero), e2);
    }

    #[test]
    fn test_mul_add() {
        let (a, b, c) = (0.3f16, 0.2f16, 0.1f16);
        let (e1, e2) = (0.16f16, 0.1599f16);

        assert_eq!(round_mul_add(a, b, c, &Mode::NearestTiesEven), e2);
        assert_eq!(round_mul_add(a, b, c, &Mode::TowardPosInf), e1);
        assert_eq!(round_mul_add(a, b, c, &Mode::TowardNegInf), e2);
        assert_eq!(round_mul_add(a, b, c, &Mode::TowardZero), e2);

        let (a, b, c) = (0.3f16, -0.2f16, -0.1f16);
        let (e1, e2) = (-0.16f16, -0.1599f16);

        assert_eq!(round_mul_add(a, b, c, &Mode::NearestTiesEven), e2);
        assert_eq!(round_mul_add(a, b, c, &Mode::TowardPosInf), e2);
        assert_eq!(round_mul_add(a, b, c, &Mode::TowardNegInf), e1);
        assert_eq!(round_mul_add(a, b, c, &Mode::TowardZero), e2);
    }
}

#[cfg(test)]
mod test_fn_math {
    use crate::RoundingMode as Mode;

    use super::*;

    #[test]
    fn test_sqrt() {
        let a = 2.0f16;
        let (e1, e2) = (1.414f16, 1.415f16);

        assert_eq!(round_sqrt(a, &Mode::NearestTiesEven), e1);
        assert_eq!(round_sqrt(a, &Mode::TowardPosInf), e2);
        assert_eq!(round_sqrt(a, &Mode::TowardNegInf), e1);
        assert_eq!(round_sqrt(a, &Mode::TowardZero), e1);
    }
}
