use regex::*;
use lazy_static::lazy_static;

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

fn to_number
pub fn convert_to_dec_bignum(arg: Number) -> i64 {
    unsafe {
        match arg {
          Number { str } => {
            lazy_static! { 
                static ref RE: Regex = Regex::new(r"^(\-)?(\d+)(\.(\d+))?\Z").unwrap();
            }
            let caps = RE.captures(str);
                
            return 1_i64 // TODO: Needs fix


          }  
          Number { int } => { (int * dec_one) as i64 } 
          Number { float } => { float as i64 } // TODO: What is actually supposed to happen here

        }
    }
}
#[derive(Default)]
pub struct Dec {
    i: i64, 

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
            i: convert_to_dec_bignum(arg),
        }
    }

    pub fn zero() -> Dec { 
        return Dec::new(Number { int: 0,}) ;
    }
    
    pub fn one() -> Dec { 
        return Dec::new(Number { int: 1,}) ;
    }

}
