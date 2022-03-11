use super::DecodeError;
use crate::util::*;

/// ## Base-x for Ascii alphabets (which is most)
pub trait Base<const BASE: usize> {
    const ALPHABET: [u8; BASE];

    const LUT: [i8; 256] = gen_lut(&Self::ALPHABET);

    const BASE: usize = Self::ALPHABET.len();

    /// decode input base encoding into buffer
    /// it is assumed buf might be non-zeroed and will be filled with zero at the start
    ///
    /// ```rust
    /// use smol_base_x::*;
    ///
    /// let mut buf = [0u8; 16];
    /// let bytes_written = Base58btc::decode_mut("ZiCa", &mut buf).unwrap();
    ///
    /// let expected = b"abc";
    /// assert_eq!(&buf[..bytes_written], expected.as_slice());
    /// ```
    fn decode_mut<I: AsRef<[u8]>>(input: I, buf: &mut [u8]) -> Result<usize, DecodeError> {
        let input = input.as_ref();
        buf.fill(0);

        if !input.is_ascii() {
            return Err(DecodeError::InvalidChar);
        }
        // thanks to https://sts10.github.io/2020/10/06/peeking-the-pivot.html for the great notes on iterators with look ahead
        let mut iter = input.iter().peekable();

        // Skip leading spaces.
        while let Some(&&ch) = iter.peek() {
            if ch == b' ' {
                iter.next();
            } else {
                break;
            }
        }

        // skip & count leading '1's.
        let mut ones = 0;
        while let Some(&&ch) = iter.peek() {
            if ch == b'1' {
                ones += 1;
                iter.next();
            } else {
                break;
            }
        }

        // in C++ this is used to allocate a vec, but this will overestimate if there are trailing zeroes
        // let size = decoded_size(Self::BASE, iter.len());
        // if size + ones > buf.len() {
        //     return Err(DecodeError::InvalidLength(size + ones));
        // }

        let mut length = 0;

        // Process the characters.
        while let Some(&&ch) = iter.peek() {
            // move forward only if next char is not a space
            if ch == b' ' {
                break;
            }
            iter.next();

            // Decode base-x character
            let mut carry = match Self::lookup_ascii(ch) {
                Some(x) => x,
                None => return Err(DecodeError::InvalidChar),
            };

            let mut rev = buf.iter_mut().rev();
            let mut i = 0;

            while i < length || carry != 0 {
                match rev.next() {
                    Some(it) => {
                        carry += BASE * (*it as usize);
                        *it = (carry % 256) as u8;
                        carry /= 256;
                        i += 1;
                    }
                    None => break,
                }
            }

            length = i + ones;
            if length > buf.len() {
                return Err(DecodeError::InvalidLength(length));
            }

            // generally we dont want to panic in release if we can avoid it, consider using debug_assert
            assert!(carry == 0);
        }
        // Skip trailing spaces.
        while let Some(&&ch) = iter.peek() {
            if ch == b' ' {
                iter.next();
            } else {
                return Err(DecodeError::CharAfterTrailingSpaces);
            }
        }

        buf.rotate_left(buf.len() - length);
        Ok(length)
    }

    /// output buff is intentionally a slice since `&mut str` is essentially useless
    /// users will have to convert output bytes into a str
    /// it is assumed buf might be non-zeroed and will be filled with zero at the start
    ///
    /// ```rust
    /// use smol_base_x::*;
    ///
    /// let mut buf = [0u8; 16];
    /// let bytes_written = Base58btc::encode_mut("abc", &mut buf).unwrap();
    ///
    /// // Here
    /// let output = core::str::from_utf8(&buf[..bytes_written]).unwrap();
    ///
    /// let expected = "ZiCa";
    /// assert_eq!(output, expected);
    /// ```
    fn encode_mut<I: AsRef<[u8]>>(input: I, buf: &mut [u8]) -> Result<usize, DecodeError> {
        let input = input.as_ref();
        buf.fill(0);
        // thanks to https://sts10.github.io/2020/10/06/peeking-the-pivot.html for the great notes on iterators with look ahead
        let mut iter = input.iter().peekable();

        let mut zeroes = 0;
        // skip & count leading zeros
        while let Some(&&ch) = iter.peek() {
            if ch == 0 {
                zeroes += 1;
                iter.next();
            } else {
                break;
            }
        }

        let size = encoded_size(BASE, iter.len());

        // buf is too small to fit string
        if size > buf.len() {
            return Err(DecodeError::InvalidLength(size));
        }

        let mut length = 0;

        while let Some(&ch) = iter.next() {
            let mut carry = ch as usize;

            let mut i = 0;
            let mut rev = buf.iter_mut().rev();
            while i < length || carry != 0 {
                match rev.next() {
                    Some(it) => {
                        carry += 256 * (*it as usize);
                        *it = (carry % BASE) as u8;
                        carry /= BASE;
                        i += 1;
                    }
                    None => break,
                }
            }

            length = i + zeroes;
            if length > buf.len() {
                return Err(DecodeError::InvalidLength(length));
            }

            assert!(carry == 0);
        }

        buf.rotate_left(buf.len() - length);

        // translate index into alphabet letter
        for i in buf.iter_mut().take(length) {
            *i = Self::ALPHABET[*i as usize];
        }

        Ok(length)
    }

    /// Lookup the value for the current char index
    fn lookup_ascii(ch: u8) -> Option<usize> {
        match Self::LUT[ch as usize] {
            -1 => None,
            i => Some(i as usize),
        }
    }

    /// This makes more assumptions about the quality of the input char array and will not do some things that would otherwise be done with the normal base-x
    /// * no spaces before or after
    /// * no leading ones to skip ?
    ///
    /// this is done in order to ensure decoded size is not over-bloated at compile time
    ///
    /// output is `(decoded bytes, bytes written)`
    #[cfg(feature = "unstable")]
    fn decode_arr<const CHARS: usize, I: AsRef<[u8; CHARS]>>(
        input: I,
    ) -> Result<([u8; decoded_arr_size(BASE, CHARS)], usize), DecodeError> {
        let input = input.as_ref();

        let mut arr = [0u8; decoded_arr_size(BASE, CHARS)];

        todo!();
        // Ok(arr)
    }

    /// output is `(encoded chars, chars written)`
    #[cfg(feature = "unstable")]
    fn encode_arr<const BYTES: usize, I: AsRef<[u8; BYTES]>>(
        input: I,
    ) -> Result<[char; encoded_arr_size(Self::BASE, BYTES)], DecodeError> {
        let input = input.as_ref();

        let mut arr = [0u8; encoded_arr_size(Self::BASE, BYTES)];

        todo!()
    }
}
