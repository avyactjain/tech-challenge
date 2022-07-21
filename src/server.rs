use tonic::{transport::Server, Request, Response, Status};

use markets::market_server::{Market, MarketServer};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

use markets::{MarketDataRequest, MarketDataResponse};

use config::Config;

use crate::exchanges::binance::Binance;
use crate::exchanges::bitstamp::Bitstamp;

mod exchanges;

// use crate::orderbook::{orderbook_raw};
mod orderbook;

pub mod markets {
    tonic::include_proto!("markets");
}

#[derive(Debug, Default)]
pub struct MarketService {}

#[tonic::async_trait]
impl Market for MarketService {
    type marketDataStream = ReceiverStream<Result<MarketDataResponse, Status>>;

    async fn market_data(
        &self,
        request: Request<MarketDataRequest>,
    ) -> Result<Response<Self::marketDataStream>, Status> {
        // println!("ListFeatures = {:?}", request);

        let (tx, rx) = mpsc::channel(4);
        //use the same transmitter for both exchanges.

        let tx_clone = tx.clone(); // sent to bitstamp
        // Binance::init_orderbook_websocket(tx); //this will spawn a new thread, and start sending to binance_rx
        Bitstamp::init_orderbook_websocket(tx_clone);

        // rx will receive whatever tx will send.
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
