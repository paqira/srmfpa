//! Functions for [`f32`].

use std::ffi::c_int;

use crate::internal::*;
use crate::RoundMode;
use crate::{CielArithmetic, FloorArithmetic, RoundTiesEvenArithmetic, TruncArithmetic};
use crate::{CielMath, FloorMath, RoundTiesEvenMath, TruncMath};
use crate::{RoundingArithmetic, RoundingMath};

extern "C" {
    fn c_add_f32(mode: c_int, a: f32, b: f32, dst: *mut f32) -> c_int;
    fn c_sub_f32(mode: c_int, a: f32, b: f32, dst: *mut f32) -> c_int;
    fn c_mul_f32(mode: c_int, a: f32, b: f32, dst: *mut f32) -> c_int;
    fn c_div_f32(mode: c_int, a: f32, b: f32, dst: *mut f32) -> c_int;
    fn c_fma_f32(mode: c_int, a: f32, b: f32, c: f32, dst: *mut f32) -> c_int;
    fn c_sqrt_f32(mode: c_int, a: f32, dst: *mut f32) -> c_int;
}

impl_round_func_binary_all!(
    f32,
    round_add => c_add_f32,
    round_sub => c_sub_f32,
    round_mul => c_mul_f32,
    round_div => c_div_f32,
    round_mul_add => c_fma_f32,
);

impl_non_round_func_binary_all!(
    f32, NearestTiesEven, "to nearest, ties to even",
    round_ties_even_add => round_add,
    round_ties_even_sub => round_sub,
    round_ties_even_mul => round_mul,
    round_ties_even_div => round_div,
    round_ties_even_mul_add => round_mul_add,
);

impl_non_round_func_binary_all!(
    f32, TowardPosInf, "toward +∞",
    ciel_add => round_add,
    ciel_sub => round_sub,
    ciel_mul => round_mul,
    ciel_div => round_div,
    ciel_mul_add => round_mul_add,
);

impl_non_round_func_binary_all!(
    f32, TowardNegInf, "toward -∞",
    floor_add => round_add,
    floor_sub => round_sub,
    floor_mul => round_mul,
    floor_div => round_div,
    floor_mul_add => round_mul_add,
);

impl_non_round_func_binary_all!(
    f32, TowardZero, "toward 0",
    trunc_add => round_add,
    trunc_sub => round_sub,
    trunc_mul => round_mul,
    trunc_div => round_div,
    trunc_mul_add => round_mul_add,
);

impl_func_unary!(
    /// Returns `a.sqrt()` as specific rounding mode.
    ///
    /// # Safety
    ///
    /// Panics when fail to set/rest rounding mode.
    #[inline]
    => f32, round_sqrt, c_sqrt_f32
);
impl_func_unary!(
    /// Returns `a.sqrt()` as rounding to nearest, ties to even.
    ///
    /// # Safety
    ///
    /// Panics when fail to set/rest rounding mode.
    #[inline]
    => f32, round_ties_even_sqrt, round_sqrt, NearestTiesEven
);
impl_func_unary!(
    /// Returns `a.sqrt()` as rounding toward +∞.
    ///
    /// # Safety
    ///
    /// Panics when fail to set/rest rounding mode.
    #[inline]
    => f32, ciel_sqrt, round_sqrt, TowardPosInf
);
impl_func_unary!(
    /// Returns `a.sqrt()` as rounding toward -∞.
    ///
    /// # Safety
    ///
    /// Panics when fail to set/rest rounding mode.
    #[inline]
    => f32, floor_sqrt, round_sqrt, TowardNegInf
);
impl_func_unary!(
    /// Returns `a.sqrt()` as rounding toward 0.
    ///
    /// # Safety
    ///
    /// Panics when fail to set/rest rounding mode.
    #[inline]
    => f32, trunc_sqrt, round_sqrt, TowardZero
);

impl RoundingArithmetic for f32 {
    type Output = Self;

    impl_round_binary!(round_add);
    impl_round_binary!(round_sub);
    impl_round_binary!(round_mul);
    impl_round_binary!(round_div);
    impl_round_trialy!(round_mul_add);
}

