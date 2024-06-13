use std::ffi::c_int;

use crate::internal::*;
use crate::RoundingMode;

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

impl_func_unary!(
    /// Returns `a.sqrt()` as specific rounding mode.
    ///
    /// # Safety
    ///
    /// Panics when fail to set/rest rounding mode.
    #[inline]
    => f32, round_sqrt, c_sqrt_f32
);
