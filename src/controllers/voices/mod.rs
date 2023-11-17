use lambda_web::actix_web::web::{self, ServiceConfig};

mod controller;

pub fn router(cfg: &mut ServiceConfig) {
    cfg.route("", web::get().to(controller::list_voices));
    cfg.route("/{id}", web::get().to(controller::get_voice_by_id));
    cfg.route("/{id}", web::delete().to(controller::delete_voice));
}
