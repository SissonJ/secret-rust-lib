use bech32::{self, FromBase32, ToBase32, Variant, u5};
use core::result::Result as result;

fn check_prefix_and_length(prefix: &str, data: &str, length: i32) -> bool {
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

// **********  AccAddress *********** //
pub struct AccAddress {
    val: String,
}

fn is_acc_address(data: &str) -> bool {
    //return check_prefix_and_length("secret", data, 45)
    check_prefix_and_length("secret", data, 45)
}

fn to_acc_address(data: ValAddress) -> Option<AccAddress> {
    let vals = bech32::decode(&data.val);
    match vals {
        Ok(t) => {
            match bech32::encode("secret", t.1, t.2) {
                Ok(string) => Some(AccAddress {
                    val: string,
                }),
                Err(e) => {
                    eprintln!("{}", e);
                    None
                },
            }
        },
        Err(e) => {
            eprintln!("{}", e);
            None
        },
    }
}


// **********  ValAddress *********** //
pub struct ValAddress {
    val: String,
}

fn is_val_address(data: &str) -> bool {
    return check_prefix_and_length("secretvaloper", data, 52);
}

fn to_val_address(data: AccAddress) -> Option<ValAddress> {
    let vals = bech32::decode(&data.val);
    match vals {
        Ok(t) => {
            match bech32::encode("secretvaloper", t.1, t.2) {
                Ok(string) => Some(ValAddress {
                    val: string,
                }),
                Err(e) => {
                    eprintln!("{}", e);
                    None
                },
            }
        },
        Err(e) => {
            eprintln!("{}", e);
            None
        },
    }

}

// // **********  AccPubKey *********** //
pub struct AccPubKey {
    val: String,
}
fn is_acc_pubkey(data: &str) -> bool {
    return check_prefix_and_length("secretpub", data, 77);
}

fn to_acc_pubkey(data: ValPubKey) -> Option<AccPubKey> {
    let vals = bech32::decode(&data.val);
    match vals {
        Ok(t) => {
            match bech32::encode("secretpub", t.1, t.2) {
                Ok(string) => Some(AccPubKey {
                    val: string,
                }),
                Err(e) => {
                    eprintln!("{}", e);
                    None
                },
            }
        },
        Err(e) => {
            eprintln!("{}", e);
            None
        },
    }
}


// // **********  ValPubKey *********** //
pub struct ValPubKey {
    val: String,
}

fn is_val_pubkey(data: &str) -> bool {
    return check_prefix_and_length("secretvaloperpub", data, 84);
}

fn to_val_pubkey(data: AccPubKey) -> Option<ValPubKey> {
    let vals = bech32::decode(&data.val);
    match vals {
        Ok(t) => {
            match bech32::encode("secretvaloperpub", t.1, t.2) {
                Ok(string) => Some(ValPubKey {
                    val: string,
                }),
                Err(e) => {
                    eprintln!("{}", e);
                    None
                },
            }
        },
        Err(e) => {
            eprintln!("{}", e);
            None
        },
    }
}



// // **********  ValConsPubKey *********** //
pub struct ValConsPubKey {
    val: String,
}

fn is_valcons_pubkey(data: &str) -> bool {
    return check_prefix_and_length("secretvalconspub", data, 83)
}

