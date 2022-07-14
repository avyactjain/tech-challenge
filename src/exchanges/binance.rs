use tungstenite::{connect, Message};
use url::Url;

pub struct Binance {
    // socket: websocket,
}

impl Binance {
    pub fn get_orderbook_websocket() -> tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>> {
        let (mut socket, response) =
            connect(Url::parse("wss://stream.binance.com:9443/ws/ethbtc@depth20@100ms").unwrap())
                .expect("Can't connect ");

        return socket;

        // loop {
        //     let msg = socket.read_message().expect("Error reading message");
        //     println!("Received: {}", msg);
        // }
    }

    // pub fn test() {
    //     let (mut socket, response) =
    //         connect(Url::parse("wss://stream.binance.com:9443/ws/ethbtc@depth20@100ms").unwrap())
    //             .expect("Can't connect");

    //     println!("Connected to the server");
    //     println!("Response HTTP code: {}", response.status());
    //     println!("Response contains the following headers:");
    //     for (ref header, _value) in response.headers() {
    //         println!("* {}", header);
    //     }

    //     socket
    //         .write_message(Message::Text("Hello, Test!".into()))
    //         .unwrap();
    //     loop {
    //         let msg = socket.read_message().expect("Error reading message");
    //         println!("Received: {}", msg);
    //     }
    // }
}

pub fn print_hello_from_binance() {
    println!("hello from binance");
}
