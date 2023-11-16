use lambda_web::actix_web::web::{self, ServiceConfig};

mod controller;

pub fn router(cfg: &mut ServiceConfig) {
    cfg.route("/upload", web::post().to(controller::request_put_url));
}
