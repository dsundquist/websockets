use std::{
    thread,
    time::Duration,
    net::TcpStream,
    sync::Arc,
};
use rustls::{client::danger::ServerCertVerifier, ClientConfig, ClientConnection, RootCertStore, StreamOwned};
use tungstenite::Message;
use rustls_pki_types::ServerName;

#[derive(Debug)]
struct CertVerifier;

impl ServerCertVerifier for CertVerifier {
        fn verify_server_cert(
        &self,
        _end_entity: &rustls_pki_types::CertificateDer<'_>,
        _intermediates: &[rustls_pki_types::CertificateDer<'_>],
        _server_name: &rustls_pki_types::ServerName<'_>,
        _ocsp_response: &[u8],
        _now: rustls_pki_types::UnixTime,
    ) -> Result<rustls::client::danger::ServerCertVerified, rustls::Error> {
        println!("⚠️  Warning: Accepting invalid certificate");
        Ok(rustls::client::danger::ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &rustls_pki_types::CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &rustls_pki_types::CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
        // Allow all
        vec![
            rustls::SignatureScheme::RSA_PKCS1_SHA1,
            rustls::SignatureScheme::ECDSA_SHA1_Legacy,
            rustls::SignatureScheme::RSA_PKCS1_SHA256,
            rustls::SignatureScheme::ECDSA_NISTP256_SHA256,
            rustls::SignatureScheme::RSA_PKCS1_SHA384,
            rustls::SignatureScheme::ECDSA_NISTP384_SHA384,
            rustls::SignatureScheme::RSA_PKCS1_SHA512,
            rustls::SignatureScheme::ECDSA_NISTP521_SHA512,
            rustls::SignatureScheme::RSA_PSS_SHA256,
            rustls::SignatureScheme::RSA_PSS_SHA384,
            rustls::SignatureScheme::RSA_PSS_SHA512,
            rustls::SignatureScheme::ED25519,
            rustls::SignatureScheme::ED448,
        ]
    }
}

fn main() {
    let dst_ip = "127.0.0.1";
    let dst_port = "9001";
    let dst_socket = format!("{}:{}", dst_ip, dst_port);

    // Connect raw TCP
    let tcp_stream = TcpStream::connect(&dst_socket).expect("Failed to connect to TCP");

    let root_store = RootCertStore::empty();
    // Optionally, load root certificates here if needed

    let mut config = ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    // Just cannot be bothered with certificates (for debugging...)
    let verifier = Arc::new(CertVerifier{});
    config.dangerous().set_certificate_verifier(verifier);

    let config = Arc::new(config);

    let server_name = ServerName::try_from(dst_ip).expect("Invalid DNS name");
    let conn = ClientConnection::new(config, server_name).expect("Failed to create TLS connection");
    let tls_stream = StreamOwned::new(conn, tcp_stream);
    
    let url = format!("wss://{}", dst_socket);
    let (mut socket, response) = tungstenite::client(url, tls_stream).unwrap();

    println!("Connected to server: {:?}", response);

        loop {
        // Read and print messages from server
        if let Ok(msg) = socket.read() {
            println!("Received: {}", msg);

            // Reply with acknowledgment
            socket
                .send(Message::Text("Received timestamp".into()))
                .unwrap();
        }

        thread::sleep(Duration::from_millis(500));
    }
}
