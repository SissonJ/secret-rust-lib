pub mod client;
pub mod core;
pub mod key;
pub mod util;

#[cfg(test)]
mod tests {
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
            client: LCDClient::new("https://api.scrt.network".to_string()),
        };
        print!(
            "{}",
            aw!(api.contract_hash("secret1qfql357amn448duf5gvp9gr48sxx9tsnhupu3d".to_string()))
        );
        assert!(false);
    }
}
