//! src/routes/subscriptions.rs
use crate::web;
use crate::FormData;
use crate::HttpResponse;
use sqlx::PgConnection; // [...]

pub async fn subscribe(
    _form: web::Form<FormData>,
    // Retrieving a connection from the application state! _connection: web::Data<PgConnection>,
) -> HttpResponse {
    HttpResponse::Ok().finish()
}
