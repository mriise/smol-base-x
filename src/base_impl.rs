use crate::Base;

#[derive(Debug, Default)]
pub struct Base58btc {}

impl Base<58> for Base58btc {
    const ALPHABET: [u8; 58] =
        const_str::to_byte_array!("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz");
}
