use newsletter::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    let addr = "127.0.0.1:8000";
    run(addr).await?.await
}
