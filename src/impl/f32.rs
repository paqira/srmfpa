//! Functions for [`f32`].
//!
//! Notes, they panic when fails to set/reset rounding mode.

use crate::internal::*;
use crate::RoundingMode;
use crate::{CielArithmetic, FloorArithmetic, RoundTiesEvenArithmetic, TruncArithmetic};
use crate::{CielMath, FloorMath, RoundTiesEvenMath, TruncMath};
use crate::{RoundingArithmetic, RoundingMath};

#[cfg(not(feature = "f32_softfloat"))]
pub use crate::r#impl::builtin::f32::*;
#[cfg(feature = "f32_softfloat")]
pub use crate::r#impl::softfloat::f32::*;

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
    /// Returns `a.sqrt()` as rounding to nearest, ties to even.
    ///
    /// # Safety
    ///
    /// Panics when fail to set/rest rounding mode.
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    => f32, round_ties_even_sqrt, round_sqrt, NearestTiesEven
);
impl_func_unary!(
    /// Returns `a.sqrt()` as rounding toward +∞.
    ///
    /// # Safety
    ///
    /// Panics when fail to set/rest rounding mode.
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    => f32, ciel_sqrt, round_sqrt, TowardPosInf
);
impl_func_unary!(
    /// Returns `a.sqrt()` as rounding toward -∞.
    ///
    /// # Safety
    ///
    /// Panics when fail to set/rest rounding mode.
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    => f32, floor_sqrt, round_sqrt, TowardNegInf
);
impl_func_unary!(
    /// Returns `a.sqrt()` as rounding toward 0.
    ///
    /// # Safety
    ///
    /// Panics when fail to set/rest rounding mode.
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    => f32, trunc_sqrt, round_sqrt, TowardZero
);

impl RoundingArithmetic for f32 {
    type Output = Self;

    impl_round_binary!(round_add);
    impl_round_binary!(round_sub);
    impl_round_binary!(round_mul);
    impl_round_binary!(round_div);
    impl_round_ternary!(round_mul_add);
}

impl RoundingMath for f32 {
    type Output = Self;

    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn round_sqrt(self, mode: &RoundingMode) -> Self::Output {
        round_sqrt(self, mode)
    }
}

impl RoundTiesEvenArithmetic for f32 {
    type Output = Self;

    impl_non_round_binary!(round_ties_even_add);
    impl_non_round_binary!(round_ties_even_sub);
    impl_non_round_binary!(round_ties_even_mul);
    impl_non_round_binary!(round_ties_even_div);
    impl_non_round_ternary!(round_ties_even_mul_add);
}

impl RoundTiesEvenMath for f32 {
    type Output = Self;

    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn round_ties_even_sqrt(self) -> Self::Output {
        round_ties_even_sqrt(self)
    }
}

impl CielArithmetic for f32 {
    type Output = Self;

    impl_non_round_binary!(ciel_add);
    impl_non_round_binary!(ciel_sub);
    impl_non_round_binary!(ciel_mul);
    impl_non_round_binary!(ciel_div);
    impl_non_round_ternary!(ciel_mul_add);
}

impl CielMath for f32 {
    type Output = Self;

    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn ciel_sqrt(self) -> Self::Output {
        ciel_sqrt(self)
    }
}

impl FloorArithmetic for f32 {
    type Output = Self;

    impl_non_round_binary!(floor_add);
    impl_non_round_binary!(floor_sub);
    impl_non_round_binary!(floor_mul);
    impl_non_round_binary!(floor_div);
    impl_non_round_ternary!(floor_mul_add);
}

impl FloorMath for f32 {
    type Output = Self;

    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn floor_sqrt(self) -> Self::Output {
        floor_sqrt(self)
    }
}

impl TruncArithmetic for f32 {
    type Output = Self;

    impl_non_round_binary!(trunc_add);
    impl_non_round_binary!(trunc_sub);
    impl_non_round_binary!(trunc_mul);
    impl_non_round_binary!(trunc_div);
    impl_non_round_ternary!(trunc_mul_add);
}

impl TruncMath for f32 {
    type Output = Self;

    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn trunc_sqrt(self) -> Self::Output {
        trunc_sqrt(self)
    }
}

/// accrual test
#[cfg(test)]
mod test_fn_arith {
    use crate::RoundingMode as Mode;

    use super::*;

