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
        let api = BaseApi::new("https://api.scrt.network".to_string());
        let seed = Some([194, 222, 240, 8, 33, 102, 225, 115, 223, 158, 59, 145, 59, 61, 25, 252, 176, 255, 243, 104, 47, 126, 238, 157, 185, 53, 72, 71, 78, 217, 39, 15]);
        let expected: [u8;32] = [202, 167, 130, 49, 225, 204, 33, 145, 241, 40, 158, 162, 118, 182, 22, 174, 82, 182, 106, 242, 53, 96, 235, 29, 29, 62, 72, 21, 71, 26, 149, 56];
        let client = LCDClient::new(&api, seed);
        assert_eq!(client.utils.pubkey.as_bytes().clone(), expected);
    }

    #[test]
    pub fn generate_tx_encryption_key_from_seed() {
        let api = BaseApi::new("https://api.scrt.network".to_string());
        let seed = [194, 139, 49, 159, 31, 125, 35, 193, 75, 49, 211, 236, 80, 124, 106, 130, 44, 147, 102, 5, 119, 125, 229, 121, 173, 0, 112, 147, 193, 136, 167, 179];
        let client = LCDClient::new(&api, Some([94, 248, 52, 125, 0, 67, 25, 156, 142, 153, 162, 53, 86, 28, 149, 1, 238, 231, 83, 108, 15, 103, 149, 6, 21, 10, 225, 28, 135, 121, 22, 213]));
        assert_eq!(aw!(client.utils.get_tx_encryption_key(seed)).unwrap(), [62, 184, 192, 184, 224, 255, 76, 58, 234, 35, 226, 187, 90, 157, 208, 153, 216, 77, 153, 180, 231, 81, 60, 154, 97, 55, 22, 118, 67, 115, 225, 117]);
    }

    #[test]
    fn consensus_io_pubkey() {
        let api = BaseApi::new("https://api.scrt.network".to_string());
        let mut client = LCDClient::new(&api, None);
        assert_eq!(
            aw!(client.utils.get_consensus_io_pubkey()).unwrap(), 
            [8, 59, 26, 3, 102, 18, 17, 213, 164, 204, 141, 57, 167, 119, 149, 121, 88, 98, 247, 115, 6, 69, 87, 59, 43, 204, 44, 25, 32, 197, 60, 4]
        )
    }

    #[test]
    fn encrypt() {
        let api = BaseApi::new("https://api.scrt.network".to_string());
        let key_seed = Some([93, 148, 94, 235, 139, 187, 191, 197, 127, 54, 210, 113, 209, 160, 73, 132, 44, 26, 39, 166, 129, 226, 178, 176, 185, 182, 24, 89, 11, 244, 21, 130]);
        let seed = Some([119, 24, 5, 30, 221, 221, 111, 134, 134, 86, 158, 102, 74, 238, 4, 79, 26, 173, 219, 190, 210, 152, 93, 222, 155, 7, 209, 37, 229, 110, 113, 214]);
        let mut client = LCDClient::new(&api, key_seed);
        let cyphertext = aw!(client.utils.encrypt(
            String::from("20a015a72cb7892680814a88308b76275c06fec7ecc7c0bcd55d0f87ee071591"), 
            "{\"get_config\":{}}".to_string(), 
            seed
        )).unwrap();
        assert_eq!(cyphertext, [119, 24, 5, 30, 221, 221, 111, 134, 134, 86, 158, 102, 74, 238, 4, 79, 26, 173, 219, 190, 210, 152, 93, 222, 155, 7, 209, 37, 229, 110, 113, 214, 102, 230, 34, 50, 34, 249, 58, 54, 210, 251, 120, 157, 252, 216, 55, 157, 176, 148, 249, 143, 125, 247, 178, 199, 156, 1, 62, 160, 166, 213, 180, 101, 53, 48, 58, 40, 79, 77, 229, 105, 67, 188, 100, 76, 167, 181, 184, 168, 123, 93, 124, 60, 55, 112, 229, 71, 163, 192, 1, 4, 156, 109, 145, 158, 110, 141, 13, 186, 98, 177, 196, 234, 226, 8, 197, 233, 2, 36, 250, 79, 137, 167, 30, 76, 121, 142, 219, 124, 19, 239, 185, 36, 204, 201, 100, 39, 178, 124, 160, 16, 189, 195, 7, 248, 201, 53, 111, 200, 93, 11, 254, 85, 27, 192, 200, 31, 63, 128, 192, 154, 16, 227, 155, 98, 136, 239, 234, 193, 96])
    }
}
