use tonic::{transport::Server, Request, Response, Status};

use markets::market_server::{Market, MarketServer};
use markets::{MarketDataRequest, MarketDataResponse};

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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let market_service = MarketService::default();
    println!("market-server active on {:?}", addr);

    Server::builder()
        .add_service(MarketServer::new(market_service))
        .serve(addr)
        .await?;

    Ok(())
}
