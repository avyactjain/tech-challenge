use tonic::{transport::Server, Request, Response, Status};

use markets::market_server::{Market, MarketServer};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

use markets::{MarketDataRequest, MarketDataResponse};

use config::Config;

use crate::exchanges::binance::Binance;

mod exchanges;

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

        // let req = request.into_inner();

        let reply = MarketDataResponse { price: 1.024 };

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
                tx.send(Ok(MarketDataResponse { price: 1.024 }))
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
