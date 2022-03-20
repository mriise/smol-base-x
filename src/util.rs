#[cfg(feature = "unstable")]
pub use const_arr_size::*;

/// generates the final decoded ceiling for a given base to be used as the slice size
/// given as `(input_byte_size * (log10(base) / log10(256))`  
/// 
/// **This is rounded up** it will sometimes say it needs one byte bigger than what it needs
pub fn decoded_size(base: usize, input_byte_size: usize) -> usize {
    (input_byte_size as f32 * (log10(base) / log10(256_usize)) + 1.0) as usize
}

/// generates the final encoded ceiling for a given base to be used as the slice size
/// given as `input_byte_size * (log10(256) / log10(base))`  
/// 
/// **This is rounded up** it will sometimes say it needs one byte bigger than what it needs
pub fn encoded_size(base: usize, input_byte_size: usize) -> usize {
    (input_byte_size as f32 * (log10(256_usize) / log10(base)) + 1.0) as usize
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

pub(crate) const fn max_utf8_char_len<const S: usize>(chars: [char; S]) -> usize {
    let mut max = 0;
    let mut ch = 0;
    while ch < S {
        if chars[ch].len_utf8() > max {
            max = chars[ch].len_utf8();
        }
        ch += 1;
    }
    max
}

/// base58 style LUT see C++ for original usage
pub const fn gen_lut<const BASE: usize>(alphabet: &[u8; BASE]) -> [i8; 256] {
    let mut lut = [-1_i8; 256];

    let mut i = 0;
    while i < alphabet.len() {
        lut[alphabet[i] as usize] = i as i8;
        i += 1;
    }

    lut
}

/// is_zeroed speedup hack from https://stackoverflow.com/questions/65367552/checking-a-vecu8-to-see-if-its-all-zero
pub(crate) fn is_zero(buf: &[u8]) -> bool {
    let (prefix, aligned, suffix) = unsafe { buf.align_to::<u64>() };

    prefix.iter().all(|&x| x == 0)
        && suffix.iter().all(|&x| x == 0)
        && aligned.iter().all(|&x| x == 0)
}

fn ln(x: f32) -> f32 {
    let mut old_sum = 0.0;
    let xmlxpl = (x - 1.0) / (x + 1.0);
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

fn log10(x: usize) -> f32 {
    return ln(x as f32) / core::f32::consts::LN_10;
}

#[cfg(feature = "unstable")]
pub(crate) mod const_arr_size {

    /// generates the final decoded ceiling for a given base to be used as the allocated array size
    /// given as `input_byte_size * (log10(256) / log10(base)) rounded up`
    pub const fn decoded_arr_size(base: usize, input_byte_size: usize) -> usize {
        (input_byte_size as f32 * (log10(base) / log10(256)) + 1.0) as usize
    }
    /// generates the final encoded ceiling for a given base to be used as the allocated array size
    /// given as `(input_byte_size * (log10(base) / log10(256)) rounded up`
    pub const fn encoded_arr_size(base: usize, input_byte_size: usize) -> usize {
        (input_byte_size as f32 * (log10(256) / log10(base)) + 1.0) as usize
    }

    const fn log10(x: usize) -> f32 {
        return ln(x as f32) / core::f32::consts::LN_10;
    }

    // https://stackoverflow.com/questions/35968963/trying-to-calculate-logarithm-base-10-without-math-h-really-close-just-having
    pub(crate) const fn ln(x: f32) -> f32 {
        let mut old_sum = 0.0;
        let xmlxpl = (x - 1.0) / (x + 1.0);
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
