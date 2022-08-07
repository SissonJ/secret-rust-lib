use base64;
use color_eyre::eyre::Result;
use rand_core::{OsRng, RngCore};
use super::api::base::BaseApi;
use x25519_dalek::{StaticSecret, PublicKey};
use sha2::Sha256;
use hkdf::Hkdf;
use hex_literal::hex;
use aes_siv::{
    aead::{KeyInit, generic_array::GenericArray},
    siv::Aes128Siv,
};

#[derive(Clone)]

pub struct LCDUtils {
    pub seed: u32,
    r: OsRng,
    privkey: StaticSecret,
    pub pubkey: PublicKey,
    api: BaseApi,
}

impl LCDUtils {
    pub fn new(url: String, seed: Option<[u8; 32]>) -> LCDUtils {
        let r = OsRng{};
        let privkey = if let Some(seed) = seed {
            StaticSecret::from(seed)
        }else{
            StaticSecret::new(r)
        };
        let pubkey = PublicKey::from(&privkey.clone());
        LCDUtils { 
            seed: 0, 
            r,
            privkey, 
            pubkey, 
            api: BaseApi::new(url),
        }
    }

    pub fn generate_new_seed(&mut self) -> [u8; 32] {
        let mut seed = [0u8; 32];
        self.r.fill_bytes(&mut seed);
        seed
    }

    pub async fn get_consensus_io_pubkey(&self) -> Result<[u8;32]> {
        let io_exch_pubkey = self.api.get(String::from("/reg/tx-key")).await?["result"]["TxKey"].clone();
        let consensus_io_pubkey = base64::decode(&format!("{}",io_exch_pubkey))?;
        Ok(consensus_io_pubkey.try_into().unwrap()) //find out how to get rid of this 
    }

    pub async fn get_tx_encryption_key(&self, nonce: [u8; 32]) -> Result<[u8; 32]> {
        let bytes = self.get_consensus_io_pubkey().await?; //TODO Figure out how to get rid of this unwrap
        let consensus_io_pubkey = PublicKey::from(bytes);
        let tx_encryption_ikm = self.privkey.diffie_hellman(&consensus_io_pubkey).as_bytes().clone();
        let master_secret = [tx_encryption_ikm, nonce.try_into().unwrap()].concat();
        let salt = hex!("000000000000000000024bead8df69990852c202db0e0097c1a12ea637d7e96d");
        let hk = Hkdf::<Sha256>::new(Some(&salt[..]), &master_secret);
        let mut okm = [0u8; 32];
        hk.expand(&[], &mut okm).unwrap(); // get rid of this one too
        Ok(okm)
    }
    
    pub async fn encrypt(&mut self, contract_code_hash: String, msg: String, nonce: Option<[u8;32]>) -> Result<Vec<u8>> {
        let seed = if let Some(nonce) = nonce{
            nonce
        }else{
            self.generate_new_seed()
        };
        let mut return_vec = [seed, self.pubkey.as_bytes().clone()].concat();
        let tx_encryption_key = self.get_tx_encryption_key(seed).await?;
        let arr = GenericArray::from(tx_encryption_key);
        
        //let siv = Aes128SivAead::new(&arr);
        let mut siv = Aes128Siv::new(&arr);
        let plaintext = contract_code_hash.clone() + &msg;
        let mut cyphertext = siv.encrypt(&[&[0u8;0]], plaintext.as_bytes()).unwrap();
        return_vec.append(&mut cyphertext);
        Ok(return_vec)
    }

    pub async fn decrypt(&self, cyphertext: Vec<u8>, nonce: [u8;32], tx_encryption_key: Option<[u8;32]> ) -> Result<Vec<u8>> {
        let arr = if let Some(tx_encryption_key) = tx_encryption_key {
            GenericArray::from(tx_encryption_key)
        } else {
            GenericArray::from(self.get_tx_encryption_key(nonce).await?)
        };
        let mut siv = Aes128Siv::new(&arr);
        let plaintext = siv.decrypt(&[&[0u8;0]], cyphertext.as_ref()).unwrap();
        Ok(plaintext)
    }
}