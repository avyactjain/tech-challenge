use serde::{Deserialize, Serialize};

use super::{local_level::LocalLevel, Orderbook};

//derive the traits for your struct
#[derive(Debug, Deserialize, Serialize)]

pub struct BinanceOrderbook {
    pub bids: Vec<Vec<String>>,
    pub asks: Vec<Vec<String>>,
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: u64,
}

impl BinanceOrderbook {
    pub fn convert_raw_orderbook_to_orderbook(
        raw_orderbook: BinanceOrderbook,
        exchange: &str,
    ) -> Orderbook {
        // add error handling here
        let mut bids: Vec<LocalLevel> = Vec::new();
        let mut asks: Vec<LocalLevel> = Vec::new();

        for bid in raw_orderbook.bids {
            let temp_level = LocalLevel {
                price: bid.get(0).unwrap().parse().unwrap(),
                amount: bid.get(1).unwrap().parse().unwrap(),
                exchange: exchange.to_string(),
            };

            bids.push(temp_level);
        }

        for ask in raw_orderbook.asks {
            let temp_level = LocalLevel {
                price: ask.get(0).unwrap().parse().unwrap(),
                amount: ask.get(1).unwrap().parse().unwrap(),
                exchange: "binance".to_string(),
            };

            asks.push(temp_level);
        }

        Orderbook {
            bids: bids,
            asks: asks,
            last_update_id: raw_orderbook.last_update_id,
        }
    }
}
