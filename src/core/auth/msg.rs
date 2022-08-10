use json::JsonValue;
use crate::core::auth::wasm;

pub enum Msg{
    WasmMsg(wasm::msgs::WasmMsg),
}

impl Msg {
    pub fn to_data(self) -> JsonValue {
        match self {
            Msg::WasmMsg(msg) => msg.to_data()
        }
    }
}