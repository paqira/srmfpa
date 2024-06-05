#include "fenv_access.h"
#include "unlikely.h"
#include <fenv.h>
#include <math.h>

// TODO: use _Float32
typedef float float32_t;

int c_add_f32(int const mode, float32_t const a, float32_t const b,
              float32_t *const restrict dst) {
  int err = 0;

  int const dflt = fegetround();
  if (unlikely(dflt < 0)) {
    return dflt;
  }

  err = fesetround(mode);
  if (unlikely(err != 0)) {
    return err;
  }

  float32_t const temp = a + b;

  err = fesetround(dflt);
  if (unlikely(err != 0)) {
    return err;
  }

  *dst = temp;

  return 0;
}

int c_sub_f32(int const mode, float32_t const a, float32_t const b,
              float32_t *const restrict dst) {
  int err = 0;

  int const dflt = fegetround();
  if (unlikely(dflt < 0)) {
    return dflt;
  }

  err = fesetround(mode);
  if (unlikely(err != 0)) {
    return err;
  }

  float32_t const temp = a - b;

  err = fesetround(dflt);
  if (unlikely(err != 0)) {
    return err;
  }

  *dst = temp;

  return 0;
}

int c_mul_f32(int const mode, float32_t const a, float32_t const b,
              float32_t *const restrict dst) {
  int err = 0;

  int const dflt = fegetround();
  if (unlikely(dflt < 0)) {
    return dflt;
  }

  err = fesetround(mode);
  if (unlikely(err != 0)) {
    return err;
  }

  float32_t const temp = a * b;

  err = fesetround(dflt);
  if (unlikely(err != 0)) {
    return err;
  }

  *dst = temp;

  return 0;
}

int c_div_f32(int const mode, float32_t const a, float32_t const b,
              float32_t *const restrict dst) {
  int err = 0;

  int const dflt = fegetround();
  if (unlikely(dflt < 0)) {
    return dflt;
  }

  err = fesetround(mode);
  if (unlikely(err != 0)) {
    return err;
  }

  float32_t const temp = a / b;

  err = fesetround(dflt);
  if (unlikely(err != 0)) {
    return err;
  }

  *dst = temp;

  return 0;
}

int c_sqrt_f32(int const mode, float32_t const a,
               float32_t *const restrict dst) {
  int err = 0;

  int const dflt = fegetround();
  if (unlikely(dflt < 0)) {
    return dflt;
  }

  err = fesetround(mode);
  if (unlikely(err != 0)) {
    return err;
  }

  float32_t const temp = sqrtf(a);

  err = fesetround(dflt);
  if (unlikely(err != 0)) {
    return err;
  }

  *dst = temp;

  return 0;
}
