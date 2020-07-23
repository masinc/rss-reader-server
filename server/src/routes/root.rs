use actix_web::{web, HttpResponse, Resource, Responder};

pub async fn get() -> impl Responder {
    HttpResponse::Ok().body("rss-reader-server")
}

pub async fn head() -> impl Responder {
    HttpResponse::Ok()
}

pub fn service() -> Resource {
    web::resource("/")
        .route(web::get().to(get))
        .route(web::head().to(head))
}
