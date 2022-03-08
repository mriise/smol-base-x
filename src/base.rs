use super::DecodeError;
use crate::util::*;

/// ## Base-x for Ascii alphabets (which is most)

pub trait Base<const BASE: usize> {
    const ALPHABET: [u8; BASE];

    const BASE: usize = Self::ALPHABET.len();

    /// decode input base encoding into buffer
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
    fn decode_mut<I: AsRef<[u8]>>(
        input: I,
        buf: &mut [u8],
    ) -> Result<usize, DecodeError> {
        let input = input.as_ref();
        if !input.is_ascii() {
            return  Err(DecodeError::InvalidChar);
        }
        
        let mut iter = input.iter().peekable();
        // Skip leading spaces.
        while iter.peek() == Some(&&(b' ')) {
            iter.next();
        }

        // Skip and count leading '1's.
        let mut zeroes = 0;

        while iter.peek() == Some(&&(b'1')) {
            iter.next();
            zeroes += 1;
            if zeroes > buf.len() {
                return Err(DecodeError::InvalidLength(zeroes)); // right???
            }
        }

        let size = decoded_size(Self::BASE, iter.len());

        // let mut arr: &[u8] = &backing[0..gen_decoded_size(Self::ALPHABET, input.chars().count())];

        let mut length = 0;

        // Process the characters.
        while iter.peek() != Some(&&(b' ')) {
            let ch = match iter.next() {
                Some(ch) => ch,
                None => break, // end of input
            };

            // Decode base-x character
            let mut carry = match Self::lookup_ascii(*ch) {
                Some(x) => x,
                None => return Err(DecodeError::InvalidChar),
            };

            let mut rev = buf.iter_mut().rev().peekable(); // might not need to peek since we already know it is big enough
            let mut i = 0;
            while rev.peek().is_some() && ( i < length || carry != 0) {
                // shouldnt ever panic because of above
                let it = rev.next().unwrap();
                carry += Self::BASE * (*it as usize);
                *it = (carry % 256) as u8;
                carry /= 256;
                i += 1;
            }

            length = i + zeroes;
            if length > buf.len() {
                return Err(DecodeError::InvalidLength(length));
            }

            assert!(carry == 0);
        }
        // Skip trailing spaces.
        while iter.next() == Some(&(b' ')) {} // this might ignore a single char after the last space

        if iter.next().is_some() {
            return Err(DecodeError::CharAfterTrailingSpaces);
        }

        buf.rotate_left(buf.len() - length);
        Ok(length)
    }

    /// output buff is intentionally a slice since `&mut str` is essentially useless
    /// users will have to convert output bytes into a str
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

        let mut zeroes = 0;
        let mut length = 0;

        let mut iter = input.iter().peekable();
        
        // skip & count leading zeros
        while iter.peek() == Some(&&0) {
            iter.next();
            zeroes += 1;
        }
        
        let size = encoded_size(Self::BASE, iter.len());        

        // buf is too small to fit string
        if buf.len() < size {
            return Err(DecodeError::InvalidLength(buf.len()))
        }

        while iter.peek().is_some() {
            
            let mut carry = *iter.next().unwrap() as usize;

            let mut i = 0;
            let mut rev = buf.iter_mut().rev().peekable(); // might not need to peek since we already know it is big enough
            while rev.peek().is_some() && ( i < length || carry != 0) {
                // shouldnt ever panic because of above
                let it = rev.next().unwrap();
            
                carry += 256 * (*it as usize);
                *it = (carry % Self::BASE) as u8;

                carry /= Self::BASE;
                i += 1;
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
            *i = Self::ALPHABET[*i as usize]
        }

        Ok(length)
    }

    /// While the C++ algorithim uses a \[i8; 256] LUT, using a match statement should be smaller 
    /// and might actually be faster (TODO check this) than using a LUT while allowing use of UTF-8 as well.
    ///  
    /// **NOTE:** associated const char array and alphabet used in the macro should be the exact same
    /// e.g.
    /// ```rust
    /// use smol_base_x::gen_ascii_match;
    /// 
    /// const ALPHABET: [char; 58] =
    ///     const_str::to_char_array!("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz");
    ///
    /// fn lookup_ascii(ch: u8) -> Option<usize> {
    ///     gen_ascii_match!(
    ///         ch,
    ///         b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz" // base58
    ///     )
    /// }
    /// ```
    fn lookup_ascii(ch: u8) -> Option<usize>;

    #[cfg(feature = "unstable")]
    /// 
    /// This makes more assumptions about the quality of the input char array and will not do some things that would otherwise be done with the normal base-x
    /// * no spaces before or after 
    /// * no leading ones to skip ?
    /// 
    /// this is done in order to ensure decoded size is not over-bloated at compile time
    /// 
    /// output is `(decoded bytes, bytes written)`
    fn decode_arr<const CHARS: usize, I: AsRef<[u8; CHARS]>>(
        input: I,
    ) -> Result<([u8; decoded_arr_size(Self::BASE, CHARS)], usize), DecodeError> {
        let input = input.as_ref();

        let mut arr = [0u8; decoded_arr_size(Self::BASE, CHARS)];

        todo!();
        // Ok(arr)
    }

    #[cfg(feature = "unstable")]
    /// output is `(encoded chars, chars written)`
    fn encode_arr<const BYTES: usize, I: AsRef<[u8; BYTES]>>(
        input: I,
    ) -> Result<[char; encoded_arr_size(Self::BASE, BYTES)], DecodeError> {
        let input = input.as_ref();

        let mut arr = [0u8; encoded_arr_size(Self::BASE, BYTES)];

        todo!()        
    }
}
