use lambda_web::actix_web::web::{scope, ServiceConfig};
mod outputs;
mod samples;
mod voices;

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(scope("/samples").configure(samples::router));
    cfg.service(scope("/voices").configure(voices::router));
    cfg.service(scope("/outputs").configure(outputs::router));
}
