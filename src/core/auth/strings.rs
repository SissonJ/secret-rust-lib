use bech32::{self, FromBase32, ToBase32, Variant, u5};
use bech32::Error as b32Err;
use bech32::Error::*;
use reqwest::Error as rqErr;
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

fn to_acc_address(data: ValAddress) -> Result<AccAddress, b32Err> {
    let vals = bech32::decode(&data.val);
    match vals {
        Ok(t) => {
            match bech32::encode("secret", t.1, t.2) {
                Ok(string) => Ok(AccAddress {
                    val: string,
                }),
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(e),
    }
}


// **********  ValAddress *********** //
pub struct ValAddress {
    val: String,
}

fn is_val_address(data: &str) -> bool {
    return check_prefix_and_length("secretvaloper", data, 52);
}

fn to_val_address(data: AccAddress) -> Result<ValAddress, b32Err> {
    let vals = bech32::decode(&data.val);
    match vals {
        Ok(t) => {
            match bech32::encode("secretvaloper", t.1, t.2) {
                Ok(string) => Ok(ValAddress {
                    val: string,
                }),
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(e),
    }

}

// // **********  AccPubKey *********** //
pub struct AccPubKey {
    val: String,
}
fn is_acc_pubkey(data: &str) -> bool {
    return check_prefix_and_length("secretpub", data, 77);
}

fn to_acc_pubkey(data: ValPubKey) -> Result<AccPubKey, b32Err> {
    let vals = bech32::decode(&data.val);
    match vals {
        Ok(t) => {
            match bech32::encode("secretpub", t.1, t.2) {
                Ok(string) => Ok(AccPubKey {
                    val: string,
                }),
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(e),
    }
}


// // **********  ValPubKey *********** //
pub struct ValPubKey {
    val: String,
}

fn is_val_pubkey(data: &str) -> bool {
    return check_prefix_and_length("secretvaloperpub", data, 84);
}

fn to_val_pubkey(data: AccPubKey) -> Result<ValPubKey, b32Err> {
    let vals = bech32::decode(&data.val);
    match vals {
        Ok(t) => {
            match bech32::encode("secretvaloperpub", t.1, t.2) {
                Ok(string) => Ok(ValPubKey {
                    val: string,
                }),
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(e),
    }
}



// // **********  ValConsPubKey *********** //
pub struct ValConsPubKey {
    val: String,
}

fn is_valcons_pubkey(data: &str) -> bool {
    return check_prefix_and_length("secretvalconspub", data, 83)
}


fn to_reqwest_error(err: b32Err) -> rqErr {
    match err {
        InvalidChar(c) => {},
        InvalidData(num) => {},
        others => {},
    }
} 
