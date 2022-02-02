use super::DecodeError;

/// base x where each letter Alphabet only contains 1 unicode byte
/// 
/// Base is object safe but I wouldn't recommend using it as such
pub trait Base<const BASE: usize> {
    const ALPHABET: [char; BASE];

    // ARRAY_SIZE determines the backing array
    // (array, bytes written)
    fn decode<'a, const ARRAY_SIZE: usize>(
        &self,
        input: &str,
    ) -> Result<([u8; ARRAY_SIZE], usize), DecodeError>;

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

    fn decode<'a, const ARRAY_SIZE: usize>(
        &self,
        input: &str,
    ) -> Result<([u8; ARRAY_SIZE], usize), DecodeError> {
        todo!()
    }

    fn encode<'a, const S: usize>(&self, input: &[u8; S]) -> &'a str
    where
        [(); gen_encoded_size(Self::ALPHABET, S)]: {
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
pub struct Base58 {}

impl Base<58> for Base58 {
    const ALPHABET: [char; 58] =
        const_str::to_char_array!("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz");

    // ARRAY_SIZE determines the backing array
    // (array, bytes written)
    fn decode<'a, const ARRAY_SIZE: usize>(
        &self,
        input: &str,
    ) -> Result<([u8; ARRAY_SIZE], usize), DecodeError> {
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
