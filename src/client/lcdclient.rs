use super::{api::{wasm::WasmAPI, base::BaseApi},lcdutils::LCDUtils};

#[derive(Clone)]

pub struct LCDClient {
    pub wasm: WasmAPI,
    pub utils: LCDUtils,
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
    pub fn new(url: String, seed: Option<[u8; 32]>) -> LCDClient {
        let api = BaseApi::new(url);
        let utils = LCDUtils::new(api.clone(), seed);
        LCDClient {
            wasm: WasmAPI::new(api, utils.clone()), 
            utils 
        }
    }

    pub fn wallet() {
        println!("I will return a wallet one day");
    }
}
