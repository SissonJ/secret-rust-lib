pub mod base;
pub mod wasm;


#[cfg(test)]
mod tests {
    use crate::client::lcdclient::LCDClient;
    use json::object;
    use tokio_test;

    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn list_code_info() {
        let client = LCDClient::new("https://api.scrt.network".to_string(), None);
        let res = aw!(client.wasm.list_code_info()).unwrap();
        println!("{:?}", res[0]);
        assert!(false);
    }

    #[test]
    fn contract_hash() {
        let client = LCDClient::new("https://api.scrt.network".to_string(), None);
        let res = aw!(client.wasm.contract_hash("secret1qfql357amn448duf5gvp9gr48sxx9tsnhupu3d".to_string())).unwrap().to_string();
        assert_eq!("fa824c4504f21fc59250da0cdf549dd392fd862baf2689d246a07b9e941f04a9", res);
    }

    #[test]
    pub fn contract_query() {
        let mut client = LCDClient::new("https://api.scrt.network".to_string(), None);
        let query = object!{get_config:{}};
        let res = aw!(client.wasm.contract_query("secret1axw6cl0sg7htg8klpnnr88hvyhlw40tfrwsa98".to_string(),query,None,None)).unwrap();
        assert_eq!(res.dump(), String::from("{\"config\":{\"config\":{\"admin\":\"secret1j7gwujpne2dl8s0t34jduvd7ptq03g8z4f634p\",\"shd_token\":{\"address\":\"secret1qfql357amn448duf5gvp9gr48sxx9tsnhupu3d\",\"code_hash\":\"FA824C4504F21FC59250DA0CDF549DD392FD862BAF2689D246A07B9E941F04A9\"},\"silk_token\":{\"address\":\"secret1qfql357amn448duf5gvp9gr48sxx9tsnhupu3d\",\"code_hash\":\"FA824C4504F21FC59250DA0CDF549DD392FD862BAF2689D246A07B9E941F04A9\"},\"sscrt_token\":{\"address\":\"secret1k0jntykt7e4g3y88ltc60czgjuqdy4c9e8fzek\",\"code_hash\":\"af74387e276be8874f07bec3a87023ee49b0e7ebe08178c49d0a49c3c98ed60e\"},\"treasury\":{\"address\":\"secret1jqpw9mtzhwytgauetkwdayt3wpry7wgdq8xgsz\",\"code_hash\":\"\"},\"payback_rate\":\"0.15\"}}}"));
    }
}

