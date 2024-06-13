use fpa_specr_berkeley_softfloat::*;

use crate::RoundingMode;

#[inline]
fn from_f16(value: f16) -> float16_t {
    float16_t { v: value.to_bits() }
}

#[inline]
fn to_f16(value: float16_t) -> f16 {
    f16::from_bits(value.v)
}

/// Returns `a + b` as specific rounding mode.
#[inline]
pub fn round_add(a: f16, b: f16, mode: &RoundingMode) -> f16 {
    let r = unsafe { c_add_f16(mode.as_berkeley_softfloat(), from_f16(a), from_f16(b)) };
    to_f16(r)
}

/// Returns `a - b` as specific rounding mode.
#[inline]
pub fn round_sub(a: f16, b: f16, mode: &RoundingMode) -> f16 {
    let r = unsafe { c_sub_f16(mode.as_berkeley_softfloat(), from_f16(a), from_f16(b)) };
    to_f16(r)
}

/// Returns `a * b` as specific rounding mode.
#[inline]
pub fn round_mul(a: f16, b: f16, mode: &RoundingMode) -> f16 {
    let r = unsafe { c_mul_f16(mode.as_berkeley_softfloat(), from_f16(a), from_f16(b)) };
    to_f16(r)
}

/// Returns `a * b` as specific rounding mode.
#[inline]
pub fn round_div(a: f16, b: f16, mode: &RoundingMode) -> f16 {
    let r = unsafe { c_div_f16(mode.as_berkeley_softfloat(), from_f16(a), from_f16(b)) };
    to_f16(r)
}

/// Returns `a * b + c` with single rounding (fused multiply-add) as specific rounding mode.
#[inline]
pub fn round_mul_add(a: f16, b: f16, c: f16, mode: &RoundingMode) -> f16 {
    let r = unsafe {
        c_fma_f16(
            mode.as_berkeley_softfloat(),
            from_f16(a),
            from_f16(b),
            from_f16(c),
        )
    };
    to_f16(r)
}

/// Returns `a.sqrt()` as specific rounding mode.
#[inline]
pub fn round_sqrt(a: f16, mode: &RoundingMode) -> f16 {
    let r = unsafe { c_sqrt_f16(mode.as_berkeley_softfloat(), from_f16(a)) };
    to_f16(r)
}
