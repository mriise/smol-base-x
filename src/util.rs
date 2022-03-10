#[cfg(feature = "unstable")]
pub use const_arr_size::*;

/// generates the final decoded ceiling for a given base to be used as the slice size
/// given as `(input_byte_size * (log10(base) / log10(256)) + 1.0`
pub fn decoded_size(base: usize, input_byte_size: usize) -> usize {
    (input_byte_size as f64 * ((base as f64).log10() / 256_f64.log10())) as usize
}

/// generates the final encoded ceiling for a given base to be used as the slice size
/// given as `input_byte_size * (log10(256) / log10(base))`
pub fn encoded_size(base: usize, input_byte_size: usize) -> usize {
    (input_byte_size as f64 * (256_f64.log10() / (base as f64).log10()) + 1.0) as usize
}

/// takes an array of ascii chars and fills a char array of the same length
/// shouldnt be necessary for users as Base<BASE> has a blanket impl for `UtfBase<BASE>`
pub(crate) const fn ascii_to_char_arr<const S: usize>(ascii: [u8; S]) -> [char; S] {
    let mut arr = [' '; S];
    let mut ch = 0;
    while ch < S {
        arr[ch] = ascii[ch] as char;
        ch += 1;
    }
    arr
}

pub(crate) const fn max_utf8_char_len<const S: usize>(chars: [char; S]) -> u8 {
    let mut max: u8 = 0;
    let mut ch = 0;
    while ch < S {
        if chars[ch].len_utf8() as u8 > max {
            max = chars[ch].len_utf8() as u8;
        }
        ch += 1;
    }
    max
}

#[cfg(feature = "unstable")]
mod const_arr_size {

    /// generates the final decoded ceiling for a given base to be used as the allocated array size
    /// given as `input_byte_size * (log10(256) / log10(base))`
    pub const fn decoded_arr_size(base: usize, input_byte_size: usize) -> usize {
        (input_byte_size as f64 * (log10(base) / log10(256))) as usize
    }
    /// generates the final encoded ceiling for a given base to be used as the allocated array size
    /// given as `(input_byte_size * (log10(base) / log10(256)) + 1.0`
    pub const fn encoded_arr_size(base: usize, input_byte_size: usize) -> usize {
        (input_byte_size as f64 * (log10(256) / log10(base)) + 1.0) as usize
    }

    const fn log10(x: usize) -> f64 {
        return ln(x) / core::f64::consts::LN_10;
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
}
