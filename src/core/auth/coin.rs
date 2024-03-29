use json::{object, JsonValue};
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Coin {
    pub denom: String,
    pub amount: i32,
}

impl Coin {
    pub fn to_data(&self) -> JsonValue {
        object! {
            "amount": self.amount.to_string(),
            "denom": self.denom.clone(),
        }
    }
    pub fn to_string(&self) -> String {
        self.denom.clone() + &self.amount.to_string()
    }

    pub fn from_string(input: String) -> Coin {
        let re = Regex::new(r"^(\-?[0-9]+(\.[0-9]+)?)([0-9a-zA-Z/]+)$").unwrap();
        if !re.is_match(&input) {
            //return Err();
            Coin {
                denom: "".to_string(),
                amount: 0,
            }
        } else {
            let caps = re.captures(&input).unwrap();
            Coin {
                denom: caps.get(1).unwrap().as_str().to_string(),
                amount: caps.get(0).unwrap().as_str().parse().unwrap(),
            }
        }
    }
}
