### A no_std centric implementation of base-x

This requires a nightly compiler to work.

Features used:
* const_fn_floating_point_arithmetic
* generic_const_exprs (under the 'unstable' flag since it is still incomplete)

Implementing a new Alphabet is rather simple:
```rust
pub struct Base58 {}

impl Base<58> for Base58 {
    const ALPHABET: [char; 58] =
        const_str::to_char_array!("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz");

    fn lookup_char(ch: char) -> Option<usize> {
        smol_base_x::gen_char_match!(
            ch,
            "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"
        )
    }
}
```
