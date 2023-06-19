use std::net::TcpListener;

// ! test.health_check.rs
use newsletter;


fn spawn_app()->String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // let listener bind a random port
    let port = listener.local_addr().unwrap().port();
    let server = newsletter::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    spawn_app();
}
