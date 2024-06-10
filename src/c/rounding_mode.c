#include <fenv.h>
#include <stdbool.h>

#ifdef FE_TONEAREST
int const c_TO_NEAREST = FE_TONEAREST;
#else
int const c_TO_NEAREST = -1;
#endif

#ifdef FE_UPWARD
int const c_UPWARD = FE_UPWARD;
#else
int const c_UPWARD = -1;
#endif

#ifdef FE_DOWNWARD
int const c_DOWNWARD = FE_DOWNWARD;
#else
int const c_DOWNWARD = -1;
#endif

#ifdef FE_TOWARDZERO
int const c_TOWARD_ZERO = FE_TOWARDZERO;
#else
int const c_TOWARD_ZERO = -1;
#endif

bool c_supported(int const round) {
    return 0 <= round;
}
