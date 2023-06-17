use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    // without echo added, normal hello world response is generated
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    // With /echo in URL, "echo" is echoed into request body
    HttpResponse::Ok().body(req_body)
}
// /hey for manual_hello
async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
async fn secretfn() -> impl Responder {
    HttpResponse::Ok().body("Welcome to my secret page!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .route("/secret", web::get().to(secretfn))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
