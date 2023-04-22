pub mod base;
pub mod wasm;
pub mod tx;


#[cfg(test)]
mod tests {
    use crate::{client::{lcdclient::LCDClient, api::base::BaseApi}, core::auth::{coin::Coin, coins::Coins, data::tx::{StdFee, StdSignMsg}}, key::{raw::RawKey, key::Key}};
    use json::{object, JsonValue::Null};
    use tokio_test;

    use super::tx::BroadcastOptions;

    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn list_code_info() {
        let api = BaseApi::new("https://loadbalancer1.api.securesecrets.org".to_string());
        let client = LCDClient::new(&api, None);
        let res = aw!(client.wasm.list_code_info()).unwrap();
        assert_eq!(res[0].dump(), "{\"id\":1,\"creator\":\"secret1qx5pppsfrqwlnmxj7prpx8rysxm2u5vzx6jm8a\",\"data_hash\":\"485767F5F18BE383E2CA1472C5E0C60571E98FD8852BFB52C67F2952D1F5AD91\"}");
    }

    #[test]
    fn code_info() {
        let api = BaseApi::new("https://loadbalancer1.api.securesecrets.org".to_string());
        let client = LCDClient::new(&api, None);
        let res = aw!(client.wasm.code_info(1)).unwrap();
    }

    #[test]
    fn list_contracts_by_code_id() {
        let api = BaseApi::new("https://loadbalancer1.api.securesecrets.org".to_string());
        let client = LCDClient::new(&api, None);
        let res = aw!(client.wasm.list_contracts_by_code_id(1)).unwrap();
        assert_eq!(res.dump(), "[{\"address\":\"secret18vd8fpwxzck93qlwghaj6arh4p7c5n8978vsyg\",\"code_id\":1,\"creator\":\"secret1qx5pppsfrqwlnmxj7prpx8rysxm2u5vzx6jm8a\",\"label\":\"addition\"},{\"address\":\"secret1zps48rnklauesrlprzv8mlaqef3kee5ppyxhct\",\"code_id\":1,\"creator\":\"secret1tn5gaa3pqx80l7lcfklhu3q8majtfaptcspqkt\",\"label\":\"lkmx-token\"}]");
    }

    #[test]
    fn contract_info() {
        let api = BaseApi::new("https://loadbalancer1.api.securesecrets.org".to_string());
        let client = LCDClient::new(&api, None);
        let res = aw!(client.wasm.contract_info("secret1qfql357amn448duf5gvp9gr48sxx9tsnhupu3d".to_string())).unwrap();
        assert_eq!(res.dump(), "{\"contract_address\":\"secret1qfql357amn448duf5gvp9gr48sxx9tsnhupu3d\",\"code_id\":372,\"creator\":\"secret1kra3lqvqs4qtw0ad6uyqkpkl42wahzxh0mr3nq\",\"label\":\"shade-24\"}");
    }

    #[test]
    fn contract_hash_by_code_id() {
        let api = BaseApi::new("https://loadbalancer1.api.securesecrets.org".to_string());
        let client = LCDClient::new(&api, None);
        let res = aw!(client.wasm.contract_hash_by_code_id(372)).unwrap();
        assert_eq!(res.to_string(), "fa824c4504f21fc59250da0cdf549dd392fd862baf2689d246a07b9e941f04a9");
    }

    #[test]
    fn contract_hash() {
        let api = BaseApi::new("https://loadbalancer1.api.securesecrets.org".to_string());
        let client = LCDClient::new(&api, None);
        let res = aw!(client.wasm.contract_hash("secret1qfql357amn448duf5gvp9gr48sxx9tsnhupu3d".to_string())).unwrap().to_string();
        assert_eq!("fa824c4504f21fc59250da0cdf549dd392fd862baf2689d246a07b9e941f04a9", res);
    }

