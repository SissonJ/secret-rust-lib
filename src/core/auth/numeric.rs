use std::ops::{ Add, Sub, Mul, Div };
use regex::*;
use lazy_static::lazy_static;
use num_bigint::BigInt;

/*
The goal of this is to be able to use the Dec type with normal operators. 
TODO: Figure out how to do that, what do the rust operators call
*/

const dec_num_digits: i32 = 18;
const dec_one: i32 = 10_i32.pow(18_u32);
// const dec_pattern: regex::Regex = regex::Regex::new(r"^(\-)?(\d+)(\.(\d+))?\Z").unwrap();

pub union Number {
    str: &'static str,
    int: i32,
    float: f32,
}

pub fn convert_to_dec_bignum(arg: Number) -> Option<BigInt> {
    unsafe {
        match arg {
          Number { str } => { return from_str(str) }, 
          Number { int } => {  return Some(BigInt::from(int * dec_one)) } 
          Number { float } => { 
            let float_string = float.to_string();
            return from_str(&float_string);
          } 
        }
    }
    unsafe fn from_str(arg: &str) -> Option<BigInt> {
        // TODO: better expect messages
        lazy_static! { 
            static ref RE: Regex = Regex::new(r"^(\-)?(\d+)(\.(\d+))?\Z").unwrap();
        }
        let parts = RE.captures(arg)?;
        let result: i32 = parts.get(2)?
            .as_str()
            .trim()
            .parse::<i32>()
            .expect("Invalid String: NAN") * dec_one;
        if let Some(str) = parts.get(3) {
            let fraction: i32 = parts.get(4)?
                .as_str()
                .trim()
                .parse::<i32>()
                .expect("Invalid String: NAN");
            result += fraction;
        }
        if let Some(str) = parts.get(1) {
            result *= -1;
        }
        return Some(BigInt::from(result));
    }
}
#[derive(Default)]
pub struct Dec {
    i: BigInt, 

}

//TODO: JSONSerializable
impl Dec {
    /*
    I am pretty sure the __operator__ methods in the python library override the 
    normal methods to call the implemented methods, so you can perform normal operations
    with the custom type. 

    */
    pub fn new(arg: Number) -> Dec {
        Dec {
            i: convert_to_dec_bignum(arg).unwrap(),
        }
    }

    pub fn zero() -> Dec { 
        return Dec::new(Number { int: 0,}) ;
    }
    
    pub fn one() -> Dec { 
        return Dec::new(Number { int: 1,}) ;
    }


    // traits to implement: Add, Sub, Mul, Div
}

impl Add for Dec { 

}

impl Sub for Dec { 

}

impl Mul for Dec { 

}

impl Div for Dec { 

}