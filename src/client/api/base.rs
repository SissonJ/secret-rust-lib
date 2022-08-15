use color_eyre::eyre::Result;
use json::JsonValue;

#[derive(Clone)]
pub struct BaseApi {
    url: String,
}

impl BaseApi {
    pub fn new(url: String) -> BaseApi {
        BaseApi { url }
    }

    pub async fn get(&self, endpoint: String, raw: Option<bool>) -> Result<JsonValue> {
        println!("{}{}", self.url, endpoint);
        let res = reqwest::get(format!("{}{}", self.url, endpoint))
            .await?.text().await?;
        if raw == Some(true) {
            return Ok(json::parse(&res)?);
        }
        println!("{}", res);
        Ok(json::parse(&res)?["result"].clone())
    }

    fn post(&self, endpoint: String, data: String, raw: bool) {
        let client = reqwest::Client::new();
        client.post(self.url.clone());
    }
}