    #[test]
    pub fn contract_query() {
        let api = BaseApi::new("https://loadbalancer1.api.securesecrets.org".to_string());
        let mut client = LCDClient::new(&api, None);
        let query = object!{get_config:{}};
        let res = aw!(client.wasm.contract_query("secret1axw6cl0sg7htg8klpsnnr88hvyhlw40tfrwsa98".to_string(),query,None,None, None, None, None)).unwrap();
        assert_eq!(res.dump(), String::from("{\"config\":{\"config\":{\"admin\":\"secret1j7gwujpne2dl8s0t34jduvd7ptq03g8z4f634p\",\"shd_token\":{\"address\":\"secret1qfql357amn448duf5gvp9gr48sxx9tsnhupu3d\",\"code_hash\":\"FA824C4504F21FC59250DA0CDF549DD392FD862BAF2689D246A07B9E941F04A9\"},\"silk_token\":{\"address\":\"secret1qfql357amn448duf5gvp9gr48sxx9tsnhupu3d\",\"code_hash\":\"FA824C4504F21FC59250DA0CDF549DD392FD862BAF2689D246A07B9E941F04A9\"},\"sscrt_token\":{\"address\":\"secret1k0jntykt7e4g3y88ltc60czgjuqdy4c9e8fzek\",\"code_hash\":\"af74387e276be8874f07bec3a87023ee49b0e7ebe08178c49d0a49c3c98ed60e\"},\"treasury\":{\"address\":\"secret1jqpw9mtzhwytgauetkwdayt3wpry7wgdq8xgsz\",\"code_hash\":\"\"},\"payback_rate\":\"0.15\"}}}"));
    }

    #[test]
    pub fn post() {
        let api = BaseApi::new("https://loadbalancer1.api.securesecrets.org".to_string());
        let client = LCDClient::new(&api, None);
        let key = Key {
            raw_key: RawKey::new([
                93, 148, 94, 235, 139, 187, 191, 197, 127, 54, 210, 113, 209, 160, 73, 132, 44, 26,
                39, 166, 129, 226, 178, 176, 185, 182, 24, 89, 11, 244, 21, 130,
            ]),
        };
        let tx = StdSignMsg {
            chain_id: "secret_4".to_string(),
            account_number: 1,
            fee: StdFee {
                amount: Coins {
                    coins: vec![Coin {
                        denom: "scrt".to_string(),
                        amount: 1000,
                    }],
                },
                gas: 100,
            },
            sequence: 1,
            msgs: vec![],
            memo: "".to_string(),
        };
        aw!(client.tx.broadcast(key.sign_tx(tx), BroadcastOptions::Sync));
        assert!(false);
    }

    #[test]
    pub fn execute_msg(){
        let seed = [
            93, 148, 94, 235, 139, 187, 191, 197, 127, 54, 210, 113, 209, 160, 73, 132, 44, 26,
            39, 166, 129, 226, 178, 176, 185, 182, 24, 89, 11, 244, 21, 130,
        ];
        let api = BaseApi::new("https://loadbalancer1.api.securesecrets.org".to_string());
        let client = LCDClient::new(&api, None);
        let encryptedMsgSienna = object!{
            "swap":{"expected_return":"0"}
        };
        assert_eq!(encryptedMsgSienna.dump(), r#"{"swap":{"expected_return":"0"}}"#);
        let handle_msg = object!{
            "send": {
                "recipient": "secret1axw6cl0sg7htg8klpnnr88hvyhlw40tfrwsa98", 
                "amount": "0", 
                "msg": base64::encode(encryptedMsgSienna.dump())
            }
        };
        let msg = aw!(client.wasm.contract_execute_msg(
            "secret1axw6cl0sg7htg8klpnnr88hvyhlw40tfrwsa98".to_string(), 
            "secret1axw6cl0sg7htg8klpnnr88hvyhlw40tfrwsa98".to_string(), 
            handle_msg, 
            None, 
            None, 
            Some(seed), 
            None, 
            None
        )).unwrap();
        assert_eq!(
            msg.to_data().dump(),
            r#"{"type":"wasm/MsgExecuteContract","value":{"contract":"secret1axw6cl0sg7htg8klpnnr88hvyhlw40tfrwsa98","msg":"XZRe64u7v8V/NtJx0aBJhCwaJ6aB4rKwubYYWQv0FYJm5iIyIvk6NtL7eJ382DedsJT5j333ssecAT6gptW0ZXfE6j7ECeADgQuykeQ040xUf1WFYqC9hOWjoG3+9632L6uW3QvwifCNgTnlwvG633pL95xJL6Oxq4+c3oVi1RW0OP5N7a1weERSJpOrKhVkmcyDGw4fVuCjOLtP8H2jSziwRa/VWZtct+LbyMLyBBA+h7j0mtLQjzUEqVDVrwZ7LOf99GWRPaG1pIsEcJzh+fnOp+wL5Bi3w2jFQGEAFh+taxq+A6Q07drjsm65nuzGIpgJI/TWDfvX5i86iufo7BdGVufUSpWm0u6Gjostxo751L6Q1dIuYQ==","sender":"secret1axw6cl0sg7htg8klpnnr88hvyhlw40tfrwsa98","sent_funds":[]}}"#
        )
    }
}

