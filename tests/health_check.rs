use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
// ! test.health_check.rs
use newsletter;
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}


async fn spawn_app() {
    let server = newsletter::run();

}

#[tokio::test]
async fn health_check_works() {
    spawn_app();
}
