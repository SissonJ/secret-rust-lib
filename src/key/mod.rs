pub mod key;
pub mod mnemonic;
pub mod raw;

mod test {
    use super::super::core::auth::data::tx::StdSignMsg;
    use super::super::core::auth::data::tx::StdSignature;
    use super::{key::Key, raw::RawKey};
    use crate::core::auth::coin::Coin;
    use crate::core::auth::coins::Coins;
    use crate::core::auth::data::tx::StdFee;

    #[test]
    fn sign() {
        let key = RawKey::new([
            93, 148, 94, 235, 139, 187, 191, 197, 127, 54, 210, 113, 209, 160, 73, 132, 44, 26, 39,
            166, 129, 226, 178, 176, 185, 182, 24, 89, 11, 244, 21, 130,
        ]);
        assert_eq!(
            key.sign(vec![
                19, 65, 136, 178, 152, 103, 204, 219, 183, 56, 101, 234, 244, 98, 186, 1, 166, 66,
                177, 232, 52, 1, 38, 132, 158, 254, 87, 41, 117, 189, 221, 116
            ]),
            [
                89, 212, 137, 109, 130, 251, 170, 5, 110, 16, 0, 227, 208, 19, 199, 162, 250, 65,
                156, 53, 141, 128, 221, 149, 105, 109, 160, 147, 94, 61, 120, 220, 122, 69, 77,
                131, 94, 192, 29, 41, 243, 92, 40, 166, 109, 161, 178, 107, 170, 231, 26, 125, 230,
                110, 21, 253, 207, 252, 180, 42, 124, 108, 1, 42
            ]
        )
    }

    #[test]
    fn sign_mnemonic() {
        let key = RawKey::from_mnemonic("region best impose vital letter champion hover journey multiply utility waste share glow inspire require vendor credit motion ribbon law attitude comic trouble scan".to_string());
        assert_eq!(
            key.sign(vec![
                20, 118, 49, 227, 86, 147, 147, 152, 228, 116, 236, 236, 224, 237, 43, 152, 140,
                35, 238, 205, 84, 97, 186, 37, 188, 87, 42, 103, 30, 231, 214, 70
            ]),
            [
                236, 98, 108, 166, 111, 249, 63, 116, 55, 191, 45, 95, 159, 241, 138, 103, 21, 171,
                142, 190, 245, 91, 39, 117, 0, 202, 245, 28, 164, 22, 83, 190, 92, 201, 133, 205,
                200, 102, 94, 60, 217, 108, 13, 242, 202, 25, 63, 130, 122, 47, 141, 185, 134, 249,
                2, 178, 234, 184, 60, 8, 152, 32, 250, 164
            ]
        )
    }

    #[test]
    fn create_signature() {
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
        println!("{}", tx.to_data().dump());
        let sig = key.create_signature(tx);
        assert_eq!(
            sig.signature,
                "hVYXSc4ZvRF7kN8cM1NgHoTIqcgnD/P+/djgMBQty38eQnU0P5N7Aux9OX/0hubURgpHBANO9azLqpHNPl67kA==",
        );
    }
}