    #[test]
    fn test_add() {
        let (a, b) = (0.1, 0.2);
        let e: f32 = 0.3;
        assert_eq!(round_add(a, b, &Mode::NearestTiesEven), e);
        assert_eq!(round_add(a, b, &Mode::TowardPosInf), e);
        assert_eq!(round_add(a, b, &Mode::TowardNegInf), e.next_down());
        assert_eq!(round_add(a, b, &Mode::TowardZero), e.next_down());

        let (a, b) = (-0.1, -0.2);
        let e: f32 = -0.3;
        assert_eq!(round_add(a, b, &Mode::NearestTiesEven), e);
        assert_eq!(round_add(a, b, &Mode::TowardPosInf), e.next_up());
        assert_eq!(round_add(a, b, &Mode::TowardNegInf), e);
        assert_eq!(round_add(a, b, &Mode::TowardZero), e.next_up());
    }

    #[test]
    fn test_sub() {
        let (a, b) = (0.3, 0.1);
        let e: f32 = 0.2;
        assert_eq!(round_sub(a, b, &Mode::NearestTiesEven), e.next_up());
        assert_eq!(round_sub(a, b, &Mode::TowardPosInf), e.next_up());
        assert_eq!(round_sub(a, b, &Mode::TowardNegInf), e);
        assert_eq!(round_sub(a, b, &Mode::TowardZero), e);

        let (a, b) = (-0.3, -0.1);
        let e: f32 = -0.2;
        assert_eq!(round_sub(a, b, &Mode::NearestTiesEven), e.next_down());
        assert_eq!(round_sub(a, b, &Mode::TowardPosInf), e);
        assert_eq!(round_sub(a, b, &Mode::TowardNegInf), e.next_down());
        assert_eq!(round_sub(a, b, &Mode::TowardZero), e);
    }

    #[test]
    fn test_mul() {
        let (a, b) = (0.3, 0.3);
        let e: f32 = 0.09;
        assert_eq!(round_mul(a, b, &Mode::NearestTiesEven), e);
        assert_eq!(round_mul(a, b, &Mode::TowardPosInf), e.next_up());
        assert_eq!(round_mul(a, b, &Mode::TowardNegInf), e);
        assert_eq!(round_mul(a, b, &Mode::TowardZero), e);

        let (a, b) = (-0.3, 0.3);
        let e: f32 = -0.09;
        assert_eq!(round_mul(a, b, &Mode::NearestTiesEven), e);
        assert_eq!(round_mul(a, b, &Mode::TowardPosInf), e);
        assert_eq!(round_mul(a, b, &Mode::TowardNegInf), e.next_down());
        assert_eq!(round_mul(a, b, &Mode::TowardZero), e);
    }

    #[test]
    fn test_div() {
        let (a, b) = (0.3, 0.2);
        let e: f32 = 1.5;
        assert_eq!(round_div(a, b, &Mode::NearestTiesEven), e);
        assert_eq!(round_div(a, b, &Mode::TowardPosInf), e.next_up());
        assert_eq!(round_div(a, b, &Mode::TowardNegInf), e);
        assert_eq!(round_div(a, b, &Mode::TowardZero), e);

        let (a, b) = (-0.3, 0.2);
        let e: f32 = -1.5;
        assert_eq!(round_div(a, b, &Mode::NearestTiesEven), e);
        assert_eq!(round_div(a, b, &Mode::TowardPosInf), e);
        assert_eq!(round_div(a, b, &Mode::TowardNegInf), e.next_down());
        assert_eq!(round_div(a, b, &Mode::TowardZero), e);
    }

    #[test]
    fn test_mul_add() {
        let (a, b, c) = (0.3, 0.2, 0.1);
        let e: f32 = 0.16;
        assert_eq!(round_mul_add(a, b, c, &Mode::NearestTiesEven), e.next_up());
        assert_eq!(round_mul_add(a, b, c, &Mode::TowardPosInf), e.next_up());
        assert_eq!(round_mul_add(a, b, c, &Mode::TowardNegInf), e);
        assert_eq!(round_mul_add(a, b, c, &Mode::TowardZero), e);

        let (a, b, c) = (-0.3, 0.2, -0.1);
        let e: f32 = -0.16;
        assert_eq!(
            round_mul_add(a, b, c, &Mode::NearestTiesEven),
            e.next_down()
        );
        assert_eq!(round_mul_add(a, b, c, &Mode::TowardPosInf), e);
        assert_eq!(round_mul_add(a, b, c, &Mode::TowardNegInf), e.next_down());
        assert_eq!(round_mul_add(a, b, c, &Mode::TowardZero), e);
    }
}

