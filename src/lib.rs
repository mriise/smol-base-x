
// #![cfg_attr(not(feature = "std"), no_std)]
#![feature(generic_const_exprs)]
#![feature(const_fn_floating_point_arithmetic)]
pub mod base;
pub use proc_macro::gen_char_match;

pub enum DecodeError {
    InvalidLength(usize),
    InvalidChar,
}



#[cfg(test)]
mod tests {

    #[test]
    fn test_proc_macro() {
        
        #[derive(Debug)]
        enum E {
            Error
        }

        fn proc(ch: char) -> Option<usize>{
            proc_macro::gen_char_match!(ch, "abc")
        }

        fn normal(ch: char) -> Result<usize, E>{
            match ch {
                'a' => Ok(0),
                'b' => Ok(1),
                'c' => Ok(2),
                _ => Err(E::Error),
            }
        }
        
        for ch in ['a', 'b', 'c'] {
            assert_eq!(proc(ch).unwrap(), normal(ch).unwrap());
        }
        
    }
}
