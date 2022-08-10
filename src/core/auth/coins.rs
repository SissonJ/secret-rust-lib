use super::coin::Coin;
use json::object;
use json::JsonValue;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Coins {
    pub coins: Vec<Coin>,
}

impl Coins {
    pub fn to_data(&self) -> Vec<JsonValue> {
        let mut coin_vec = vec![];
        for coin in self.coins.clone() {
            coin_vec.push(coin.to_data());
        }
        coin_vec
    }

    pub fn to_string(&self) -> String {
        let mut res = "[ ".to_string();
        for coin in &self.coins {
            res = res.clone() + &coin.to_string() + ", ";
        }
        res + "]"
    }
}
