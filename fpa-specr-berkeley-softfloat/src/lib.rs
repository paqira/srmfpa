#![allow(non_camel_case_types, non_upper_case_globals)]

extern "C" {
    pub fn c_add_f16(mode: u8, a: float16_t, b: float16_t) -> float16_t;
    pub fn c_sub_f16(mode: u8, a: float16_t, b: float16_t) -> float16_t;
    pub fn c_mul_f16(mode: u8, a: float16_t, b: float16_t) -> float16_t;
    pub fn c_div_f16(mode: u8, a: float16_t, b: float16_t) -> float16_t;
    pub fn c_fma_f16(mode: u8, a: float16_t, b: float16_t, c: float16_t) -> float16_t;
    pub fn c_sqrt_f16(mode: u8, a: float16_t) -> float16_t;
    //
    pub fn c_add_f32(mode: u8, a: float32_t, b: float32_t) -> float32_t;
    pub fn c_sub_f32(mode: u8, a: float32_t, b: float32_t) -> float32_t;
    pub fn c_mul_f32(mode: u8, a: float32_t, b: float32_t) -> float32_t;
    pub fn c_div_f32(mode: u8, a: float32_t, b: float32_t) -> float32_t;
    pub fn c_fma_f32(mode: u8, a: float32_t, b: float32_t, c: float32_t) -> float32_t;
    pub fn c_sqrt_f32(mode: u8, a: float32_t) -> float32_t;
    //
    pub fn c_add_f64(mode: u8, a: float64_t, b: float64_t) -> float64_t;
    pub fn c_sub_f64(mode: u8, a: float64_t, b: float64_t) -> float64_t;
    pub fn c_mul_f64(mode: u8, a: float64_t, b: float64_t) -> float64_t;
    pub fn c_div_f64(mode: u8, a: float64_t, b: float64_t) -> float64_t;
    pub fn c_fma_f64(mode: u8, a: float64_t, b: float64_t, c: float64_t) -> float64_t;
    pub fn c_sqrt_f64(mode: u8, a: float64_t) -> float64_t;
    //
    pub fn c_add_f128(mode: u8, a: float128_t, b: float128_t) -> float128_t;
    pub fn c_sub_f128(mode: u8, a: float128_t, b: float128_t) -> float128_t;
    pub fn c_mul_f128(mode: u8, a: float128_t, b: float128_t) -> float128_t;
    pub fn c_div_f128(mode: u8, a: float128_t, b: float128_t) -> float128_t;
    pub fn c_fma_f128(mode: u8, a: float128_t, b: float128_t, c: float128_t) -> float128_t;
    pub fn c_sqrt_f128(mode: u8, a: float128_t) -> float128_t;
}

#[repr(C)]
pub struct float16_t {
    pub v: u16,
}

#[repr(C)]
pub struct float32_t {
    pub v: u32,
}

#[repr(C)]
pub struct float64_t {
    pub v: u64,
}

#[repr(C)]
pub struct float128_t {
    pub v: [u64; 2],
}

pub const softfloat_round_near_even: u8 = 0;
pub const softfloat_round_minMag: u8 = 1;
pub const softfloat_round_min: u8 = 2;
pub const softfloat_round_max: u8 = 3;
