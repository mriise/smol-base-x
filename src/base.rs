use super::DecodeError;
use crate::{gen_char_match, util::*};

/// base x where each letter Alphabet only contains 1 unicode byte
///
/// Base is object safe but I wouldn't recommend using it as such
pub trait Base<const BASE: usize> {
    const ALPHABET: [char; BASE];

    // ARRAY_SIZE determines the backing array
    // (array, bytes written)
    fn decode<'a, const S: usize>(
        &self,
        input: &str,
    ) -> Result<([u8; gen_decoded_size(Self::ALPHABET, S)], usize), DecodeError>
    where
        [(); gen_decoded_size(Self::ALPHABET, S)]:;

    fn encode<'a, const S: usize>(&self, input: &[u8; S]) -> &'a str
    where
        [(); gen_encoded_size(Self::ALPHABET, S)]:;

    fn decode_arr<'a, const S: usize>(
        &self,
        input: impl Into<&'a [u8; S]>,
    ) -> Result<[u8; gen_decoded_size(Self::ALPHABET, S)], DecodeError>;

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
    ///
    fn lookup_char(ch: char) -> Option<usize>;
}

pub trait AsciiBase<const BASE: usize> {
    const ASCII_ALPHABET: [u8; BASE];

    // const ASCII_TABLE: [u8; 256] = gen_ascii_table();
}

impl<T: AsciiBase<BASE>, const BASE: usize> Base<BASE> for T {
    const ALPHABET: [char; BASE] = ascii_to_char_arr(Self::ASCII_ALPHABET);

    fn decode<'a, const S: usize>(
        &self,
        input: &str,
    ) -> Result<([u8; gen_decoded_size(Self::ALPHABET, S)], usize), DecodeError>
    where
        [(); gen_decoded_size(Self::ALPHABET, S)]:,
    {
        todo!()
    }

    fn encode<'a, const S: usize>(&self, input: &[u8; S]) -> &'a str
    where
        [(); gen_encoded_size(Self::ALPHABET, S)]:,
    {
        todo!()
    }

    fn decode_arr<'a, const S: usize>(
        &self,
        input: impl Into<&'a [u8; S]>,
    ) -> Result<[u8; gen_decoded_size(Self::ALPHABET, S)], DecodeError> {
        todo!()
    }

    fn lookup_char(ch: char) -> Option<usize> {
        todo!()
    }
}

#[derive(Debug)]
pub struct Base58btc {}

impl Base<58> for Base58btc {
    const ALPHABET: [char; 58] =
        const_str::to_char_array!("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz");

    // ARRAY_SIZE determines the backing array
    // (array, bytes written)
    fn decode<'a, const S: usize>(
        &self,
        input: &str,
    ) -> Result<([u8; gen_decoded_size(Self::ALPHABET, S)], usize), DecodeError>
    where
        [(); gen_decoded_size(Self::ALPHABET, S)]:,
    {
        todo!()
    }

    fn encode<'a, const S: usize>(&self, input: &[u8; S]) -> &'a str
    where
        [(); gen_encoded_size(Self::ALPHABET, S)]:,
    {
        let arr = &[0u8; gen_encoded_size(Self::ALPHABET, S)];
        // SAFETY: uhh this isnt safe xd
        let s: &str = unsafe { core::str::from_utf8_unchecked(arr) };
        todo!()
    }

    fn decode_arr<'a, const S: usize>(
        &self,
        input: impl Into<&'a [u8; S]>,
    ) -> Result<[u8; gen_decoded_size(Self::ALPHABET, S)], DecodeError> {
        todo!()
    }

    fn lookup_char(ch: char) -> Option<usize> {
        crate::gen_char_match!(
            ch,
            "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"
        )
    }
}

