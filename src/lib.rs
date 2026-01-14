#![cfg_attr(
    feature = "unstable",
    feature(generic_const_exprs)
)]
#![no_std]
#![doc = include_str!("../README.md")]

#[cfg(test)]
#[macro_use]
extern crate std;

mod base;

mod base_impl;
#[cfg(feature = "unstable")]
mod utf_base;

/// `encode`/`decode_arr` need to use `log10` in order to estimate the size of the output.
/// Since there is no `log10` in core or in const scopes, there is are custom implementations of log10.
/// One day (soon™) the log implemenation should be replaced with the proper function from rustlang
pub mod util;

#[cfg(feature = "unstable")]
/// proc macros for generating match statments for alphabets
pub use match_lookup::{gen_ascii_match, gen_char_match};

pub use base::Base;
pub use base_impl::*;
#[cfg(feature = "unstable")]
/// UTF base shoudn't really be used (especially if you are reaching for this crate) and the implementation is probably broken
pub use utf_base::UtfBase;

#[derive(Debug)]
/// when decode/encode returns this, it is safe to assume passed in buffer has partially written data.
/// Both functions assume input buffers are zeroed, so do that before re-using the same buffer.
pub enum DecodeError {
    /// invalid len where `usize` is the min size expected
    InvalidLength(usize),
    /// char not in alphabet
    InvalidChar,
    CharAfterTrailingSpaces,
}

#[cfg(test)]
mod tests {

    use crate::{base::*, Base58Btc};

    #[cfg(feature = "unstable")]
    mod proc_macro {
        #[test]
        fn utf_char() {
            fn proc(ch: char) -> Option<usize> {
                match_lookup::gen_char_match!(ch, "abc")
            }

            fn normal(ch: char) -> Option<usize> {
                match ch {
                    'a' => Some(0),
                    'b' => Some(1),
                    'c' => Some(2),
                    _ => None,
                }
            }

            for ch in ['a', 'b', 'c'] {
                assert_eq!(proc(ch).unwrap(), normal(ch).unwrap());
            }
        }

        #[test]
        fn ascii() {
            fn proc(ch: u8) -> Option<usize> {
                match_lookup::gen_ascii_match!(ch, b"abc")
            }

            fn normal(ch: u8) -> Option<usize> {
                match ch {
                    97 => Some(0), // a
                    98 => Some(1), // b
                    99 => Some(2), // c
                    _ => None,
                }
            }

            for ch in ['a', 'b', 'c'] {
                assert_eq!(proc(ch as u8).unwrap(), normal(ch as u8).unwrap());
            }
        }
    }

    #[test]
    fn full_cycle() {
        let input = "44Y6qTgSvRMkdqpQ5ufkN";
        let mut output = [0u8; 128];

        let written = Base58Btc::decode_mut(input, &mut output).unwrap();
        let expected = hex::decode("6d691bdd736346aa5a0a95b373b2ab").unwrap();

        println!("{:x?}\n{:x?}", &output[..written], &expected.as_slice());
        assert!(&output[..written] == expected.as_slice());

        let expected = input;

        let mut input = [0u8; 128];
        input.copy_from_slice(&output);
        output.fill(0);

        let written = Base58Btc::encode_mut(&input[..written], &mut output).unwrap();

        println!("{:x?}\n{:x?}", &output[..written], expected.as_bytes());
        assert!(&output[..written] == expected.as_bytes());
    }

    #[test]
    fn abc_decode() {
        let mut output = [0u8; 32];

        let written = Base58Btc::decode_mut("ZiCa", &mut output).unwrap();

        let expected = b"abc";

        println!("{:x?}\n{:x?}", &output[..written], &expected.as_slice());

        assert!(&output[..written] == expected.as_ref());
    }

    #[test]
    fn abc_encode() {
        let mut output = [0u8; 32]; // 3 spaces takes the same space as 'abc'

        let written = Base58Btc::encode_mut("abc", &mut output).unwrap();

        let expected = "ZiCa";

        let output = core::str::from_utf8(&output[..written]).unwrap();

        println!("{:x?}\n{:x?}", output, expected);

        assert!(output == expected);
    }

    #[test]
    fn leading_zeros_preserved() {
        use crate::Base10;

        // Test that leading zeros roundtrip correctly for bases where '0' is first char
        let input = &[0x00, 0x00, 0x01]; // two leading zero bytes
        let mut encoded = [0u8; 32];

        let written = Base10::encode_mut(input, &mut encoded).unwrap();
        let encoded_str = core::str::from_utf8(&encoded[..written]).unwrap();

        // Should have two leading '0's (one per zero byte) followed by "1"
        assert_eq!(encoded_str, "001", "leading zeros should be preserved in encoding");

        // Decode back
        let mut decoded = [0u8; 32];
        let written = Base10::decode_mut(encoded_str, &mut decoded).unwrap();

        assert_eq!(&decoded[..written], input, "leading zeros should roundtrip");
    }

    #[test]
    fn decode_invalid_char() {
        use crate::DecodeError;

        let mut buf = [0u8; 32];
        // '0' and 'O' and 'I' and 'l' are not in Base58 alphabet
        let result = Base58Btc::decode_mut("abc0def", &mut buf);
        assert!(matches!(result, Err(DecodeError::InvalidChar)));
    }