#[cfg(test)]
mod test_fn_arith_mono {
    use crate::RoundingMode as Mode;

    use super::*;

    #[test]
    fn test_add() {
        let (a, b) = (0.1, 0.2);
        assert_eq!(
            round_ties_even_add(a, b),
            round_add(a, b, &Mode::NearestTiesEven)
        );
        assert_eq!(ciel_add(a, b), round_add(a, b, &Mode::TowardPosInf));
        assert_eq!(floor_add(a, b), round_add(a, b, &Mode::TowardNegInf));
        assert_eq!(trunc_add(a, b), round_add(a, b, &Mode::TowardZero));

        let (a, b) = (-0.1, -0.2);
        assert_eq!(
            round_ties_even_add(a, b),
            round_add(a, b, &Mode::NearestTiesEven)
        );
        assert_eq!(ciel_add(a, b), round_add(a, b, &Mode::TowardPosInf));
        assert_eq!(floor_add(a, b), round_add(a, b, &Mode::TowardNegInf));
        assert_eq!(trunc_add(a, b), round_add(a, b, &Mode::TowardZero));
    }

    #[test]
    fn test_sub() {
        let (a, b) = (0.3, 0.1);
        assert_eq!(
            round_ties_even_sub(a, b),
            round_sub(a, b, &Mode::NearestTiesEven)
        );
        assert_eq!(ciel_sub(a, b), round_sub(a, b, &Mode::TowardPosInf));
        assert_eq!(floor_sub(a, b), round_sub(a, b, &Mode::TowardNegInf));
        assert_eq!(trunc_sub(a, b), round_sub(a, b, &Mode::TowardZero));

        let (a, b) = (-0.3, -0.1);
        assert_eq!(
            round_ties_even_sub(a, b),
            round_sub(a, b, &Mode::NearestTiesEven)
        );
        assert_eq!(ciel_sub(a, b), round_sub(a, b, &Mode::TowardPosInf));
        assert_eq!(floor_sub(a, b), round_sub(a, b, &Mode::TowardNegInf));
        assert_eq!(trunc_sub(a, b), round_sub(a, b, &Mode::TowardZero));
    }

    #[test]
    fn test_mul() {
        let (a, b) = (0.3, 0.3);
        assert_eq!(
            round_ties_even_mul(a, b),
            round_mul(a, b, &Mode::NearestTiesEven)
        );
        assert_eq!(ciel_mul(a, b), round_mul(a, b, &Mode::TowardPosInf));
        assert_eq!(floor_mul(a, b), round_mul(a, b, &Mode::TowardNegInf));
        assert_eq!(trunc_mul(a, b), round_mul(a, b, &Mode::TowardZero));

        let (a, b) = (0.3, 0.3);
        assert_eq!(
            round_ties_even_mul(a, b),
            round_mul(a, b, &Mode::NearestTiesEven)
        );
        assert_eq!(ciel_mul(a, b), round_mul(a, b, &Mode::TowardPosInf));
        assert_eq!(floor_mul(a, b), round_mul(a, b, &Mode::TowardNegInf));
        assert_eq!(trunc_mul(a, b), round_mul(a, b, &Mode::TowardZero));
    }

    #[test]
    fn test_div() {
        let (a, b) = (0.3, 0.2);
        assert_eq!(
            round_ties_even_div(a, b),
            round_div(a, b, &Mode::NearestTiesEven)
        );
        assert_eq!(ciel_div(a, b), round_div(a, b, &Mode::TowardPosInf));
        assert_eq!(floor_div(a, b), round_div(a, b, &Mode::TowardNegInf));
        assert_eq!(trunc_div(a, b), round_div(a, b, &Mode::TowardZero));

        let (a, b) = (0.3, 0.2);
        assert_eq!(
            round_ties_even_div(a, b),
            round_div(a, b, &Mode::NearestTiesEven)
        );
        assert_eq!(ciel_div(a, b), round_div(a, b, &Mode::TowardPosInf));
        assert_eq!(floor_div(a, b), round_div(a, b, &Mode::TowardNegInf));
        assert_eq!(trunc_div(a, b), round_div(a, b, &Mode::TowardZero));
    }

