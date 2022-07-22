use crate::{
    markets::Level,
    orderbook::{local_level::LocalLevel, orderbook_raw::OrderbookRaw},
};

use crate::MarketDataResponse;
use tungstenite::connect;
use url::Url;

pub struct Binance {
    // socket: websocket,
}

impl Binance {
    pub fn get_orderbook_websocket(
    ) -> tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>> {
        let (socket, _response) =
            connect(Url::parse("wss://stream.binance.com:9443/ws/ethbtc@depth20@100ms").unwrap())
                .expect("Can't connect to binance websocket");

        return socket;
    }

    pub fn init_orderbook_websocket(
        tx: tokio::sync::mpsc::Sender<Result<MarketDataResponse, tonic::Status>>,
    ) {
        println!("Binance Websocket initialized");
        //add error handling in this function
        let mut socket = Binance::get_orderbook_websocket();
        tokio::spawn(async move {
            loop {
                let msg = socket.read_message().expect("Error reading message");

                let message = match msg {
                    tungstenite::Message::Text(_) => msg,
                    tungstenite::Message::Binary(_) => todo!(),
                    tungstenite::Message::Ping(_) => todo!(),
                    tungstenite::Message::Pong(_) => todo!(),
                    tungstenite::Message::Close(_) => todo!(),
                    tungstenite::Message::Frame(_) => todo!(),
                };

                let _orderbook: OrderbookRaw = serde_json::from_str(&message.to_string())
                    .unwrap_or_else(|error| {
                        panic!("Error while parsing orderbook as JSON. Error {}", error);
                    });

                let orderbook =
                    OrderbookRaw::convert_raw_orderbook_to_orderbook(_orderbook, "binance");
                // println!("order book is --> {:?}", orderbook);

                let mut _asks: Vec<Level> = Vec::new();
                let mut _bids: Vec<Level> = Vec::new();

                for bid in orderbook.bids {
                    let temp_level = LocalLevel::conver_local_level_to_proto_level(bid);
                    _bids.push(temp_level);
                }

                for ask in orderbook.asks {
                    let temp_level = LocalLevel::conver_local_level_to_proto_level(ask);
                    _asks.push(temp_level);
                }

                tx.send(Ok(MarketDataResponse {
                    bids: _bids,
                    asks: _asks,
                }))
                .await
                .unwrap();
            }
        });
        //this function will tokio spawn an independent thread.
    }
}
