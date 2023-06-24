use newsletter::run;
use std::net::TcpListener;
mod configuration;
mod routes;
pub use configuration::get_configuration;
#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    let configuration = get_configuration().expect("Failed to read App configuration.");
    let address= format!("127.0.0.1:{}",configuration.application_port);
    // was listen bind successful for random TCP choice of local port?
    let listener: TcpListener = TcpListener::bind(address).expect("Failed to call TCP LISTEN");
    // is run(listener) an error?  await if so. else, return the server result
    run(listener)?.await
}
