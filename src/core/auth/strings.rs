// TODO:: Every function should return a Result for custom error type
use bech32::{self, FromBase32, ToBase32, Variant, u5};
// use bech32::Error as b32Err;
use bech32::Error::*;
// use reqwest::Error as rqErr;
use core::result::Result as Result;

fn check_prefix_and_length(prefix: &str, data: &str, length: i32) -> bool {
    match bech32::decode(data) { 
        Result::Ok(t) => {
            t.0 == prefix && data.chars().count() == length as usize
        }
        Result::Err(e) => {
            eprintln!("{}", e);
            false
        },
    }
}

// **********  AccAddress *********** //
pub struct AccAddress {
    val: String,
}

fn is_acc_address(data: &str) -> bool {
    //return check_prefix_and_length("secret", data, 45)
    check_prefix_and_length("secret", data, 45)
}

fn to_acc_address(data: ValAddress) -> Result<AccAddress, bech32::Error> {
    let decode = bech32::decode(&data.val)?;
    let encode = bech32::encode("secret", decode.1, decode.2)?;
    Ok(AccAddress {
        val: encode,
    })
}


// **********  ValAddress *********** //
pub struct ValAddress {
    val: String,
}

fn is_val_address(data: &str) -> bool {
    return check_prefix_and_length("secretvaloper", data, 52);
}


fn to_val_address(data: AccAddress) -> Result<ValAddress, bech32::Error> {
    let decode = bech32::decode(&data.val)?;
    let encode = bech32::encode("secretvaloper", decode.1, decode.2)?;
    Ok(ValAddress {
        val: encode,
    })
}

// // **********  AccPubKey *********** //
pub struct AccPubKey {
    val: String,
}
fn is_acc_pubkey(data: &str) -> bool {
    return check_prefix_and_length("secretpub", data, 77);
}

fn to_acc_pubkey(data: ValPubKey) -> Result<AccPubKey, bech32::Error> {
    let decode = bech32::decode(&data.val)?;
    let encode = bech32::encode("secretpub", decode.1, decode.2)?;
    Ok(AccPubKey {
        val: encode,
    })
}

// // **********  ValPubKey *********** //
pub struct ValPubKey {
    val: String,
}

fn is_val_pubkey(data: &str) -> bool {
    return check_prefix_and_length("secretvaloperpub", data, 84);
}

fn to_val_pubkey(data: AccPubKey) -> Result<ValPubKey, bech32::Error> {
    let decode = bech32::decode(&data.val)?;
    let encode = bech32::encode("secretvaloperpub", decode.1, decode.2)?;
    Ok(ValPubKey {
        val: encode,
    })
}


// // **********  ValConsPubKey *********** //
pub struct ValConsPubKey {
    val: String,
}

fn is_valcons_pubkey(data: &str) -> bool {
    return check_prefix_and_length("secretvalconspub", data, 83)
}