    #[test]
    fn test_mul_add() {
        let (a, b, c) = (0.3, 0.2, 0.1);
        assert_eq!(
            round_ties_even_mul_add(a, b, c),
            round_mul_add(a, b, c, &Mode::NearestTiesEven)
        );
        assert_eq!(
            ciel_mul_add(a, b, c),
            round_mul_add(a, b, c, &Mode::TowardPosInf)
        );
        assert_eq!(
            floor_mul_add(a, b, c),
            round_mul_add(a, b, c, &Mode::TowardNegInf)
        );
        assert_eq!(
            trunc_mul_add(a, b, c),
            round_mul_add(a, b, c, &Mode::TowardZero)
        );

        let (a, b) = (0.3, 0.2);
        assert_eq!(
            round_ties_even_mul_add(a, b, c),
            round_mul_add(a, b, c, &Mode::NearestTiesEven)
        );
        assert_eq!(
            ciel_mul_add(a, b, c),
            round_mul_add(a, b, c, &Mode::TowardPosInf)
        );
        assert_eq!(
            floor_mul_add(a, b, c),
            round_mul_add(a, b, c, &Mode::TowardNegInf)
        );
        assert_eq!(
            trunc_mul_add(a, b, c),
            round_mul_add(a, b, c, &Mode::TowardZero)
        );
    }
}

/// accrual test
#[cfg(test)]
mod test_fn_math {
    use crate::RoundingMode as Mode;

    use super::*;

    macro_rules! assert_nan {
        ($a:expr) => {
            assert!($a.is_nan())
        };
    }

    #[test]
    fn test_sqrt() {
        let a = 2.0;
        let e: f32 = 1.4142135;
        assert_eq!(round_sqrt(a, &Mode::NearestTiesEven), e);
        assert_eq!(round_sqrt(a, &Mode::TowardPosInf), e.next_up());
        assert_eq!(round_sqrt(a, &Mode::TowardNegInf), e);
        assert_eq!(round_sqrt(a, &Mode::TowardZero), e);

        let a = -2.0;
        assert_nan!(round_sqrt(a, &Mode::NearestTiesEven));
        assert_nan!(round_sqrt(a, &Mode::TowardPosInf));
        assert_nan!(round_sqrt(a, &Mode::TowardNegInf));
        assert_nan!(round_sqrt(a, &Mode::TowardZero));
    }
}

#[cfg(test)]
mod test_fn_math_mono {
    use crate::RoundingMode as Mode;

    use super::*;

    #[test]
    fn test_sqrt() {
        let a = 2.0;
        assert_eq!(
            round_ties_even_sqrt(a),
            round_sqrt(a, &Mode::NearestTiesEven)
        );
        assert_eq!(ciel_sqrt(a), round_sqrt(a, &Mode::TowardPosInf));
        assert_eq!(floor_sqrt(a), round_sqrt(a, &Mode::TowardNegInf));
        assert_eq!(trunc_sqrt(a), round_sqrt(a, &Mode::TowardZero));
    }
}

#[cfg(test)]
mod test_trait_arith {
    use crate::RoundingMode as Mode;

    use super::*;

    #[test]
    fn test_add() {
        let (a, b) = (0.1, 0.2);
        assert_eq!(
            a.round_add(b, &Mode::NearestTiesEven),
            round_add(a, b, &Mode::NearestTiesEven)
        );
        assert_eq!(
            a.round_add(b, &Mode::TowardPosInf),
            round_add(a, b, &Mode::TowardPosInf)
        );
        assert_eq!(
            a.round_add(b, &Mode::TowardNegInf),
            round_add(a, b, &Mode::TowardNegInf)
        );
        assert_eq!(
            a.round_add(b, &Mode::TowardZero),
            round_add(a, b, &Mode::TowardZero)
        );

        let (a, b) = (-0.1, -0.2);
        assert_eq!(
            a.round_add(b, &Mode::NearestTiesEven),
            round_add(a, b, &Mode::NearestTiesEven)
        );
        assert_eq!(
            a.round_add(b, &Mode::TowardPosInf),
            round_add(a, b, &Mode::TowardPosInf)
        );
        assert_eq!(
            a.round_add(b, &Mode::TowardNegInf),
            round_add(a, b, &Mode::TowardNegInf)
        );
        assert_eq!(
            a.round_add(b, &Mode::TowardZero),
            round_add(a, b, &Mode::TowardZero)
        );
    }

