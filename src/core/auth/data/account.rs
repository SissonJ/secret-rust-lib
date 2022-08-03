use serde::{Serializable, Deserialize};

#[derive(Serializable, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
struct Account {
    }
