use json::{JsonValue, object};
use crate::core::auth::coins::Coins;

pub enum WasmMsg{
    MsgStoreCode,
    MsgMigrateCode,
    MsgInstantiateContract,
    MsgExecuteContract{
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
    pub fn to_data(self)->JsonValue{
        match self {
            WasmMsg::MsgExecuteContract { 
                sender, 
                contract, 
                msg, 
                sent_funds 
            } => object!(
                "sender": sender,
                "contract": contract,
                "msg": msg,
                "sent_funds": sent_funds.to_string(),
            ),
            _ => object!()
        }
    }
}