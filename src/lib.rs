#![cfg_attr(
    feature = "unstable",
    feature(generic_const_exprs, const_fn_floating_point_arithmetic)
)]
#![no_std]
#![doc = include_str!("../README.md")]

#[cfg(test)]
#[macro_use]
extern crate std;

mod base;

mod base_impl;
mod utf_base;

/// encode/decode_arr need to use log in order to estimate the size, which is why unstable floating point arithmetic is needed
/// since there is also no const fn for log, there is an implementation of log10 hidden
///
/// one day the log implemenation should be replaced with the proper function when it comes out   
pub mod util;

pub use proc_macro::{gen_ascii_match, gen_char_match};

pub use base::Base;
pub use base_impl::Base58btc;
#[cfg(feature = "unstable")]
pub use utf_base::UtfBase;

#[derive(Debug)]
pub enum DecodeError {
    /// invalid len where `usize` is the min size expected
    InvalidLength(usize),
    /// char not in alphabet
    InvalidChar,
    CharAfterTrailingSpaces,
}

#[cfg(test)]
mod tests {

    use crate::{base::*, Base58btc};

    // TODO: make tests for multi byte chars

    #[test]
    fn char_proc_macro() {
        fn proc(ch: char) -> Option<usize> {
            proc_macro::gen_char_match!(ch, "abc")
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
    fn ascii_proc_macro() {
        fn proc(ch: u8) -> Option<usize> {
            proc_macro::gen_ascii_match!(ch, b"abc")
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

    #[test]
    fn full_cycle() {
        let input = "44Y6qTgSvRMkdqpQ5ufkN";
        let mut output = [0u8; 128];

        let written = Base58btc::decode_mut(input, &mut output).unwrap();
        let expected = hex::decode("6d691bdd736346aa5a0a95b373b2ab").unwrap();

        println!("{:x?}\n{:x?}", &output[..written], &expected.as_slice());
        assert!(&output[..written] == expected.as_slice());

        let expected = input;

        let mut input = [0u8; 128];
        input.copy_from_slice(&output);
        output.fill(0);

        let written = Base58btc::encode_mut(&input[..written], &mut output).unwrap();

        println!("{:x?}\n{:x?}", &output[..written], expected.as_bytes());
        assert!(&output[..written] == expected.as_bytes());
    }

    #[test]
    fn abc_decode() {
        let mut output = [0u8; 32];

        let written = Base58btc::decode_mut("ZiCa", &mut output).unwrap();

        let expected = b"abc";

        println!("{:x?}\n{:x?}", &output[..written], &expected.as_slice());

        assert!(&output[..written] == expected.as_ref());
    }

    #[test]
    fn abc_encode() {
        let mut output = [0u8; 32]; // 3 spaces takes the same space as 'abc'

        let written = Base58btc::encode_mut("abc", &mut output).unwrap();

        let expected = "ZiCa";

        let output = core::str::from_utf8(&output[..written]).unwrap();

        println!("{:x?}\n{:x?}", output, expected);

        assert!(output == expected);
    }

    #[test]
    #[cfg(feature = "unstable")]
    fn arr_decode() {
        let (output, written) = Base58btc::decode_arr(*b"ZiCa").unwrap();

        let expected = b"abc";

        println!("{:x?}\n{:x?}", &output[..written], &expected.as_slice());

        assert!(&output[..written] == expected.as_ref());
    }

    #[test]
    #[cfg(feature = "unstable")]
    fn arr_encode() {
        let (output, written) = Base58btc::encode_arr(*b"abc").unwrap();

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
