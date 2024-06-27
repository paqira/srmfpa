#include "softfloat.h"

float16_t c_add_f16(uint_fast8_t const mode, float16_t const a, float16_t const b) {
  volatile uint_fast8_t const dflt = softfloat_roundingMode;
  softfloat_roundingMode = mode;

  volatile float16_t const temp = f16_add(a, b);

  softfloat_roundingMode = dflt;

  return temp;
}

float16_t c_sub_f16(uint_fast8_t const mode, float16_t const a, float16_t const b) {
  volatile uint_fast8_t const dflt = softfloat_roundingMode;
  softfloat_roundingMode = mode;

  volatile float16_t const temp = f16_sub(a, b);

  softfloat_roundingMode = dflt;

  return temp;
}

float16_t c_mul_f16(uint_fast8_t const mode, float16_t const a, float16_t const b) {
  volatile uint_fast8_t const dflt = softfloat_roundingMode;
  softfloat_roundingMode = mode;

  volatile float16_t const temp = f16_mul(a, b);

  softfloat_roundingMode = dflt;

  return temp;
}

float16_t c_div_f16(uint_fast8_t const mode, float16_t const a, float16_t const b) {
  volatile uint_fast8_t const dflt = softfloat_roundingMode;
  softfloat_roundingMode = mode;

  volatile float16_t const temp = f16_div(a, b);

  softfloat_roundingMode = dflt;

  return temp;
}

float16_t c_fma_f16(uint_fast8_t const mode, float16_t const a, float16_t const b, float16_t const c) {
  volatile uint_fast8_t const dflt = softfloat_roundingMode;
  softfloat_roundingMode = mode;

  volatile float16_t const temp = f16_mulAdd(a, b, c);

  softfloat_roundingMode = dflt;

  return temp;
}

float16_t c_sqrt_f16(uint_fast8_t const mode, float16_t const a) {
  volatile uint_fast8_t const dflt = softfloat_roundingMode;
  softfloat_roundingMode = mode;

  volatile float16_t const temp = f16_sqrt(a);

  softfloat_roundingMode = dflt;

  return temp;
}
