mod api;
mod root;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    api::config(cfg);
    root::config(cfg);
}
