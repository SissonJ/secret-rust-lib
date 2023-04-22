use secp256k1::{Secp256k1, PublicKey, SecretKey, Message};
use sha2::{Sha256, Digest};
use bip32::{Mnemonic, XPrv, Language };

pub struct RawKey {
    pub public_key: PublicKey,
    private_key: SecretKey,
}

impl RawKey {
    pub fn new(bytes: [u8; 32]) -> RawKey {
        let secp = Secp256k1::new();
        let private_key = SecretKey::from_slice(&bytes).unwrap();
        RawKey{
            public_key: PublicKey::from_secret_key(&secp, &private_key),
            private_key,
        }
    }

    pub fn from_mnemonic(mnemonic: String) -> RawKey { // NOT WORKING
        let seed = Mnemonic::new(mnemonic, Language::English).unwrap().to_seed("");
        let secp = Secp256k1::new();
        let child_path = "m/44'/0'/0'/0/0";
        let child_xprv = XPrv::derive_from_path(&seed, &child_path.parse().unwrap()).unwrap();
        let private_key = SecretKey::from_slice(child_xprv.private_key().to_bytes().as_slice()).unwrap();
        RawKey { public_key: PublicKey::from_secret_key(&secp, &private_key), private_key, }
    }

    pub fn sign(&self, payload: Vec<u8>) -> [u8;64] {
        let secp = Secp256k1::new();
        let mut hasher = Sha256::new();
        hasher.update(payload);
        secp.sign_ecdsa(&Message::from_slice(&hasher.finalize()).unwrap(), &self.private_key).serialize_compact()
    }
}