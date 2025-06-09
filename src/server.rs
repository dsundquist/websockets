use std::net::TcpListener;
use std::thread;
use std::time::{Duration, Instant};
use tungstenite::accept;
use tungstenite::Message;

fn main() {
    let server = TcpListener::bind("0.0.0.0:9001").expect("Failed to bind server"); 

    println!("WebSocket server started on ws://0.0.0.0:9001");

    for stream in server.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    let mut websocket = accept(stream).expect("Error accepting WebSocket connection");
                    println!("Client connected!");

                    let start_time = Instant::now(); // Track when the connection started

                    loop {
                        let elapsed = start_time.elapsed();
                        let minutes = elapsed.as_secs() / 60;
                        let seconds = elapsed.as_secs() % 60;

                        let message = format!("Connection Duration: {} min {} sec", minutes, seconds);
                        let message = tungstenite::Bytes::from(message);
                        websocket.send(Message::Binary(message)).unwrap();

                        thread::sleep(Duration::from_secs(1));

                        // Check if client is still responding
                        if let Ok(msg) = websocket.read() {
                            println!("Client says: {}", msg);
                        }
                    }
                });
            }
            Err(e) => println!("Connection failed: {}", e),
        }
    }
}
