use std::time::Duration;

use lambda_web::actix_web::{web, HttpResponse};
use mongoose::{bson::doc, Model};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{aws::s3::Client, env, errors::ApiResponse, models::voice::Voice};

#[derive(Deserialize, Serialize)]
pub struct UploadSampleBody {
    pub voice_name: String,
    pub description: String,
}

pub async fn request_put_url(body: web::Json<UploadSampleBody>) -> ApiResponse {
    let config = env::Config::new()?;
    let s3 = Client::new(&config.samples_bucket_name).await;
    let voice = Voice {
        name: body.voice_name.to_string(),
        description: Some(body.description.to_string()),
        ..Default::default()
    }
    .save()
    .await?;
    let key = format!("{}.mp3", voice.id);
    let url = s3.put_presigned_url(&key, Duration::from_secs(60)).await?;
    Ok(HttpResponse::Ok().json(json!({ "url": url, "voice": voice })))
}
