// use std::io::{BufReader};
use std::net::TcpListener;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use rustls::server::ServerConfig;
use rustls_pki_types::pem::PemObject;
use tungstenite::Message;

fn load_from_file(filename: &str) -> String {
    std::fs::read_to_string(&filename)
        .unwrap_or_else(|e| panic!("Failed to read {:?}: {}", filename, e))
}

fn certs_from_pem_string(pem: &str) -> Vec<rustls_pki_types::CertificateDer<'static>> {
    use std::io::Cursor;
    let mut cursor = Cursor::new(pem.as_bytes());
    rustls_pemfile::certs(&mut cursor)
        .map(|result| result.expect("Failed to parse certificate"))
        .map(rustls_pki_types::CertificateDer::from)
        .collect()
}

fn main() {
    // Update these if necessary: 
    let cert_dir_path = "/home/hans/.mitm/";
    let cert_filename = "ws.example.com.crt";
    let key_filename = "ws.example.com.key";
    let socket_str = "127.0.0.1:9001"; // Where the server will listen

    // Public X509 Certificate
    let cert = certs_from_pem_string(load_from_file(format!("{}{}", cert_dir_path, cert_filename).as_str()).as_str());
    
    // Private Key
    let key = rustls_pki_types::PrivateKeyDer::from_pem_file(format!("{}{}", cert_dir_path, key_filename)).unwrap();

    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert, key)
        .unwrap();
    
    let config = Arc::new(config);

    let server = TcpListener::bind(socket_str).expect("Failed to bind server");
    println!("Secure WebSocket server started on wss://{}", socket_str);

    for stream in server.incoming() {
        match stream {
            Ok(stream) => {
                let config = config.clone();
                thread::spawn(move || {
                    let tls_stream = rustls::StreamOwned::new(
                        rustls::ServerConnection::new(config).unwrap(),
                        stream,
                    );
                    let mut websocket = tungstenite::accept(tls_stream).expect("Error accepting WebSocket connection");
                    println!("Client connected!");

                    let start_time = Instant::now();

                    loop {
                        let elapsed = start_time.elapsed();
                        let minutes = elapsed.as_secs() / 60;
                        let seconds = elapsed.as_secs() % 60;

                        let message = format!("Connection Duration: {} min {} sec", minutes, seconds);
                        let message = tungstenite::Bytes::from(message);
                        websocket.send(Message::Binary(message)).unwrap();

                        thread::sleep(Duration::from_secs(1));

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