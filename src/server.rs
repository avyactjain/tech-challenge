use OrderbookAggregator::OrderbookAggregator::aggregate_orderbooks_from_recievers;
use tonic::{transport::Server, Request, Response, Status};

use markets::market_server::{Market, MarketServer};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

use markets::{MarketDataRequest, MarketDataResponse};

use config::Config;

use crate::exchanges::binance::Binance;
use crate::exchanges::bitstamp::Bitstamp;

mod exchanges;
mod OrderbookAggregator;

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

        let (tx_binance, rx_binance) = mpsc::channel(1);
        // let (tx_bitstamp, rx_bitstamp) = mpsc::channel(2);


        //pass in both the receivers and let it do it's job
        aggregate_orderbooks_from_recievers();



        Binance::init_orderbook_websocket(tx_binance); //this will spawn a new thread, and start sending orderbooks from binance to rx
                                                       // Bitstamp::init_orderbook_websocket(tx_clone); //this will spawn a new thread and start sending orderbooks from bitstamp to rx

        // Bitstamp::init_orderbook_websocket(tx_bitstamp);
        // rx will receive whatever tx will send.

        //this will be changed to receiver for aggregator
        Ok(Response::new(ReceiverStream::new(rx_binance)))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: Config = Config::load_config("./config.json");

    let market = &config.markets[0];

    println!("base: {}", market[0]);
    println!("quote: {}", market[1]);

    // println!("{}",config.market);

    let addr = "[::1]:50050".parse()?;
    let market_service = MarketService::default();
    println!("market-server active on {:?}", addr);

    Server::builder()
        .add_service(MarketServer::new(market_service))
        .serve(addr)
        .await?;

    Ok(())
}
