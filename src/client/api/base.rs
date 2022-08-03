use color_eyre::eyre::Result;
use json::JsonValue;
use std::collections::HashMap;

#[derive(Clone)]
pub struct BaseApi {
    url: String,
}

impl BaseApi {
    pub fn new(url: String) -> BaseApi {
        BaseApi { url }
    }

    pub async fn get(&self, endpoint: String) -> Result<JsonValue> {
        println!("{}{}", self.url, endpoint);
        let res = reqwest::get(format!("{}{}", self.url, endpoint))
            .await?.text().await?;
        println!("{:?}", res);
        Ok(json::parse(&res)?)
    }

    fn post(&self, endpoint: String, data: String, raw: bool) {
        let client = reqwest::Client::new();
        client.post(self.url.clone());
    }
}
