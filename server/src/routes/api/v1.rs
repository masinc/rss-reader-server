mod feed;

use actix_web::{web, HttpResponse, Resource, Responder};

const PATH: &str = "/api/v1";

async fn head() -> impl Responder {
    HttpResponse::Ok()
}

async fn get() -> impl Responder {
    HttpResponse::Ok().body("GET ".to_string() + PATH)
}

fn service() -> Resource {
    web::resource(PATH)
        .route(web::head().to(head))
        .route(web::get().to(get))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(service());
    feed::config(cfg);
}
