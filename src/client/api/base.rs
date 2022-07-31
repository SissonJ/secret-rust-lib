use color_eyre::eyre::Result;
use std::collections::HashMap;

#[derive(Clone)]
pub struct BaseApi {
    url: String,
}

impl BaseApi {
    pub fn new(url: String) -> BaseApi {
        BaseApi { url }
    }

    pub async fn get(&self, endpoint: String) -> Result<HashMap<String,String>> {
        let res = reqwest::get(format!("{}{}", self.url, endpoint))
            .await?.json::<HashMap<String, String>>().await?;
        Ok(res)
    }

    fn post(&self, endpoint: String, data: String, raw: bool) {
        let client = reqwest::Client::new();
        client.post(self.url.clone());
    }
}
