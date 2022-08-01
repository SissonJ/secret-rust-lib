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
        #[derive(Debug)]
        struct get_config {};
        let key_seed = Some([176, 171, 233, 54, 29, 113, 164, 92, 182, 181, 117, 119, 92, 206, 145, 55, 29, 94, 54, 225, 224, 130, 230, 249, 158, 100, 165, 221, 247, 248, 77, 202]);
        let seed = Some([209, 154, 208, 239, 12, 6, 4, 195, 15, 116, 81, 232, 114, 146, 188, 147, 201, 142, 137, 42, 158, 108, 202, 26, 163, 167, 40, 245, 125, 25, 191, 183]);
        let mut client = LCDClient::new("https://api.scrt.network".to_string(), key_seed);
        let cyphertext = aw!(client.utils.encrypt(
            String::from("20a015a72cb7892680814a88308b76275c06fec7ecc7c0bcd55d0f87ee071591"), 
            format!("{:?}", {get_config{}}), 
            seed
        )).unwrap();
        assert_eq!(client.utils.pubkey.as_bytes().clone(), [30, 244, 175, 35, 135, 3, 24, 23, 99, 54, 137, 85, 60, 250, 185, 84, 239, 164, 42, 37, 174, 162, 173, 28, 126, 139, 174, 182, 70, 59, 111, 74]);
        assert_eq!(cyphertext, [209, 154, 208, 239, 12, 6, 4, 195, 15, 116, 81, 232, 114, 146, 188, 147, 201, 142, 137, 42, 158, 108, 202, 26, 163, 167, 40, 245, 125, 25, 191, 183, 30, 244, 175, 35, 135, 3, 24, 23, 99, 54, 137, 85, 60, 250, 185, 84, 239, 164, 42, 37, 174, 162, 173, 28, 126, 139, 174, 182, 70, 59, 111, 74, 71, 58, 141, 47, 9, 17, 77, 5, 211, 210, 207, 143, 126, 131, 244, 107, 81, 233, 44, 167, 121, 182, 34, 172, 123, 82, 38, 187, 42, 82, 102, 222, 75, 202, 132, 183, 186, 147, 119, 88, 146, 120, 127, 35, 224, 129, 9, 137, 37, 83, 233, 64, 217, 199, 145, 133, 65, 248, 52, 217, 107, 34, 121, 6, 167, 9, 111, 30, 79, 11, 78, 19, 43, 111, 101, 20, 120, 9, 29, 233, 94, 58, 241, 152, 232, 130, 246, 218, 13, 12, 195, 63, 15, 126, 76, 47, 72])
    }
}
