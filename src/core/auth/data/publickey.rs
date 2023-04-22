use json::{JsonValue, object};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct PublicKey {
    pub key_type: PubKeyType,
    pub value: String,
}

impl PublicKey {
    pub fn to_data(&self) -> JsonValue {
        match self.key_type {
            PubKeyType::Simple => object!{
                "type": "tendermint/PubKeySecp256k1",
                "value": self.value.clone(),
            },
            PubKeyType::Multisig => object!{
                "type": "tendermint/PubKeyMultisigThreshold",
                "value": self.value.clone(),
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PubKeyType {
    Simple,
    Multisig,
}