use newsletter::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    let address: &str = "http://127.0.0.1:";
    let listener:TcpListener = TcpListener::bind(address).expect("Failed to call TCP LISTEN");
    // is run(addr) an error?  await if so. else, return the server result
    run(listener)?.await
}
