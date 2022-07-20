//imports
pub mod local_level;
pub mod orderbook_raw;

use self::local_level::LocalLevel;

#[derive(Debug)]
pub struct Orderbook {
    pub bids : Vec<LocalLevel>,
    pub asks : Vec<LocalLevel>,
    pub last_update_id: u64,
}
