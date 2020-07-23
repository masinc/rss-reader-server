use actix_web::{web, HttpResponse, Resource, Responder};

const PATH: &str = "/";

async fn get() -> impl Responder {
    HttpResponse::Ok().body("GET ".to_string() + PATH)
}

async fn head() -> impl Responder {
    HttpResponse::Ok()
}

fn service() -> Resource {
    web::resource(PATH)
        .route(web::get().to(get))
        .route(web::head().to(head))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(service());
}
