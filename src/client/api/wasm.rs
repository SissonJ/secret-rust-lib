use crate::client::lcdclient::LCDClient;

pub struct WasmAPI {
    pub client: LCDClient,
}

impl WasmAPI {
    pub async fn contract_hash(self, contract_address: String) -> String {
        self.client
            .get(format!("/wasm/contract/{}/code-hash", contract_address))
            .await
    }
}
