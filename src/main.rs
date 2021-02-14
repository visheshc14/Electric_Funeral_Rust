extern crate env_logger;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use actix_web::middleware::Logger;

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .route("/", web::get().to(hello))
    })
    .bind("127.0.0.1:5000")?
    .run()
    .await
}