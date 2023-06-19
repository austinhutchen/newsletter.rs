use newsletter::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    let address: &str = "127.0.0.1:8000";
    // was listen bind successful for random TCP choice of local port?
    let listener:TcpListener = TcpListener::bind(address).expect("Failed to call TCP LISTEN");
    // is run(listener) an error?  await if so. else, return the server result
    run(listener)?.await
}
