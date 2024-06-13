use std::ffi::c_int;

use crate::internal::*;
use crate::RoundingMode;

extern "C" {
    fn c_add_f16(mode: c_int, a: f16, b: f16, dst: *mut f16) -> c_int;
    fn c_sub_f16(mode: c_int, a: f16, b: f16, dst: *mut f16) -> c_int;
    fn c_mul_f16(mode: c_int, a: f16, b: f16, dst: *mut f16) -> c_int;
    fn c_div_f16(mode: c_int, a: f16, b: f16, dst: *mut f16) -> c_int;
    fn c_fma_f16(mode: c_int, a: f16, b: f16, c: f16, dst: *mut f16) -> c_int;
    fn c_sqrt_f16(mode: c_int, a: f16, dst: *mut f16) -> c_int;
}

impl_round_func_binary_all!(
    f16,
    round_add => c_add_f16,
    round_sub => c_sub_f16,
    round_mul => c_mul_f16,
    round_div => c_div_f16,
    round_mul_add => c_fma_f16,
);

impl_func_unary!(
    /// Returns `a.sqrt()` as specific rounding mode.
    ///
    /// # Safety
    ///
    /// Panics when fail to set/rest rounding mode.
    #[inline]
    => f16, round_sqrt, c_sqrt_f16
);
