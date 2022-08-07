use regex::*;

/*
The goal of this is to be able to use the Dec type with normal operators. 
TODO: Figure out how to do that, what do the rust operators call
*/

const dec_num_digits: i32 = 18;
const dec_one: i32 = 10_i32.pow(18_u32);
const dec_pattern: regex::Regex = regex::Regex::new(r"^(\-)?(\d+)(\.(\d+))?\Z").unwrap();

union Number {
    str: &'static str,
    int: i32,
    float: f32,
}

pub fn convert_to_dec_bignum(arg: Number) -> f32 {
    unsafe {
        match arg {
          Number { str } => {}  
          Number { int } => { (int * dec_one) as f32 } 
          Number { float } => {} 

        }

    }
}
pub struct Dec {

}

impl Dec {
    /*
    I am pretty sure the __operator__ methods in the python library override the 
    normal methods to call the implemented methods, so you can perform normal operations
    with the custom type. 

    */
}
