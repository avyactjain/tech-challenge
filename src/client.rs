use markets::market_client::MarketClient;
use markets::MarketDataRequest;
use std::str::FromStr;

pub mod markets {
    tonic::include_proto!("markets");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = MarketClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(MarketDataRequest {
        market: String::from_str("ethbtc").unwrap(),
    });

    let mut stream = client.market_data(request).await?.into_inner();

    while let Some(data) = stream.message().await? {
        println!("market_data = {:?}", data);
    }

    loop {
        // let request = tonic::Request::new(MarketDataRequest {
        //     market: String::from_str("ethbtc").unwrap(),
        // });
        // let response = client.get_market_data(request).await?;
        // println!("RESPONSE={:?}", response);

        println!("{:?}",stream.message().await?);
    }
    Ok(())
}
