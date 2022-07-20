use bech32::{self, FromBase32, ToBase32, Variant, u5};
use core::result::Result as result;

pub enum StringWrapper {
    AccAddress { val: String },
    ValAddress { val: String },
    AccPubKey { val: String },
    ValPubKey { val: String },
    ValConsPubKey { val: String },
}

impl StringWrapper {
    pub fn check_prefix_and_length(prefix: &str, data: &str, length: i32) -> bool {
        match bech32::decode(data) { 
            result::Ok(t) => {
                t.0 == prefix && data.chars().count() == length as usize
            }
            result::Err(e) => {
                eprintln!("{}", e);
                false
            },
        }
    }

    pub fn is_valid(&self) -> bool {
        let vals = bech32::decode(self.g);
        match self {
            StringWrapper::AccAddress { val } => { StringWrapper::check_prefix_and_length("secret", vals.1, vals.2)}
            StringWrapper::ValAddress { val } => { check_prefix_and_length()}
            StringWrapper::AccPubKey { val } => { check_prefix_and_length()}
            StringWrapper::ValPubKey { val }=> { check_prefix_and_length()}
            StringWrapper::ValConsPubKey { val } => { check_prefix_and_length()}
        }
    }
}

