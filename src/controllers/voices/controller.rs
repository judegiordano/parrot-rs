use lambda_web::actix_web::HttpResponse;
use mongoose::Model;

use crate::{errors::ApiResponse, models::voice::Voice};

pub async fn list_voices() -> ApiResponse {
    let voices = Voice::list(None, None).await?;
    Ok(HttpResponse::Ok().json(voices))
}
