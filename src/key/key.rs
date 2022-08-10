use super::raw::RawKey;
use crate::core::auth::data::{
    publickey::{PubKeyType, PublicKey},
    tx::{StdSignMsg, StdSignature},
};

pub struct Key {
    pub raw_key: RawKey,
}

impl Key {
    pub fn create_signature(&self, tx: StdSignMsg) -> StdSignature {
        println!("{:?}", tx.to_data().dump().as_bytes().to_vec());
        assert_eq!(
            r#"{"account_number":"1","chain_id":"secret_4","fee":{"amount":[{"amount":"1000","denom":"scrt"}],"gas":"100"},"memo":"","msgs":[],"sequence":"1"}"#,
            tx.to_data().dump()
        );
        let sig = self.raw_key.sign(tx.to_data().dump().as_bytes().to_vec());
        StdSignature {
            signature: base64::encode(sig),
            pub_key: PublicKey {
                key_type: PubKeyType::Simple,
                value: base64::encode(self.raw_key.public_key.serialize()),
            },
        }
    }
}
