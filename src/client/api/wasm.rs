use std::collections::HashMap;
use crate::client::lcdclient::LCDClient;
use color_eyre::eyre::Result;
use hex_literal::hex;
use json::JsonValue;
use super::base::BaseApi;

#[derive(Clone)]
pub struct WasmAPI {
    pub client: LCDClient,
    pub api: BaseApi,
}

impl WasmAPI {
    pub async fn contract_hash(self, contract_address: String) -> Result<HashMap<String, String>> {
        self.client
            .get(format!("/wasm/contract/{}/code-hash", contract_address))
            .await
    }

    pub async fn contract_query(
        &mut self, 
        contract_address: String, 
        query: JsonValue, 
        height: Option<i32>
    ) -> Result<HashMap<String, String>> {
        let query_str = json::stringify(query);
        let contract_code_hash = if let Some(hash) = self.clone().contract_hash(contract_address.clone()).await?.get("result"){
            hash.clone()
        }else{String::from("")};
        let encrypted = self.client.utils.encrypt(contract_code_hash, query_str, None).await?;
        let nonce:[u8;32] = encrypted[0..32].try_into().unwrap();
        let encoded = base64::encode(encrypted);
        let query_path = if let Some(height) = height{
            format!("/wasm/contract/{}/query/{}?encoding=hex&height={}", contract_address, encoded, height)
        }else{
            format!("/wasm/contract/{}/query/{}?encoding=hex&height={}", contract_address, encoded, 0)
        };
        let query_res = self.api.get(query_path).await?;
        println!("{}", query_res);
        Ok(HashMap::new())
    }
}
