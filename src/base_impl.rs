use crate::base::Base;

#[derive(Debug, Default)]
pub struct Base58btc {}

impl Base<58> for Base58btc {
    const ALPHABET: [char; 58] =
        const_str::to_char_array!("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz");

    fn lookup_char(ch: char) -> Option<usize> {
        crate::gen_char_match!(
            ch,
            "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"
        )
    }
}
