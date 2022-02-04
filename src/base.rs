use std::{io::Read, marker::PhantomData};

use super::DecodeError;
use crate::{gen_char_match, util::*};

/// base x where each letter Alphabet only contains 1 unicode byte
///
/// Base is object safe but I wouldn't recommend using it as such
pub trait Base<const BASE: usize> {
    const ALPHABET: [char; BASE];

    // ARRAY_SIZE determines the backing array
    // bytes written
    fn decode_mut<'a, const ARRAY_SIZE: usize>(
        &self,
        input: &str,
        buf: &'a mut [u8],
    ) -> Result<usize, DecodeError> {
        // Skip leading spaces.
        let mut iter = input.chars().peekable();
        while iter.peek() == Some(&' ') {
            iter.next();
        }
        // Skip and count leading '1's.
        let mut zeroes: usize = 0;

        while iter.next() == Some('1') {
            zeroes += 1;
            if zeroes as usize > ARRAY_SIZE {
                return Err(DecodeError::InvalidLength(zeroes)); // right???
            }
        }

        // Allocate enough space in big-endian base256 representation.

        // TODO: remove the iterator count if possible this doesnt account for things!!AHH
        // let mut arr: &[u8] = &backing[0..gen_decoded_size(Self::ALPHABET, input.chars().count())];

        let mut length = 0;

        // Process the characters.
        while iter.peek() != Some(&' ') {
            let ch = match iter.next() {
                Some(ch) => ch,
                None => break,
            };

            // Decode base58 character
            let mut carry = match Self::lookup_char(ch) {
                Some(x) => x,
                None => return Err(DecodeError::InvalidChar),
            };

            let mut rev = buf.iter_mut().rev();

            for it in rev.next() {
                carry += Self::ALPHABET.len() * (*it as usize);
                *it = (carry % 256) as u8;
                carry /= 256;

                if carry == 0 {
                    break;
                }

                length += 1;
            }

            if length + zeroes > ARRAY_SIZE {
                return Err(DecodeError::InvalidLength(length + zeroes));
            }

            assert!(carry == 0);
        }
        // Skip trailing spaces.
        while iter.next() == Some(' ') {}

        if iter.next().is_some() {
            return Err(DecodeError::CharAfterTrailingSpaces);
        }

        // >>>><<<><>
        let size = gen_decoded_size(Self::ALPHABET, input.chars().count());

        // Skip leading zeroes in b256.

        // // Copy result into output vector.
        // vch.reserve(zeroes + (b256.end() - it));
        // vch.assign(zeroes, 0x00);
        // while (it != b256.end())
        //     vch.push_back(*(it++));
        Ok(size)
    }

    //
    fn encode_mut(&self, input: &[u8], buf: &mut str) -> Result<usize, DecodeError> {
        let a = buf;
        todo!()
    }

    /// C++ algorithim uses a \[i8; 256] LUT
    ///
    /// since we want to use any UTF-8, we instead generate a match statement for each char
    /// the result should be smaller and might actually be faster than using a LUT while allow use of UTF-8 as well
    ///
    /// ### NOTE: associated const and alphabet used in the macro should be the exact same
    ///
    /// e.g.
    /// ```rust
    /// const ALPHABET: [char; 58] =
    ///     const_str::to_char_array!("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz");
    ///
    /// fn lookup_char(ch: char) -> Option<usize> {
    ///     gen_char_match!(
    ///         ch,
    ///         "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz" // base58
    ///     )
    /// }
    /// ```
    fn lookup_char(ch: char) -> Option<usize>;

    #[cfg(feature = "unstable")]
    /// input array must include only chars defined in ALPHABET with no spaces before or after in order to keep decoded size deterministic at compile time
    fn decode_arr<const CHARS: usize>(
        &self,
        input: &[char; CHARS],
    ) -> Result<[u8; gen_decoded_size(Self::ALPHABET, CHARS)], DecodeError> {
        let mut arr = [0u8; gen_decoded_size(Self::ALPHABET, CHARS)];


        Ok(arr)
    }

    #[cfg(feature = "unstable")]
    ///
    fn encode_arr<const BYTES: usize>(
        &self,
        input: &[u8; BYTES],
    ) -> Result<[char; gen_encoded_size(Self::ALPHABET, BYTES)], DecodeError> {
        let mut arr = [0u8; gen_encoded_size(Self::ALPHABET, BYTES)];

        todo!()        
    }
}

pub trait AsciiBase<const BASE: usize> {
    const ASCII_ALPHABET: [u8; BASE];

    fn lookup_ascii(ch: char) -> Option<usize>;
}

impl<const BASE: usize, T: AsciiBase<BASE>> Base<BASE> for T {
    const ALPHABET: [char; BASE] = ascii_to_char_arr(Self::ASCII_ALPHABET);

    fn lookup_char(ch: char) -> Option<usize> {
        Self::lookup_ascii(ch)
    }
}

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

pub const fn gen_encoded_size<const S: usize>(_base: [char; S], input_byte_size: usize) -> usize {
    (input_byte_size as f64 * (log10(S) / log10(256)) + 1.0) as usize
}

pub const fn gen_decoded_size<const S: usize>(_base: [char; S], input_byte_size: usize) -> usize {
    (input_byte_size as f64 * (log10(256) / log10(S))) as usize
}

const fn ascii_to_char_arr<const S: usize>(ascii: [u8; S]) -> [char; S] {
    let mut arr = [' '; S];
    let mut ch = 0;
    while ch < S {
        arr[ch] = ascii[ch] as char;
        ch += 1;
    }
    arr
}

// https://stackoverflow.com/questions/35968963/trying-to-calculate-logarithm-base-10-without-math-h-really-close-just-having
const fn ln(x: usize) -> f64 {
    let mut old_sum = 0.0;
    let xmlxpl = (x as f64 - 1.0) / (x as f64 + 1.0);
    let xmlxpl_2 = xmlxpl * xmlxpl;
    let mut denom = 1.0;
    let mut frac = xmlxpl;
    let term = frac;
    let mut sum = term;

    while sum != old_sum {
        old_sum = sum;
        denom += 2.0;
        frac *= xmlxpl_2;
        sum += frac / denom;
    }
    return 2.0 * sum;
}
const fn log10(x: usize) -> f64 {
    return ln(x) / core::f64::consts::LN_10;
}
