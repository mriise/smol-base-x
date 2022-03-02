## A no_std centric implementation of base-x


TODO: Determine what the Array API should look like

This requires a nightly compiler to use the array API.

Features used:
* const_fn_floating_point_arithmetic
* generic_const_exprs (under the 'unstable' flag since it is still incomplete)

Implementing a new Alphabet is rather simple:
```rust
use smol_base_x::*;
pub struct Base58 {}

impl Base<58> for Base58 {
    const ALPHABET: [u8; 58] =
        const_str::to_byte_array!("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz");

    fn lookup_ascii(ch: u8) -> Option<usize> {
        smol_base_x::gen_ascii_match!(
            ch,
            b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"
        )
    }
}
```
