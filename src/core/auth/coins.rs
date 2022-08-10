use super::coin::Coin;

pub struct Coins{
    pub coins: Vec<Coin>,
}

impl Coins {
    pub fn to_string(&self) -> String {
        let mut res = "[ ".to_string();
        for coin in &self.coins {
            res = res.clone() + &coin.to_string() + ", ";
        }
        res + "]"
    }
}