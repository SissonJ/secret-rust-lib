use color_eyre::eyre::Result;
use json::JsonValue;

#[derive(Clone)]
pub struct BaseApi {
    url: String,
    client: reqwest::Client,
    pub last_request_height: i32,
}

impl BaseApi {
    pub fn new(url: String) -> BaseApi {
        BaseApi { url, client: reqwest::Client::new(), last_request_height: 0 }
    }

    pub async fn get(&self, endpoint: String, raw: Option<bool>) -> Result<JsonValue> {
        let res = self.client.get(format!("{}{}", self.url, endpoint))
            .send().await?.text().await?;
        if raw == Some(true) {
            return Ok(json::parse(&res)?);
        }
        Ok(json::parse(&res)?["result"].clone())
    }

    pub async fn post(&self, endpoint: String, data: String, raw: Option<bool>) -> Result<JsonValue> {
        let res = self.client.post(format!("{}{}",self.url,endpoint))
            .body(data).send().await?.text().await?;
        println!("{}", res);
        if raw == Some(true) {
            return Ok(json::parse(&res)?);
        }
        Ok(json::parse(&res)?["result"].clone())
    }
}
