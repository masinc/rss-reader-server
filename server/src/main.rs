mod routes;

use actix_web::{App, HttpServer};
use listenfd::ListenFd;
use std::io;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let app = || App::new().configure(routes::config);
    let mut server = HttpServer::new(app);
    let mut listenfd = ListenFd::from_env();

    server = if let Some(l) = listenfd.take_tcp_listener(0)? {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:3000")?
    };

    server.run().await
}