    #[test]
    fn test_sub() {
        let (a, b) = (0.3, 0.1);
        assert_eq!(
            a.round_sub(b, &Mode::NearestTiesEven),
            round_sub(a, b, &Mode::NearestTiesEven)
        );
        assert_eq!(
            a.round_sub(b, &Mode::TowardPosInf),
            round_sub(a, b, &Mode::TowardPosInf)
        );
        assert_eq!(
            a.round_sub(b, &Mode::TowardNegInf),
            round_sub(a, b, &Mode::TowardNegInf)
        );
        assert_eq!(
            a.round_sub(b, &Mode::TowardZero),
            round_sub(a, b, &Mode::TowardZero)
        );

        let (a, b) = (-0.3, -0.1);
        assert_eq!(
            a.round_sub(b, &Mode::NearestTiesEven),
            round_sub(a, b, &Mode::NearestTiesEven)
        );
        assert_eq!(
            a.round_sub(b, &Mode::TowardPosInf),
            round_sub(a, b, &Mode::TowardPosInf)
        );
        assert_eq!(
            a.round_sub(b, &Mode::TowardNegInf),
            round_sub(a, b, &Mode::TowardNegInf)
        );
        assert_eq!(
            a.round_sub(b, &Mode::TowardZero),
            round_sub(a, b, &Mode::TowardZero)
        );
    }

    #[test]
    fn test_mul() {
        let (a, b) = (0.3, 0.3);
        assert_eq!(
            a.round_mul(b, &Mode::NearestTiesEven),
            round_mul(a, b, &Mode::NearestTiesEven)
        );
        assert_eq!(
            a.round_mul(b, &Mode::TowardPosInf),
            round_mul(a, b, &Mode::TowardPosInf)
        );
        assert_eq!(
            a.round_mul(b, &Mode::TowardNegInf),
            round_mul(a, b, &Mode::TowardNegInf)
        );
        assert_eq!(
            a.round_mul(b, &Mode::TowardZero),
            round_mul(a, b, &Mode::TowardZero)
        );

        let (a, b) = (-0.3, 0.3);
        assert_eq!(
            a.round_mul(b, &Mode::NearestTiesEven),
            round_mul(a, b, &Mode::NearestTiesEven)
        );
        assert_eq!(
            a.round_mul(b, &Mode::TowardPosInf),
            round_mul(a, b, &Mode::TowardPosInf)
        );
        assert_eq!(
            a.round_mul(b, &Mode::TowardNegInf),
            round_mul(a, b, &Mode::TowardNegInf)
        );
        assert_eq!(
            a.round_mul(b, &Mode::TowardZero),
            round_mul(a, b, &Mode::TowardZero)
        );
    }

    #[test]
    fn test_div() {
        let (a, b) = (0.3, 0.2);
        assert_eq!(
            a.round_div(b, &Mode::NearestTiesEven),
            round_div(a, b, &Mode::NearestTiesEven)
        );
        assert_eq!(
            a.round_div(b, &Mode::TowardPosInf),
            round_div(a, b, &Mode::TowardPosInf)
        );
        assert_eq!(
            a.round_div(b, &Mode::TowardNegInf),
            round_div(a, b, &Mode::TowardNegInf)
        );
        assert_eq!(
            a.round_div(b, &Mode::TowardZero),
            round_div(a, b, &Mode::TowardZero)
        );

        let (a, b) = (-0.3, 0.2);
        assert_eq!(
            a.round_div(b, &Mode::NearestTiesEven),
            round_div(a, b, &Mode::NearestTiesEven)
        );
        assert_eq!(
            a.round_div(b, &Mode::TowardPosInf),
            round_div(a, b, &Mode::TowardPosInf)
        );
        assert_eq!(
            a.round_div(b, &Mode::TowardNegInf),
            round_div(a, b, &Mode::TowardNegInf)
        );
        assert_eq!(
            a.round_div(b, &Mode::TowardZero),
            round_div(a, b, &Mode::TowardZero)
        );
    }

