use std::collections::HashMap;

use crate::{
    markets::Level,
    orderbook::{local_level::LocalLevel, orderbook_raw::OrderbookRaw},
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

            println!("message from bitstamp {:?}", message);

            let response: BitstampConnectionMessage =
                serde_json::from_str(&message.to_string()).unwrap();

            if (response.event == "bts:subscription_succeeded") {
                loop {
                    println!("inside the loop");
                    let message_new = socket.read_message().expect("Error reading message");

                    let _orderbook: BitstampOrderbook =
                        serde_json::from_str(&message_new.to_string()).unwrap_or_else(|error| {
                            panic!("Error while parsing orderbook as JSON. Error {}", error);
                        });

                    println!("message from bitstamp {:?}", _orderbook);
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

impl Bitstamp {
    
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
