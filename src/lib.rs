#![cfg_attr(feature = "unstable", feature(generic_const_exprs, const_fn_floating_point_arithmetic))]
// #![no_std]

#![doc = include_str!("../README.md")]

mod base;

mod utf_base;
mod base_impl;

/// encode/decode_arr need to use log in order to estimate the size, which is why unstable floating point arithmetic is needed
/// since there is also no const fn for log, there is an implementation of log10 hidden
/// 
/// one day the log implemenation should be replaced with the proper function when it comes out   
pub mod util;


pub use proc_macro::{gen_char_match, gen_ascii_match};

pub use base_impl::Base58btc;
pub use base::Base;
pub use utf_base::UtfBase;

#[derive(Debug)]
pub enum DecodeError {
    InvalidLength(usize),
    /// char not in alphabet
    InvalidChar,
    CharAfterTrailingSpaces
}


#[cfg(test)]
mod tests {
    use crate::{base::*, Base58btc};

    // TODO: make tests for multi byte chars

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

        // let c = Base58btc::encode_arr(&[0u8; 4]).unwrap();

    }

    #[test]
    fn full_cycle() {
        
        let input = b"44Y6qTgSvRMkdqpQ5ufkN";
        let mut output = [0u8; 128];

        let written = Base58btc::decode_mut(input, &mut output).unwrap();
        let expected = hex::decode("6d691bdd736346aa5a0a95b373b2ab").unwrap();
            
        println!("{:x?}\n{:x?}", &output[..written], &expected.as_slice());
        assert!(&output[..written] == expected.as_slice());


        let expected = input;

        let mut input = [0u8; 128];
        input.copy_from_slice(&output);
        output.fill(0);

        let written = Base58btc::encode_mut(&input[..written], &mut output).unwrap();

        
        
        println!("{:x?}\n{:x?}", &output[..written], expected);
        assert!(&output[..written] == expected);
        
    }
    

    #[test]
    fn abc_decode() {
        let mut output = [0u8; 32];

        let written = Base58btc::decode_mut(b"ZiCa", &mut output).unwrap();

        let expected = b"abc";

        println!("{:x?}\n{:x?}", &output[..written], &expected.as_slice());

        assert!(&output[..written] == expected.as_slice());
    }

    #[test]
    fn abc_encode() {
        let mut output = [0u8; 32]; // 3 spaces takes the same space as 'abc'

        let written = Base58btc::encode_mut(b"abc", &mut output).unwrap();

        let expected = "ZiCa";

        let output = core::str::from_utf8(&output[..written]).unwrap();

        println!("{:x?}\n{:x?}", output, expected);


        assert!(output == expected);
    }
    
}
