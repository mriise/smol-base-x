pub const fn gen_encoded_size<const S: usize>(_base: [char; S], input_byte_size: usize) -> usize {
    (input_byte_size as f64 * (log10(S) / log10(256)) + 1.0) as usize
}

pub const fn gen_decoded_size<const S: usize>(_base: [char; S], input_byte_size: usize) -> usize {
    (input_byte_size as f64 * (log10(256) / log10(S))) as usize
}

pub const fn ascii_to_char_arr<const S: usize>(ascii: [u8; S]) -> [char; S] {
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
