pub mod client;
pub mod core;
pub mod key;
pub mod util;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::client::api::base::BaseApi;
    use crate::client::api::wasm::WasmAPI;
    use crate::client::lcdclient::LCDClient;
    use tokio_test;

    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn contract_hash() {
        let api = WasmAPI {
            client: LCDClient::new("https://api.scrt.network".to_string(), None),
            api: BaseApi::new("https://api.scrt.network".to_string())
        };
        print!(
            "{:?}",
            aw!(api.contract_hash("secret1qfql357amn448duf5gvp9gr48sxx9tsnhupu3d".to_string()))
        );
        assert!(true);
    }

    #[test]
    fn consensus_io_pubkey() {
        let mut client = LCDClient::new("https://api.scrt.network".to_string(), None);
        assert_eq!(
            aw!(client.utils.get_consensus_io_pubkey()).unwrap(), 
            [8, 59, 26, 3, 102, 18, 17, 213, 164, 204, 141, 57, 167, 119, 149, 121, 88, 98, 247, 115, 6, 69, 87, 59, 43, 204, 44, 25, 32, 197, 60, 4]
        )
    }

    #[test]
    fn encrypt() {
        let key_seed = Some([93, 148, 94, 235, 139, 187, 191, 197, 127, 54, 210, 113, 209, 160, 73, 132, 44, 26, 39, 166, 129, 226, 178, 176, 185, 182, 24, 89, 11, 244, 21, 130]);
        let seed = Some([119, 24, 5, 30, 221, 221, 111, 134, 134, 86, 158, 102, 74, 238, 4, 79, 26, 173, 219, 190, 210, 152, 93, 222, 155, 7, 209, 37, 229, 110, 113, 214]);
        let mut client = LCDClient::new("https://api.scrt.network".to_string(), key_seed);
        let cyphertext = aw!(client.utils.encrypt(
            String::from("20a015a72cb7892680814a88308b76275c06fec7ecc7c0bcd55d0f87ee071591"), 
            "{\"get_config\":{}}".to_string(), 
            seed
        )).unwrap();
        assert_eq!(client.utils.pubkey.as_bytes().clone(), [102, 230, 34, 50, 34, 249, 58, 54, 210, 251, 120, 157, 252, 216, 55, 157, 176, 148, 249, 143, 125, 247, 178, 199, 156, 1, 62, 160, 166, 213, 180, 101]);
        assert_eq!(cyphertext, [119, 24, 5, 30, 221, 221, 111, 134, 134, 86, 158, 102, 74, 238, 4, 79, 26, 173, 219, 190, 210, 152, 93, 222, 155, 7, 209, 37, 229, 110, 113, 214, 102, 230, 34, 50, 34, 249, 58, 54, 210, 251, 120, 157, 252, 216, 55, 157, 176, 148, 249, 143, 125, 247, 178, 199, 156, 1, 62, 160, 166, 213, 180, 101, 53, 48, 58, 40, 79, 77, 229, 105, 67, 188, 100, 76, 167, 181, 184, 168, 123, 93, 124, 60, 55, 112, 229, 71, 163, 192, 1, 4, 156, 109, 145, 158, 110, 141, 13, 186, 98, 177, 196, 234, 226, 8, 197, 233, 2, 36, 250, 79, 137, 167, 30, 76, 121, 142, 219, 124, 19, 239, 185, 36, 204, 201, 100, 39, 178, 124, 160, 16, 189, 195, 7, 248, 201, 53, 111, 200, 93, 11, 254, 85, 27, 192, 200, 31, 63, 128, 192, 154, 16, 227, 155, 98, 136, 239, 234, 193, 96])
    }
}
