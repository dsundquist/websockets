use std::thread;
use std::time::Duration;
use tungstenite::{connect, Message};
use url::Url;


fn main() {
    let url = Url::parse("wss://127.0.0.1:9001").expect("Invalid WebSocket URL");

    let (mut socket, response) = connect(url).expect("Failed to connect to server");

    println!("Connected to server: {:?}", response);

    loop {
        // Read and print messages from server
        if let Ok(msg) = socket.read() {
            println!("Received: {}", msg);

            // Reply with acknowledgment
            socket.send(Message::Text("Received timestamp".into())).unwrap();
        }

        thread::sleep(Duration::from_millis(500));
    }
}