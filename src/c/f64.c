#include "fenv_access.h"
#include "unlikely.h"
#include <fenv.h>
#include <math.h>

// TODO: use _Float64
typedef double float64_t;

int c_add_f64(int const mode, float64_t const a, float64_t const b,
              float64_t *const restrict dst) {
  int err = 0;

  int const dflt = fegetround();
  if (unlikely(dflt < 0)) {
    return dflt;
  }

  err = fesetround(mode);
  if (unlikely(err != 0)) {
    return err;
  }

  float64_t const temp = a + b;

  err = fesetround(dflt);
  if (unlikely(err != 0)) {
    return err;
  }

  *dst = temp;

  return 0;
}

int c_sub_f64(int const mode, float64_t const a, float64_t const b,
              float64_t *const restrict dst) {
  int err = 0;

  int const dflt = fegetround();
  if (unlikely(dflt < 0)) {
    return dflt;
  }

  err = fesetround(mode);
  if (unlikely(err != 0)) {
    return err;
  }

  float64_t const temp = a - b;

  err = fesetround(dflt);
  if (unlikely(err != 0)) {
    return err;
  }

  *dst = temp;

  return 0;
}

int c_mul_f64(int const mode, float64_t const a, float64_t const b,
              float64_t *const restrict dst) {
  int err = 0;

  int const dflt = fegetround();
  if (unlikely(dflt < 0)) {
    return dflt;
  }

  err = fesetround(mode);
  if (unlikely(err != 0)) {
    return err;
  }

  float64_t const temp = a * b;

  err = fesetround(dflt);
  if (unlikely(err != 0)) {
    return err;
  }

  *dst = temp;

  return 0;
}

int c_div_f64(int const mode, float64_t const a, float64_t const b,
              float64_t *const restrict dst) {
  int err = 0;

  int const dflt = fegetround();
  if (unlikely(dflt < 0)) {
    return dflt;
  }

  err = fesetround(mode);
  if (unlikely(err != 0)) {
    return err;
  }

  float64_t const temp = a / b;

  err = fesetround(dflt);
  if (unlikely(err != 0)) {
    return err;
  }

  *dst = temp;

  return 0;
}

int c_sqrt_f64(int const mode, float64_t const a,
               float64_t *const restrict dst) {
  int err = 0;

  int const dflt = fegetround();
  if (unlikely(dflt < 0)) {
    return dflt;
  }

  err = fesetround(mode);
  if (unlikely(err != 0)) {
    return err;
  }

  float64_t const temp = sqrt(a);

  err = fesetround(dflt);
  if (unlikely(err != 0)) {
    return err;
  }

  *dst = temp;

  return 0;
}