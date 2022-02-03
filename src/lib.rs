
#![cfg_attr(feature = "unstable", feature(generic_const_exprs))]
#![feature(const_fn_floating_point_arithmetic)]

pub mod base;
pub use proc_macro::gen_char_match;

#[derive(Debug)]
pub enum DecodeError {
    InvalidLength(usize),
    InvalidChar,
    CharAfterTrailingSpaces
}


#[cfg(test)]
mod tests {
    use crate::base::Base;


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

    #[test]
    fn aa() {
        use crate::base::Base58;

        let a = Base58::default();

        let c = a.encode_arr(&[0u8; 4]).unwrap();


    }
}
