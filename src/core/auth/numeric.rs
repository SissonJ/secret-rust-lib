use std::ops::{ Add, Sub, Mul, Div };
use regex::*;
use lazy_static::lazy_static;
use num_bigint::BigInt;

/*
The goal of this is to be able to use the Dec type with normal operators. 
TODO: Figure out how to do that, what do the rust operators call
*/

const DEC_NUM_DIGITS: i32 = 18;
const DEC_ONE: i32 = 10_i32.pow(18_u32);

pub enum Number {
    Str(&'static str),
    Int(i32),
    Float(f32),
}

pub fn convert_to_dec_bignum(arg: Number) -> Option<BigInt> {
    match arg {
        Number::Str(str) => { return from_str(str) }, 
        Number::Int(int) => {  return Some(BigInt::from(int * DEC_ONE)) } 
        Number::Float(float) => { 
        let float_string = float.to_string();
        return from_str(&float_string);
        } 
    }
    fn from_str(arg: &str) -> Option<BigInt> {
        lazy_static! { 
            static ref RE: Regex = Regex::new(r"^(\-)?(\d+)(\.(\d+))?\Z").unwrap();
        }
        let parts = RE.captures(arg)?;
        let result: i32 = parts.get(2)?
            .as_str()
            .trim()
            .parse::<i32>()
            .expect("Invalid String: NAN") * DEC_ONE;
        if let Some(str) = parts.get(3) {
            let fraction: i32 = parts.get(4)?
                .as_str()
                .trim() // TODO: slice
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

fn chop_precision_and_round(d: i32) -> i32 {
    if d < 0 {
        return -1 * chop_precision_and_round(d * -1);
    } 

    let quo = d / DEC_ONE;
    let rem = d % DEC_ONE;

    if rem == 0 {
        return quo;
    }

    if rem < DEC_ONE / 2 {
        return quo;
    } else if rem > DEC_ONE / 2 {
        return quo + 1;
    } else {
        if quo % 2 == 0 {
            return quo;
        }
        return quo;
    }
}


#[derive(Default)] // Defaults i to 0 (https://docs.rs/num-bigint/latest/src/num_bigint/bigint.rs.html#132-137)
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