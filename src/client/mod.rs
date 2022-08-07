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
        let seed = [194, 139, 49, 159, 31, 125, 35, 193, 75, 49, 211, 236, 80, 124, 106, 130, 44, 147, 102, 5, 119, 125, 229, 121, 173, 0, 112, 147, 193, 136, 167, 179];
        let client = LCDUtils::new("https://api.scrt.network".to_string(), Some([94, 248, 52, 125, 0, 67, 25, 156, 142, 153, 162, 53, 86, 28, 149, 1, 238, 231, 83, 108, 15, 103, 149, 6, 21, 10, 225, 28, 135, 121, 22, 213]));
        assert_eq!(aw!(client.get_tx_encryption_key(seed)).unwrap(), [62, 184, 192, 184, 224, 255, 76, 58, 234, 35, 226, 187, 90, 157, 208, 153, 216, 77, 153, 180, 231, 81, 60, 154, 97, 55, 22, 118, 67, 115, 225, 117]);
    }

    #[test]
    pub fn contract_query() {
        let mut client = WasmAPI{
            client: LCDClient::new("https://api.scrt.network".to_string(), None),
            api: BaseApi::new("https://api.scrt.network".to_string())
        };
        let query = object!{get_config:{}};
        let res = aw!(client.contract_query("secret1axw6cl0sg7htg8klpnnr88hvyhlw40tfrwsa98".to_string(),query,None)).unwrap();
        assert_eq!(res.dump(), String::from("{\"config\":{\"config\":{\"admin\":\"secret1j7gwujpne2dl8s0t34jduvd7ptq03g8z4f634p\",\"shd_token\":{\"address\":\"secret1qfql357amn448duf5gvp9gr48sxx9tsnhupu3d\",\"code_hash\":\"FA824C4504F21FC59250DA0CDF549DD392FD862BAF2689D246A07B9E941F04A9\"},\"silk_token\":{\"address\":\"secret1qfql357amn448duf5gvp9gr48sxx9tsnhupu3d\",\"code_hash\":\"FA824C4504F21FC59250DA0CDF549DD392FD862BAF2689D246A07B9E941F04A9\"},\"sscrt_token\":{\"address\":\"secret1k0jntykt7e4g3y88ltc60czgjuqdy4c9e8fzek\",\"code_hash\":\"af74387e276be8874f07bec3a87023ee49b0e7ebe08178c49d0a49c3c98ed60e\"},\"treasury\":{\"address\":\"secret1jqpw9mtzhwytgauetkwdayt3wpry7wgdq8xgsz\",\"code_hash\":\"\"},\"payback_rate\":\"0.15\"}}}"));
    }
}
