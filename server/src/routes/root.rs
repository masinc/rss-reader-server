use actix_web::{web, HttpResponse, Resource, Responder};

async fn get() -> impl Responder {
    HttpResponse::Ok().body("rss-reader-server")
}

async fn head() -> impl Responder {
    HttpResponse::Ok()
}

fn service() -> Resource {
    web::resource("/")
        .route(web::get().to(get))
        .route(web::head().to(head))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(service());
}
