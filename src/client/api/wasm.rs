use std::collections::HashMap;
use crate::client::lcdclient::LCDClient;
use color_eyre::eyre::Result;

#[derive(Clone)]
pub struct WasmAPI {
    pub client: LCDClient,
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
        query: HashMap<String, String>, 
        height: Option<i32>
    ) -> Result<HashMap<String, String>> {
        let query_str = format!("{:?}", query);
        let contract_code_hash = if let Some(hash) = self.clone().contract_hash(contract_address.clone()).await?.get("result"){
            hash.clone()
        }else{String::from("")};
        let encrypted = self.client.utils.encrypt(contract_code_hash, query_str).await?;
        Ok(HashMap::new())
    }
}
