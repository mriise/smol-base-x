use crate::Base;

#[derive(Debug, Default)]
pub struct Base58Btc;

impl Base<58> for Base58Btc {
    const ALPHABET: [u8; 58] = *b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
}

macro_rules! base {
    ($name:ident, $len:literal $alph:literal) => {
        #[derive(Debug, Default)]
        pub struct $name;

        impl Base<$len> for $name {
            const ALPHABET: [u8; $len] = *$alph;
        }
    };
}

// bases from https://github.com/cryptocoinjs/base-x
base!(Base2, 2 b"01");
base!(Base8, 8 b"01234567");
base!(Base11, 11 b"0123456789a");
base!(Base36, 36 b"0123456789abcdefghijklmnopqrstuvwxyz");
base!(Base62, 62 b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");
base!(Base67, 67 b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_.!~");

// etc
base!(Base10, 10 b"0123456789");
base!(Base58Flickr, 58 b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz");
