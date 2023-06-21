use actix_web::{
    dev::Server, get, post, web::{self}, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use std::net::TcpListener;

#[get("/")]
async fn hello() -> impl Responder {
    // without echo added, normal hello world response is generated
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    // With /echo in URL, "echo" is echoed into request bod
    let body: String = req_body.replace("echo", "");

    HttpResponse::Ok().body(body)
}
// /hey for manual_hello
async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
async fn secretfn() -> impl Responder {
    HttpResponse::Ok().body("Welcome to my secret page!")
}
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("All in good health!")
}
async fn subscribe() -> HttpResponse { 
    HttpResponse::Ok().finish()
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name: &str = req.match_info().get("name").unwrap_or("NONAME");
    format!("Hello {}!", &name)
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(hello) 
            .route("/health_check", web::get().to(health_check))
            .route("/hey", web::get().to(manual_hello))
            .route("/secret", web::get().to(secretfn))
            .route("/{name}", web::get().to(greet))
            .route("/subscriptions",web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
