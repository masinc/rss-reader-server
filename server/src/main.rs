use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("rss-reader-server")
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
