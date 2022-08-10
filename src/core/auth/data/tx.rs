use super::publickey::PublicKey;
use crate::core::auth::{coins::Coins, msg::Msg};
use json::{object, JsonValue};
use serde::{Deserialize, Serialize};

//#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
//#[serde(rename_all = "snake_case")]
pub struct StdSignature {
    pub signature: String,
    pub pub_key: PublicKey,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct StdFee {
    pub gas: i32,
    pub amount: Coins,
}

impl StdFee {
    pub fn to_data(self) -> JsonValue {
        object! {
            "amount": self.amount.to_data(),
            "gas": self.gas.to_string(),
        }
    }
}

pub struct StdSignMsg {
    pub chain_id: String,
    pub account_number: i32,
    pub sequence: i32,
    pub fee: StdFee,
    pub msgs: Vec<Msg>,
    pub memo: String,
}

impl StdSignMsg {
    pub fn to_data(&self) -> JsonValue {
        let mut messages = vec![];
        for msg in self.msgs.clone() {
            messages.push(msg.to_data());
        }
        object! {
            "account_number": self.account_number.to_string(),
            "chain_id": self.chain_id.clone(),
            "fee": self.fee.clone().to_data(),
            "memo": self.memo.clone(),
            "msgs": messages,
            "sequence": self.sequence.to_string(),
        }
    }
}
