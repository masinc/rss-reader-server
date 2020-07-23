mod api;
mod root;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(root::service()).service(api::service());
}
