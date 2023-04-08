use crate::core::auth::coins::Coins;
use json::{object, JsonValue};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum WasmMsg {
    MsgStoreCode,
    MsgMigrateCode,
    MsgInstantiateContract,
    MsgExecuteContract {
        sender: String,
        contract: String,
        msg: String,
        sent_funds: Coins,
    },
    MsgMigrateContract,
    MsgUpdateContractAdmin,
    MsgClearContractAdmin,
}

impl WasmMsg {
    pub fn to_data(self) -> JsonValue {
        match self {
            WasmMsg::MsgExecuteContract {
                sender,
                contract,
                msg,
                sent_funds,
            } => object!{
                "type":"wasm/MsgExecuteContract",
                "value":{
                    "contract": contract,
                    "msg": msg,
                    "sender": sender,
                    "sent_funds": sent_funds.to_string(),
                }
            },
            _ => object!(),
        }
    }
}
