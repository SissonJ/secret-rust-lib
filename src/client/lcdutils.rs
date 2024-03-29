use base64;
use color_eyre::eyre::Result;
use json::JsonValue;
use rand_core::{OsRng, RngCore};
use super::api::base::BaseApi;
use x25519_dalek::{StaticSecret, PublicKey};
use sha2::Sha256;
use hkdf::Hkdf;
use hex_literal::hex;
use aes_siv::{
    aead::{KeyInit, generic_array::GenericArray},
    siv::{Aes128Siv, Siv},
    
};

#[derive(Clone)]

pub struct LCDUtils<'a> {
    pub seed: [u8; 32],
    r: OsRng,
    privkey: StaticSecret,
    pub pubkey: PublicKey,
    api: &'a BaseApi,
    pub network_pubkey: Option<[u8; 32]>
}

impl<'a> LCDUtils<'a> {
    pub fn new(api: &'a BaseApi, seed: Option<[u8; 32]>) -> LCDUtils {
        let r = OsRng{};
        let key_seed;
        let privkey = if let Some(seed) = seed {
            key_seed = seed;
            StaticSecret::from(seed)
        }else{
            key_seed = [0u8; 32]; //TODO
            StaticSecret::new(r)
        };
        let pubkey = PublicKey::from(&privkey.clone());
        LCDUtils { 
            seed: key_seed, 
            r,
            privkey, 
            pubkey, 
            api,
            network_pubkey: None,
        }
    }

    pub fn generate_new_seed(&self) -> [u8; 32] {
        let mut seed = [0u8; 32];
        self.clone().r.fill_bytes(&mut seed);
        seed
    }

    pub async fn get_consensus_io_pubkey(&self) -> Result<[u8;32]> {
        let io_exch_pubkey = self.api.get(String::from("/reg/tx-key"), None).await?["TxKey"].clone();
        let consensus_io_pubkey = base64::decode(&format!("{}",io_exch_pubkey))?;
        Ok(consensus_io_pubkey.try_into().unwrap()) //find out how to get rid of this 
    }

    pub async fn get_tx_encryption_key(&self, nonce: [u8; 32], network_pubkey: Option<[u8; 32]>) -> Result<[u8; 32]> {
        let bytes = if let Some(key) = network_pubkey {
            key
        } else {
            self.get_consensus_io_pubkey().await? //TODO Figure out how to get rid of this unwrap
        };
        let consensus_io_pubkey = PublicKey::from(bytes);
        let tx_encryption_ikm = self.privkey.diffie_hellman(&consensus_io_pubkey).as_bytes().clone();
        let master_secret = [tx_encryption_ikm, nonce.try_into().unwrap()].concat();
        let salt = hex!("000000000000000000024bead8df69990852c202db0e0097c1a12ea637d7e96d");
        let hk = Hkdf::<Sha256>::new(Some(&salt[..]), &master_secret);
        let mut okm = [0u8; 32];
        hk.expand(&[], &mut okm).unwrap(); // get rid of this one too
        Ok(okm)
    }

    pub fn get_tx_encryption_key_sync(&self, nonce: [u8; 32], network_pubkey: [u8; 32]) -> Result<[u8; 32]> {
        let consensus_io_pubkey = PublicKey::from(network_pubkey);
        let tx_encryption_ikm = self.privkey.diffie_hellman(&consensus_io_pubkey).as_bytes().clone();
        let master_secret = [tx_encryption_ikm, nonce.try_into().unwrap()].concat();
        let salt = hex!("000000000000000000024bead8df69990852c202db0e0097c1a12ea637d7e96d");
        let hk = Hkdf::<Sha256>::new(Some(&salt[..]), &master_secret);
        let mut okm = [0u8; 32];
        hk.expand(&[], &mut okm).unwrap(); // get rid of this one too
        Ok(okm)
    }
    
    pub async fn encrypt(
        &self, 
        contract_code_hash: String, 
        msg: String, 
        nonce: Option<[u8;32]>, 
        tx_encryption_key: Option<[u8;32]>, 
        network_pubkey: Option<[u8; 32]>
    ) -> Result<Vec<u8>> {
        let seed = if let Some(nonce) = nonce{
            nonce
        }else{
            self.generate_new_seed()
        };
        let mut return_vec = [seed, self.pubkey.as_bytes().clone()].concat();
        let tx_encryption_key = if let Some(key) = tx_encryption_key {
            key
        } else {
            self.get_tx_encryption_key(seed, network_pubkey).await?
        };
        let arr = GenericArray::from(tx_encryption_key);
        
        let mut siv = Aes128Siv::new(&arr);
        let plaintext = contract_code_hash.clone() + &msg;
        let mut cyphertext = siv.encrypt(&[&[0u8;0]], plaintext.as_bytes()).unwrap();
        return_vec.append(&mut cyphertext);
        Ok(return_vec)
    }

    pub fn encrypt_sync(
        &self, 
        contract_code_hash: String, 
        msg: String, 
        nonce: [u8;32], 
        network_pubkey: [u8; 32]
    ) -> Result<(Vec<u8>, [u8;32])>{
        let mut return_vec = [nonce, self.pubkey.as_bytes().clone()].concat();
        let tx_encryption_key = self.get_tx_encryption_key_sync(nonce, network_pubkey)?;

        let arr = GenericArray::from(tx_encryption_key);
        
        let mut siv = Aes128Siv::new(&arr);
        let plaintext = contract_code_hash.clone() + &msg;
        let mut cyphertext = siv.encrypt(&[&[0u8;0]], plaintext.as_bytes()).unwrap();
        return_vec.append(&mut cyphertext);
        Ok((return_vec, tx_encryption_key))
    }

    pub async fn decrypt(&self, cyphertext: Vec<u8>, nonce: [u8;32], tx_encryption_key: Option<[u8;32]>, network_pubkey: Option<[u8; 32]> ) -> Result<Vec<u8>> {
        let arr = if let Some(tx_encryption_key) = tx_encryption_key {
            GenericArray::from(tx_encryption_key)
        } else {
            GenericArray::from(self.get_tx_encryption_key(nonce, network_pubkey).await?)
        };
        let mut siv = Aes128Siv::new(&arr);
        let plaintext = siv.decrypt(&[&[0u8;0]], cyphertext.as_ref()).unwrap();
        Ok(plaintext)
    }

    pub fn decrypt_sync(
        &self, 
        input: String, 
        tx_encryption_key: [u8;32],
    ) -> Result<JsonValue> {
        let cyphertext = base64::decode(input)?;
        let arr = GenericArray::from(tx_encryption_key);
        let mut siv = Aes128Siv::new(&arr);
        let plaintext = siv.decrypt(&[&[0u8;0]], cyphertext.as_ref()).unwrap();
        Ok(json::parse(&String::from_utf8(base64::decode(plaintext)?)?)?)
    }

    pub async fn decrypt_data_field(&self, data_field: String, nonces: Vec<[u8 ; 32]>) -> Result<Vec<u8>> { //TODO!
        let wasm_output_data_cipher_bz = data_field.as_bytes();
        Ok(vec![])
    }
}