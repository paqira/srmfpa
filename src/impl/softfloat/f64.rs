use fpa_specr_berkeley_softfloat::*;

use crate::RoundingMode;

#[inline]
fn from_f64(value: f64) -> float64_t {
    float64_t { v: value.to_bits() }
}

#[inline]
fn to_f64(value: float64_t) -> f64 {
    f64::from_bits(value.v)
}

pub fn round_add(a: f64, b: f64, mode: &RoundingMode) -> f64 {
    let r = unsafe { c_add_f64(mode.as_berkeley_softfloat(), from_f64(a), from_f64(b)) };
    to_f64(r)
}

pub fn round_sub(a: f64, b: f64, mode: &RoundingMode) -> f64 {
    let r = unsafe { c_sub_f64(mode.as_berkeley_softfloat(), from_f64(a), from_f64(b)) };
    to_f64(r)
}

pub fn round_mul(a: f64, b: f64, mode: &RoundingMode) -> f64 {
    let r = unsafe { c_mul_f64(mode.as_berkeley_softfloat(), from_f64(a), from_f64(b)) };
    to_f64(r)
}

pub fn round_div(a: f64, b: f64, mode: &RoundingMode) -> f64 {
    let r = unsafe { c_div_f64(mode.as_berkeley_softfloat(), from_f64(a), from_f64(b)) };
    to_f64(r)
}

pub fn round_mul_add(a: f64, b: f64, c: f64, mode: &RoundingMode) -> f64 {
    let r = unsafe {
        c_fma_f64(
            mode.as_berkeley_softfloat(),
            from_f64(a),
            from_f64(b),
            from_f64(c),
        )
    };
    to_f64(r)
}

pub fn round_sqrt(a: f64, mode: &RoundingMode) -> f64 {
    let r = unsafe { c_sqrt_f64(mode.as_berkeley_softfloat(), from_f64(a)) };
    to_f64(r)
}
