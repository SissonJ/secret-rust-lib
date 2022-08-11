use super::{api::{wasm::WasmAPI, tx::TxApi, base::BaseApi},lcdutils::LCDUtils};

#[derive(Clone)]

pub struct LCDClient<'b> {
    pub wasm: WasmAPI<'b>,
    pub utils: LCDUtils<'b>,
    pub tx: TxApi<'b>,
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

impl<'b> LCDClient<'b> {
    pub fn new(api: &'b BaseApi, seed: Option<[u8; 32]>) -> LCDClient<'b> {
        LCDClient {
            wasm: WasmAPI::new(api, LCDUtils::new(api, seed)), 
            utils: LCDUtils::new(api, seed), 
            tx: TxApi::new(api)
        }
    }

    pub fn wallet() {
        println!("I will return a wallet one day");
    }
}
