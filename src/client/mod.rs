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
    pub fn generate_tx_encryption_key_from_seed() {
        let seed = [227, 97, 56, 56, 47, 57, 217, 64, 113, 225, 93, 25, 66, 70, 198, 0, 160, 41, 0, 93, 35, 222, 139, 78, 141, 23, 21, 129, 25, 189, 220, 241];
        let client = LCDUtils::new("https://api.scrt.network".to_string(), Some([212, 136, 255, 246, 160, 167, 202, 224, 126, 102, 92, 236, 165, 223, 158, 125, 2, 22, 145, 224, 66, 170, 95, 147, 150, 59, 80, 50, 77, 98, 229, 152]));
        println!("tx_ecnryption_key: {:?}", aw!(client.get_tx_encryption_key(seed)));
        assert!(false);
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
