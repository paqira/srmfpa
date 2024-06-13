use std::ffi::c_int;

use crate::internal::*;
use crate::RoundingMode;

extern "C" {
    fn c_add_f128(mode: c_int, a: f128, b: f128, dst: *mut f128) -> c_int;
    fn c_sub_f128(mode: c_int, a: f128, b: f128, dst: *mut f128) -> c_int;
    fn c_mul_f128(mode: c_int, a: f128, b: f128, dst: *mut f128) -> c_int;
    fn c_div_f128(mode: c_int, a: f128, b: f128, dst: *mut f128) -> c_int;
    fn c_fma_f128(mode: c_int, a: f128, b: f128, c: f128, dst: *mut f128) -> c_int;
    fn c_sqrt_f128(mode: c_int, a: f128, dst: *mut f128) -> c_int;
}

impl_round_func_binary_all!(
    f128,
    round_add => c_add_f128,
    round_sub => c_sub_f128,
    round_mul => c_mul_f128,
    round_div => c_div_f128,
    round_mul_add => c_fma_f128,
);

impl_func_unary!(
    /// Returns `a.sqrt()` as specific rounding mode.
    ///
    /// # Safety
    ///
    /// Panics when fail to set/rest rounding mode.
    #[inline]
    => f128, round_sqrt, c_sqrt_f128
);
