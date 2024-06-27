#include "softfloat.h"

float128_t c_add_f128(uint_fast8_t const mode, float128_t const a, float128_t const b) {
  volatile uint_fast8_t const dflt = softfloat_roundingMode;
  softfloat_roundingMode = mode;

  volatile float128_t const temp = f128_add(a, b);

  softfloat_roundingMode = dflt;

  return temp;
}

float128_t c_sub_f128(uint_fast8_t const mode, float128_t const a, float128_t const b) {
  volatile uint_fast8_t const dflt = softfloat_roundingMode;
  softfloat_roundingMode = mode;

  volatile float128_t const temp = f128_sub(a, b);

  softfloat_roundingMode = dflt;

  return temp;
}

float128_t c_mul_f128(uint_fast8_t const mode, float128_t const a, float128_t const b) {
  volatile uint_fast8_t const dflt = softfloat_roundingMode;
  softfloat_roundingMode = mode;

  volatile float128_t const temp = f128_mul(a, b);

  softfloat_roundingMode = dflt;

  return temp;
}

float128_t c_div_f128(uint_fast8_t const mode, float128_t const a, float128_t const b) {
  volatile uint_fast8_t const dflt = softfloat_roundingMode;
  softfloat_roundingMode = mode;

  volatile float128_t const temp = f128_div(a, b);

  softfloat_roundingMode = dflt;

  return temp;
}

float128_t c_fma_f128(uint_fast8_t const mode, float128_t const a, float128_t const b, float128_t const c) {
  volatile uint_fast8_t const dflt = softfloat_roundingMode;
  softfloat_roundingMode = mode;

  volatile float128_t const temp = f128_mulAdd(a, b, c);

  softfloat_roundingMode = dflt;

  return temp;
}

float128_t c_sqrt_f128(uint_fast8_t const mode, float128_t const a) {
  volatile uint_fast8_t const dflt = softfloat_roundingMode;
  softfloat_roundingMode = mode;

  volatile float128_t const temp = f128_sqrt(a);

  softfloat_roundingMode = dflt;

  return temp;
}