impl RoundingMath for f32 {
    type Output = Self;

    #[inline]
    fn round_sqrt(self, mode: &RoundMode) -> Self::Output {
        round_sqrt(self, mode)
    }
}

impl RoundTiesEvenArithmetic for f32 {
    type Output = Self;

    impl_non_round_binary!(round_ties_even_add);
    impl_non_round_binary!(round_ties_even_sub);
    impl_non_round_binary!(round_ties_even_mul);
    impl_non_round_binary!(round_ties_even_div);
    impl_non_round_trialy!(round_ties_even_mul_add);
}

impl RoundTiesEvenMath for f32 {
    type Output = Self;

    #[inline]
    fn round_ties_even_sqrt(self) -> Self::Output {
        round_sqrt(self, &RoundMode::NearestTiesEven)
    }
}

impl CielArithmetic for f32 {
    type Output = Self;

    impl_non_round_binary!(ciel_add);
    impl_non_round_binary!(ciel_sub);
    impl_non_round_binary!(ciel_mul);
    impl_non_round_binary!(ciel_div);
    impl_non_round_trialy!(ciel_mul_add);
}

impl CielMath for f32 {
    type Output = Self;

    #[inline]
    fn ciel_sqrt(self) -> Self::Output {
        round_sqrt(self, &RoundMode::TowardPosInf)
    }
}

impl FloorArithmetic for f32 {
    type Output = Self;

    impl_non_round_binary!(floor_add);
    impl_non_round_binary!(floor_sub);
    impl_non_round_binary!(floor_mul);
    impl_non_round_binary!(floor_div);
    impl_non_round_trialy!(floor_mul_add);
}

impl FloorMath for f32 {
    type Output = Self;

    #[inline]
    fn floor_sqrt(self) -> Self::Output {
        round_sqrt(self, &RoundMode::TowardNegInf)
    }
}

impl TruncArithmetic for f32 {
    type Output = Self;

    impl_non_round_binary!(trunc_add);
    impl_non_round_binary!(trunc_sub);
    impl_non_round_binary!(trunc_mul);
    impl_non_round_binary!(trunc_div);
    impl_non_round_trialy!(trunc_mul_add);
}

impl TruncMath for f32 {
    type Output = Self;

    #[inline]
    fn trunc_sqrt(self) -> Self::Output {
        round_sqrt(self, &RoundMode::TowardZero)
    }
}

#[cfg(test)]
mod test_func {
    use crate::RoundMode as Mode;

    use super::*;

    macro_rules! assert_nan {
        ($a:expr) => {
            assert!($a.is_nan())
        };
    }

    #[test]
    fn test_round_add() {
        assert_eq!(round_add(0.1, 0.2, &Mode::NearestTiesEven), 0.3);
        assert_eq!(round_add(0.1, 0.2, &Mode::TowardPosInf), 0.3);
        assert_eq!(round_add(0.1, 0.2, &Mode::TowardNegInf), 0.29999998);
        assert_eq!(round_add(0.1, 0.2, &Mode::TowardZero), 0.29999998);

        assert_eq!(round_add(-0.1, -0.2, &Mode::NearestTiesEven), -0.3);
        assert_eq!(round_add(-0.1, -0.2, &Mode::TowardPosInf), -0.29999998);
        assert_eq!(round_add(-0.1, -0.2, &Mode::TowardNegInf), -0.3);
        assert_eq!(round_add(-0.1, -0.2, &Mode::TowardZero), -0.29999998);
    }

    #[test]
    fn test_round_sub() {
        assert_eq!(round_sub(0.3, 0.1, &Mode::NearestTiesEven), 0.20000002);
        assert_eq!(round_sub(0.3, 0.1, &Mode::TowardPosInf), 0.20000002);
        assert_eq!(round_sub(0.3, 0.1, &Mode::TowardNegInf), 0.2);
        assert_eq!(round_sub(0.3, 0.1, &Mode::TowardZero), 0.2);

        assert_eq!(round_sub(-0.3, -0.1, &Mode::NearestTiesEven), -0.20000002);
        assert_eq!(round_sub(-0.3, -0.1, &Mode::TowardPosInf), -0.2);
        assert_eq!(round_sub(-0.3, -0.1, &Mode::TowardNegInf), -0.20000002);
        assert_eq!(round_sub(-0.3, -0.1, &Mode::TowardZero), -0.2);
    }

