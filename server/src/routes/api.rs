use actix_web::{web, HttpResponse, Resource, Responder};

async fn get() -> impl Responder {
    HttpResponse::Ok().body("body")
}

async fn head() -> impl Responder {
    HttpResponse::Ok()
}

pub fn service() -> Resource {
    web::resource("/api")
        .route(web::get().to(get))
        .route(web::head().to(head))
}