    #[test]
    fn decode_non_ascii() {
        use crate::DecodeError;

        let mut buf = [0u8; 32];
        // Non-ASCII character
        let result = Base58Btc::decode_mut("abc\u{00e9}def", &mut buf);
        assert!(matches!(result, Err(DecodeError::InvalidChar)));
    }

    #[test]
    fn decode_buffer_too_small_leading_zeros() {
        use crate::DecodeError;

        // Leading '1's in Base58 represent zero bytes - use many to exceed buffer
        let mut buf = [0u8; 2];
        let result = Base58Btc::decode_mut("1111111111", &mut buf); // 10 leading zeros won't fit
        assert!(matches!(result, Err(DecodeError::InvalidLength(_))));
    }

    #[test]
    fn decode_buffer_too_small_overflow() {
        use crate::DecodeError;

        // Buffer too small for decoded data (not leading zeros)
        let mut buf = [0u8; 1];
        let result = Base58Btc::decode_mut("ZiCa", &mut buf); // "abc" needs 3 bytes
        assert!(matches!(result, Err(DecodeError::InvalidLength(_))));
    }

    #[test]
    fn encode_buffer_too_small() {
        use crate::DecodeError;

        let mut buf = [0u8; 1]; // way too small
        let result = Base58Btc::encode_mut(b"abc", &mut buf);
        assert!(matches!(result, Err(DecodeError::InvalidLength(_))));
    }

    #[test]
    fn decode_with_spaces() {
        // Leading and trailing spaces should be handled
        let mut buf = [0u8; 32];
        let written = Base58Btc::decode_mut("  ZiCa  ", &mut buf).unwrap();
        assert_eq!(&buf[..written], b"abc");
    }

    #[test]
    fn decode_char_after_trailing_spaces() {
        use crate::DecodeError;

        let mut buf = [0u8; 32];
        let result = Base58Btc::decode_mut("ZiCa  x", &mut buf);
        assert!(matches!(result, Err(DecodeError::CharAfterTrailingSpaces)));
    }

    #[test]
    fn encode_all_zeros() {
        use crate::Base10;

        // All zero bytes should produce all first-alphabet chars
        let mut buf = [0u8; 32];
        let written = Base10::encode_mut(&[0, 0, 0], &mut buf).unwrap();
        let encoded = core::str::from_utf8(&buf[..written]).unwrap();
        assert_eq!(encoded, "000");
    }

    #[test]
    fn util_size_calculations() {
        use crate::util::{decoded_size, encoded_size};

        // Test size calculations for various bases
        assert!(decoded_size(58, 10) >= 1);
        assert!(encoded_size(58, 10) >= 1);

        // Encoded should be larger than decoded for base < 256
        assert!(encoded_size(58, 10) > decoded_size(58, 10));
    }

    
    #[test]
    fn lookup_ascii_valid_and_invalid() {
        // Valid character
        assert_eq!(Base58Btc::lookup_ascii(b'1'), Some(0));
        assert_eq!(Base58Btc::lookup_ascii(b'z'), Some(57));
        
        // Invalid characters not in alphabet
        assert_eq!(Base58Btc::lookup_ascii(b'0'), None);
        assert_eq!(Base58Btc::lookup_ascii(b'O'), None);
        assert_eq!(Base58Btc::lookup_ascii(b'I'), None);
        assert_eq!(Base58Btc::lookup_ascii(b'l'), None);
    }
    
    #[test]
    #[cfg(feature = "unstable")]
    fn util_const_arr_size() {
        use crate::util::const_arr_size::{decoded_arr_size, encoded_arr_size};

        // Test const size calculations match runtime ones
        assert_eq!(decoded_arr_size(58, 10), crate::util::decoded_size(58, 10));
        assert_eq!(encoded_arr_size(58, 10), crate::util::encoded_size(58, 10));

        // Test a few more bases
        for base in [2, 10, 36, 58, 62] {
            for size in [1, 5, 10, 100] {
                assert_eq!(
                    decoded_arr_size(base, size),
                    crate::util::decoded_size(base, size)
                );
                assert_eq!(
                    encoded_arr_size(base, size),
                    crate::util::encoded_size(base, size)
                );
            }
        }
    }
    
    #[test]
    #[cfg(feature = "unstable")]
    fn arr_decode() {
        let (output, written) = Base58Btc::decode_arr(*b"ZiCa").unwrap();

        let expected = b"abc";

        println!("{:x?}\n{:x?}", &output[..written], &expected.as_slice());

        assert!(&output[..written] == expected.as_ref());
    }

    #[test]
    #[cfg(feature = "unstable")]
    fn arr_encode() {
        let (output, written) = Base58Btc::encode_arr(*b"abc").unwrap();

        let expected = "ZiCa";

        let output = core::str::from_utf8(&output[..written]).unwrap();
        println!("{:x?}\n{:x?}", output, expected);

        assert!(output == expected);
    }

    #[test]
    #[cfg(feature = "unstable")]
    fn size_util() {
        use crate::util::{self, const_arr_size::*};

        for base in 2..98 {
            // println!("- base {base}");
            for size in 1..256 {
                // println!("-- size {size}");
                // println!("de {} : {}", util::decoded_size(base, size), decoded_arr_size(base, size));
                assert_eq!(util::decoded_size(base, size), decoded_arr_size(base, size));

                // println!("en {} : {}", util::encoded_size(base, size), encoded_arr_size(base, size));
                assert_eq!(util::encoded_size(base, size), encoded_arr_size(base, size));
            }
        }
    }
}
