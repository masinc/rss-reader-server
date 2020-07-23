mod routes;

use actix_web::{App, HttpServer};
use std::io;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let app = || App::new().configure(routes::config);
    HttpServer::new(app).bind("127.0.0.1:8080")?.run().await
}
