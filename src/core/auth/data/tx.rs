use json::{JsonValue, object};
use crate::core::auth::{msg::Msg, coins::Coins};
use super::publickey::PublicKey;

pub struct StdSignature{
    pub signature: String,
    pub pub_key: PublicKey,
}
pub struct StdFee{
    pub gas: i32,
    pub amount: Coins,
}

impl StdFee{
    pub fn to_data(self)->JsonValue{
        object!{
            "gas": self.gas,
            "amount": self.amount.to_string(),
        }
    }
}

pub struct StdSignMsg{
    pub chain_id: String,
    pub account_number: i32,
    pub sequence: i32,
    pub fee: StdFee,
    pub msgs: Vec<Msg>,
    pub memo: String,
}

impl StdSignMsg{
    pub fn to_data(&self) -> JsonValue {
        let mut messages = vec![];
        for msg in self.msgs {
            messages.push(msg.to_data());
        }
        object! {
            "chain_id": self.chain_id,
            "account_number": self.account_number,
            "sequence": self.sequence,
            "fee": self.fee.to_data(),
            "msgs": messages,
            memo: self.memo,
        }
    }
}