use std::ops::{ Add, Sub, Mul, Div };
use std::fmt;
use regex::*;
use lazy_static::lazy_static;
use crate::core::errors::SecretError;

/*
The goal of this is to be able to use the Dec type with normal operators. 
TODO: Figure out how to do that, what do the rust operators call
*/

const DEC_NUM_DIGITS: i128 = 18;
const DEC_ONE: i128 = 10_i128.pow(18_u32);

pub enum Number {
    Str(&'static str),
    Int(i32),
    Float(f32),
}

pub fn convert_to_dec_bignum(arg: Number) -> Result<i128, SecretError> {
    // TODO: Better Error messages
    match arg {
        Number::Str(str) => { 
            return match from_str(str) {
                Some(BigInt) => Ok(BigInt),
                None => Err(SecretError::Error("Not found".to_string()))
            }
        }, 
        Number::Float(float) => { 
            let float_string = float.to_string();
            return match from_str(&float_string) {
                Some(BigInt) => Ok(BigInt),
                None => Err(SecretError::Error("Not found".to_string()))
            }
        } 
        Number::Int(int) => { return Ok(int as i128 * DEC_ONE) } 
    }

    fn from_str(arg: &str) -> Option<i128> {
        lazy_static! { 
            static ref RE: Regex = Regex::new(r"^(\-)?(\d+)(\.(\d+))?\Z").unwrap();
        }
        let parts = RE.captures(arg)?;
        let result: i128 = parts.get(2)?
            .as_str()
            .trim()
            .parse::<i128>()
            .expect("Invalid String: NAN") * DEC_ONE;
        if let Some(str) = parts.get(3) {
            let fraction: i128 = parts.get(4)?
                .as_str()
                .trim() // TODO: slice
                .parse::<i128>()
                .expect("Invalid String: NAN");
            result += fraction;
        }
        if let Some(str) = parts.get(1) {
            result *= -1;
        }
        return Some(result as i128);
    }
}

fn chop_precision_and_round(d: i128) -> i128 {
    if d < 0 {
        return -1 * chop_precision_and_round(d * -1);
    } 

    let quo: i128 =  d / DEC_ONE;
    let rem: i128 = d % DEC_ONE;

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
    i: i128, 
}

//TODO: JSONSerializable
impl Dec {
    /// Decimal represntation num_bigint::BigInt with basic aritmetic operations 
    /// compatible with basic Rust numeric types. 
    /// 
    /// To implement: 
    /// - [x] display
    /// - [x] zero
    /// - [x] one 
    /// - [ ] int
    /// - [ ] float
    /// - [x] parity
    /// - [x] whole
    /// - [x] frac
    /// - [ ] to_data
    /// - [ ] equalities (less than, equal, greater than)
    /// - [x] aritmetic (add, sub, mul, div, mod)
    


    /*
    I am pretty sure the __operator__ methods in the python library override the 
    normal methods to call the implemented methods, so you can perform normal operations
    with the custom type. 

    */

    pub fn from(arg: Number) -> Result<Dec, SecretError> {
        Ok(Dec { i: convert_to_dec_bignum(arg)?, })
    }

    pub fn zero() -> Result<Dec, SecretError> { Dec::from(Number::Int(0)) }
    
    pub fn one() -> Result<Dec, SecretError> { Dec::from(Number::Int(1)) }

    pub fn whole(&self) -> String {
        format!("{}", self.i.abs() / DEC_ONE)
    }

    pub fn frac(&self) -> String {
        format!("{}", self.i.abs() % DEC_ONE).trim().to_string()
    }

    pub fn parity(&self) -> i32 {
        if self.i < 0 { -1 } else { 1 }
    }

    pub fn add_dec(&self, addend: Dec) -> i128 {
        self.i + addend.i
    }

    pub fn sub_dec(&self, subtrahend: Dec) -> i128 {
        self.i - subtrahend.i
    }

    pub fn mul_dec(&self, multiplier: Dec) -> i128 {
        let x = self.i;
        let y = multiplier.i;
        chop_precision_and_round(x * y)
    }

    pub fn div_dec(&self, divisor: Dec) -> i128 {
        if divisor.i == 0 {
            panic!("Error: Tried to divide by 0 for {} / {}", self.i, divisor.i);
        } else {
            chop_precision_and_round((self.i * DEC_ONE * DEC_ONE) / divisor.i)
        }
    }


    // traits to implement: Add, Sub, Mul, Div
}

impl fmt::Display for Dec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.i == 0 {
            write!(f, "{}", "0.".to_owned() + &"0".repeat(DEC_NUM_DIGITS as usize));
        }
        let parity = if self.i > 0 { "-" } else { "" };
        write!(f, "{}{}.{}", parity, self.whole(), self.frac())
    }

}

impl Add for Dec { 
    type Output = Self;

    fn add(self, addend: Self) -> Self {
        Self {
            i: self.add_dec(addend),
        }
    }
}

impl Sub for Dec { 
    type Output = Self;

    fn sub(self, addend: Self) -> Self {
        Self {
            i: self.sub_dec(addend),
        }
    }
}

impl Mul for Dec { 
    type Output = Self;

    fn mul(self, multiplier: Self) -> Self {
        Self {
            i: self.mul_dec(multiplier),
        }
    }
}

impl Div for Dec { 
    type Output = Self;

    fn div(self, multiplier: Self) -> Self {
        Self {
            i: self.div_dec(multiplier),
        }
    }

}