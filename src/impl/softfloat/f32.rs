use fpa_specr_berkeley_softfloat::*;

use crate::RoundingMode;

#[inline]
fn from_f32(value: f32) -> float32_t {
    float32_t { v: value.to_bits() }
}

#[inline]
fn to_f32(value: float32_t) -> f32 {
    f32::from_bits(value.v)
}

pub fn round_add(a: f32, b: f32, mode: &RoundingMode) -> f32 {
    let r = unsafe { c_add_f32(mode.as_berkeley_softfloat(), from_f32(a), from_f32(b)) };
    to_f32(r)
}

pub fn round_sub(a: f32, b: f32, mode: &RoundingMode) -> f32 {
    let r = unsafe { c_sub_f32(mode.as_berkeley_softfloat(), from_f32(a), from_f32(b)) };
    to_f32(r)
}

pub fn round_mul(a: f32, b: f32, mode: &RoundingMode) -> f32 {
    let r = unsafe { c_mul_f32(mode.as_berkeley_softfloat(), from_f32(a), from_f32(b)) };
    to_f32(r)
}

pub fn round_div(a: f32, b: f32, mode: &RoundingMode) -> f32 {
    let r = unsafe { c_div_f32(mode.as_berkeley_softfloat(), from_f32(a), from_f32(b)) };
    to_f32(r)
}

pub fn round_mul_add(a: f32, b: f32, c: f32, mode: &RoundingMode) -> f32 {
    let r = unsafe {
        c_fma_f32(
            mode.as_berkeley_softfloat(),
            from_f32(a),
            from_f32(b),
            from_f32(c),
        )
    };
    to_f32(r)
}

pub fn round_sqrt(a: f32, mode: &RoundingMode) -> f32 {
    let r = unsafe { c_sqrt_f32(mode.as_berkeley_softfloat(), from_f32(a)) };
    to_f32(r)
}
