use color_eyre::eyre::Result;
use hex;
use json::{JsonValue, object};
use crate::client::{lcdutils::LCDUtils};
use super::base::BaseApi;

#[derive(Clone)]
pub struct WasmAPI {
    utils: LCDUtils,
    api: BaseApi,
}

impl WasmAPI {
    pub fn new(api: BaseApi, utils: LCDUtils) -> WasmAPI{
        WasmAPI{
            utils,
            api,
        }
    }

    pub async fn list_code_info(&self) -> Result<JsonValue>{
        Ok(self.api.get("/wasm/code".to_string(), None).await?)
    }

    pub async fn code_info(&self, code_id: i32) -> Result<JsonValue>{
        Ok(self.api.get(format!("/wasm/code/{}", code_id),None).await?)
    }

    pub async fn list_contracts_by_code_id(&self, code_id: i32) -> Result<JsonValue>{
        Ok(self.api.get(format!("/wasm/code/{}/contracts", code_id),None).await?)
    }

    pub async fn contract_info(&self, contract_address: String) -> Result<JsonValue> {
        Ok(self.api.get(format!("/wasm/contracts/{}", contract_address), None).await?)
    }

    pub async fn contract_hash_by_code_id(&self, code_id: i32) -> Result<JsonValue> {
        Ok(self.api.get(format!("/wasm/code/{}/hash", code_id), None).await?)
    }

    pub async fn contract_hash(&self, contract_address: String) -> Result<JsonValue> {
        self.api
            .get(format!("/wasm/contract/{}/code-hash", contract_address), None)
            .await
    }

    pub async fn contract_query(
        &self, 
        contract_address: String, 
        query: JsonValue, 
        height: Option<i32>,
        contract_hash: Option<String>,
    ) -> Result<JsonValue> {
        let query_str = json::stringify(query);
        let contract_code_hash = if let Some(hash) = contract_hash{
            hash.clone()
        }else{
            self.clone().contract_hash(contract_address.clone()).await?.to_string()
        };
        let encrypted = self.utils.encrypt(contract_code_hash, query_str, None).await?;
        let nonce:[u8;32] = encrypted[0..32].try_into().unwrap();
        let encoded = hex::encode(base64::encode(encrypted));
        let query_path = if let Some(height) = height{
            format!("/wasm/contract/{}/query/{}?encoding=hex&height={}", contract_address, encoded, height)
        }else{
            format!("/wasm/contract/{}/query/{}?encoding=hex&height={}", contract_address, encoded, 0)
        };
        let query_res = self.api.get(query_path, None).await?;
        let encoded_result = base64::decode(query_res["smart"].to_string())?;
        let decrypted = self.utils.decrypt(encoded_result, nonce, None).await?;
        Ok(json::parse(&String::from_utf8(base64::decode(decrypted)?)?)?)
    }
}
