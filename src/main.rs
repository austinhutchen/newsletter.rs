use newsletter::run;

#[actix_web::main]
async fn main() {
    let addr = "127.0.0.1:8000";
    run(addr).await;
}
