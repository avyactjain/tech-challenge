//imports
use serde::{Deserialize, Serialize};

//derive the traits for your struct
#[derive(Debug, Deserialize, Serialize)]

pub struct Orderbook {
    pub bids: Vec<Vec<String>>,
    pub asks: Vec<Vec<String>>,
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: u64,
}

impl Orderbook {}
