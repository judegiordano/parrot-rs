use lambda_web::actix_web::web::{self, ServiceConfig};

mod controller;

pub fn router(cfg: &mut ServiceConfig) {
    cfg.route("/ping", web::get().to(controller::ping));
    cfg.route("/records", web::post().to(controller::create_record));
    cfg.route("/records", web::get().to(controller::list_records));
    cfg.route("/records/{id}", web::get().to(controller::read_record));
}
