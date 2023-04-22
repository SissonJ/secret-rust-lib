use json::{object, JsonValue};
use color_eyre::eyre::Result;
use crate::core::auth::data::tx::StdTx;
use super::base::BaseApi;

#[derive(Clone)]
pub struct TxApi<'a> {
    api: &'a BaseApi,
}

impl<'a> TxApi<'a> {
    pub fn new(api: &'a BaseApi) -> TxApi {
        TxApi { api, }
    }

    pub async fn broadcast(&self, tx: StdTx, mode: BroadcastOptions) -> Result<JsonValue> {
        let data = object! {"tx": tx.to_data()["value"].clone(), "mode": "sync"};
        Ok(self.api.post("/txs".to_string(), data.dump(), Some(true)).await?)
    }
}

pub enum BroadcastOptions {
    Sync
}