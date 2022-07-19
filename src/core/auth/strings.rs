use bech32::{self, FromBase32, ToBase32, Variant, u5};

fn check_prefix_and_length(prefix: &str, data: &str, length: i32) -> bool {
    let vals = bech32::decode(data); 
    // -> Result<(String, Vec<u5>, Variant), Error> 
    match vals {
        // .chars().count() for UTF-8, does not take ownership
        Result => {
            vals.unwrap().0 == prefix && data.chars().count() == length; 
        }
        //TODO: should this return false? Throw an error?
        Error => false,
    }
}

// **********  AccAddress *********** //
pub struct AccAddress {
    val: String,
}

fn is_acc_address(data: &str) -> bool {
    //return check_prefix_and_length("secret", data, 45)
    check_prefix_and_length("secret", data, 45);
}

fn to_acc_address(data: ValAddress) -> AccAddress {
    let vals = bech32::decode(data);
    match vals.1 {
        Vec => {
            AccAddress {
                val: bech32::encode("secret", vals.1),
            }
        },
        Error => None, // TODO: Throw an error? what is that?
    }
}


// **********  ValAddress *********** //
pub struct ValAddress {
    val: String,
}

fn is_val_address(data: &str) -> bool {
    return check_prefix_and_length("secretvaloper", data, 52);
}

fn to_val_address(data: AccAddress) -> ValAddress {
    let vals = bech32::decode(data);
    match vals.1 {
        Vec => {
            AccAddress {
                val: bech32::encode("secretvaloper", vals.1),
            }
        },
        Error => None, // TODO: Throw an error? what is that?
    }

}


pub struct AccPubKey 
{
    val: String,
}

pub struct ValPubKey {
    val: String,
}

pub struct ValConsPubKey {
    val: String,
}

