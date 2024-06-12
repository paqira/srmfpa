#include "softfloat.h"

float32_t c_add_f32(uint_fast8_t const mode, float32_t const a, float32_t const b) {
  volatile uint_fast8_t const dflt = softfloat_roundingMode;
  softfloat_roundingMode = mode;

  volatile float32_t const temp = f32_add(a, b);

  softfloat_roundingMode = dflt;

  return temp;
}

float32_t c_sub_f32(uint_fast8_t const mode, float32_t const a, float32_t const b) {
  volatile uint_fast8_t const dflt = softfloat_roundingMode;
  softfloat_roundingMode = mode;

  volatile float32_t const temp = f32_sub(a, b);

  softfloat_roundingMode = dflt;

  return temp;
}

float32_t c_mul_f32(uint_fast8_t const mode, float32_t const a, float32_t const b) {
  volatile uint_fast8_t const dflt = softfloat_roundingMode;
  softfloat_roundingMode = mode;

  volatile float32_t const temp = f32_mul(a, b);

  softfloat_roundingMode = dflt;

  return temp;
}

float32_t c_div_f32(uint_fast8_t const mode, float32_t const a, float32_t const b) {
  volatile uint_fast8_t const dflt = softfloat_roundingMode;
  softfloat_roundingMode = mode;

  volatile float32_t const temp = f32_div(a, b);

  softfloat_roundingMode = dflt;

  return temp;
}

float32_t c_fma_f32(uint_fast8_t const mode, float32_t const a, float32_t const b, float32_t const c) {
  volatile uint_fast8_t const dflt = softfloat_roundingMode;
  softfloat_roundingMode = mode;

  volatile float32_t const temp = f32_mulAdd(a, b, c);

  softfloat_roundingMode = dflt;

  return temp;
}

float32_t c_sqrt_f32(uint_fast8_t const mode, float32_t const a) {
  volatile uint_fast8_t const dflt = softfloat_roundingMode;
  softfloat_roundingMode = mode;

  volatile float32_t const temp = f32_sqrt(a);

  softfloat_roundingMode = dflt;

  return temp;
}
