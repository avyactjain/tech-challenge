use std::collections::HashMap;

use tonic::{transport::Server, Request, Response, Status};

use markets::market_server::{Market, MarketServer};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

use markets::{Level, MarketDataRequest, MarketDataResponse};

use config::Config;

use crate::exchanges::binance::Binance;

mod exchanges;
use crate::orderbook::Orderbook;
mod orderbook;

pub mod markets {
    tonic::include_proto!("markets");
}

#[derive(Debug, Default)]
pub struct MarketService {}

#[tonic::async_trait]
impl Market for MarketService {
    async fn get_market_data(
        &self,
        request: Request<MarketDataRequest>,
    ) -> Result<Response<MarketDataResponse>, Status> {
        println!("Got a request: {:?}", request);

        let req = request.into_inner();

        let reply = MarketDataResponse {
            bids: [Level {
                exchange: "Binance".to_string(),
                amount: 1.2,
                price: 2.3,
            }]
            .to_vec(),
            asks: [Level {
                exchange: "Binance".to_string(),
                amount: 1.2,
                price: 2.3,
            }]
            .to_vec(),
        };

        Ok(Response::new(reply))
    }

    type marketDataStream = ReceiverStream<Result<MarketDataResponse, Status>>;

    async fn market_data(
        &self,
        request: Request<MarketDataRequest>,
    ) -> Result<Response<Self::marketDataStream>, Status> {
        println!("ListFeatures = {:?}", request);

        let (tx, rx) = mpsc::channel(4);

        let mut binance_socket = Binance::get_orderbook_websocket();

        tokio::spawn(async move {
            // place websocket streaming here.

            loop {
                //create marketDataResponse from Orderbook Object, as soon as I get a response. Convert that response to OB structure, then use that structure to create MarketDataResponse object.
                let msg = binance_socket
                    .read_message()
                    .expect("Error reading message");

                let _orderbook: Orderbook = serde_json::from_str(msg.to_string().as_str())
                    .unwrap_or_else(|error| {
                        panic!("Error while parsing orderbook as JSON. Error {}", error);
                    });

                println!("{:?}", _orderbook.bids);

                let mut _bid_levels: Vec<Level> = Vec::new();
                let mut _ask_levels: Vec<Level> = Vec::new();

                for _bid in _orderbook.bids.iter() {
                    let _x = Level {
                        exchange: "binance".to_string(),
                        amount: _bid[0].parse().unwrap(),
                        price: _bid[1].parse().unwrap(),
                    };

                    _bid_levels.push(_x);
                }

                for _ask in _orderbook.asks.iter() {
                    let _x = Level {
                        exchange: "binance".to_string(),
                        amount: _ask[0].parse().unwrap(),
                        price: _ask[1].parse().unwrap(),
                    };

                    _ask_levels.push(_x);
                }

                tx.send(Ok(MarketDataResponse {
                    bids: _bid_levels,
                    asks: _ask_levels,
                }))
                .await
                .unwrap();
            }
            println!(" /// done sending");
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: Config = Config::load_config("./config.json");

    let market = &config.markets[0];

    println!("base: {}", market[0]);
    println!("quote: {}", market[1]);

    // println!("{}",config.market);

    let addr = "[::1]:50051".parse()?;
    let market_service = MarketService::default();
    println!("market-server active on {:?}", addr);

    Server::builder()
        .add_service(MarketServer::new(market_service))
        .serve(addr)
        .await?;

    Ok(())
}
