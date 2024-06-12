use fpa_specr_berkeley_softfloat::*;

use crate::RoundingMode;

#[inline]
fn from_f128(value: f128) -> float128_t {
    float128_t {
        v: [
            (value.to_bits() >> 64) as u64,
            (value.to_bits() & 0xFFFF_FFFF) as u64,
        ],
    }
}

#[inline]
fn to_f128(value: float128_t) -> f128 {
    f128::from_bits((value.v[0] as u128) << 64 + value.v[1] as u128)
}

/// Returns `a + b` as specific rounding mode.
#[inline]
pub fn round_add(a: f128, b: f128, mode: &RoundingMode) -> f128 {
    let r = unsafe { c_add_f128(mode.as_berkeley_softfloat(), from_f128(a), from_f128(b)) };
    to_f128(r)
}

/// Returns `a - b` as specific rounding mode.
#[inline]
pub fn round_sub(a: f128, b: f128, mode: &RoundingMode) -> f128 {
    let r = unsafe { c_sub_f128(mode.as_berkeley_softfloat(), from_f128(a), from_f128(b)) };
    to_f128(r)
}

/// Returns `a * b` as specific rounding mode.
#[inline]
pub fn round_mul(a: f128, b: f128, mode: &RoundingMode) -> f128 {
    let r = unsafe { c_mul_f128(mode.as_berkeley_softfloat(), from_f128(a), from_f128(b)) };
    to_f128(r)
}

/// Returns `a / b` as specific rounding mode.
#[inline]
pub fn round_div(a: f128, b: f128, mode: &RoundingMode) -> f128 {
    let r = unsafe { c_div_f128(mode.as_berkeley_softfloat(), from_f128(a), from_f128(b)) };
    to_f128(r)
}

/// Returns `a * b + c` with single rounding (fused multiply-add) as specific rounding mode.
#[inline]
pub fn round_mul_add(a: f128, b: f128, c: f128, mode: &RoundingMode) -> f128 {
    let r = unsafe {
        c_fma_f128(
            mode.as_berkeley_softfloat(),
            from_f128(a),
            from_f128(b),
            from_f128(c),
        )
    };
    to_f128(r)
}

/// Returns `a.sqrt()` as specific rounding mode.
#[inline]
pub fn round_sqrt(a: f128, mode: &RoundingMode) -> f128 {
    let r = unsafe { c_sqrt_f128(mode.as_berkeley_softfloat(), from_f128(a)) };
    to_f128(r)
}
