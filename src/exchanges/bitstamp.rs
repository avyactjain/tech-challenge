use std::collections::HashMap;

use crate::{
    markets::Level,
    orderbook::{local_level::LocalLevel, orderbook_raw::OrderbookRaw, Orderbook},
};

use crate::MarketDataResponse;
use serde::{Deserialize, Serialize};
use tungstenite::{connect, Message};
use url::Url;

pub struct Bitstamp {
    // socket: websocket,
}

impl Bitstamp {
    pub fn get_orderbook_websocket(
    ) -> tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>> {
        let (mut socket, _response) = connect(Url::parse("wss://ws.bitstamp.net").unwrap())
            .expect("Can't connect to Bitstamp websocket");

        socket
            .write_message(Message::Text(
                r#"{
                    "event": "bts:subscribe",
                    "data": {
                        "channel": "order_book_ethbtc"
                    }
                }"#
                .into(),
            ))
            .unwrap();

        return socket;
    }

    pub fn init_orderbook_websocket(
        tx: tokio::sync::mpsc::Sender<Result<MarketDataResponse, tonic::Status>>,
    ) {
        println!("bitstamp Websocket initialized");

        //add error handling in this function
        let mut socket = Bitstamp::get_orderbook_websocket();

        tokio::spawn(async move {
            let msg = socket.read_message().expect("Error reading message");

            let message = match msg {
                tungstenite::Message::Text(_) => msg,
                tungstenite::Message::Binary(_) => todo!(),
                tungstenite::Message::Ping(_) => todo!(),
                tungstenite::Message::Pong(_) => todo!(),
                tungstenite::Message::Close(_) => todo!(),
                tungstenite::Message::Frame(_) => todo!(),
            };

            // println!("message from bitstamp {:?}", message);

            let response: BitstampConnectionMessage =
                serde_json::from_str(&message.to_string()).unwrap();

            if response.event == "bts:subscription_succeeded" {
                loop {
                    let message_new = socket
                        .read_message()
                        .expect("Error reading message")
                        .to_string();

                    if (message_new.len() != 0) {
                        let raw_orderbook: BitstampOrderbook = serde_json::from_str(&message_new)
                            .unwrap_or_else(|error| {
                                panic!("Error while parsing orderbook as JSON. Error {}", error);
                            });
                        // println!("------------------------------------");

                        // println!("Raw OB --> {:?}", raw_orderbook);

                        let orderbook: Orderbook =
                            BitstampOrderbook::convert_bitstamp_orderbook_to_orderbook(
                                raw_orderbook,
                            );

                        // println!("message from bitstamp {:?}", orderook);

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
                }
            }
        });
        //this function will tokio spawn an independent thread.
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BitstampOrderbook {
    pub data: BitstampData,
    pub channel: String,
    pub event: String,
}

impl BitstampOrderbook {
    pub fn convert_bitstamp_orderbook_to_orderbook(bitsamp_orderbook: Self) -> Orderbook {
        let mut bids: Vec<LocalLevel> = Vec::new();
        let mut asks: Vec<LocalLevel> = Vec::new();

        for bid in bitsamp_orderbook.data.bids {
            let temp_level = LocalLevel {
                amount: bid.get(0).unwrap().parse().unwrap(),
                price: bid.get(1).unwrap().parse().unwrap(),
                exchange: "bitstamp".to_string(),
            };

            bids.push(temp_level);
        }

        for ask in bitsamp_orderbook.data.asks {
            let temp_level = LocalLevel {
                amount: ask.get(0).unwrap().parse().unwrap(),
                price: ask.get(1).unwrap().parse().unwrap(),
                exchange: "bitstamp".to_string(),
            };

            asks.push(temp_level);
        }

        Orderbook {
            bids: bids,
            asks: asks,
            last_update_id: bitsamp_orderbook.data.timestamp.parse().unwrap(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BitstampData {
    pub timestamp: String,
    pub microtimestamp: String,
    pub bids: Vec<Vec<String>>,
    pub asks: Vec<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BitstampConnectionMessage {
    pub event: String,
    pub channel: String,
    pub data: BitstampConnectionData,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BitstampConnectionData {}
