use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

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
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("NONAME");
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .route("/secret", web::get().to(secretfn))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