    #[test]
    fn test_mul_add() {
        let (a, b, c) = (0.3, 0.2, 0.1);
        assert_eq!(
            a.round_mul_add(b, c, &Mode::NearestTiesEven),
            round_mul_add(a, b, c, &Mode::NearestTiesEven)
        );
        assert_eq!(
            a.round_mul_add(b, c, &Mode::TowardPosInf),
            round_mul_add(a, b, c, &Mode::TowardPosInf)
        );
        assert_eq!(
            a.round_mul_add(b, c, &Mode::TowardNegInf),
            round_mul_add(a, b, c, &Mode::TowardNegInf)
        );
        assert_eq!(
            a.round_mul_add(b, c, &Mode::TowardZero),
            round_mul_add(a, b, c, &Mode::TowardZero)
        );

        let (a, b, c) = (-0.3, 0.2, -0.1);
        assert_eq!(
            a.round_mul_add(b, c, &Mode::NearestTiesEven),
            round_mul_add(a, b, c, &Mode::NearestTiesEven)
        );
        assert_eq!(
            a.round_mul_add(b, c, &Mode::TowardPosInf),
            round_mul_add(a, b, c, &Mode::TowardPosInf)
        );
        assert_eq!(
            a.round_mul_add(b, c, &Mode::TowardNegInf),
            round_mul_add(a, b, c, &Mode::TowardNegInf)
        );
        assert_eq!(
            a.round_mul_add(b, c, &Mode::TowardZero),
            round_mul_add(a, b, c, &Mode::TowardZero)
        );
    }
}

#[cfg(test)]
mod test_trait_arith_mono {
    use crate::RoundingMode as Mode;

    use super::*;
    #[test]
    fn test_add() {
        let (a, b) = (0.1, 0.2);
        assert_eq!(
            a.round_ties_even_add(b),
            a.round_add(b, &Mode::NearestTiesEven)
        );
        assert_eq!(a.ciel_add(b), a.round_add(b, &Mode::TowardPosInf));
        assert_eq!(a.floor_add(b), a.round_add(b, &Mode::TowardNegInf));
        assert_eq!(a.trunc_add(b), a.round_add(b, &Mode::TowardZero));

        let (a, b) = (-0.1, -0.2);
        assert_eq!(
            a.round_ties_even_add(b),
            a.round_add(b, &Mode::NearestTiesEven)
        );
        assert_eq!(a.ciel_add(b), a.round_add(b, &Mode::TowardPosInf));
        assert_eq!(a.floor_add(b), a.round_add(b, &Mode::TowardNegInf));
        assert_eq!(a.trunc_add(b), a.round_add(b, &Mode::TowardZero));
    }

    #[test]
    fn test_sub() {
        let (a, b) = (0.3, 0.1);
        assert_eq!(
            a.round_ties_even_sub(b),
            a.round_sub(b, &Mode::NearestTiesEven)
        );
        assert_eq!(a.ciel_sub(b), a.round_sub(b, &Mode::TowardPosInf));
        assert_eq!(a.floor_sub(b), a.round_sub(b, &Mode::TowardNegInf));
        assert_eq!(a.trunc_sub(b), a.round_sub(b, &Mode::TowardZero));

        let (a, b) = (-0.3, -0.1);
        assert_eq!(
            a.round_ties_even_sub(b),
            a.round_sub(b, &Mode::NearestTiesEven)
        );
        assert_eq!(a.ciel_sub(b), a.round_sub(b, &Mode::TowardPosInf));
        assert_eq!(a.floor_sub(b), a.round_sub(b, &Mode::TowardNegInf));
        assert_eq!(a.trunc_sub(b), a.round_sub(b, &Mode::TowardZero));
    }

    #[test]
    fn test_mul() {
        let (a, b) = (0.3, 0.3);
        assert_eq!(
            a.round_ties_even_mul(b),
            a.round_mul(b, &Mode::NearestTiesEven)
        );
        assert_eq!(a.ciel_mul(b), a.round_mul(b, &Mode::TowardPosInf));
        assert_eq!(a.floor_mul(b), a.round_mul(b, &Mode::TowardNegInf));
        assert_eq!(a.trunc_mul(b), a.round_mul(b, &Mode::TowardZero));

        let (a, b) = (0.3, 0.3);
        assert_eq!(
            a.round_ties_even_mul(b),
            a.round_mul(b, &Mode::NearestTiesEven)
        );
        assert_eq!(a.ciel_mul(b), a.round_mul(b, &Mode::TowardPosInf));
        assert_eq!(a.floor_mul(b), a.round_mul(b, &Mode::TowardNegInf));
        assert_eq!(a.trunc_mul(b), a.round_mul(b, &Mode::TowardZero));
    }

