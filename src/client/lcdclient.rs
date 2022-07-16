use reqwest;

pub struct LCDClient {
    pub url: String,
    /*chain_id: Option<String>,
    gas_prices: Option<u32>, //Need to add Coins
    gas_adjustment: Option<u32>,
    custom_fees: Option<u32>, // Needs custom struct
    last_resquest_height: Option<u32>,
    auth: Option<u32>, // Needs AuthAPI
    band: Option<u32>, // Needs BankAPI
    distribution: Option<u32>, // ^
    staking: Option<u32>, // ^
    tendermint: Option<u32>, // ^
    wasm: Option<u32>, // ^
    tx: Option<u32>, // ^*/
}

impl LCDClient {
    pub fn new(url: String) -> LCDClient {
        LCDClient { url }
    }

    pub fn wallet() {
        println!("I will return a wallet one day");
    }

    pub async fn get(&self, endpoint: String) -> String {
        reqwest::get(format!("{}{}", self.url, endpoint))
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
    }

    fn post(&self, endpoint: String, data: String, raw: bool) {
        let client = reqwest::Client::new();
        client.post(self.url.clone());
    }
}
