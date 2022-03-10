use super::DecodeError;
use crate::{base::Base, util::*};

/// Base-X that can use utf-8 scalar chars, somewhat esoteric and less effecient.  
///
/// Consider using [base-x](https://crates.io/crates/base-x) as I belive they have a better implementation
/// for this use case (as well as for larger encoding inputs)  
///
/// **NOTE:** You are probably looking for [`Base`]
pub trait UtfBase<const BASE: usize> {
    const ALPHABET: [char; BASE];

    const BASE: usize = Self::ALPHABET.len();

    /// The longest length char (in bytes) of the alphabet always 1-4
    const MAX_CHAR_LEN: u8 = max_utf8_char_len(Self::ALPHABET);

    // returns bytes written to buffer
    fn utf_decode_mut<I: AsRef<str>>(input: I, buf: &mut [u8]) -> Result<usize, DecodeError> {
        let input = input.as_ref();
        let mut iter = input.chars().peekable();
        // Skip leading spaces.
        while iter.peek() == Some(&' ') {
            iter.next();
        }

        // Skip and count leading '1's.
        let mut zeroes = 0;

        while iter.peek() == Some(&'1') {
            iter.next();
            zeroes += 1;
            if zeroes > buf.len() {
                return Err(DecodeError::InvalidLength(zeroes)); // right???
            }
        }

        // Allocate enough space in big-endian base256 representation.
        // clone probably gets optimized to copy here? size_hint() may be better but IDK really
        let size = decoded_size(Self::BASE, iter.clone().count());

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

            let mut rev = buf.iter_mut().rev().peekable(); // might not need to peek since we already know it is big enough
            let mut i = 0;
            while rev.peek().is_some() && (i < length || carry != 0) {
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
        while iter.next() == Some(' ') {} // this might ignore a single char after the last space

        if iter.next().is_some() {
            return Err(DecodeError::CharAfterTrailingSpaces);
        }
        buf.rotate_left(buf.len() - length);
        Ok(length)
    }

    /// output buff is intentionally a slice since &mut str is essentially useless
    ///
    /// users will have to convert output bytes into a str
    fn utf_encode_mut(input: &[u8], buf: &mut [u8]) -> Result<usize, DecodeError> {
        let mut zeroes = 0;
        let mut length = 0;
        // extra bytes may be un-needed and can just be put into i or length
        let mut extra_bytes = 0; // extra bytes added by utf char being more than one byte

        let mut iter = input.iter().peekable();

        // skip & count leading zeros
        while iter.peek() == Some(&&0) {
            iter.next();
            zeroes += 1;
        }

        // size in # of chars not bytes
        let size = encoded_size(Self::BASE, iter.len());

        // buf is too small to fit string
        if (buf.len() * Self::MAX_CHAR_LEN as usize) < size {
            return Err(DecodeError::InvalidLength(buf.len()));
        }

        while iter.peek().is_some() {
            let mut carry = *iter.next().unwrap() as usize;

            let mut i = 0;
            let mut rev = buf.iter_mut().rev().peekable(); // might not need to peek since we already know it is big enough
            while rev.peek().is_some() && (i < length || carry != 0) {
                // shouldnt ever panic because of above
                let it = rev.next().unwrap();

                carry += 256 * (*it as usize);
                *it = (carry % Self::BASE) as u8;

                carry /= Self::BASE;
                i += 1;
            }

            length = i + zeroes;
            if length + extra_bytes > buf.len() {
                return Err(DecodeError::InvalidLength(length));
            }

            assert!(carry == 0);
        }
        buf.rotate_left(buf.len() - length);

        // goes thru each index shifting remaining over to fit utf bigger than one byte
        let mut remaining_chars = length;
        let mut current_index = 0;
        while remaining_chars != 0 {
            // current char
            let ch = Self::ALPHABET[buf[current_index] as usize];
            let ch_len = ch.len_utf8();

            if ch_len > 1 {
                let mut ch_bytes = [0u8; 4];
                ch.encode_utf8(&mut ch_bytes);

                // shift bytes over to fit char
                buf.copy_within(
                    current_index + 1..buf.len() - ch_len,
                    current_index + ch_len,
                );

                for byte in &ch_bytes[..ch_len] {
                    buf[current_index] = *byte;
                    current_index += 1;
                }
            } else {
                buf[current_index] = ch as u8; // single byte char
                current_index += 1;
            }
            extra_bytes += ch_len - 1;
            remaining_chars -= 1;
        }

        Ok(length + extra_bytes)
    }

    /// C++ algorithim uses a \[i8; 256] LUT
    ///
    /// since we want to use any UTF-8, we instead generate a match statement for each char
    /// the result should be smaller and might actually be faster (TODO check this) than using a LUT while allow use of UTF-8 as well
    ///
    /// **NOTE:** associated const char array and alphabet used in the macro should be the exact same
    ///
    /// e.g.
    /// ```rust
    /// use smol_base_x::gen_char_match;
    ///
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
    ///
    /// This makes more assumptions about the quality of the input char array and will not do some things that would otherwise be done with the normal base-x
    /// * no spaces before or after
    /// * no leading ones to skip ?
    ///
    /// this is done in order to ensure decoded size is not over-bloated at compile time
    ///
    /// output is `(decoded bytes, bytes written)`
    fn utf_decode_arr<'a, const CHARS: usize>(
        input: &[char; CHARS],
    ) -> Result<([u8; decoded_arr_size(Self::BASE, CHARS)], &'a [u8]), DecodeError> {
        let mut arr = [0u8; decoded_arr_size(Self::BASE, CHARS)];

        todo!();
        // Ok(arr)
    }

    #[cfg(feature = "unstable")]
    /// output is `(encoded chars, chars written)`
    fn utf_encode_arr<const BYTES: usize>(
        input: &[u8; BYTES],
    ) -> Result<[char; encoded_arr_size(Self::BASE, BYTES)], DecodeError> {
        let mut arr = [0u8; encoded_arr_size(Self::BASE, BYTES)];

        todo!()
    }
}

impl<const BASE: usize, T: Base<BASE>> UtfBase<BASE> for T {
    const ALPHABET: [char; BASE] = ascii_to_char_arr(Self::ALPHABET);

    fn lookup_char(ch: char) -> Option<usize> {
        Self::lookup_ascii(ch as u8)
    }
}
