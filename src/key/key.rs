use crate::core::auth::data::{tx::{StdSignMsg, StdSignature}, publickey::{PublicKey, PubKeyType}};
use super::raw::RawKey;

pub struct Key {
    pub raw_key: RawKey,
}

impl Key {
    pub fn create_signature(&self, tx: StdSignMsg)->StdSignature{
        println!();
        let sig = self.raw_key.sign(tx.to_data().dump().as_bytes().to_vec());
        StdSignature { 
            signature: base64::encode(sig), 
            pub_key: PublicKey{
                key_type: PubKeyType::Simple,
                value: base64::encode(self.raw_key.public_key.serialize())
            } 
        }
    }
}