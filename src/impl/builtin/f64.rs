use std::ffi::c_int;

use crate::internal::*;
use crate::RoundingMode;

extern "C" {
    fn c_add_f64(mode: c_int, a: f64, b: f64, dst: *mut f64) -> c_int;
    fn c_sub_f64(mode: c_int, a: f64, b: f64, dst: *mut f64) -> c_int;
    fn c_mul_f64(mode: c_int, a: f64, b: f64, dst: *mut f64) -> c_int;
    fn c_div_f64(mode: c_int, a: f64, b: f64, dst: *mut f64) -> c_int;
    fn c_fma_f64(mode: c_int, a: f64, b: f64, c: f64, dst: *mut f64) -> c_int;
    fn c_sqrt_f64(mode: c_int, a: f64, dst: *mut f64) -> c_int;
}

impl_round_func_binary_all!(
    f64,
    round_add => c_add_f64,
    round_sub => c_sub_f64,
    round_mul => c_mul_f64,
    round_div => c_div_f64,
    round_mul_add => c_fma_f64,
);

impl_func_unary!(
    /// Returns `a.sqrt()` as specific rounding mode.
    ///
    /// # Safety
    ///
    /// Panics when fail to set/rest rounding mode.
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    => f64, round_sqrt, c_sqrt_f64
);
