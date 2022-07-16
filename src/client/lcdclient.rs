use reqwest;

pub struct LCDClient {
    url: String,
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
    pub fn create(url: String) -> LCDClient {
        LCDClient {
            url,
        }
    }

    pub fn wallet() {
        println!("I will return a wallet one day");
    }

    fn get(&self, endpoint: String) -> None {
        let result = reqwest::get("https://api.spotify.com/v1/search");
    }
}