    #[test]
    fn test_round_mul() {
        let b = 0.3 + f32::EPSILON;
        assert_eq!(round_mul(0.3, b, &Mode::NearestTiesEven), 0.09000004);
        assert_eq!(round_mul(0.3, b, &Mode::TowardPosInf), 0.09000005);
        assert_eq!(round_mul(0.3, b, &Mode::TowardNegInf), 0.09000004);
        assert_eq!(round_mul(0.3, b, &Mode::TowardZero), 0.09000004);

        assert_eq!(round_mul(-0.3, b, &Mode::NearestTiesEven), -0.09000004);
        assert_eq!(round_mul(-0.3, b, &Mode::TowardPosInf), -0.09000004);
        assert_eq!(round_mul(-0.3, b, &Mode::TowardNegInf), -0.09000005);
        assert_eq!(round_mul(-0.3, b, &Mode::TowardZero), -0.09000004);
    }

    #[test]
    fn test_round_div() {
        let b = 0.3 + f32::EPSILON;
        assert_eq!(round_div(0.3, b, &Mode::NearestTiesEven), 0.9999996);
        assert_eq!(round_div(0.3, b, &Mode::TowardPosInf), 0.99999964);
        assert_eq!(round_div(0.3, b, &Mode::TowardNegInf), 0.9999996);
        assert_eq!(round_div(0.3, b, &Mode::TowardZero), 0.9999996);

        assert_eq!(round_div(-0.3, b, &Mode::NearestTiesEven), -0.9999996);
        assert_eq!(round_div(-0.3, b, &Mode::TowardPosInf), -0.9999996);
        assert_eq!(round_div(-0.3, b, &Mode::TowardNegInf), -0.99999964);
        assert_eq!(round_div(-0.3, b, &Mode::TowardZero), -0.9999996);
    }

    #[test]
    fn test_round_sqrt() {
        assert_eq!(round_sqrt(2.0, &Mode::NearestTiesEven), 1.4142135);
        assert_eq!(round_sqrt(2.0, &Mode::TowardPosInf), 1.4142137);
        assert_eq!(round_sqrt(2.0, &Mode::TowardNegInf), 1.4142135);
        assert_eq!(round_sqrt(2.0, &Mode::TowardZero), 1.4142135);

        assert_nan!(round_sqrt(-2.0, &Mode::NearestTiesEven));
        assert_nan!(round_sqrt(-2.0, &Mode::TowardPosInf));
        assert_nan!(round_sqrt(-2.0, &Mode::TowardNegInf));
        assert_nan!(round_sqrt(-2.0, &Mode::TowardZero));
    }

    #[test]
    fn test_rest_add() {
        assert_eq!(
            round_ties_even_add(0.1, 0.2),
            round_add(0.1, 0.2, &Mode::NearestTiesEven)
        );
        assert_eq!(ciel_add(0.1, 0.2), round_add(0.1, 0.2, &Mode::TowardPosInf));
        assert_eq!(
            floor_add(0.1, 0.2),
            round_add(0.1, 0.2, &Mode::TowardNegInf)
        );
        assert_eq!(trunc_add(0.1, 0.2), round_add(0.1, 0.2, &Mode::TowardZero));

        assert_eq!(
            round_ties_even_add(-0.1, -0.2),
            round_add(-0.1, -0.2, &Mode::NearestTiesEven)
        );
        assert_eq!(
            ciel_add(-0.1, -0.2),
            round_add(-0.1, -0.2, &Mode::TowardPosInf)
        );
        assert_eq!(
            floor_add(-0.1, -0.2),
            round_add(-0.1, -0.2, &Mode::TowardNegInf)
        );
        assert_eq!(
            trunc_add(-0.1, -0.2),
            round_add(-0.1, -0.2, &Mode::TowardZero)
        );
    }
}
