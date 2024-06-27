#include "softfloat.h"

float64_t c_add_f64(uint_fast8_t const mode, float64_t const a, float64_t const b) {
  volatile uint_fast8_t const dflt = softfloat_roundingMode;
  softfloat_roundingMode = mode;

  volatile float64_t const temp = f64_add(a, b);

  softfloat_roundingMode = dflt;

  return temp;
}

float64_t c_sub_f64(uint_fast8_t const mode, float64_t const a, float64_t const b) {
  volatile uint_fast8_t const dflt = softfloat_roundingMode;
  softfloat_roundingMode = mode;

  volatile float64_t const temp = f64_sub(a, b);

  softfloat_roundingMode = dflt;

  return temp;
}

float64_t c_mul_f64(uint_fast8_t const mode, float64_t const a, float64_t const b) {
  volatile uint_fast8_t const dflt = softfloat_roundingMode;
  softfloat_roundingMode = mode;

  volatile float64_t const temp = f64_mul(a, b);

  softfloat_roundingMode = dflt;

  return temp;
}

float64_t c_div_f64(uint_fast8_t const mode, float64_t const a, float64_t const b) {
  volatile uint_fast8_t const dflt = softfloat_roundingMode;
  softfloat_roundingMode = mode;

  volatile float64_t const temp = f64_div(a, b);

  softfloat_roundingMode = dflt;

  return temp;
}

float64_t c_fma_f64(uint_fast8_t const mode, float64_t const a, float64_t const b, float64_t const c) {
  volatile uint_fast8_t const dflt = softfloat_roundingMode;
  softfloat_roundingMode = mode;

  volatile float64_t const temp = f64_mulAdd(a, b, c);

  softfloat_roundingMode = dflt;

  return temp;
}

float64_t c_sqrt_f64(uint_fast8_t const mode, float64_t const a) {
  volatile uint_fast8_t const dflt = softfloat_roundingMode;
  softfloat_roundingMode = mode;

  volatile float64_t const temp = f64_sqrt(a);

  softfloat_roundingMode = dflt;

  return temp;
}
