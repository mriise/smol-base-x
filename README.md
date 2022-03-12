## A no_std centric implementation of base-x


TODO: Determine what the Array API should look like

This requires a nightly compiler to use the array API.

Features used under the 'unstable' flag:
* const_fn_floating_point_arithmetic
* generic_const_exprs

Implementing a new Alphabet is rather simple:
```rust
use smol_base_x::*;
pub struct Base58 {}

impl Base<58> for Base58 {
    const ALPHABET: [u8; 58] = *b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
}
```

Implementing `Base` will automatically generate a 256 byte LUT, which was chosen over a match statement based off of benchmarks in `benches/lut_vs_matches`

this lib provides a macro for generating a match statement equivalent to a LUT, but is mostly useful for when dealing with non-ascii alphabets (which are currently unsupported).
