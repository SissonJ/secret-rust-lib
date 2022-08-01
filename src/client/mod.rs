pub mod api;
pub mod lcdclient;
pub mod lcdutils;

mod tests {
    use json::object;

    use super::{lcdutils::LCDUtils, api::{wasm::WasmAPI, base::BaseApi}, lcdclient::LCDClient};

    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    pub fn generate_key_from_seed() {
        let seed = Some([194, 222, 240, 8, 33, 102, 225, 115, 223, 158, 59, 145, 59, 61, 25, 252, 176, 255, 243, 104, 47, 126, 238, 157, 185, 53, 72, 71, 78, 217, 39, 15]);
        let expected: [u8;32] = [202, 167, 130, 49, 225, 204, 33, 145, 241, 40, 158, 162, 118, 182, 22, 174, 82, 182, 106, 242, 53, 96, 235, 29, 29, 62, 72, 21, 71, 26, 149, 56];
        let client = LCDUtils::new("https://api.scrt.network".to_string(), seed);
        assert_eq!(client.pubkey.as_bytes().clone(), expected);
    }

    #[test]
    pub fn contract_query() {
        let mut client = WasmAPI{
            client: LCDClient::new("https://api.scrt.network".to_string(), None),
            api: BaseApi::new("https://api.scrt.network".to_string())
        };
        let query = object!{get_config:{}};
        aw!(client.contract_query("secret1axw6cl0sg7htg8klpnnr88hvyhlw40tfrwsa98".to_string(),query,None));
        assert!(false);
    }
}