    #[test]
    fn test_div() {
        let (a, b) = (0.3, 0.2);
        assert_eq!(
            a.round_ties_even_div(b),
            a.round_div(b, &Mode::NearestTiesEven)
        );
        assert_eq!(a.ciel_div(b), a.round_div(b, &Mode::TowardPosInf));
        assert_eq!(a.floor_div(b), a.round_div(b, &Mode::TowardNegInf));
        assert_eq!(a.trunc_div(b), a.round_div(b, &Mode::TowardZero));

        let (a, b) = (0.3, 0.2);
        assert_eq!(
            a.round_ties_even_div(b),
            a.round_div(b, &Mode::NearestTiesEven)
        );
        assert_eq!(a.ciel_div(b), a.round_div(b, &Mode::TowardPosInf));
        assert_eq!(a.floor_div(b), a.round_div(b, &Mode::TowardNegInf));
        assert_eq!(a.trunc_div(b), a.round_div(b, &Mode::TowardZero));
    }

    #[test]
    fn test_mul_add() {
        let (a, b, c) = (0.3, 0.2, 0.1);
        assert_eq!(
            a.round_ties_even_mul_add(b, c),
            a.round_mul_add(b, c, &Mode::NearestTiesEven)
        );
        assert_eq!(
            a.ciel_mul_add(b, c),
            a.round_mul_add(b, c, &Mode::TowardPosInf)
        );
        assert_eq!(
            a.floor_mul_add(b, c),
            a.round_mul_add(b, c, &Mode::TowardNegInf)
        );
        assert_eq!(
            a.trunc_mul_add(b, c),
            a.round_mul_add(b, c, &Mode::TowardZero)
        );

        let (a, b) = (0.3, 0.2);
        assert_eq!(
            a.round_ties_even_mul_add(b, c),
            a.round_mul_add(b, c, &Mode::NearestTiesEven)
        );
        assert_eq!(
            a.ciel_mul_add(b, c),
            a.round_mul_add(b, c, &Mode::TowardPosInf)
        );
        assert_eq!(
            a.floor_mul_add(b, c),
            a.round_mul_add(b, c, &Mode::TowardNegInf)
        );
        assert_eq!(
            a.trunc_mul_add(b, c),
            a.round_mul_add(b, c, &Mode::TowardZero)
        );
    }
}

#[cfg(test)]
mod test_trait_math {
    use crate::RoundingMode as Mode;

    use super::*;

    macro_rules! assert_nan {
        ($a:expr) => {
            assert!($a.is_nan())
        };
    }

    #[test]
    fn test_sqrt() {
        let a = 2.0;
        assert_eq!(
            a.round_sqrt(&Mode::NearestTiesEven),
            round_sqrt(a, &Mode::NearestTiesEven)
        );
        assert_eq!(
            a.round_sqrt(&Mode::TowardPosInf),
            round_sqrt(a, &Mode::TowardPosInf)
        );
        assert_eq!(
            a.round_sqrt(&Mode::TowardNegInf),
            round_sqrt(a, &Mode::TowardNegInf)
        );
        assert_eq!(
            a.round_sqrt(&Mode::TowardZero),
            round_sqrt(a, &Mode::TowardZero)
        );

        let a: f32 = -2.0;
        assert_nan!(a.round_sqrt(&Mode::NearestTiesEven));
        assert_nan!(a.round_sqrt(&Mode::TowardPosInf));
        assert_nan!(a.round_sqrt(&Mode::TowardNegInf));
        assert_nan!(a.round_sqrt(&Mode::TowardZero));
    }
}

#[cfg(test)]
mod test_trait_math_mono {
    use crate::RoundingMode as Mode;

    use super::*;

    #[test]
    fn test_sqrt() {
        let a = 2.0;
        assert_eq!(
            a.round_ties_even_sqrt(),
            a.round_sqrt(&Mode::NearestTiesEven)
        );
        assert_eq!(a.ciel_sqrt(), a.round_sqrt(&Mode::TowardPosInf));
        assert_eq!(a.floor_sqrt(), a.round_sqrt(&Mode::TowardNegInf));
        assert_eq!(a.trunc_sqrt(), a.round_sqrt(&Mode::TowardZero));
    }
}
