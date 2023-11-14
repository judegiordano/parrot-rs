use lambda_web::actix_web::web::{ServiceConfig, scope};
mod dev;

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(scope("/dev").configure(dev::router));
}
