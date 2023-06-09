use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgConnection;
use std::net::TcpListener;
// ARC pointer will allow changes in application state and multiple app workers running simultaneously using actix-web::get
pub fn run(
    listener: TcpListener,
    // New parameter!
    connection: PgConnection,
) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check::health_check()))
            .route("/subscriptions", web::post().to(subscribe))
        // Register the connection as part of the application state .app_data(connection)
    })
    .listen(listener)?
    .run();
    Ok(server)
}