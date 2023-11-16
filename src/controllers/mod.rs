use lambda_web::actix_web::web::{scope, ServiceConfig};
mod samples;

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(scope("/samples").configure(samples::router));
}
