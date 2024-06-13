# fpa-specr

Four floating-point arithmetic operations (including fused multiply-add) and square root,
in specified rounding mode.

```rust
use fpa_specr::prelude::*;

// Support add, sub, mul, div, mul_add (fma) and sqrt

assert_eq!(0.1.round_ties_even_add(0.2), 0.30000000000000004);
assert_eq!(0.1.ciel_add(0.2), 0.30000000000000004);
assert_eq!(0.1.floor_add(0.2), 0.3);
assert_eq!(0.1.trunc_add(0.2), 0.3);

assert_eq!((-0.1).round_ties_even_add(-0.2), -0.30000000000000004);
assert_eq!((-0.1).ciel_add(-0.2), -0.3);
assert_eq!((-0.1).floor_add(-0.2), -0.30000000000000004);
assert_eq!((-0.1).trunc_add(-0.2), -0.3);

// Generic ops
assert_eq!(0.1.round_add(0.2, &RoundingMode::NearestTiesEven), 0.30000000000000004);

// Functions are available
use fpa_specr::f64::{ciel_add, floor_add};
assert_eq!(ciel_add(0.1, 0.2), 0.30000000000000004);
assert_eq!(floor_add(0.1, 0.2), 0.3);
```

## Features

- `f32-softfloat` and `f64-softfloat`: support softfloat `f32` and `f64` by [Berkeley SoftFloat 3][softfloat].
- `f16-softfloat` and `f128-softfloat`: support softfloat `f16` and `f128` by [Berkeley SoftFloat 3][softfloat].
- `softfloat`: use softfloat for `f32` and `f64` (enable `f32-softfloat` and `f64-softfloat`).

[softfloat]: https://github.com/ucb-bar/berkeley-softfloat-3

## Licence

MIT or Apache-2.0
