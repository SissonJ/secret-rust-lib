use crate::core::auth::wasm;
use json::JsonValue;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Msg {
    WasmMsg(wasm::msgs::WasmMsg),
}

impl Msg {
    pub fn to_data(self) -> JsonValue {
        match self {
            Msg::WasmMsg(msg) => msg.to_data(),
        }
    }
}
