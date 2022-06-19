use markets::market_client::MarketClient;
use markets::MarketDataRequest;
use std::str::FromStr;

pub mod markets {
    tonic::include_proto!("markets");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = MarketClient::connect("http://[::1]:50051").await?;

    loop {
        let request = tonic::Request::new(MarketDataRequest {
            market: String::from_str("ethbtc").unwrap(),
        });
        let response = client.get_market_data(request).await?;
        println!("RESPONSE={:?}", response);
    }
    Ok(())
}
