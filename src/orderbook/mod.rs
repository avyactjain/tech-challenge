//imports
pub mod binance_orderbook;
pub mod local_level;

use self::local_level::LocalLevel;

#[derive(Debug)]
pub struct Orderbook {
    pub bids: Vec<LocalLevel>,
    pub asks: Vec<LocalLevel>,
    pub last_update_id: u64,
}
