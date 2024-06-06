# fpa-specr

Floating point arithmetics (`+`, `-`, `*` and `/`) and `sqrt`
with specified rounding mode.

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
assert_eq!(0.1.round_add(0.2, &RoundMode::NearestTiesEven), 0.30000000000000004);

// Functions are available
use fpa_specr::f64::{ciel_add, floor_add};
assert_eq!(ciel_add(0.1, 0.2), 0.30000000000000004);
assert_eq!(floor_add(0.1, 0.2), 0.3);
```

## Licence

MIT or Apache-2.0