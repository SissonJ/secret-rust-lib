use base64;
use color_eyre::eyre::Result;
use rand_core::{OsRng, RngCore};
use super::api::base::BaseApi;
use x25519_dalek::{StaticSecret, PublicKey};
use sha2::Sha256;
use hkdf::Hkdf;
use hex_literal::hex;
use aes_siv::{
    aead::{Aead, KeyInit, generic_array::GenericArray},
    Aes256SivAead, Nonce // Or `Aes128SivAead`
};

#[derive(Clone)]

pub struct LCDUtils {
    pub seed: u32,
    r: OsRng,
    privkey: StaticSecret,
    pubkey: PublicKey,
    api: BaseApi,
}

impl LCDUtils {
    pub fn new(url: String) -> LCDUtils {
        let r = OsRng{};
        let privkey = StaticSecret::new(r);
        let pubkey = PublicKey::from(&privkey.clone());
        LCDUtils { 
            seed: 0, 
            r,
            privkey, 
            pubkey, 
            api: BaseApi::new(url),
        }
    }

    pub fn generate_new_seed(&mut self) -> Vec<u8> {
        let mut seed = vec![0u8; 32];
        self.r.fill_bytes(&mut seed);
        seed
    }

    pub async fn get_consensus_io_pubkey(&self) -> Result<[u8;32]> {
        let io_exch_pubkey = if let Some(key) = self.api.get(String::from("/reg/tx-key")).await?.get("TxKey"){
            key.clone()
        }else{String::from("")};
        let consensus_io_pubkey = base64::decode(io_exch_pubkey)?;
        Ok(consensus_io_pubkey.try_into().unwrap()) //find out how to get rid of this 
    }

    pub async fn get_tx_encryption_key(&self, nonce: Vec<u8>) -> Result<[u8; 42]> {
        let bytes = self.get_consensus_io_pubkey().await?; //TODO Figure out how to get rid of this unwrap
        let consensus_io_pubkey = PublicKey::from(bytes);
        let tx_encryption_ikm = self.privkey.diffie_hellman(&consensus_io_pubkey).as_bytes().clone();
        let master_secret = [tx_encryption_ikm, nonce.try_into().unwrap()].concat();
        let salt = hex!("000000000000000000024bead8df69990852c202db0e0097c1a12ea637d7e96d");
        let hk = Hkdf::<Sha256>::new(Some(&salt[..]), &master_secret);
        let mut okm = [0u8; 42];
        hk.expand(&[], &mut okm).unwrap(); // get rid of this one too
        Ok(okm)
    }
    
    pub async fn encrypt(&mut self, contract_code_hash: String, msg: String) -> Result<String> {
        let seed = self.generate_new_seed();
        let tx_encryption_key = self.get_tx_encryption_key(seed).await?;
        
        let arr = GenericArray::from(tx_encryption_key);
        
        let siv = Aes256SivAead::new(&GenericArray<u8,42>::from(tx_encryption_key));
        Ok("".to_string())
    }
}