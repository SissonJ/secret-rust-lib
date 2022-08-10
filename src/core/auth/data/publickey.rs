pub struct PublicKey {
    pub key_type: PubKeyType,
    pub value: String,
}

pub enum PubKeyType {
    Simple,
    Multisig,
}