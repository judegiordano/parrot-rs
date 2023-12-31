use lambda_web::actix_web::web::{self, ServiceConfig};

mod controller;

pub fn router(cfg: &mut ServiceConfig) {
    cfg.route("", web::post().to(controller::create_output));
    cfg.route("/search", web::post().to(controller::search_outputs_text));
    cfg.route(
        "/{id}/presigned",
        web::get().to(controller::get_output_presigned),
    );
    cfg.route("/{id}", web::get().to(controller::get_output));
}
