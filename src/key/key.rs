use super::raw::RawKey;
use crate::core::auth::data::{
    publickey::{PubKeyType, PublicKey},
    tx::{StdSignMsg, StdSignature, StdTx},
};

pub struct Key {
    pub raw_key: RawKey,
}

impl Key {
    pub fn create_signature(&self, tx: StdSignMsg) -> StdSignature {
        let sig = self.raw_key.sign(tx.to_data().dump().as_bytes().to_vec());
        StdSignature {
            signature: base64::encode(sig),
            pub_key: PublicKey {
                key_type: PubKeyType::Simple,
                value: base64::encode(self.raw_key.public_key.serialize()),
            },
        }
    }

    pub fn sign_tx(&self, tx: StdSignMsg) -> StdTx {
        StdTx{
            signatures: vec![self.create_signature(tx.clone())],
            memo: tx.memo,
            fee: tx.fee,
            msg: tx.msgs,
        }
    }
}
