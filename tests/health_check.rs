use actix_web::{web, App, HttpResponse, HttpServer};
// ! test.health_check.rs
use newsletter;
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn spawn_app() {
    let server = newsletter::run("127.0.0.1:8000");
}

#[tokio::test]
async fn health_check_works() {
    spawn_app();
}
