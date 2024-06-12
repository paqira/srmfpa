#include "fenv_access.h"
#include "unlikely.h"
#define __STDC_WANT_IEC_60559_TYPES_EXT__
#include <fenv.h>
#include <math.h>

int c_add_f16(int const mode, _Float16 const a, _Float16 const b,
              _Float16 *const restrict dst) {
  int err = 0;

  int const dflt = fegetround();
  if (unlikely(dflt < 0)) {
    return dflt;
  }

  err = fesetround(mode);
  if (unlikely(err != 0)) {
    return err;
  }

  _Float16 const temp = a + b;

  err = fesetround(dflt);
  if (unlikely(err != 0)) {
    return err;
  }

  *dst = temp;

  return 0;
}

int c_sub_f16(int const mode, _Float16 const a, _Float16 const b,
              _Float16 *const restrict dst) {
  int err = 0;

  int const dflt = fegetround();
  if (unlikely(dflt < 0)) {
    return dflt;
  }

  err = fesetround(mode);
  if (unlikely(err != 0)) {
    return err;
  }

  _Float16 const temp = a - b;

  err = fesetround(dflt);
  if (unlikely(err != 0)) {
    return err;
  }

  *dst = temp;

  return 0;
}

int c_mul_f16(int const mode, _Float16 const a, _Float16 const b,
              _Float16 *const restrict dst) {
  int err = 0;

  int const dflt = fegetround();
  if (unlikely(dflt < 0)) {
    return dflt;
  }

  err = fesetround(mode);
  if (unlikely(err != 0)) {
    return err;
  }

  _Float16 const temp = a * b;

  err = fesetround(dflt);
  if (unlikely(err != 0)) {
    return err;
  }

  *dst = temp;

  return 0;
}

int c_div_f16(int const mode, _Float16 const a, _Float16 const b,
              _Float16 *const restrict dst) {
  int err = 0;

  int const dflt = fegetround();
  if (unlikely(dflt < 0)) {
    return dflt;
  }

  err = fesetround(mode);
  if (unlikely(err != 0)) {
    return err;
  }

  _Float16 const temp = a / b;

  err = fesetround(dflt);
  if (unlikely(err != 0)) {
    return err;
  }

  *dst = temp;

  return 0;
}

int c_fma_f16(int const mode, _Float16 const a, _Float16 const b,
              _Float16 const c, _Float16 *const restrict dst) {
  int err = 0;

  int const dflt = fegetround();
  if (unlikely(dflt < 0)) {
    return dflt;
  }

  err = fesetround(mode);
  if (unlikely(err != 0)) {
    return err;
  }

  _Float16 const temp = fmaf16(a, b, c);

  err = fesetround(dflt);
  if (unlikely(err != 0)) {
    return err;
  }

  *dst = temp;

  return 0;
}

int c_sqrt_f16(int const mode, _Float16 const a, _Float16 *const restrict dst) {
  int err = 0;

  int const dflt = fegetround();
  if (unlikely(dflt < 0)) {
    return dflt;
  }

  err = fesetround(mode);
  if (unlikely(err != 0)) {
    return err;
  }

  _Float16 const temp = sqrtf16(a);

  err = fesetround(dflt);
  if (unlikely(err != 0)) {
    return err;
  }

  *dst = temp;

  return 0;
}
