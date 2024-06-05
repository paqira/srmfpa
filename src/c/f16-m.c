#include "fenv_access.h"
#include "unlikely.h"
#define __STDC_WANT_IEC_60559_TYPES_EXT__
#include <fenv.h>
#include <math.h>

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
