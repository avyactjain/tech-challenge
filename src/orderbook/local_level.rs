use crate::markets::Level;

#[derive(Debug)]
pub struct LocalLevel {
    pub exchange: String,
    pub amount: f64,
    pub price: f64,
}

impl LocalLevel {
    pub fn conver_local_level_to_proto_level(local_level: LocalLevel) -> Level {
        // To Do : Add error handling here.
        Level {
            exchange: local_level.exchange,
            price: local_level.price,
            amount: local_level.amount,
        }
    }
}
