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

            length = i;
            if length + ones > buf.len() {
                return Err(DecodeError::InvalidLength(length));
            }

            // generally we dont want to panic in release if we can avoid it, consider using debug_assert
            assert!(carry == 0);
        }
        length += ones;

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

        for &ch in iter {
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

            length = i;
            if length > buf.len() {
                return Err(DecodeError::InvalidLength(length));
            }

            assert!(carry == 0);
        }

        length += zeroes;

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

    /// output is `(decoded bytes, bytes written)`
    #[cfg(feature = "unstable")]
    fn decode_arr<const LEN: usize, I: Into<[u8; LEN]>>(
        input: I,
    ) -> Result<([u8; decoded_arr_size(BASE, LEN)], usize), DecodeError> {
        let input = input.into();
        let mut arr = [0u8; decoded_arr_size(BASE, LEN)];

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

            let mut rev = arr.iter_mut().rev();
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

            length = i;
            // size is already known to be correct
            // if length > arr.len() {
            //     return Err(DecodeError::InvalidLength(length));
            // }

            // generally we dont want to panic in release if we can avoid it, consider using debug_assert
            assert!(carry == 0);
        }

        length += ones;

        // Skip trailing spaces.
        while let Some(&&ch) = iter.peek() {
            if ch == b' ' {
                iter.next();
            } else {
                return Err(DecodeError::CharAfterTrailingSpaces);
            }
        }

        let mid = arr.len() - length;
        arr.rotate_left(mid);

        Ok((arr, length))
    }

    /// output is `(encoded chars, chars written)`
    #[cfg(feature = "unstable")]
    fn encode_arr<const BYTES: usize, I: Into<[u8; BYTES]>>(
        input: I,
    ) -> Result<([u8; encoded_arr_size(Self::BASE, BYTES)], usize), DecodeError> {
        let input = input.into();

        let mut arr = [0u8; encoded_arr_size(Self::BASE, BYTES)];

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

        // let size = encoded_size(BASE, iter.len());

        // array is properly sized at compile time
        // // buf is too small to fit string
        // if size > buf.len() {
        //     return Err(DecodeError::InvalidLength(size));
        // }

        let mut length = 0;

        for &ch in iter {
            let mut carry = ch as usize;

            let mut i = 0;
            let mut rev = arr.iter_mut().rev();
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

            length = i;
            // length known at compile time
            // if length > arr.len() {
            //     return Err(DecodeError::InvalidLength(length));
            // }

            assert!(carry == 0);
        }

        length += zeroes;

        let mid = arr.len() - length;
        arr.rotate_left(mid);

        // translate index into alphabet letter
        for i in arr.iter_mut().take(length) {
            *i = Self::ALPHABET[*i as usize];
        }

        Ok((arr, length))
    }
}
