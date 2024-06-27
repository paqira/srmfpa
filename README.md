# srmfpa

Floating-point's four arithmetic operations (including fused multiply-add) and square root
with strict rounding mode.

```rust
use srmfpa::prelude::*;

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
use srmfpa::f64::{ciel_add, floor_add};
assert_eq!(ciel_add(0.1, 0.2), 0.30000000000000004);
assert_eq!(floor_add(0.1, 0.2), 0.3);
```

## Features

- `f32-softfloat`: support softfloat `f32` by [Berkeley SoftFloat 3][softfloat].
- `f64-softfloat`: support softfloat `f64` by [Berkeley SoftFloat 3][softfloat].
- `f16-softfloat`: support softfloat `f16` by [Berkeley SoftFloat 3][softfloat].
- `f128-softfloat`: support softfloat `f128` by [Berkeley SoftFloat 3][softfloat].
- `softfloat`: use softfloat for `f32` and `f64` (enable `f32-softfloat` and `f64-softfloat`).

[softfloat]: https://github.com/ucb-bar/berkeley-softfloat-3

## Licence

MIT or Apache-2.0
