## Running the Server

```
# Over HTTP
cargo run --bin server
# or using HTTPS
cargo run --bin sserver
```

## Running the Client

1. Update the destination
2. Run the lcient

```
# Normal (insecure) Websocket client
cargo run --bin client
# Secure (TLS) Websocket Client 
cargo run --bin sclient
```