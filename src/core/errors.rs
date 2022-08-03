pub enum SecretError {
    Bech32Error(String),
    Error(String),
}