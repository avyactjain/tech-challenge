use tungstenite::{connect, Message};
use url::Url;


struct Binance {
    socket: websocket,
}

impl Binance {
    fn get_orderbook_websocket() -> websocket {
        let (mut socket, response) =
            connect(Url::parse("wss://data.alpaca.markets/stream").unwrap())
                .expect("Can't connect");
    }
}
